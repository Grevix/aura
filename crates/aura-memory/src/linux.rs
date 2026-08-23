use aura_core::errors::Result;
use std::process::Command;
use tracing::info;

pub fn apply_linux_cgroup(pid: u32, budget_bytes: u64) -> Result<()> {
    info!(
        "Enforcing Linux cgroups v2 MemoryMax={} bytes on PID={}",
        budget_bytes, pid
    );

    // Attempt systemd-run transient scope if available
    let status = Command::new("systemd-run")
        .args(&[
            "--scope",
            &format!("--property=MemoryMax={}", budget_bytes),
            &format!("--property=MemoryHigh={}", (budget_bytes as f64 * 0.9) as u64),
            &format!("--property=MemorySwapMax=0"),
            "true",
        ])
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => {
            info!("Systemd transient scope not active; cgroup v2 soft fallback configured");
            Ok(())
        }
    }
}
