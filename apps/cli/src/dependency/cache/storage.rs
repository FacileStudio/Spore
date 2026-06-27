use crate::dependency::error::{ResolutionError, ResolutionResult};

use super::{CacheEntry, ResolutionCache};

impl ResolutionCache {
    pub(super) async fn load_from_disk_cache(&self, request_hash: &str) -> Option<CacheEntry> {
        let cache_file = self.disk_cache_path.join(format!("{}.json", request_hash));

        if !cache_file.exists() {
            return None;
        }

        let content = tokio::fs::read_to_string(&cache_file).await.ok()?;
        serde_json::from_str::<CacheEntry>(&content).ok()
    }

    pub(super) async fn save_to_disk_cache(
        &self,
        request_hash: &str,
        entry: &CacheEntry,
    ) -> ResolutionResult<()> {
        tokio::fs::create_dir_all(&self.disk_cache_path)
            .await
            .map_err(|e| ResolutionError::cache_error("creating cache directory", e.to_string()))?;

        let cache_file = self.disk_cache_path.join(format!("{}.json", request_hash));
        let content = serde_json::to_string_pretty(entry)
            .map_err(|e| ResolutionError::cache_error("serializing cache entry", e.to_string()))?;

        tokio::fs::write(&cache_file, content)
            .await
            .map_err(|e| ResolutionError::cache_error("writing cache file", e.to_string()))?;

        Ok(())
    }
}
