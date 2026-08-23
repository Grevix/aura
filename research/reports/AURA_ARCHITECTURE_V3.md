# AURA Architecture V3 Specifications: Virtual Tensor Memory (VTM)

**Date:** 23 August 2026  
**Version:** 3.0.0 Architecture Blueprint  

---

## High-Level VTM Systems Diagram

```
                              ┌────────────────────────┐
                              │      AURA CLI / API    │
                              └───────────┬────────────┘
                                          │
                              ┌───────────▼────────────┐
                              │    Hardware Profiler   │
                              │ (CPU SIMD, RAM, NVMe)  │
                              └───────────┬────────────┘
                                          │
                              ┌───────────▼────────────┐
                              │   Execution Planner    │
                              │ (Context/Quant Search) │
                              └───────────┬────────────┘
                                          │
                              ┌───────────▼────────────┐
                              │ Virtual Tensor Memory  │
                              │     (VTM Scheduler)    │
                              └───────────┬────────────┘
                                          │
               ┌──────────────────────────┼──────────────────────────┐
               ▼                          ▼                          ▼
     [Layer Streamer & Pager]   [Expert LRU Cache Manager]   [Dynamic KV Compressor]
     (Async Direct I/O Ring)    (Hot Experts in RAM)         (PagedAttention KV)
               │                          │                          │
               └──────────────────────────┼──────────────────────────┘
                                          │
                              ┌───────────▼────────────┐
                              │  TurboVec SIMD Engine  │
                              │ (AVX-512 / AVX2 / NEON)│
                              └────────────────────────┘
```

---

## Component Specifications

### 1. Hardware Profiler (`crates/aura-hardware`)
Probes SIMD capability (AVX2, AVX-512BW, ARM NEON), measures physical RAM headroom, and benchmark-tests sequential & random storage read bandwidth.

### 2. Execution Planner (`crates/aura-planner`)
Computes theoretical memory footprint ($W + KV + \text{Overhead}$) and runs auto-tuning search loop to select context lengths and quantization fallbacks.

### 3. VTM Memory Scheduler (`crates/aura-memory`)
Coordinates tensor allocation across RAM, NVMe, VRAM, and CPU cache. Enforces OS memory limits via Win32 Job Objects (`ProcessMemoryLimit`) and Linux `cgroups v2` (`memory.max`).

### 4. Layer Streamer & Pager (`crates/aura-backends`)
Manages zero-copy `mmap` tensor access, background prefetching ring-buffers, and per-expert MoE sub-module hooking.

### 5. TurboVec SIMD Kernels (`crates/aura-core`)
Provides nibble-split lookup kernels (`vpshufb`) for 4-bit and 2-bit coordinate matrix-vector multiplication.
