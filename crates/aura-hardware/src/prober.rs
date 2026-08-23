use crate::{cpu, gpu, memory, storage};
use aura_core::types::HardwareProfile;
use std::env;

pub fn probe_hardware() -> HardwareProfile {
    let cpu = cpu::detect_cpu();
    let memory = memory::detect_memory();
    let temp_dir = env::temp_dir();
    let storage = storage::benchmark_storage(&temp_dir);
    let gpu = gpu::detect_gpu();

    let os_name = env::consts::OS.to_string();
    let os_version = std::env::consts::ARCH.to_string();
    let kernel_version = "native".to_string();

    HardwareProfile {
        cpu,
        memory,
        storage,
        gpu,
        os_name,
        os_version,
        kernel_version,
    }
}
