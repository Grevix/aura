use aura_core::types::{ModelManifest, QuantType};

#[derive(Debug, Clone)]
pub struct MemoryEstimate {
    pub weight_bytes: u64,
    pub kv_cache_bytes: u64,
    pub overhead_bytes: u64,
    pub total_peak_rss_bytes: u64,
}

pub fn estimate_memory_footprint(
    manifest: &ModelManifest,
    context_length: usize,
    _quant: &QuantType,
) -> MemoryEstimate {
    let weight_bytes = manifest.required_file_bytes;

    // KV Cache estimation formula: 2 * layers * kv_heads * head_dim * context * bytes_per_element (2 for FP16)
    let kv_cache_bytes = 2
        * (manifest.layer_count as u64)
        * (manifest.key_value_heads as u64)
        * (manifest.head_dimension as u64)
        * (context_length as u64)
        * 2;

    let overhead_bytes = 512 * 1024 * 1024; // 512 MB baseline allocation overhead

    let total_peak_rss_bytes = weight_bytes + kv_cache_bytes + overhead_bytes;

    MemoryEstimate {
        weight_bytes,
        kv_cache_bytes,
        overhead_bytes,
        total_peak_rss_bytes,
    }
}
