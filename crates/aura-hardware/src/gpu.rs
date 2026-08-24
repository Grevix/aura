use aura_core::types::GpuProfile;
use std::process::Command;

pub fn detect_gpu() -> GpuProfile {
    // Attempt nvidia-smi query
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,memory.total,driver_version",
            "--format=csv,noheader,nounits",
        ])
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            let line = text.lines().next().unwrap_or("").trim();
            if !line.is_empty() {
                let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 2 {
                    let name = parts[0].to_string();
                    let vram_mb: u64 = parts[1].parse().unwrap_or(0);
                    let driver = if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        "Unknown".to_string()
                    };

                    let vram_bytes = if vram_mb > 0 {
                        Some(vram_mb * 1024 * 1024)
                    } else {
                        None
                    };

                    return GpuProfile {
                        present: true,
                        model_name: Some(name),
                        vram_bytes,
                        backend_supported: format!("NVIDIA CUDA (Driver: {})", driver),
                    };
                }
            }
        }
    }

    // Windows WMI Fallback check
    #[cfg(target_os = "windows")]
    {
        let wmi_out = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name",
            ])
            .output();

        if let Ok(out) = wmi_out {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.contains("NVIDIA")
                    || trimmed.contains("Radeon")
                    || trimmed.contains("Intel")
                {
                    return GpuProfile {
                        present: true,
                        model_name: Some(trimmed.to_string()),
                        vram_bytes: None,
                        backend_supported: if trimmed.contains("NVIDIA") {
                            "NVIDIA CUDA".to_string()
                        } else {
                            "DirectML / Vulkan".to_string()
                        },
                    };
                }
            }
        }
    }

    GpuProfile {
        present: false,
        model_name: None,
        vram_bytes: None,
        backend_supported: "CPU-only".to_string(),
    }
}
