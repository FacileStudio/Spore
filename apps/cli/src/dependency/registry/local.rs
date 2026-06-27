use async_trait::async_trait;
use semver::Version;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::config::PackageConfig;
use crate::dependency::error::{ResolutionError, ResolutionResult};
use crate::dependency::types::{
    DependencySpec, PackageId, PackageMetadata, PackageSource, PackageVersion,
};
use crate::dependency::DependencyRef;

use super::PackageRegistry;

pub struct LocalPackageRegistry {
    packages_path: PathBuf,
    cache: HashMap<PackageId, Vec<PackageVersion>>,
}

impl LocalPackageRegistry {
    pub fn new(packages_path: PathBuf) -> Self {
        Self {
            packages_path,
            cache: HashMap::new(),
        }
    }

    pub async fn discover_packages(&mut self) -> ResolutionResult<()> {
        self.cache.clear();

        if !self.packages_path.exists() {
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.packages_path).map_err(|e| {
            ResolutionError::io_error(
                "reading packages directory",
                Some(self.packages_path.to_string_lossy().into_owned()),
                e.to_string(),
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                ResolutionError::io_error(
                    "reading directory entry",
                    Some(self.packages_path.to_string_lossy().into_owned()),
                    e.to_string(),
                )
            })?;

            if entry.path().is_dir() {
                if let Ok(package_version) = self.load_local_package(&entry.path()).await {
                    let package_id = package_version.id.clone();
                    self.cache
                        .entry(package_id)
                        .or_default()
                        .push(package_version);
                }
            }
        }

        Ok(())
    }

    async fn load_local_package(&self, package_path: &Path) -> ResolutionResult<PackageVersion> {
        let package_yml = package_path.join("package.yml");

        if !package_yml.exists() {
            return Err(ResolutionError::configuration_error(
                format!("package.yml not found in {}", package_path.display()),
                None,
            ));
        }

        let config = PackageConfig::from_file(&package_yml).map_err(|e| {
            ResolutionError::configuration_error(
                format!("Failed to load package configuration: {}", e),
                Some("package.yml".to_string()),
            )
        })?;

        let package_id = PackageId {
            name: config.name.clone(),
            source: PackageSource::Local,
        };

        let version = Version::parse(&config.version).map_err(|e| {
            ResolutionError::invalid_version(
                package_id.clone(),
                config.version.clone(),
                e.to_string(),
            )
        })?;

        let dependencies =
            self.parse_dependencies(&config.dependencies.unwrap_or_default(), false, false)?;
        let dev_dependencies =
            self.parse_dependencies(&config.dev_dependencies.unwrap_or_default(), true, false)?;
        let optional_dependencies = self.parse_dependencies(
            &config.optional_dependencies.unwrap_or_default(),
            false,
            true,
        )?;
        let peer_dependencies =
            self.parse_dependencies(&config.peer_dependencies.unwrap_or_default(), false, false)?;

        let metadata = Some(PackageMetadata {
            name: config.name.clone(),
            version: config.version.clone(),
            description: config.description.clone(),
            author: config.author.clone(),
            license: config.license.clone(),
            repository: config.repository.clone(),
            keywords: config.keywords.clone(),
            exports: config.exports.clone(),
            features: config.features.clone(),
            checksum: None,
        });

        Ok(PackageVersion {
            id: package_id,
            version,
            dependencies,
            dev_dependencies,
            optional_dependencies,
            peer_dependencies,
            source_path: Some(package_path.to_path_buf()),
            metadata,
        })
    }

    fn parse_dependencies(
        &self,
        deps: &[String],
        is_dev: bool,
        is_optional: bool,
    ) -> ResolutionResult<Vec<DependencySpec>> {
        let mut dependency_specs = Vec::new();

        for dep_str in deps {
            let dependency_spec = DependencyRef::parse(dep_str)
                .and_then(|dep| dep.to_dependency_spec(is_dev, is_optional))
                .map_err(|e| {
                    ResolutionError::configuration_error(
                        e.to_string(),
                        Some("dependency".to_string()),
                    )
                })?;
            dependency_specs.push(dependency_spec);
        }

        Ok(dependency_specs)
    }
}

#[async_trait]
impl PackageRegistry for LocalPackageRegistry {
    async fn list_versions(&self, package_id: &PackageId) -> ResolutionResult<Vec<PackageVersion>> {
        Ok(self.cache.get(package_id).cloned().unwrap_or_default())
    }

    async fn get_package_metadata(
        &self,
        package_id: &PackageId,
        version: &Version,
    ) -> ResolutionResult<PackageMetadata> {
        let versions = self.list_versions(package_id).await?;
        let package_version = versions
            .iter()
            .find(|v| v.version == *version)
            .ok_or_else(|| {
                ResolutionError::package_not_found(
                    package_id.clone(),
                    vec![format!(
                        "Local packages directory: {}",
                        self.packages_path.display()
                    )],
                    vec![],
                )
            })?;

        package_version.metadata.clone().ok_or_else(|| {
            ResolutionError::configuration_error("Package metadata not available", None)
        })
    }

    async fn download_package(
        &self,
        package_id: &PackageId,
        _version: &Version,
        destination: &Path,
    ) -> ResolutionResult<()> {
        let versions = self.list_versions(package_id).await?;
        let package_version = versions.first().ok_or_else(|| {
            ResolutionError::package_not_found(
                package_id.clone(),
                vec![format!(
                    "Local packages directory: {}",
                    self.packages_path.display()
                )],
                vec![],
            )
        })?;

        let source_path = package_version.source_path.as_ref().ok_or_else(|| {
            ResolutionError::configuration_error("No source path for local package", None)
        })?;

        if destination.exists() {
            std::fs::remove_dir_all(destination)?;
        }

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(source_path, destination).map_err(|e| {
                ResolutionError::io_error(
                    "creating symlink",
                    Some(destination.to_string_lossy().into_owned()),
                    e.to_string(),
                )
            })?;
        }

        #[cfg(not(unix))]
        {
            copy_dir_recursive(source_path, destination)?;
        }

        Ok(())
    }

    async fn search_packages(&self, query: &str) -> ResolutionResult<Vec<String>> {
        let mut matches = Vec::new();

        for package_id in self.cache.keys() {
            if package_id.name.contains(query) {
                matches.push(package_id.name.clone());
            }
        }

        matches.sort();
        Ok(matches)
    }
}

#[cfg(not(unix))]
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    use std::fs;

    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
