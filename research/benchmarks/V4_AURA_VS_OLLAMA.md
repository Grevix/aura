# AURA V4 vs Ollama — Apples-to-Apples Head-to-Head Evaluation

**Date:** 23 August 2026  
**Artifact Data:** `benchmarks/results/v4_4gb_raw_results.json`  

---

## Direct Comparison Table

| Metric / Scenario | Ollama Baseline | AURA Engine | Empirical Winner | Key Driver |
|---|---|---|---|---|
| **Non-Interactive Batch Automation** | Failed (Timed out >60s) | **1.94s per prompt** | **AURA** | TTY stdin buffer dependency in Ollama CLI |
| **Budget Enforcement (4GB RAM)** | None (Crashes / Swap) | **4.11 GB Enforced** | **AURA** | Win32 Job Objects / cgroups v2 |
| **Warm-Cache Decode Speed (16GB)** | **14.50 tok/s** | **14.50 tok/s** | **TIED** | Shared native `llama.cpp` AVX2 GEMM kernel |
| **Model Load Latency (GGUF)** | ~50 ms | **<5 ms** | **AURA** | Zero-copy header mmap resolver |
| **MoE Storage Read Bandwidth** | 1.00 GB/token | **0.15 GB/token** | **AURA** | **85.0% LRU Expert Cache Hit Rate** |
| **GPU Acceleration** | **Full CUDA Support** | CPU-Only Mode | **Ollama** | Discrete GPU backend |
