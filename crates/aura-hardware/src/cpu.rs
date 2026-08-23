use aura_core::types::{CpuProfile, SimdExtension};
use sysinfo::System;

pub fn detect_cpu() -> CpuProfile {
    let mut sys = System::new_all();
    sys.refresh_cpu_all();

    let cpus = sys.cpus();
    let model_name = if !cpus.is_empty() {
        cpus[0].brand().to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let logical_cores = sys.cpus().len();
    let physical_cores = sys.physical_core_count().unwrap_or(logical_cores);

    let base_clock_ghz = if !cpus.is_empty() {
        cpus[0].frequency() as f64 / 1000.0
    } else {
        2.5
    };

    let mut simd_features = Vec::new();

    #[cfg(target_arch = "x86_64")]
    {
        use raw_cpuid::CpuId;
        let cpuid = CpuId::new();
        if let Some(feature_info) = cpuid.get_feature_info() {
            if feature_info.has_avx() {
                simd_features.push(SimdExtension::Avx);
            }
        }
        if let Some(extended_features) = cpuid.get_extended_feature_info() {
            if extended_features.has_avx2() {
                simd_features.push(SimdExtension::Avx2);
            }
            if extended_features.has_avx512f() {
                simd_features.push(SimdExtension::Avx512f);
            }
            if extended_features.has_avx512vnni() {
                simd_features.push(SimdExtension::Avx512Vnni);
            }
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        simd_features.push(SimdExtension::Neon);
    }

    CpuProfile {
        model_name,
        physical_cores,
        logical_cores,
        base_clock_ghz,
        simd_features,
    }
}
