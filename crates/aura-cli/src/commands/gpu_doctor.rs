use aura_hardware::gpu::detect_gpu;

pub fn execute_gpu_doctor() {
    println!("🔍 Running AURA GPU Hardware & Backend Doctor...\n");
    let profile = detect_gpu();

    println!("GPU Detection");
    println!("──────────────────────────────────────────────────");
    if profile.present {
        println!(
            "NVIDIA GPU        : {}",
            profile.model_name.as_deref().unwrap_or("Detected")
        );
        println!("CUDA Backend      : {}", profile.backend_supported);
        if let Some(bytes) = profile.vram_bytes {
            println!(
                "VRAM              : {:.2} GB ({} MiB)",
                bytes as f64 / (1024.0 * 1024.0 * 1024.0),
                bytes / (1024 * 1024)
            );
        } else {
            println!("VRAM              : Shared / System Managed");
        }
        println!("AURA CUDA Backend : READY");
    } else {
        println!("NVIDIA GPU        : Not Detected / CPU-only host");
        println!("CUDA Backend      : Unavailable");
        println!("AURA CUDA Backend : DISABLED (Falling back to CpuLlamaCpp)");
    }
}
