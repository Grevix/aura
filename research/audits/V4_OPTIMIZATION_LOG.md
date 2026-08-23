# AURA V4 Optimization Log

**Date:** 23 August 2026  

---

| Optimization ID | Subsystem | Code Change | Target Metric | Pre-Opt Metric | Post-Opt Metric | Status |
|---|---|---|---|---|---|---|
| **OPT-01** | `aura-planner` | Preflight plan caching in `search.rs` | Startup Latency | 35 ms | **<1 ms** | **PRODUCTION** |
| **OPT-02** | `aura-memory` | Win32 Job Object working set cap | RAM Budget Ceiling | 5.45 GB (Unbounded) | **4.11 GB (Enforced)** | **PRODUCTION** |
| **OPT-03** | `aura-planner` | MoE Expert LRU Caching (`expert_cache.rs`) | NVMe Read Bytes | 1.00 GB/token | **0.15 GB/token** | **PRODUCTION** |
| **OPT-04** | `aura-hardware` | Win32 `PrefetchVirtualMemory` | Cold Start TTFT | 3850 ms | **1880.45 ms** | **PRODUCTION** |
