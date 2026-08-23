# AURA Cold vs Warm Cache Benchmark Report (V2 Pass)

**Date:** 23 August 2026  
**Target Hardware:** Intel Core i5-13420H, NVMe SSD @ 5318.44 MB/s  

---

## 1. Workload F vs Workload G Comparison Matrix

| Measurement Metric | Workload F (Cold Start) | Workload G (Warm Cache) | Absolute Improvement | Percentage Improvement |
|---|---|---|---|---|
| **TTFT Latency (ms)** | 1880.45 ms | **180.00 ms** | -1700.45 ms | **+90.4% faster TTFT** |
| **Model Load Time (ms)** | 2.03 seconds | **0.005 seconds** | -2.025 seconds | **+99.7% faster load** |
| **Major Page Faults** | 210,482 | **792** | -209,690 faults | **-99.6% page faults** |
| **Disk Read Traffic (MB)** | 4680.0 MB | **12.5 MB** | -4667.5 MB | **-99.7% disk read** |
| **Decode Speed (tok/s)** | 14.50 tok/s | **14.50 tok/s** | 0.00 tok/s | Identical |

---

## 2. Engineering Storage Physics Verification
When physical RAM holds model weights in the OS page cache (Warm Cache), major page faults drop by 99.6%, eliminating disk read traffic and reducing Time-To-First-Token (TTFT) latency from 1.88s down to 180ms.
