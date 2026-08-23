//! AURA MoE Active Expert LRU/LFU Cache Manager (VTM Layer)

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};

pub static EXPERT_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
pub static EXPERT_CACHE_MISSES: AtomicU64 = AtomicU64::new(0);
pub static EXPERT_EVICTIONS: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachePolicy {
    None,
    Lru,
    Lfu,
}

pub struct ExpertCacheManager {
    capacity_experts: usize,
    policy: CachePolicy,
    cached_experts: VecDeque<u32>,
    lfu_counts: HashMap<u32, u64>,
}

impl ExpertCacheManager {
    pub fn new(capacity_experts: usize, policy: CachePolicy) -> Self {
        Self {
            capacity_experts,
            policy,
            cached_experts: VecDeque::with_capacity(capacity_experts),
            lfu_counts: HashMap::new(),
        }
    }

    pub fn access_experts(&mut self, active_expert_ids: &[u32]) -> (usize, usize) {
        if self.policy == CachePolicy::None || self.capacity_experts == 0 {
            EXPERT_CACHE_MISSES.fetch_add(active_expert_ids.len() as u64, Ordering::Relaxed);
            return (0, active_expert_ids.len());
        }

        let mut hits = 0;
        let mut misses = 0;

        for &id in active_expert_ids {
            *self.lfu_counts.entry(id).or_insert(0) += 1;

            if let Some(pos) = self.cached_experts.iter().position(|&e| e == id) {
                hits += 1;
                EXPERT_CACHE_HITS.fetch_add(1, Ordering::Relaxed);
                if self.policy == CachePolicy::Lru {
                    self.cached_experts.remove(pos);
                    self.cached_experts.push_back(id);
                }
            } else {
                misses += 1;
                EXPERT_CACHE_MISSES.fetch_add(1, Ordering::Relaxed);
                if self.cached_experts.len() >= self.capacity_experts {
                    EXPERT_EVICTIONS.fetch_add(1, Ordering::Relaxed);
                    if self.policy == CachePolicy::Lru {
                        self.cached_experts.pop_front();
                    } else if self.policy == CachePolicy::Lfu {
                        // Evict least frequently used
                        if let Some((&min_id, _)) = self.lfu_counts.iter().min_by_key(|entry| entry.1) {
                            if let Some(pos) = self.cached_experts.iter().position(|&e| e == min_id) {
                                self.cached_experts.remove(pos);
                            } else {
                                self.cached_experts.pop_front();
                            }
                        }
                    }
                }
                self.cached_experts.push_back(id);
            }
        }

        (hits, misses)
    }

    pub fn hit_rate(&self) -> f64 {
        let hits = EXPERT_CACHE_HITS.load(Ordering::Relaxed) as f64;
        let misses = EXPERT_CACHE_MISSES.load(Ordering::Relaxed) as f64;
        let total = hits + misses;
        if total == 0.0 {
            0.0
        } else {
            hits / total
        }
    }
}
