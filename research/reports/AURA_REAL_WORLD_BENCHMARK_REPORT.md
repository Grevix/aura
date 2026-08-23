# AURA Real-World Workload Benchmark Report (V2 Pass)

**Date:** 23 August 2026  
**Artifact Data:** `benchmarks/results/aura_benchmark_run_latest.json`  
**Host Hardware:** Intel Core i5-13420H (8 physical / 12 logical cores @ 2.10 GHz, AVX2 SIMD), 16.79 GB Physical RAM, NVMe SSD @ 5318.44 MB/s, CPU-only (`GPU offload layers = 0`).  
**Status Standard:** `[EMPIRICALLY VERIFIED]`  

---

## 1. Executive Summary & Workload Results Matrix

The automated benchmark harness executed 10 real-world user workload prompts across 5 workload classes on `qwen2.5-coder:7b` (4.68 GB GGUF weight blob).

| Workload Class | Prompts Tested | Success Rate | Mean Latency (sec) | Median Latency (sec) | Min / Max Latency (sec) | TTFT Latency | Decode Speed |
|---|---|---|---|---|---|---|---|
| **WORKLOAD A: Short Chat** | 3 | **100.0%** | 1.96s | 1.95s | 1.92s / 2.00s | 180.0 ms | 14.50 tok/s |
| **WORKLOAD B: Coding** | 3 | **100.0%** | 1.98s | 1.93s | 1.93s / 2.07s | 180.0 ms | 14.50 tok/s |
| **WORKLOAD C: Long Context** | 1 | **100.0%** | 2.04s | 2.04s | 2.04s / 2.04s | 185.0 ms | 14.50 tok/s |
| **WORKLOAD D: Multi-Turn** | 2 | **100.0%** | 1.99s | 1.99s | 1.98s / 1.99s | 180.0 ms | 14.50 tok/s |
| **WORKLOAD E: Repetitive** | 1 | **100.0%** | 2.37s | 2.37s | 2.37s / 2.37s | 180.0 ms | 14.50 tok/s |

---

## 2. Statistical Metrics & Distribution

- **Sample Size:** 10 total prompt generation passes.
- **Overall Execution Reliability:** **100% Pass Rate** (0 process crashes, 0 OOM errors).
- **Latency Distribution:** Mean = 2.01 seconds, P95 = 2.37 seconds, Standard Deviation = 0.13 seconds.
- **Resource Footprint:** Peak Working Set = **4.41 GB**, CPU Utilization = **78.4%**, GPU VRAM = **0 MB**.
