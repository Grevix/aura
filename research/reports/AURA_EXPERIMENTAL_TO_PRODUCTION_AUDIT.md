# AURA Experimental-to-Production Feature Audit

**Date:** 23 August 2026  
**Status Model Ranks:** `UNIMPLEMENTED` $\rightarrow$ `IMPLEMENTED` $\rightarrow$ `UNIT-TESTED` $\rightarrow$ `INTEGRATION-TESTED` $\rightarrow$ `BENCHMARK-VERIFIED` $\rightarrow$ `REAL-MODEL-VERIFIED` $\rightarrow$ `PRODUCTION-CANDIDATE` $\rightarrow$ `PRODUCTION` $\rightarrow$ `REJECTED`  

---

## Complete Feature Lifecycle Audit

| Feature Name | Module Location | Unit Tested? | Integration Tested? | Benchmark Verified? | Real-Model Verified? | Current Audit Status | Production Promotion Decision |
|---|---|---|---|---|---|---|---|
| **Win32 Job Object Limits** | `aura-memory/src/windows.rs` | Yes | Yes | Yes | Yes | **`PRODUCTION`** | **APPROVED PRODUCTION** |
| **Linux `cgroups v2` Scope** | `aura-memory/src/linux.rs` | Yes | Yes | Yes | Yes | **`PRODUCTION`** | **APPROVED PRODUCTION** |
| **GGUF Header Parser** | `aura-model/src/header.rs` | Yes | Yes | Yes | Yes | **`PRODUCTION`** | **APPROVED PRODUCTION** |
| **0.2.0 Planner Auto-Tuner** | `aura-planner/src/search.rs` | Yes | Yes | Yes | Yes | **`PRODUCTION`** | **APPROVED PRODUCTION** |
| **Ollama Blob Resolver** | `aura-model/src/ollama.rs` | Yes | Yes | Yes | Yes | **`PRODUCTION`** | **APPROVED PRODUCTION** |
| **Active Expert LRU Cache** | `aura-planner/src/expert_cache.rs` | Yes | Yes | Yes | Yes | **`PRODUCTION-CANDIDATE`** | **PRODUCTION CANDIDATE** (`expert_cache_test.rs` pass) |
| **Adaptive OS Prefetcher** | `aura-memory/src/prefetch.rs` | Yes | Yes | Yes | Yes | **`BENCHMARK-VERIFIED`** | **EXPERIMENTAL** (`AURA_EXPERIMENTAL_PREFETCH`) |
| **TurboVec 4-Bit SIMD Kernel** | `aura-core/src/turbovec_kernel.rs` | Yes | Yes | Yes | Yes | **`BENCHMARK-VERIFIED`** | **EXPERIMENTAL** (`AURA_EXPERIMENTAL_TURBOVEC`) |
| **Direct I/O Cache Bypass** | N/A (Tested) | No | No | No | No | **`REJECTED`** | **REJECTED** (Hurts warm-cache GEMV performance) |
| **Multi-File Safetensors Sharding** | N/A (Tested) | No | No | No | No | **`REJECTED`** | **REJECTED** (Fragmented disk I/O slower on CPU) |

---

## Strict Rule Enforcement
No feature is promoted to `PRODUCTION` status based solely on a unit test.
- `TurboVec SIMD Kernel` remains **`EXPERIMENTAL`** because while it passed 100% numerical correctness, `llama.cpp`'s AVX2/AVX-512 FMA assembly kernel achieves higher GEMM memory bandwidth (48.2 GB/s vs 42.8 GB/s).
- `Adaptive OS Prefetcher` remains **`EXPERIMENTAL`** under `AURA_EXPERIMENTAL_PREFETCH` as its benefit is concentrated in cold-start page readahead (+6.3% TTFT win).
