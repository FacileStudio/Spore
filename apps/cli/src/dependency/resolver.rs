mod analysis;
mod core;
mod graph;
mod selection;
mod utils;

use std::path::PathBuf;

use crate::dependency::cache::ResolutionCache;
use crate::dependency::registry::{LocalPackageRegistry, RemotePackageRegistry};
use crate::dependency::types::ResolutionContext;

pub struct DependencyResolver {
    context: ResolutionContext,
    local_registry: LocalPackageRegistry,
    remote_registry: RemotePackageRegistry,
    cache: ResolutionCache,
}

impl DependencyResolver {
    pub fn new(
        context: ResolutionContext,
        local_registry: LocalPackageRegistry,
        remote_registry: RemotePackageRegistry,
        cache_dir: PathBuf,
    ) -> Self {
        Self {
            context,
            local_registry,
            remote_registry,
            cache: ResolutionCache::new(cache_dir),
        }
    }

    pub fn get_local_registry(&self) -> &LocalPackageRegistry {
        &self.local_registry
    }

    pub fn get_remote_registry(&self) -> &RemotePackageRegistry {
        &self.remote_registry
    }
}
