# AURA Breakthrough Opportunities & Innovation Map

**Date:** 23 August 2026  

---

## Breakthrough Concept: Virtual Tensor Memory (VTM)

AURA can pioneer **Virtual Tensor Memory (VTM)** for LLMs — treating physical RAM, NVMe SSDs, VRAM, and CPU caches as a single unified memory space.

```
                         ┌───────────────────────┐
                         │   AURA VTM Runtime    │
                         └───────────┬───────────┘
                                     │
                 ┌───────────────────┼───────────────────┐
                 ▼                   ▼                   ▼
           [Host RAM]           [NVMe SSD]           [dGPU VRAM]
          (Hot Layers)        (Cold Layers)       (Attention KV)
                 │                   │                   │
                 └───────────────────┼───────────────────┘
                                     │
                                     ▼
                        [TurboVec SIMD Execution]
```

---

## 3 Pillars of AURA VTM Innovation

1. **Activation-Predictive Prefetching (PowerInfer + LLM in a Flash):**
   Predict which 20% of feed-forward neurons will activate in layer $i+1$ based on layer $i$'s output embeddings, loading only required weights across the storage bus.

2. **Per-Expert MoE Streaming & Caching (AirLLM + VTM):**
   Dynamically route and retain hot experts in physical RAM while leaving cold experts on NVMe storage.

3. **TurboVec-Accelerated CPU GEMV Kernels:**
   Use 4-bit nibble-split SIMD lookups (`vpshufb`) for CPU-based quantized matrix multiplications.
