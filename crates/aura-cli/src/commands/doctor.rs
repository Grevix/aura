use aura_hardware::probe_hardware;

pub fn execute_doctor() {
    println!("🔍 Running AURA Hardware Telemetry Diagnostics...\n");
    let hw = probe_hardware();

    println!("=== HOST SYSTEM TELEMETRY ===");
    println!("Operating System : {} ({})", hw.os_name, hw.os_version);
    println!("CPU Model        : {}", hw.cpu.model_name);
    println!(
        "Cores (Phys/Log) : {} / {}",
        hw.cpu.physical_cores, hw.cpu.logical_cores
    );
    println!("Base Clock       : {:.2} GHz", hw.cpu.base_clock_ghz);

    let simd_str: Vec<String> = hw
        .cpu
        .simd_features
        .iter()
        .map(|s| format!("{:?}", s))
        .collect();
    println!("SIMD Extensions  : {}", simd_str.join(", "));

    println!("\n=== MEMORY TOPOLOGY ===");
    println!(
        "Total Physical RAM : {:.2} GB",
        hw.memory.total_ram_bytes as f64 / 1e9
    );
    println!(
        "Available RAM      : {:.2} GB",
        hw.memory.available_ram_bytes as f64 / 1e9
    );
    println!(
        "Total Swap Space   : {:.2} GB",
        hw.memory.total_swap_bytes as f64 / 1e9
    );
    println!("OS Page Size       : {} bytes", hw.memory.page_size_bytes);

    println!("\n=== STORAGE PERFORMANCE ===");
    println!("Storage Device     : {}", hw.storage.storage_type);
    println!("Seq Read Bandwidth : {:.2} MB/s", hw.storage.seq_read_mbps);
    println!("Est. Random IOPS   : {}", hw.storage.random_iops);

    println!("\n=== ACCELERATOR / GPU ===");
    println!("GPU Present        : {}", hw.gpu.present);
    println!("Backend Support    : {}", hw.gpu.backend_supported);

    println!("\n✅ AURA Doctor Diagnostics Completed Successfully.");
}
