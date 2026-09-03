use aura_core::{ExpertTier, ThreeTierExpertCache};

#[test]
fn test_three_tier_moe_expert_cache_migration() {
    let vram_cap = 50 * 1024 * 1024; // 50 MB (fits ~2 experts of 20MB)
    let ram_cap = 100 * 1024 * 1024; // 100 MB (fits ~5 experts)
    let expert_size = 20 * 1024 * 1024;

    let mut cache = ThreeTierExpertCache::new(vram_cap, ram_cap);

    // Initial access: expert 1 is cold from disk, staged into RAM
    let tier_e1_step1 = cache.access_expert(1, 0, expert_size).expect("access ok");
    assert_eq!(tier_e1_step1, ExpertTier::Tier3DiskCold);

    // Second access: expert 1 is promoted to VRAM hot cache
    let tier_e1_step2 = cache.access_expert(1, 0, expert_size).expect("access ok");
    assert_eq!(tier_e1_step2, ExpertTier::Tier2HostRamWarm);

    // Third access: expert 1 is already hot in VRAM
    let tier_e1_step3 = cache.access_expert(1, 0, expert_size).expect("access ok");
    assert_eq!(tier_e1_step3, ExpertTier::Tier1VramHot);

    let (vram_hits, ram_hits, disk_misses) = cache.stats();
    assert_eq!(vram_hits, 1);
    assert_eq!(ram_hits, 1);
    assert_eq!(disk_misses, 1);
}
