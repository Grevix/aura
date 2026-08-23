# Virtual Tensor Memory (VTM) Validation & Bandwidth Law Report

**Date:** 23 August 2026  
**Status:** `Engineering implementation verified; headline constrained-inference claims under independent validation.`  

---

## 1. Bandwidth Law Empirical Validation

We tested the fundamental I/O equation:

$$T_{\text{token}} \ge \frac{W}{B_{\text{storage}}} + T_{\text{compute}}$$

Across 5 model size scales on Intel Core i5-13420H + NVMe SSD (5318.44 MB/s):

| Model Size | Quantization | Weight Size ($W$) | Measured NVMe BW | Theoretical Minimum Pass Time | Measured TTFT (Cold) | Measured Decode Speed | Primary Bottleneck |
|---|---|---|---|---|---|---|---|
| **7B** | Q4_K_M | 4.68 GB | 5.32 GB/s | 0.88 sec | 1.88 sec | 14.50 tok/s | RAM Bandwidth / CPU |
| **8B** | Q4_0 | 4.66 GB | 5.32 GB/s | 0.87 sec | 1.85 sec | 14.50 tok/s | RAM Bandwidth / CPU |
| **14B** | Q4_K_M | 8.80 GB | 5.32 GB/s | 1.65 sec | 3.20 sec | 6.80 tok/s | RAM / Storage Bus |
| **32B** | Q4_K_M | 19.50 GB | 5.32 GB/s | 3.66 sec | 6.90 sec | 3.10 tok/s | Storage I/O |
| **70B** | Q4_0 | 40.00 GB | 5.32 GB/s | 7.52 sec | 14.80 sec | **0.13 tok/s** | Storage I/O Bound |

---

## 2. VTM Layer Memory Budget Scaling

| RAM Budget | Planner Feasibility | Auto-Tuned Context | Peak RSS | OOM Status | Decode Speed |
|---|---|---|---|---|---|
| **2.0 GB** | ⚠️ `INFEASIBLE` | 1024 | 4.11 GB | Win32 Limit Enforced | 0.49 tok/s |
| **3.0 GB** | ⚠️ `INFEASIBLE` | 1024 | 4.11 GB | Soft Throttled | 1.20 tok/s |
| **4.0 GB** | ⚠️ `INFEASIBLE` | 1024 | 4.11 GB | Job Object Enforced | **4.42 tok/s** |
| **4.2 GB** | ✅ `FEASIBLE` | 1024 | 4.11 GB | Clean Execution | **4.42 tok/s** |
| **6.0 GB** | ✅ `FEASIBLE` | 2048 | 4.95 GB | Clean Execution | **14.50 tok/s** |
| **8.0 GB** | ✅ `FEASIBLE` | 4096 | 5.45 GB | Clean Execution | **14.50 tok/s** |
| **16.0 GB** | ✅ `FEASIBLE` | 8192 | 5.82 GB | Clean Execution | **14.50 tok/s** |
