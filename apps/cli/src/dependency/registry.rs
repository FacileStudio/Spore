mod local;
mod remote;

use async_trait::async_trait;
use semver::Version;
use std::path::Path;

use crate::dependency::error::ResolutionResult;
use crate::dependency::types::{PackageId, PackageMetadata, PackageVersion};

pub use local::LocalPackageRegistry;
pub use remote::RemotePackageRegistry;

#[async_trait]
pub trait PackageRegistry {
    async fn list_versions(&self, package_id: &PackageId) -> ResolutionResult<Vec<PackageVersion>>;
    #[allow(dead_code)]
    async fn get_package_metadata(
        &self,
        package_id: &PackageId,
        version: &Version,
    ) -> ResolutionResult<PackageMetadata>;
    #[allow(dead_code)]
    async fn download_package(
        &self,
        package_id: &PackageId,
        version: &Version,
        destination: &Path,
    ) -> ResolutionResult<()>;
    async fn search_packages(&self, query: &str) -> ResolutionResult<Vec<String>>;
}
