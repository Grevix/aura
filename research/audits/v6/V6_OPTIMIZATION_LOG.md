# AURA V6 — Optimization Log (Before/After Measurements)

**Date:** 23 August 2026  
**Status Standard:** Only retained optimizations have `[KEPT]` status.

---

## OPT-V6-01 — Small-Model Fast Path (aura-planner/search.rs)

| | Before | After |
|---|---|---|
| Preflight planning latency (1B model) | 15.0 ms | **<1.0 ms** |
| Preflight planning latency (3B model) | 15.0 ms | **<1.0 ms** |

- **BASELINE:** Multi-pass context search on all model sizes.  
- **CHANGE:** Skip search loop for `total_parameters ≤ 3.5 B` OR `required_file_bytes ≤ 2.5 GB` under 4 GB+ RAM budget.  
- **BENCHMARK:** `test_small_model_fast_path` unit test + manual timing.  
- **RESULT:** 15× latency reduction on 1B/3B preflight.  
- **REGRESSION CHECK:** `cargo test --workspace` → **100% PASS**.  
- **DECISION:** `[KEPT]` — Production default.

---

## OPT-V6-02 — Win32 Job Object Memory Scoping (aura-memory/windows.rs)

| | Before | After |
|---|---|---|
| Peak RSS (7B model) | >5.45 GB (OOM risk) | **4.11 GB (capped)** |

- **BASELINE:** No memory limit enforcement.  
- **CHANGE:** `SetInformationJobObject(JobObjectExtendedLimitInformation)` enforces hard working set ceiling.  
- **RESULT:** Zero OOM crashes across all testable 4 GB models.  
- **DECISION:** `[KEPT]` — Production default.

---

## OPT-V6-03 — TurboVec AVX2 Nibble GEMV Kernel (aura-core)

| | Before | After |
|---|---|---|
| Memory bandwidth (GEMV) | ~42.8 GB/s | 42.8 GB/s |
| llama.cpp baseline (GEMV) | ~48.2 GB/s | — |

- **RESULT:** TurboVec is **12.6% SLOWER** than llama.cpp native AVX2 on GEMV.  
- **DECISION:** `[NOT KEPT]` — llama.cpp GEMM backend remains production. TurboVec retained as `experimental` path.

---

## OPT-V6-04 — Win32 PrefetchVirtualMemory for Cold-Start TTFT (aura-hardware)

| | Before | After |
|---|---|---|
| Cold start TTFT (7B model) | 3850 ms | **1880 ms** |

- **RESULT:** 51.2% cold TTFT reduction.  
- **REGRESSION CHECK:** `cargo test --workspace` → **100% PASS**.  
- **DECISION:** `[KEPT]` — Production default.
