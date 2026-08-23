use aura_hardware::probe_hardware;

#[test]
fn test_hardware_probing() {
    let hw = probe_hardware();
    assert!(hw.cpu.physical_cores >= 1);
    assert!(hw.memory.total_ram_bytes > 0);
    assert!(hw.storage.seq_read_mbps > 0.0);
}
