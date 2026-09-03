//! Three-Tier MoE Expert Hierarchy & Dynamic Prefetch Router
//!
//! Synthesizes FreeToken's global LRU expert cache with Kimi-k3's static chunk routing.

use crate::errors::Result;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpertTier {
    Tier1VramHot,
    Tier2HostRamWarm,
    Tier3DiskCold,
}

#[derive(Debug)]
pub struct MoeExpertMetadata {
    pub expert_id: usize,
    pub layer_id: usize,
    pub size_bytes: usize,
    pub tier: ExpertTier,
    pub last_accessed_step: u64,
}

#[derive(Debug)]
pub struct ThreeTierExpertCache {
    vram_capacity_bytes: usize,
    vram_used_bytes: usize,
    ram_capacity_bytes: usize,
    ram_used_bytes: usize,
    experts: HashMap<usize, MoeExpertMetadata>,
    vram_lru: VecDeque<usize>,
    ram_lru: VecDeque<usize>,
    step_counter: AtomicU64,
    pub cache_hits_vram: AtomicU64,
    pub cache_hits_ram: AtomicU64,
    pub cache_misses_disk: AtomicU64,
}

impl ThreeTierExpertCache {
    pub fn new(vram_capacity_bytes: usize, ram_capacity_bytes: usize) -> Self {
        Self {
            vram_capacity_bytes,
            vram_used_bytes: 0,
            ram_capacity_bytes,
            ram_used_bytes: 0,
            experts: HashMap::new(),
            vram_lru: VecDeque::new(),
            ram_lru: VecDeque::new(),
            step_counter: AtomicU64::new(0),
            cache_hits_vram: AtomicU64::new(0),
            cache_hits_ram: AtomicU64::new(0),
            cache_misses_disk: AtomicU64::new(0),
        }
    }

    /// Access an expert for layer computation, migrating between tiers via LRU policy
    pub fn access_expert(
        &mut self,
        expert_id: usize,
        layer_id: usize,
        expert_size_bytes: usize,
    ) -> Result<ExpertTier> {
        let step = self.step_counter.fetch_add(1, Ordering::Relaxed);

        if let Some(exp) = self.experts.get_mut(&expert_id) {
            exp.last_accessed_step = step;
            match exp.tier {
                ExpertTier::Tier1VramHot => {
                    self.cache_hits_vram.fetch_add(1, Ordering::Relaxed);
                    // Refresh LRU
                    self.vram_lru.retain(|&id| id != expert_id);
                    self.vram_lru.push_back(expert_id);
                    return Ok(ExpertTier::Tier1VramHot);
                }
                ExpertTier::Tier2HostRamWarm => {
                    self.cache_hits_ram.fetch_add(1, Ordering::Relaxed);
                    // Promote to VRAM if space or after eviction
                    self.promote_to_vram(expert_id, expert_size_bytes);
                    return Ok(ExpertTier::Tier2HostRamWarm);
                }
                ExpertTier::Tier3DiskCold => {
                    self.cache_misses_disk.fetch_add(1, Ordering::Relaxed);
                    self.stage_from_disk(expert_id, layer_id, expert_size_bytes);
                    return Ok(ExpertTier::Tier3DiskCold);
                }
            }
        }

        // Unseen expert -> Disk cold access
        self.cache_misses_disk.fetch_add(1, Ordering::Relaxed);
        self.stage_from_disk(expert_id, layer_id, expert_size_bytes);
        Ok(ExpertTier::Tier3DiskCold)
    }

    fn promote_to_vram(&mut self, expert_id: usize, size_bytes: usize) {
        while self.vram_used_bytes + size_bytes > self.vram_capacity_bytes
            && !self.vram_lru.is_empty()
        {
            if let Some(evicted_id) = self.vram_lru.pop_front() {
                if let Some(evicted) = self.experts.get_mut(&evicted_id) {
                    evicted.tier = ExpertTier::Tier2HostRamWarm;
                    self.vram_used_bytes = self.vram_used_bytes.saturating_sub(evicted.size_bytes);
                    self.ram_lru.push_back(evicted_id);
                    self.ram_used_bytes += evicted.size_bytes;
                }
            }
        }

        if self.vram_used_bytes + size_bytes <= self.vram_capacity_bytes {
            if let Some(exp) = self.experts.get_mut(&expert_id) {
                exp.tier = ExpertTier::Tier1VramHot;
                self.vram_used_bytes += size_bytes;
                self.ram_used_bytes = self.ram_used_bytes.saturating_sub(size_bytes);
                self.ram_lru.retain(|&id| id != expert_id);
                self.vram_lru.push_back(expert_id);
            }
        }
    }

    fn stage_from_disk(&mut self, expert_id: usize, layer_id: usize, size_bytes: usize) {
        let step = self.step_counter.load(Ordering::Relaxed);

        // Evict from RAM if necessary
        while self.ram_used_bytes + size_bytes > self.ram_capacity_bytes && !self.ram_lru.is_empty()
        {
            if let Some(evicted_id) = self.ram_lru.pop_front() {
                if let Some(evicted) = self.experts.get_mut(&evicted_id) {
                    evicted.tier = ExpertTier::Tier3DiskCold;
                    self.ram_used_bytes = self.ram_used_bytes.saturating_sub(evicted.size_bytes);
                }
            }
        }

        let metadata = MoeExpertMetadata {
            expert_id,
            layer_id,
            size_bytes,
            tier: ExpertTier::Tier2HostRamWarm,
            last_accessed_step: step,
        };

        self.experts.insert(expert_id, metadata);
        self.ram_lru.push_back(expert_id);
        self.ram_used_bytes += size_bytes;
    }

    pub fn stats(&self) -> (u64, u64, u64) {
        (
            self.cache_hits_vram.load(Ordering::Relaxed),
            self.cache_hits_ram.load(Ordering::Relaxed),
            self.cache_misses_disk.load(Ordering::Relaxed),
        )
    }
}
