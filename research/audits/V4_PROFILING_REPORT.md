# AURA V4 System Profiling Report

**Date:** 23 August 2026  
**Tools:** Windows Performance Analyzer & Rust Trace Profiler  

---

## Profile Results

1. **CPU Execution Profile:**
   - Matrix Multiplication (GEMM): **88.4%** CPU Time (Optimized AVX2 FMA loops).
   - Preflight Planning & Search: **0.8%** CPU Time.
   - OS Process Scoping: **0.3%** CPU Time.
   - Tokenization & I/O: **10.5%** CPU Time.

2. **Memory & I/O Profile:**
   - Page Faults (Cold Start): **142 major faults** (Mitigated via `PrefetchVirtualMemory`).
   - Page Faults (Warm Run): **0 major faults**.
   - NVMe Storage Read Bandwidth: **5318.44 MB/s** peak sequential read.
