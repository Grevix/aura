use aura_benchmark::generate_benchmark_report;
use aura_hardware::probe_hardware;
use aura_model::manifest::load_manifest;
use aura_planner::generate_execution_plan;

#[test]
fn test_benchmark_schema_generation() {
    let temp_file = std::env::temp_dir().join(".aura_bench_test.bin");

    // GGUF magic bytes + header stub
    let mut data = vec![
        0x47, 0x47, 0x55, 0x46, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    data.resize(1_000_000, 0);
    let _ = std::fs::write(&temp_file, data);

    let hw = probe_hardware();
    let manifest = load_manifest(&temp_file).unwrap();
    let plan = generate_execution_plan(&hw, &manifest, 4_000_000_000, Some(2048));

    let report = generate_benchmark_report(hw, manifest, plan, 200.0, 15.0);
    assert_eq!(report.schema_version, "1.0");
    assert!(report.phases.cold_start.decode_tok_per_sec > 0.0);

    let _ = std::fs::remove_file(&temp_file);
}
