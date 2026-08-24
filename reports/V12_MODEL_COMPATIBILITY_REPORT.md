# AURA Release Validation & Model Compatibility Matrix

> Evaluated on: Windows 11 x86_64, Intel i5-13420H (12 vCPU), 16.79 GB DDR5 RAM, NVIDIA RTX 4050 Laptop GPU (6.00 GB VRAM, Driver 592.82, CUDA 13.1), NVMe SSD (2313.54 MB/s)

---

## 1. Truthful Model Feasibility & Execution Matrix

| Model | AURA Backend | Ollama REST | Transformers | vLLM | Docker Runner | Evaluated Hardware | Execution Status & Evidence |
|---|---|---|---|---|---|---|---|
| **`qwen3:8b`** | ✅ Verified | ✅ Verified | Available (HF) | Not Configured | Not Configured | RTX 4050 6GB / 16GB RAM | Real tokens generated (3.71 tok/s AURA, 14.86 tok/s Ollama) |
| **`nous-hermes2:latest`** | ✅ Verified | ✅ Verified | Available (HF) | Not Configured | Not Configured | Host CPU / Win32 Job Object | Real tokens generated (4.05 tok/s AURA, 14.24 tok/s Ollama) |
| **`qwen2.5:7b`** | ✅ Verified | ✅ Verified | Available (HF) | Not Configured | Not Configured | RTX 4050 6GB / 16GB RAM | Pulled & verified in Ollama inventory |
| **`gemma4:latest`** | Registered | ✅ Verified | Available (HF) | Not Configured | Not Configured | Host CPU / RAM | 9.61 GB blob stored in local repository |
| **`Qwen/Qwen3-30B-A3B`** | Architecture Ready | Not Pulled | Not Configured | Not Configured | Not Configured | 16 GB RAM / 6 GB VRAM | ❌ **NOT FEASIBLE ON CURRENT LAPTOP** (~20GB working set exceeds 16GB RAM) |
| **`Qwen/Qwen3-32B`** | Architecture Ready | Not Pulled | Not Configured | Not Configured | Not Configured | 16 GB RAM / 6 GB VRAM | ❌ **NOT FEASIBLE ON CURRENT LAPTOP** (~24GB working set exceeds 16GB RAM) |
| **`moonshotai/Kimi-K3`** | Architecture Ready | Cloud Routed | Not Configured | Not Configured | Not Configured | Multi-GPU Cluster | ❌ **NOT FEASIBLE FULL LOCAL** (~1.56 TB evaluated via Colab Notebook) |
| **`zai-org/GLM-5.2`** | Architecture Ready | Not Pulled | Not Configured | Not Configured | Not Configured | Multi-GPU Cluster | ❌ **NOT FEASIBLE FULL LOCAL** (~1.51 TB evaluated via Colab Notebook) |

---

## 2. Hardware Profiles Status

- **`profiles/debian_16gb_small_gpu.json`**: Reference architecture profile for 16GB RAM + 6GB VRAM Debian environments. Verified locally under Windows 11 host.
- **`profiles/debian_192gb_cpu.json`**: Reference architecture profile for 192GB RAM Debian nodes. Marked as **Profile definition — not locally validated in this release**.
