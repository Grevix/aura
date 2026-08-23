# AURA V5 Bottleneck & Profiling Report

**Date:** 23 August 2026  

---

## Profiling Breakdown Across Model Sizes

| Model Size | Preflight Planning | Model Load (GGUF mmap) | GEMM Compute | Memory / Page Fault Overhead |
|---|---|---|---|---|
| **1B Model** | **<1 ms (0.1%)** | <2 ms (0.2%) | **91.5%** | 8.2% |
| **3B Model** | **<1 ms (0.1%)** | <3 ms (0.2%) | **92.1%** | 7.6% |
| **7B Model** | 15 ms (0.8%) | <5 ms (0.2%) | **88.4%** | 10.6% |
| **8B Model** | 15 ms (0.8%) | <5 ms (0.2%) | **87.9%** | 11.1% |
