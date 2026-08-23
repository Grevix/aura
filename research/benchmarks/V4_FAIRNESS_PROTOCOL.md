# AURA V4 — Apples-to-Apples Fairness & Methodology Protocol

**Date:** 23 August 2026  
**Status Standard:** `[EMPIRICALLY VERIFIED]`  

---

## 1. Fixed Hardware & OS Environment Scoping
- **CPU Target:** Intel Core i5-13420H (8 physical / 12 logical cores, 2.10 GHz base).
- **SIMD Architecture:** AVX2 + FMA.
- **Memory Limit Scoping:** **HARD 4.0 GB PROCESS WORKING SET BUDGET** enforced via Win32 Job Objects (`JobObjectExtendedLimitInformation`).
- **Accelerator Isolation:** `GPU offload layers = 0` (CPU-only execution). Zero CUDA or Metal offload allowed.

---

## 2. Experimental Fairness Controls
1. **Identical Model Blobs:** Both AURA and Ollama execute against the exact same GGUF binary files stored in `C:\Users\Aaryan Rawat\.ollama\models\blobs\`.
2. **Identical Thread Allocation:** Threads pinned to **8 physical cores** (`-t 8`).
3. **Disaggregated Metric Measurement:**
   $$\text{Total Latency} = t_{\text{startup}} + t_{\text{load}} + t_{\text{TTFT}} + \frac{N_{\text{tokens}} - 1}{S_{\text{decode}}}$$
4. **Cold vs Warm Cache Isolation:** Cold runs executed after clearing file system caches; Warm runs executed on immediate repetitions.
5. **Statistical Repetitions:** All prompts executed across **10 repetitions**, reporting Median, Mean, Standard Deviation, and P95 latency.
