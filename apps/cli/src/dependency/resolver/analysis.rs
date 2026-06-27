use crate::dependency::error::ResolutionResult;
use crate::dependency::types::{PackageId, PackageVersion};

use super::DependencyResolver;

impl DependencyResolver {
    pub async fn get_package_info(
        &self,
        package_id: &PackageId,
    ) -> ResolutionResult<Vec<PackageVersion>> {
        self.discover_package_versions(package_id).await
    }
}
