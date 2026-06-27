mod memory;
mod ops;
mod storage;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

use crate::dependency::types::ResolutionResult as TypesResolutionResult;

#[derive(Debug)]
pub struct ResolutionCache {
    memory_cache: RwLock<HashMap<String, CacheEntry>>,
    disk_cache_path: PathBuf,
    cache_ttl: Duration,
    max_memory_entries: usize,
    max_memory_size: usize,
    cache_statistics: RwLock<CacheStatistics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    result: TypesResolutionResult,
    timestamp: SystemTime,
    request_hash: String,
    access_count: u64,
    last_access: SystemTime,
    size_bytes: usize,
}

#[derive(Debug, Clone)]
struct CacheStatistics {
    hits: u64,
    misses: u64,
    evictions: u64,
    total_requests: u64,
    memory_usage: usize,
}

impl ResolutionCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            memory_cache: RwLock::new(HashMap::with_capacity(128)),
            disk_cache_path: cache_dir.join("resolutions"),
            cache_ttl: Duration::from_secs(3600),
            max_memory_entries: 1000,
            max_memory_size: 100 * 1024 * 1024,
            cache_statistics: RwLock::new(CacheStatistics {
                hits: 0,
                misses: 0,
                evictions: 0,
                total_requests: 0,
                memory_usage: 0,
            }),
        }
    }
}
