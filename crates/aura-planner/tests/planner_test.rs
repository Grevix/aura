use aura_hardware::probe_hardware;
use aura_model::manifest::load_manifest;
use aura_planner::generate_execution_plan;

#[test]
fn test_planner_feasibility() {
    let temp_file = std::env::temp_dir().join(".aura_planner_test.bin");
    let mut data = vec![
        0x47, 0x47, 0x55, 0x46, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    data.resize(100_000_000, 0);
    let _ = std::fs::write(&temp_file, data);

    let hw = probe_hardware();
    let manifest = load_manifest(&temp_file).unwrap();

    let plan = generate_execution_plan(&hw, &manifest, 4_000_000_000, Some(2048));
    assert!(plan.is_feasible);
    assert_eq!(plan.recommended_context, 2048);

    let _ = std::fs::remove_file(&temp_file);
}

#[test]
fn test_small_model_fast_path() {
    let hw = probe_hardware();
    let small_manifest = aura_core::types::ModelManifest {
        name: "llama3.2:1b".to_string(),
        source_hash_sha256: "synthetic_hash".to_string(),
        architecture_family: "llama".to_string(),
        total_parameters: 1_200_000_000,
        active_parameters: 1_200_000_000,
        is_moe: false,
        expert_count: None,
        active_experts_per_token: None,
        layer_count: 16,
        attention_heads: 32,
        key_value_heads: 8,
        head_dimension: 64,
        context_length_max: 8192,
        quantization_type: aura_core::types::QuantType::Q4_K_M,
        required_file_bytes: 1_200_000_000,
        file_path: "synthetic_path".to_string(),
    };

    let plan = generate_execution_plan(&hw, &small_manifest, 4_000_000_000, Some(4096));
    assert!(plan.is_feasible);
    assert!(plan.feasibility_notes.contains("FAST-PATH FEASIBLE"));
}
