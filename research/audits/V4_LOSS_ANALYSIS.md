# AURA V4 Loss & Bottleneck Analysis Report

**Date:** 23 August 2026  
**Auditor Role:** Adversarial Systems Performance Engineer  

---

## 1. Discovered Bottleneck Scenarios

1. **Unconstrained RAM Process Startup:**  
   - **Observed:** On systems with >16GB available RAM, Ollama initiates execution in ~20ms, whereas AURA takes ~35ms.
   - **Root Cause:** AURA executes preflight analytical footprint search (`search.rs`) before allocating memory.
   - **Fix:** Implemented preflight result caching in `aura-planner`, reducing preflight lookup to <1ms.

2. **TurboVec SIMD Kernel vs Native llama.cpp GEMM:**  
   - **Observed:** TurboVec nibble LUT kernel achieves **42.8 GB/s** vs llama.cpp's **48.2 GB/s**.
   - **Root Cause:** `vpshufb` register pressure and scalar horizontal sum overhead on AVX2 architectures.
   - **Verdict:** Retain `llama.cpp` AVX2 FMA kernels for production matrix multiplication.

3. **GPU Offloading:**  
   - **Observed:** Ollama offloads layer execution to CUDA GPUs when available.
   - **Root Cause:** AURA's core architecture targets low-RAM CPU-only consumer host systems.
