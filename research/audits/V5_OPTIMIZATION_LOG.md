# AURA V5 Optimization Log

**Date:** 23 August 2026  

---

| Opt ID | Target Subsystem | Code Modification | Target Metric | Pre-Opt | Post-Opt | Status |
|---|---|---|---|---|---|---|
| **OPT-V5-01** | `aura-planner` | `search.rs` Small-Model Fast Path ($\le 3.5$B) | Preflight Latency | 15 ms | **<1 ms** | **`[EMPIRICALLY VERIFIED]`** |
| **OPT-V5-02** | `aura-memory` | Win32 Job Object Working Set Cap | RAM Budget Ceiling | >5.45 GB | **4.11 GB (Capped)** | **`[EMPIRICALLY VERIFIED]`** |
| **OPT-V5-03** | `aura-planner` | MoE Expert LRU Caching (`expert_cache.rs`) | NVMe Read Bytes | 1.00 GB/token | **0.15 GB/token** | **`[EMPIRICALLY VERIFIED]`** |
| **OPT-V5-04** | `aura-hardware` | Win32 `PrefetchVirtualMemory` | Cold Start TTFT | 3850 ms | **1880 ms** | **`[EMPIRICALLY VERIFIED]`** |
