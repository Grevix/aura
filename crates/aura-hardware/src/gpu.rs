use aura_core::types::GpuProfile;

pub fn detect_gpu() -> GpuProfile {
    // Stage 0.1.0 fallback detection
    GpuProfile {
        present: false,
        model_name: None,
        vram_bytes: None,
        backend_supported: "CPU-only".to_string(),
    }
}
