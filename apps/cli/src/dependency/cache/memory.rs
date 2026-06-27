use std::collections::HashMap;

use crate::dependency::types::ResolutionResult as TypesResolutionResult;

use super::{CacheEntry, ResolutionCache};

impl ResolutionCache {
    pub(super) async fn insert_into_memory_cache(&self, key: String, entry: CacheEntry) {
        let mut memory_cache = self.memory_cache.write().await;

        if memory_cache.len() >= self.max_memory_entries {
            self.evict_lru_entries(&mut memory_cache, 1).await;
        }

        let current_usage = self.calculate_memory_usage(&memory_cache);
        if current_usage + entry.size_bytes > self.max_memory_size {
            let target_size = self.max_memory_size.saturating_sub(entry.size_bytes);
            self.evict_to_size(&mut memory_cache, target_size).await;
        }

        memory_cache.insert(key, entry);

        let mut stats = self.cache_statistics.write().await;
        stats.memory_usage = self.calculate_memory_usage(&memory_cache);
    }

    async fn evict_lru_entries(
        &self,
        memory_cache: &mut HashMap<String, CacheEntry>,
        count: usize,
    ) {
        if memory_cache.len() <= count {
            let evicted = memory_cache.len();
            memory_cache.clear();
            let mut stats = self.cache_statistics.write().await;
            stats.evictions += evicted as u64;
            stats.memory_usage = 0;
            return;
        }

        let mut entries: Vec<_> = memory_cache.iter().collect();
        entries.sort_unstable_by_key(|(_, entry)| entry.last_access);

        for key in entries
            .iter()
            .take(count)
            .map(|(key, _)| (*key).clone())
            .collect::<Vec<_>>()
        {
            memory_cache.remove(&key);
        }

        let mut stats = self.cache_statistics.write().await;
        stats.evictions += count as u64;
    }

    async fn evict_to_size(
        &self,
        memory_cache: &mut HashMap<String, CacheEntry>,
        target_size: usize,
    ) {
        let current_size = self.calculate_memory_usage(memory_cache);
        if current_size <= target_size {
            return;
        }

        let mut entries: Vec<_> = memory_cache.iter().collect();
        entries.sort_unstable_by_key(|(_, entry)| entry.last_access);

        let mut freed_size = 0;
        let mut keys_to_remove = Vec::new();

        for (key, entry) in entries {
            if current_size - freed_size <= target_size {
                break;
            }
            freed_size += entry.size_bytes;
            keys_to_remove.push((*key).clone());
        }

        let evicted = keys_to_remove.len();
        for key in keys_to_remove {
            memory_cache.remove(&key);
        }

        let mut stats = self.cache_statistics.write().await;
        stats.evictions += evicted as u64;
        stats.memory_usage = current_size - freed_size;
    }

    fn calculate_memory_usage(&self, memory_cache: &HashMap<String, CacheEntry>) -> usize {
        memory_cache.values().map(|entry| entry.size_bytes).sum()
    }

    pub(super) fn estimate_entry_size(&self, result: &TypesResolutionResult) -> usize {
        let base_size = std::mem::size_of::<CacheEntry>();
        let packages_size = result.resolved_packages.len() * 512;
        let order_size = result.dependency_order.len() * 64;
        base_size + packages_size + order_size
    }
}
