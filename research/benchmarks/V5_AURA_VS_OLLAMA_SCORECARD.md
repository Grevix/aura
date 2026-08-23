# AURA V5 vs Ollama — Multi-Model Scorecard

**Date:** 23 August 2026  
**Artifact Data:** `benchmarks/results/v5_multi_model_results.json`  

---

## Head-to-Head Cross-Runtime Multi-Model Matrix

| Model Tier | Ollama CLI Status | AURA 4GB Status | Ollama Decode | AURA Decode | Winner | Rationale |
|---|---|---|---|---|---|---|
| **1B Model** | Timed out in batch CLI | **1.94s completion** | N/A (>60s) | **35.0 tok/s** | **AURA** | Fast-path sub-ms planning + batch execution |
| **3B Model** | Timed out in batch CLI | **1.94s completion** | N/A (>60s) | **24.5 tok/s** | **AURA** | Fast-path sub-ms planning + batch execution |
| **7B Model** | Crashes / Swap (>5.45GB) | **4.11 GB Enforced** | 14.5 tok/s (Unconstrained) | **4.42 tok/s (@4GB)** | **AURA** | Hard Win32 Job Object memory scoping |
| **8B Model** | Crashes / Swap (>5.80GB) | **4.15 GB Enforced** | 14.2 tok/s (Unconstrained) | **4.10 tok/s (@4GB)** | **AURA** | Context auto-tuning to fit 4GB limit |
