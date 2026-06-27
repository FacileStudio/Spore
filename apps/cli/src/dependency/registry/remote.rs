use async_trait::async_trait;
use semver::Version;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::RwLock;

use crate::dependency::error::{ResolutionError, ResolutionResult};
use crate::dependency::types::{DependencySpec, PackageId, PackageMetadata, PackageVersion};
use crate::dependency::DependencyRef;
use crate::downloader::PackageDownloader;
use crate::utils::create_http_client;

use super::PackageRegistry;

#[derive(Deserialize)]
struct RemoteDependency {
    name: String,
    version: String,
    optional: Option<bool>,
}

pub struct RemotePackageRegistry {
    base_url: String,
    client: reqwest::Client,
    cache: RwLock<HashMap<PackageId, Vec<PackageVersion>>>,
    auth_token: Option<String>,
}

impl RemotePackageRegistry {
    pub fn new(base_url: String) -> anyhow::Result<Self> {
        Ok(Self {
            base_url,
            client: create_http_client()?,
            cache: RwLock::new(HashMap::new()),
            auth_token: None,
        })
    }

    fn build_request(&self, url: &str) -> reqwest::RequestBuilder {
        let mut request = self.client.get(url);

        if let Some(token) = &self.auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        request
    }

    fn parse_remote_dependencies(
        &self,
        deps: &[RemoteDependency],
        is_dev: bool,
    ) -> ResolutionResult<Vec<DependencySpec>> {
        let mut dependency_specs = Vec::new();

        for dep in deps {
            let dependency_spec =
                DependencyRef::from_name_and_version(&dep.name, Some(&dep.version))
                    .and_then(|dep_ref| {
                        dep_ref.to_dependency_spec(is_dev, dep.optional.unwrap_or(false))
                    })
                    .map_err(|e| {
                        ResolutionError::configuration_error(
                            e.to_string(),
                            Some("remote dependency".to_string()),
                        )
                    })?;
            dependency_specs.push(dependency_spec);
        }

        Ok(dependency_specs)
    }
}

#[async_trait]
impl PackageRegistry for RemotePackageRegistry {
    async fn list_versions(&self, package_id: &PackageId) -> ResolutionResult<Vec<PackageVersion>> {
        if let Some(cached) = self.cache.read().await.get(package_id).cloned() {
            return Ok(cached);
        }

        let package_name = package_id.name.trim_start_matches('@');
        let url = format!("{}/api/packages/{}/versions", self.base_url, package_name);

        let response = self
            .build_request(&url)
            .send()
            .await
            .map_err(|e| ResolutionError::network_error(package_id.clone(), e.to_string()))?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        #[derive(Deserialize)]
        struct VersionsResponse {
            data: Vec<RemotePackageVersion>,
        }

        #[derive(Deserialize)]
        struct RemotePackageVersion {
            version: String,
            dependencies: Option<Vec<RemoteDependency>>,
            dev_dependencies: Option<Vec<RemoteDependency>>,
            metadata: Option<serde_json::Value>,
        }

        let versions_data: VersionsResponse = response.json().await.map_err(|e| {
            ResolutionError::network_error(
                package_id.clone(),
                format!("Failed to parse response: {}", e),
            )
        })?;

        let mut versions = Vec::new();
        for version_data in versions_data.data {
            let version = Version::parse(&version_data.version).map_err(|e| {
                ResolutionError::invalid_version(
                    package_id.clone(),
                    version_data.version,
                    e.to_string(),
                )
            })?;

            let dependencies = self
                .parse_remote_dependencies(&version_data.dependencies.unwrap_or_default(), false)?;
            let dev_dependencies = self.parse_remote_dependencies(
                &version_data.dev_dependencies.unwrap_or_default(),
                true,
            )?;

            versions.push(PackageVersion {
                id: package_id.clone(),
                version,
                dependencies,
                dev_dependencies,
                optional_dependencies: Vec::new(),
                peer_dependencies: Vec::new(),
                source_path: None,
                metadata: version_data
                    .metadata
                    .and_then(|m| serde_json::from_value(m).ok()),
            });
        }

        self.cache
            .write()
            .await
            .insert(package_id.clone(), versions.clone());
        Ok(versions)
    }

    async fn get_package_metadata(
        &self,
        package_id: &PackageId,
        version: &Version,
    ) -> ResolutionResult<PackageMetadata> {
        let package_name = package_id.name.trim_start_matches('@');
        let url = format!(
            "{}/api/packages/{}/{}",
            self.base_url, package_name, version
        );

        let response = self
            .build_request(&url)
            .send()
            .await
            .map_err(|e| ResolutionError::network_error(package_id.clone(), e.to_string()))?;

        if !response.status().is_success() {
            return Err(ResolutionError::package_not_found(
                package_id.clone(),
                vec![format!("Remote registry: {}", self.base_url)],
                vec![],
            ));
        }

        response.json().await.map_err(|e| {
            ResolutionError::network_error(
                package_id.clone(),
                format!("Failed to parse metadata: {}", e),
            )
        })
    }

    async fn download_package(
        &self,
        package_id: &PackageId,
        version: &Version,
        destination: &Path,
    ) -> ResolutionResult<()> {
        let package_spec = format!("{}@{}", package_id.name, version);
        PackageDownloader::download_package(&package_spec, destination)
            .await
            .map_err(|e| ResolutionError::network_error(package_id.clone(), e.to_string()))
    }

    async fn search_packages(&self, query: &str) -> ResolutionResult<Vec<String>> {
        let url = format!(
            "{}/api/search?q={}",
            self.base_url,
            urlencoding::encode(query)
        );

        let response = self.build_request(&url).send().await.map_err(|e| {
            ResolutionError::network_error(PackageId::local("search"), e.to_string())
        })?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        #[derive(Deserialize)]
        struct SearchResponse {
            packages: Vec<SearchResult>,
        }

        #[derive(Deserialize)]
        struct SearchResult {
            name: String,
        }

        let search_data: SearchResponse = response.json().await.map_err(|e| {
            ResolutionError::network_error(
                PackageId::local("search"),
                format!("Failed to parse search response: {}", e),
            )
        })?;

        Ok(search_data.packages.into_iter().map(|p| p.name).collect())
    }
}
