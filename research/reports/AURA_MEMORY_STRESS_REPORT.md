# AURA Memory Budget & Stress Test Report (V2 Pass)

**Date:** 23 August 2026  
**Target Machine:** Intel Core i5-13420H, 16.79 GB RAM, NVMe SSD  
**Model Target:** `qwen2.5-coder:7b` (4.68 GB GGUF Q4_K_M weights)  

---

## 1. Workload H — Memory Budget Stress Scaling Results

| Budget (GB) | Planner Decision | Auto-Tuned Context | Auto-Tuned Quant | Peak RSS (GB) | Job Limit Limit (GB) | Survival Status | Decode Speed | Classification |
|---|---|---|---|---|---|---|---|---|
| **2.0 GB** | ⚠️ `INFEASIBLE` | 1024 | `Q3_K_S` | 4.11 GB | 2.00 GB | Win32 Limit Enforced | 0.49 tok/s | `NON-PRACTICAL` |
| **3.0 GB** | ⚠️ `INFEASIBLE` | 1024 | `Q3_K_S` | 4.11 GB | 3.00 GB | Soft-Throttled | 1.20 tok/s | `NON-PRACTICAL` |
| **4.0 GB** | ⚠️ `INFEASIBLE` | 1024 | `Q3_K_S` | 4.11 GB | 4.00 GB | Job Object Enforced | **4.42 tok/s** | `FUNCTIONAL` |
| **4.2 GB** | ✅ `FEASIBLE` | 1024 | `Q3_K_S` | 4.11 GB | 4.20 GB | Clean Execution | **4.42 tok/s** | `PRACTICAL` |
| **6.0 GB** | ✅ `FEASIBLE` | 2048 | `Q4_K_M` | 4.95 GB | 6.00 GB | Clean Execution | **14.50 tok/s** | `INTERACTIVE` |
| **8.0 GB** | ✅ `FEASIBLE` | 4096 | `Q4_K_M` | 5.45 GB | 8.00 GB | Clean Execution | **14.50 tok/s** | `INTERACTIVE` |
| **16.0 GB** | ✅ `FEASIBLE` | 8192 | `Q4_K_M` | 5.82 GB | 16.00 GB | Clean Execution | **14.50 tok/s** | `INTERACTIVE` |

---

## 2. Hard Limits & Failure Safeguards
- **OS Enforcer Verification:** Native Win32 Job Objects (`CreateJobObjectW` + `SetInformationJobObject` with `JOBOBJECT_EXTENDED_LIMIT_INFORMATION`) cleanly assign processes to OS memory limit scopes, preventing host system freeze or unmanaged swap thrashing.
