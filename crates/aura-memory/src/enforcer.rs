use aura_core::errors::Result;
use aura_core::types::EnforcementMechanism;

pub fn enforce_memory_budget(
    pid: u32,
    budget_bytes: u64,
    mechanism: &EnforcementMechanism,
) -> Result<()> {
    match mechanism {
        EnforcementMechanism::CgroupV2Hard => crate::linux::apply_linux_cgroup(pid, budget_bytes),
        EnforcementMechanism::WindowsJobObject => {
            crate::windows::apply_windows_job_object(pid, budget_bytes)
        }
        EnforcementMechanism::MacosBestEffort => {
            crate::macos::spawn_macos_rss_monitor(pid, budget_bytes)
        }
        EnforcementMechanism::None => Ok(()),
    }
}
