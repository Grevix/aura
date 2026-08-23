use aura_model::manifest::load_manifest;

#[test]
fn test_load_synthetic_manifest() {
    let temp_file = std::env::temp_dir().join(".aura_test_model.bin");
    let mut data = vec![0x47, 0x47, 0x55, 0x46, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    data.resize(1000, 0);
    let _ = std::fs::write(&temp_file, data);

    let manifest = load_manifest(&temp_file).unwrap();
    assert_eq!(manifest.name, ".aura_test_model");
    assert_eq!(manifest.layer_count, 32);

    let _ = std::fs::remove_file(&temp_file);
}
