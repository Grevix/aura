use aura_core::types::{EnforcementMechanism, HardwareProfile};

pub fn determine_enforcement_mechanism(hw: &HardwareProfile) -> EnforcementMechanism {
    match hw.os_name.to_lowercase().as_str() {
        "linux" | "ubuntu" | "debian" | "fedora" | "arch" => EnforcementMechanism::CgroupV2Hard,
        "windows" => EnforcementMechanism::WindowsJobObject,
        "macos" | "darwin" => EnforcementMechanism::MacosBestEffort,
        _ => EnforcementMechanism::None,
    }
}
