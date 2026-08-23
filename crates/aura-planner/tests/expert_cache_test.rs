use aura_planner::expert_cache::{CachePolicy, ExpertCacheManager};

#[test]
fn test_expert_cache_lru_behavior() {
    let mut cache = ExpertCacheManager::new(4, CachePolicy::Lru);

    // Initial load: 4 active experts
    let (hits, misses) = cache.access_experts(&[1, 2, 3, 4]);
    assert_eq!(hits, 0);
    assert_eq!(misses, 4);

    // Repeated access: should be 100% hits
    let (hits2, misses2) = cache.access_experts(&[1, 2, 3, 4]);
    assert_eq!(hits2, 4);
    assert_eq!(misses2, 0);

    // Access new expert 5 -> evicts LRU (expert 1 if not moved, or earliest)
    let (hits3, misses3) = cache.access_experts(&[5]);
    assert_eq!(hits3, 0);
    assert_eq!(misses3, 1);

    println!(
        "MoE Expert Cache Hit Rate: {:.2}%",
        cache.hit_rate() * 100.0
    );
    assert!(cache.hit_rate() > 0.40);
}
