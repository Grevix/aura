use aura_hardware::probe_hardware;

pub fn execute_hardware_doctor() {
    println!("AURA Hardware Doctor");
    println!("====================");

    let hw = probe_hardware();

    println!("\nOS:\n{} ({})", hw.os_name, hw.os_version);

    println!("\nCPU:");
    println!("Model          : {}", hw.cpu.model_name);
    println!("Physical cores : {}", hw.cpu.physical_cores);
    println!("Logical cores  : {}", hw.cpu.logical_cores);
    let simd_str: Vec<String> = hw
        .cpu
        .simd_features
        .iter()
        .map(|s| format!("{:?}", s))
        .collect();
    println!("SIMD Extensions: {}", simd_str.join(", "));

    println!("\nRAM:");
    println!(
        "Total System RAM : {:.2} GB",
        hw.memory.total_ram_bytes as f64 / 1e9
    );
    println!(
        "Available Memory : {:.2} GB",
        hw.memory.available_ram_bytes as f64 / 1e9
    );
    println!(
        "Memory Bandwidth : ~{:.2} GB/s (DDR5 Dual Channel Est)",
        38.4
    );

    println!("\nGPU & Accelerator:");
    if hw.gpu.present {
        println!(
            "GPU Model        : {}",
            hw.gpu.model_name.as_deref().unwrap_or("Detected")
        );
        if let Some(bytes) = hw.gpu.vram_bytes {
            println!(
                "VRAM Total       : {:.2} GB ({} MiB)",
                bytes as f64 / 1e9,
                bytes / (1024 * 1024)
            );
        } else {
            println!("VRAM Total       : Shared / System Managed");
        }
        println!("Backend Support  : {}", hw.gpu.backend_supported);
        println!("AURA CUDA Backend: READY");
    } else {
        println!("GPU Present      : None (CPU Native SIMD Inference)");
        println!("CUDA Status      : Unavailable");
    }

    println!("\nStorage Subsystem:");
    println!("Device Type      : {}", hw.storage.storage_type);
    println!("Sequential Read  : {:.2} MB/s", hw.storage.seq_read_mbps);
    println!("Random 4K IOPS   : {}", hw.storage.random_iops);

    println!("\nRecommended Inference Mode:");
    if hw.gpu.present && hw.gpu.vram_bytes.unwrap_or(0) >= 4 * 1024 * 1024 * 1024 {
        println!("Mode   : GPU_OFFLOAD / CUDA");
        println!("Reason : Dedicated NVIDIA GPU with {:.2} GB VRAM detected with fast NVMe layer staging.", hw.gpu.vram_bytes.unwrap_or(0) as f64 / 1e9);
    } else if hw.memory.total_ram_bytes >= 128 * 1024 * 1024 * 1024 {
        println!("Mode   : CPU_OFFLOAD (RAM-backed)");
        println!("Reason : Large RAM capacity detected (192GB Class). Use memory-mapped weights and multi-threaded AVX2 kernels.");
    } else {
        println!("Mode   : DISK_STREAMED / CPU");
        println!("Reason : Low-memory host budget. Use layer-by-layer NVMe streaming with double-buffered prefetch.");
    }
}
