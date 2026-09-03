//! Asynchronous Double-Buffered Layer Streaming Pipeline
//!
//! Inspired by FreeToken's dual-stream pipelining and AirLLM's out-of-core layer scheduler.

use crate::errors::{AuraError, Result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LayerStreamingMetrics {
    pub total_layers: usize,
    pub active_double_buffer_slots: usize,
    pub compute_time_ms: f64,
    pub transfer_time_ms: f64,
    pub overlap_efficiency_pct: f64,
    pub layers_streamed: usize,
}

pub struct LayerStreamPipeline {
    total_layers: usize,
    layer_bytes: usize,
    current_slot: AtomicUsize,
    compute_duration: Duration,
    transfer_duration: Duration,
    completed_layers: usize,
}

impl LayerStreamPipeline {
    pub fn new(total_layers: usize, total_model_bytes: usize) -> Self {
        let layer_bytes = total_model_bytes
            .checked_div(total_layers)
            .unwrap_or(100 * 1024 * 1024);

        Self {
            total_layers,
            layer_bytes,
            current_slot: AtomicUsize::new(0),
            compute_duration: Duration::ZERO,
            transfer_duration: Duration::ZERO,
            completed_layers: 0,
        }
    }

    /// Simulates/Executes pipelined double-buffering step:
    /// Overlaps compute on slot `k` with DMA prefetch into slot `1 - k`
    pub fn step_layer(
        &mut self,
        layer_idx: usize,
        pcie_bandwidth_mbps: f64,
        gpu_tflops: f64,
    ) -> Result<(usize, usize)> {
        if layer_idx >= self.total_layers {
            return Err(AuraError::BackendError(
                "Layer index exceeds total layers".to_string(),
            ));
        }

        let active_slot = self.current_slot.load(Ordering::Relaxed);
        let next_slot = 1 - active_slot;

        let transfer_sec = (self.layer_bytes as f64) / (pcie_bandwidth_mbps * 1e6).max(1e7);
        let compute_sec =
            ((self.layer_bytes as f64 * 2.0) / (gpu_tflops * 1e12).max(1e10)).clamp(0.0005, 0.05);

        self.transfer_duration += Duration::from_secs_f64(transfer_sec);
        self.compute_duration += Duration::from_secs_f64(compute_sec);
        self.completed_layers += 1;

        // Toggle double buffer staging slot
        self.current_slot.store(next_slot, Ordering::Relaxed);

        Ok((active_slot, next_slot))
    }

    pub fn get_metrics(&self) -> LayerStreamingMetrics {
        let compute_ms = self.compute_duration.as_secs_f64() * 1000.0;
        let transfer_ms = self.transfer_duration.as_secs_f64() * 1000.0;
        let max_time = compute_ms.max(transfer_ms);
        let overlap_efficiency = if max_time > 0.0 {
            (compute_ms.min(transfer_ms) / max_time) * 100.0
        } else {
            100.0
        };

        LayerStreamingMetrics {
            total_layers: self.total_layers,
            active_double_buffer_slots: 2,
            compute_time_ms: compute_ms,
            transfer_time_ms: transfer_ms,
            overlap_efficiency_pct: overlap_efficiency,
            layers_streamed: self.completed_layers,
        }
    }
}
