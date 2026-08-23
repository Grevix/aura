# AURA V7 — Mixture-of-Experts (MoE) & Expert Cache Benchmark

> **Protocol Version**: V7.0  
> **Evaluated Component**: `aura-planner::expert_cache` (LRU expert eviction policy)  

---

## 1. Classification of Claims

To ensure full scientific transparency, all MoE performance claims are strictly categorized by verification tier:

| Verification Tier | Description | Status |
|---|---|---|
| **UNIT VERIFIED** | Synthetic LRU cache hit/miss unit tests (`test_expert_cache_lru_behavior`) | **85% hit rate verified on synthetic expert trace** |
| **SYNTHETIC BENCHMARK** | Simulated expert weight streaming on 5318 MB/s NVMe | **Verified in scratch benchmarks** |
| **EMPIRICALLY VERIFIED REAL MODEL** | Full end-to-end 70B+ MoE execution on 48GB+ hardware | **COMMUNITY TRACK ONLY** (Requires 48GB+ RAM/VRAM) |

---

## 2. Synthetic Expert Cache Performance (Unit & Simulation Verification)

Using synthetic routing traces simulating token expert assignments (2 active experts out of 8 total per layer):

| Cache Configuration | Cache Size (MB) | NVMe Read / Token (MB) | Cache Hit Rate (%) | Simulated Decode Latency / Token (ms) |
|---|---|---|---|---|
| **Expert Cache OFF** (Full Disk Read) | 0 | 120.0 | 0.0% | 22.5 ms |
| **Expert Cache ON (LRU 4 Experts)** | 2,048 | 18.0 | 85.0% | 3.38 ms |
| **Expert Cache ON (LRU 8 Experts)** | 4,096 | 0.0 | 100.0% | 0.05 ms (All in RAM) |

---

## 3. Findings & Limitations

1. **Synthetic Verification**:
   The LRU expert cache algorithm (`crates/aura-planner/src/expert_cache.rs`) correctly maintains active experts in RAM and evicts least-recently-used experts. Under a synthetic zipfian distribution of token routing, the cache achieves an 85% hit rate.

2. **Laptop Constraint**:
   Real 70B MoE models (e.g. `mixtral:8x7b` or `qwen2.5-max` GGUF blobs) exceed 30–45 GB in weight size. On a 16 GB laptop with a 4.0 GB process limit, real MoE model inference is limited by NVMe bandwidth (5318 MB/s).

3. **Community Track**:
   Full empirical benchmarking of 70B MoE models is delegated to the **Community Hardware Track** (`docs/COMMUNITY_BENCHMARKS.md`) for contributors with 48GB–96GB+ hardware.
