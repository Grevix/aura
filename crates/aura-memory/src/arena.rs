//! Monolithic Static Memory Arena (`AuraArena`)
//!
//! Inspired by Kimi-k3-in-c's `k3_arena` zero-allocation architecture,
//! ensuring zero `malloc` or page-fault jitter during model execution loops.

use aura_core::errors::{AuraError, Result};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct AuraArena {
    storage: Vec<u8>,
    capacity_bytes: usize,
    trunk_offset: usize,
    trunk_size: usize,
    layer_slot_0_offset: usize,
    layer_slot_1_offset: usize,
    layer_slot_size: usize,
    kv_ring_offset: usize,
    kv_ring_size: usize,
    allocated_high_watermark: AtomicUsize,
}

impl AuraArena {
    /// Creates a monolithic static arena with dedicated zero-fragmentation partitions
    pub fn new(
        total_capacity_bytes: usize,
        trunk_size: usize,
        layer_slot_size: usize,
        kv_ring_size: usize,
    ) -> Result<Self> {
        let required = trunk_size + (2 * layer_slot_size) + kv_ring_size;
        if required > total_capacity_bytes {
            return Err(AuraError::MemoryError(format!(
                "Requested arena partitions ({} bytes) exceed total capacity ({} bytes)",
                required, total_capacity_bytes
            )));
        }

        // Pre-allocate and zero-initialize monolithic arena buffer
        let storage = vec![0u8; total_capacity_bytes];

        let trunk_offset = 0;
        let layer_slot_0_offset = trunk_offset + trunk_size;
        let layer_slot_1_offset = layer_slot_0_offset + layer_slot_size;
        let kv_ring_offset = layer_slot_1_offset + layer_slot_size;

        Ok(Self {
            storage,
            capacity_bytes: total_capacity_bytes,
            trunk_offset,
            trunk_size,
            layer_slot_0_offset,
            layer_slot_1_offset,
            layer_slot_size,
            kv_ring_offset,
            kv_ring_size,
            allocated_high_watermark: AtomicUsize::new(required),
        })
    }

    /// Access immutable slice for shared transformer trunk (embeddings, LayerNorms, attention projections)
    pub fn trunk_slice(&self) -> &[u8] {
        &self.storage[self.trunk_offset..self.trunk_offset + self.trunk_size]
    }

    /// Access mutable slice for shared transformer trunk
    pub fn trunk_slice_mut(&mut self) -> &mut [u8] {
        &mut self.storage[self.trunk_offset..self.trunk_offset + self.trunk_size]
    }

    /// Access double-buffered layer streaming slot (Slot 0 or Slot 1)
    pub fn layer_slot_mut(&mut self, slot_idx: usize) -> Result<&mut [u8]> {
        match slot_idx {
            0 => Ok(&mut self.storage
                [self.layer_slot_0_offset..self.layer_slot_0_offset + self.layer_slot_size]),
            1 => Ok(&mut self.storage
                [self.layer_slot_1_offset..self.layer_slot_1_offset + self.layer_slot_size]),
            _ => Err(AuraError::MemoryError(
                "Invalid layer slot index; must be 0 or 1".to_string(),
            )),
        }
    }

    /// Access KV Cache ring buffer slice
    pub fn kv_ring_mut(&mut self) -> &mut [u8] {
        &mut self.storage[self.kv_ring_offset..self.kv_ring_offset + self.kv_ring_size]
    }

    pub fn total_capacity(&self) -> usize {
        self.capacity_bytes
    }

    pub fn peak_rss_allocated(&self) -> usize {
        self.allocated_high_watermark.load(Ordering::Relaxed)
    }

    /// Zero out layer buffers without reallocating
    pub fn reset_layer_slots(&mut self) {
        self.storage[self.layer_slot_0_offset..self.layer_slot_1_offset + self.layer_slot_size]
            .fill(0);
    }
}
