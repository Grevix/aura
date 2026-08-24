# AirLLM-Inspired Out-of-Core Architecture Analysis & Reverse-Engineering

> Deep Systems Analysis of Layer-Wise Weight Streaming, Double-Buffered Prefetching, and MoE Expert Caching

---

## 1. Executive Summary

AirLLM demonstrated that inference of high-parameter language models (e.g., 70B, 405B) on memory-constrained consumer GPUs (4 GB - 8 GB VRAM) is achievable by decomposing monolithic model execution into sequential, disk-backed layer/module stages.

AURA adopts these architectural principles to establish a four-tier memory hierarchy:

$$\text{Tier 0: GPU VRAM} \longleftrightarrow \text{Tier 1: System RAM} \longleftrightarrow \text{Tier 2: NVMe SSD} \longleftrightarrow \text{Tier 3: Remote / Cloud}$$

---

## 2. Reverse-Engineering AirLLM Core Mechanics

Based on the forensic analysis of `airllm_base.py` and `utils.py`:

1. **Meta-Device Construction (`accelerate.init_empty_weights`)**:
   - The entire Hugging Face `AutoModelForCausalLM` graph is instantiated on PyTorch's `meta` device.
   - Initial memory footprint is **0 bytes** of physical VRAM and RAM.
2. **Pre/Post Forward Hook Orchestration**:
   - **Pre-Forward Hook**: Intercepts the forward pass before each layer (embeddings, decoder blocks, norm, lm_head), reads the specific layer's safetensors shard from disk/RAM into pinned memory, and moves it to `cuda:0`.
   - **Post-Forward Hook**: Immediately returns the module's parameters to `meta` and triggers memory cleanup (`clean_memory` / `malloc_trim`), freeing the VRAM for the next layer.
3. **Double-Buffered Asynchronous Prefetching**:
   - A background `ThreadPoolExecutor` worker thread loads layer $N+1$ from NVMe storage into pinned host RAM while layer $N$ is actively computing on the GPU tensor cores.
4. **Selective MoE Expert Tensor Streaming**:
   - For Mixture-of-Experts architectures, AirLLM uses `safetensors.safe_open` to dynamically extract only the specific routed top-K expert keys for that layer, rather than loading the entire multi-gigabyte MoE layer.

---

## 3. Physical Storage Bandwidth Boundaries ($W/B$)

The fundamental physical constraint of out-of-core streaming is the weight-transfer bottleneck:

$$\text{Minimum Latency per Token} \ge \frac{\text{Active Parameters per Token (Bytes)}}{\text{Sustainable Storage Throughput (Bytes/s)}}$$

| Model Size / Scale | Precision / Quant | Working Set per Token | NVMe Bandwidth (3.5 GB/s) | SATA SSD (500 MB/s) | Feasibility on 16GB Laptop |
|---|---|---|---|---|---|
| **7B - 8B Dense** | 4-bit (GGUF Q4_K_M) | ~4.5 GB | ~1.28 s / tok | ~9.0 s / tok | ✅ **FEASIBLE (VRAM/RAM Resident)** |
| **27B - 32B Dense** | 4-bit (GGUF Q4_K_M) | ~18.0 GB | ~5.14 s / tok | ~36.0 s / tok | ⚠️ **STREAMABLE (Slow Interactive)** |
| **70B Dense** | 4-bit (GGUF Q4_K_M) | ~38.0 GB | ~10.8 s / tok | ~76.0 s / tok | ⚠️ **STREAMABLE (Non-Interactive)** |
| **DeepSeek V4-Flash** | 13B Active / 284B Total | ~7.5 GB | ~2.14 s / tok | ~15.0 s / tok | 🔬 **RESEARCH TARGET** |
| **Kimi K3 (2.8T MoE)**| 104B Active / 2.8T Total| ~52.0 GB (Active) | ~14.8 s / tok | ~104.0 s / tok | ❌ **NOT FEASIBLE FULL LOCAL** (~1.56 TB Disk) |

---

## 4. Key Distinctions Between AURA and AirLLM

1. **Native Rust Control Plane**: AURA's core scheduling, hardware probing, and GGUF resolution run entirely in native Rust.
2. **OS Kernel Memory Enforcement**: AURA directly attaches child processes to **Win32 Job Objects (`JOB_OBJECT_LIMIT_PROCESS_MEMORY`)** on Windows and **Linux cgroup v2 (`MemoryMax`)**, preventing runaway allocations.
3. **Deterministic Cloud Validation Suite**: AURA integrates `benchmarks/notebooks/AURA_Frontier_Validation.ipynb` and `benchmarks/runners/frontier_validation_runner.py` for cloud-based verification of multi-hundred-billion parameter models with zero synthetic metrics.
