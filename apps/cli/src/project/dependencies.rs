use crate::dependency::registry::{LocalPackageRegistry, PackageRegistry, RemotePackageRegistry};
use crate::dependency::{
    DependencyRef, DependencyResolver, DependencySpec, ResolutionContext, ResolutionStrategy,
};
use crate::downloader::get_spore_space_url;
use anyhow::{Context, Result};
use console::style;

use super::Project;

impl Project {
    pub async fn resolve_dependencies(
        &mut self,
        app_name: &str,
        include_dev: bool,
        _strategy: Option<ResolutionStrategy>,
    ) -> Result<crate::dependency::types::ResolutionResult> {
        let deps = self.get_app_dependency_specs(app_name, include_dev)?;
        let resolver = self.get_or_create_resolver().await?;

        println!(
            "🔍 Resolving {} dependencies for app '{}'...",
            deps.len(),
            style(app_name).green()
        );

        let result = resolver
            .resolve_dependencies(deps)
            .await
            .with_context(|| format!("Failed to resolve dependencies for app '{}'", app_name))?;

        println!(
            "✅ Successfully resolved {} packages",
            result.resolved_packages.len()
        );

        if !result.conflicts.is_empty() {
            println!(
                "⚠️  {} conflicts were automatically resolved",
                result.conflicts.len()
            );
        }

        if !result.warnings.is_empty() {
            for warning in &result.warnings {
                println!("⚠️  {}", warning);
            }
        }

        Ok(result)
    }

    pub async fn install_dependencies(
        &mut self,
        app_name: &str,
        resolution: &crate::dependency::types::ResolutionResult,
        use_symlinks: bool,
    ) -> Result<()> {
        let app_packages_dir = self.root.join("apps").join(app_name).join("spore_packages");

        if app_packages_dir.exists() {
            std::fs::remove_dir_all(&app_packages_dir).with_context(|| {
                format!("Failed to clean packages directory for app '{}'", app_name)
            })?;
        }
        std::fs::create_dir_all(&app_packages_dir).with_context(|| {
            format!("Failed to create packages directory for app '{}'", app_name)
        })?;

        println!(
            "📦 Installing {} packages for app '{}'...",
            resolution.resolved_packages.len(),
            style(app_name).green()
        );

        let resolver = self.get_or_create_resolver().await?;

        for package_id in &resolution.dependency_order {
            if let Some(package_version) = resolution.resolved_packages.get(package_id) {
                let package_dir = app_packages_dir.join(
                    crate::dependency::spec::install_subpath_for_name(&package_id.name),
                );

                print!(
                    "  📦 Installing {}@{}... ",
                    style(&package_id.name).cyan(),
                    style(&package_version.version).yellow()
                );

                match &package_id.source {
                    crate::dependency::types::PackageSource::Local => {
                        if let Some(source_path) = &package_version.source_path {
                            if use_symlinks {
                                Self::create_package_link_static(source_path, &package_dir)
                                    .with_context(|| {
                                        format!(
                                            "Failed to link local package '{}'",
                                            package_id.name
                                        )
                                    })?;
                            } else {
                                Self::copy_dir_recursive_static(source_path, &package_dir)
                                    .with_context(|| {
                                        format!(
                                            "Failed to copy local package '{}'",
                                            package_id.name
                                        )
                                    })?;
                            }
                        }
                    }
                    crate::dependency::types::PackageSource::Remote { .. } => {
                        let download_result = {
                            let local_result = resolver
                                .get_local_registry()
                                .download_package(
                                    package_id,
                                    &package_version.version,
                                    &package_dir,
                                )
                                .await;
                            match local_result {
                                Ok(_) => Ok(()),
                                Err(_) => {
                                    resolver
                                        .get_remote_registry()
                                        .download_package(
                                            package_id,
                                            &package_version.version,
                                            &package_dir,
                                        )
                                        .await
                                }
                            }
                        };
                        download_result.with_context(|| {
                            format!("Failed to download package '{}'", package_id.name)
                        })?;
                    }
                }

                println!("✅");
            }
        }

        println!(
            "🎉 Successfully installed all dependencies for app '{}'",
            style(app_name).green()
        );
        Ok(())
    }

    pub fn get_app_dependency_specs(
        &self,
        app_name: &str,
        _include_dev: bool,
    ) -> Result<Vec<DependencySpec>> {
        let raw_deps = self.get_app_dependencies(app_name);
        let mut dep_specs = Vec::new();

        for dep_str in raw_deps {
            dep_specs.push(DependencyRef::parse(&dep_str)?.to_dependency_spec(false, false)?);
        }

        Ok(dep_specs)
    }

    pub fn validate_dependency_tree(&self, app_name: &str) -> Result<Vec<String>> {
        let mut issues = Vec::new();
        let app_deps = self.get_app_dependencies(app_name);
        let mut seen = std::collections::HashSet::new();

        for dep in &app_deps {
            let dep_name = DependencyRef::parse(dep)?.name;
            if !seen.insert(dep_name.clone()) {
                issues.push(format!("Duplicate dependency: '{}'", dep_name));
            }
        }

        for dep in &app_deps {
            let dep_ref = DependencyRef::parse(dep)?;
            if matches!(
                dep_ref.source,
                crate::dependency::types::PackageSource::Local
            ) {
                let dep_name = dep_ref.name;
                if !self.packages.contains_key(&dep_name) {
                    issues.push(format!(
                        "Local package '{}' not found in packages directory",
                        dep_name
                    ));
                }
            }
        }

        Ok(issues)
    }

    async fn get_or_create_resolver(&mut self) -> Result<&mut DependencyResolver> {
        if self.dependency_resolver.is_none() {
            let context = ResolutionContext {
                strategy: ResolutionStrategy::Compatible,
                allow_prerelease: false,
                max_depth: 50,
                include_dev: false,
                include_optional: true,
                platform: Some(std::env::consts::OS.to_string()),
                arch: Some(std::env::consts::ARCH.to_string()),
                environment: Some("production".to_string()),
            };

            let mut local_registry = LocalPackageRegistry::new(self.root.join("packages"));
            local_registry
                .discover_packages()
                .await
                .context("Failed to discover local packages")?;

            let remote_registry = RemotePackageRegistry::new(get_spore_space_url())
                .context("Failed to create remote package registry")?;
            let cache_dir = self.root.join(".spore").join("cache");

            self.dependency_resolver = Some(DependencyResolver::new(
                context,
                local_registry,
                remote_registry,
                cache_dir,
            ));
        }

        Ok(self.dependency_resolver.as_mut().unwrap())
    }
}
