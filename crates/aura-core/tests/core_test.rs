use aura_core::types::*;

#[test]
fn test_quant_type_display() {
    assert_eq!(QuantType::Q4_K_M.to_string(), "Q4_K_M");
    assert_eq!(QuantType::Q3_K_S.to_string(), "Q3_K_S");
}

#[test]
fn test_enforcement_mechanism_display() {
    assert_eq!(
        EnforcementMechanism::CgroupV2Hard.to_string(),
        "cgroup_v2_hard"
    );
    assert_eq!(
        EnforcementMechanism::WindowsJobObject.to_string(),
        "windows_job_object"
    );
}
