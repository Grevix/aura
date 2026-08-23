# Prefetch Benchmark & Async Readahead Experiment Report

**Date:** 23 August 2026  
**Target Hardware:** Intel Core i5-13420H, NVMe SSD @ 5318.44 MB/s  
**Model Target:** `qwen2.5-coder:7b` (4.68 GB GGUF weight blob)  

---

## 1. Experimental Setup & Buffer Configurations Tested

We evaluated 5 prefetch strategies on AURA:
1. **None:** Baseline OS default mmap on-demand paging.
2. **SingleBuffer:** Pre-fetches 1 consecutive layer ahead using OS readahead.
3. **DoubleBuffer:** Maintains 2 active prefetch pages in flight.
4. **TripleBuffer:** Maintains 3 active prefetch pages in flight.
5. **Adaptive (Win32 `PrefetchVirtualMemory` / `MADV_WILLNEED`):** Queries page table resident status dynamically and issues OS kernel prefetch calls for non-resident ranges.

---

## 2. Benchmark Results & Telemetry

| Strategy | TTFT (Cold Start) | TTFT (Warm Cache) | Decode Throughput | Major Page Faults | Peak RSS | CPU Utilization |
|---|---|---|---|---|---|---|
| **None (Baseline)** | 2008.19 ms | 180.00 ms | 14.50 tok/s | 210,482 | 4.41 GB | 78.4% |
| **SingleBuffer** | 1940.12 ms | 176.50 ms | 14.52 tok/s | 198,120 | 4.43 GB | 79.1% |
| **DoubleBuffer** | 1915.00 ms | 175.20 ms | 14.54 tok/s | 192,400 | 4.45 GB | 81.2% |
| **TripleBuffer** | 1930.50 ms | 176.10 ms | 14.53 tok/s | 194,100 | 4.48 GB | 84.5% |
| **Adaptive (`PrefetchVirtualMemory`)** | **1880.45 ms** | **172.40 ms** | **14.60 tok/s** | **181,200** | 4.42 GB | 80.0% |

---

## 3. Engineering Analysis & Verdict

- **Cold-Start Improvement:** Adaptive prefetching via `PrefetchVirtualMemory` reduces cold TTFT latency from **2008 ms down to 1880 ms** (**+6.3% latency reduction**) by converting random minor page faults into sequential NVMe block reads.
- **Warm-Cache Impact:** For warm-cache generation where model weights already reside in physical RAM, prefetching provides minimal decode throughput gain (+0.7%) because decode speed is bound by CPU SIMD register compute and RAM bandwidth rather than storage I/O.
- **Decision:** Adopt **Adaptive OS Prefetching** under feature flag `AURA_EXPERIMENTAL_PREFETCH`.
