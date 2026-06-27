use sha2::{Digest, Sha256};
use std::time::Duration;

use crate::dependency::error::ResolutionResult;
use crate::dependency::types::{ResolutionRequest, ResolutionResult as TypesResolutionResult};

use super::ResolutionCache;

impl ResolutionCache {
    pub async fn get_cached_resolution(
        &self,
        request: &ResolutionRequest,
    ) -> Option<TypesResolutionResult> {
        let request_hash = self.hash_request(request);

        {
            let mut stats = self.cache_statistics.write().await;
            stats.total_requests += 1;
        }

        {
            let mut memory_cache = self.memory_cache.write().await;
            if let Some(entry) = memory_cache.get_mut(&request_hash) {
                if !self.is_cache_expired(&entry.timestamp) {
                    entry.access_count += 1;
                    entry.last_access = std::time::SystemTime::now();

                    let mut stats = self.cache_statistics.write().await;
                    stats.hits += 1;
                    return Some(entry.result.clone());
                }

                memory_cache.remove(&request_hash);
            }
        }

        if let Some(mut entry) = self.load_from_disk_cache(&request_hash).await {
            if !self.is_cache_expired(&entry.timestamp) {
                entry.access_count += 1;
                entry.last_access = std::time::SystemTime::now();

                let result = entry.result.clone();
                self.insert_into_memory_cache(request_hash, entry).await;

                let mut stats = self.cache_statistics.write().await;
                stats.hits += 1;
                return Some(result);
            }
        }

        {
            let mut stats = self.cache_statistics.write().await;
            stats.misses += 1;
        }

        None
    }

    pub async fn cache_resolution(
        &self,
        request: &ResolutionRequest,
        result: &TypesResolutionResult,
    ) -> ResolutionResult<()> {
        let request_hash = self.hash_request(request);
        let size_bytes = self.estimate_entry_size(result);

        let entry = super::CacheEntry {
            result: result.clone(),
            timestamp: std::time::SystemTime::now(),
            request_hash: request_hash.clone(),
            access_count: 1,
            last_access: std::time::SystemTime::now(),
            size_bytes,
        };

        self.insert_into_memory_cache(request_hash.clone(), entry.clone())
            .await;
        self.save_to_disk_cache(&request_hash, &entry).await?;
        Ok(())
    }

    fn hash_request(&self, request: &ResolutionRequest) -> String {
        let mut hasher = Sha256::new();

        for dep in &request.dependencies {
            hasher.update(dep.id.name.as_bytes());
            hasher.update(dep.version_req.to_string().as_bytes());
            hasher.update([dep.optional as u8, dep.dev_only as u8]);

            if let Some(conditions) = &dep.conditions {
                if let Some(platform) = &conditions.platform {
                    for p in platform {
                        hasher.update(p.as_bytes());
                    }
                }
                if let Some(arch) = &conditions.arch {
                    for a in arch {
                        hasher.update(a.as_bytes());
                    }
                }
                if let Some(env) = &conditions.env {
                    for e in env {
                        hasher.update(e.as_bytes());
                    }
                }
            }
        }

        hasher.update(format!("{:?}", request.context.strategy).as_bytes());
        hasher.update([
            request.context.allow_prerelease as u8,
            request.context.include_dev as u8,
            request.context.include_optional as u8,
        ]);
        hasher.update(request.context.max_depth.to_string().as_bytes());

        if let Some(platform) = &request.context.platform {
            hasher.update(platform.as_bytes());
        }
        if let Some(arch) = &request.context.arch {
            hasher.update(arch.as_bytes());
        }
        if let Some(env) = &request.context.environment {
            hasher.update(env.as_bytes());
        }

        for (package_id, version) in &request.overrides {
            hasher.update(package_id.name.as_bytes());
            hasher.update(version.to_string().as_bytes());
        }

        for package_id in &request.excludes {
            hasher.update(package_id.name.as_bytes());
        }

        format!("{:x}", hasher.finalize())
    }

    fn is_cache_expired(&self, timestamp: &std::time::SystemTime) -> bool {
        timestamp.elapsed().unwrap_or(Duration::MAX) > self.cache_ttl
    }
}
