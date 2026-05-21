//! KV cache optimization for efficient inference

use std::collections::VecDeque;

/// Compressed KV cache with selective eviction
pub struct KVCache {
    max_size: usize,
    cache: VecDeque<(Vec<f32>, Vec<f32>)>, // (key, value) pairs
}

impl KVCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            cache: VecDeque::with_capacity(max_size),
        }
    }

    /// Add key-value pair to cache
    pub fn add(&mut self, key: Vec<f32>, value: Vec<f32>) {
        if self.cache.len() >= self.max_size {
            // LRU eviction
            self.cache.pop_front();
        }
        self.cache.push_back((key, value));
    }

    /// Retrieve cached value by key
    pub fn get(&self, key: &[f32]) -> Option<&Vec<f32>> {
        self.cache
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_cache() {
        let mut cache = KVCache::new(2);
        cache.add(vec![1.0, 2.0], vec![3.0, 4.0]);
        assert_eq!(cache.size(), 1);
        assert_eq!(cache.get(&vec![1.0, 2.0]), Some(&vec![3.0, 4.0]));
    }
}
