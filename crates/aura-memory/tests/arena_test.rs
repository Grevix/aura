use aura_memory::AuraArena;

#[test]
fn test_aura_arena_zero_allocation_and_slicing() {
    let total_capacity = 100 * 1024 * 1024; // 100 MB
    let trunk_size = 20 * 1024 * 1024; // 20 MB
    let layer_slot_size = 15 * 1024 * 1024; // 15 MB x 2 = 30 MB
    let kv_ring_size = 40 * 1024 * 1024; // 40 MB

    let mut arena = AuraArena::new(total_capacity, trunk_size, layer_slot_size, kv_ring_size)
        .expect("Failed to allocate static arena");

    assert_eq!(arena.total_capacity(), total_capacity);
    assert!(arena.peak_rss_allocated() <= total_capacity);

    // Verify trunk partition
    let trunk = arena.trunk_slice_mut();
    assert_eq!(trunk.len(), trunk_size);
    trunk[0] = 0xAA;
    assert_eq!(arena.trunk_slice()[0], 0xAA);

    // Verify double-buffer staging slots
    let slot0 = arena.layer_slot_mut(0).expect("slot 0 valid");
    assert_eq!(slot0.len(), layer_slot_size);
    slot0[0] = 0xBB;

    let slot1 = arena.layer_slot_mut(1).expect("slot 1 valid");
    assert_eq!(slot1.len(), layer_slot_size);
    slot1[0] = 0xCC;

    assert_eq!(arena.layer_slot_mut(0).unwrap()[0], 0xBB);
    assert_eq!(arena.layer_slot_mut(1).unwrap()[0], 0xCC);

    // Verify KV ring partition
    let kv_ring = arena.kv_ring_mut();
    assert_eq!(kv_ring.len(), kv_ring_size);

    // Reset layer slots without reallocation
    arena.reset_layer_slots();
    assert_eq!(arena.layer_slot_mut(0).unwrap()[0], 0x00);
    assert_eq!(arena.layer_slot_mut(1).unwrap()[0], 0x00);
}
