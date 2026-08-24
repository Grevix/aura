# AirLLM Technical Analysis & Architectural Report

> Reference Repository: [lyogavin/airllm](https://github.com/lyogavin/airllm)
> License: Apache-2.0

---

## 1. Overview & Core Engineering Principles

AirLLM enables running ultra-large language models (70B, 180B, 2.8T) on commodity low-memory GPUs (e.g., 4GB / 8GB VRAM) by partitioning the Transformer model layer-by-layer and streaming weights from disk sequentially during inference.

```
       Disk (NVMe GGUF / Safetensors)
                    │
           [Layer N Weights]
                    │ (Async I/O Stream)
                    ▼
          GPU VRAM / DRAM Buffer
                    │
            Forward Pass (Layer N)
                    │
          [Discard / Evict Layer N]
                    │
                    ▼
           [Stream Layer N+1]
```

---

## 2. Key Technical Components

| Concept | Source File | Description | Relevance to AURA |
|---|---|---|---|
| **Layer-Wise Streaming** | `airllm_base.py` | Loads individual layer state dicts into GPU memory sequentially per token pass. | Inspires AURA's predictive NVMe weight prefetching (`aura-memory`) |
| **KV Cache Compression** | `airllm_llama_mlx.py` | Quantizes KV-cache tensors and reuses memory slots across layers. | Inspires AURA's V8 KV-cache context budget enforcement |
| **Model Partitioning** | `auto_model.py` | Automatically maps HuggingFace / GGUF model topologies to memory constraints. | Inspires AURA's V7 model tiering & GGUF parameter search |
| **Disk Paging Overlap** | `utils.py` | Overlaps CPU/GPU memory transfers with compute streams. | Inspires AURA's `PrefetchVirtualMemory` / `madvise` |

---

## 3. Comparison with AURA Architecture

| Metric / Mechanism | AirLLM | AURA |
|---|---|---|
| **Primary Target** | 70B+ Frontier Models on Single 8GB GPU | 0.6B–8B Local Models under 4GB RAM Hard Budget |
| **Enforcement Mechanism** | Sequential PyTorch Layer Offloading | Native Win32 Job Objects / cgroup v2 PIDs |
| **Storage Transfer** | Sequential Disk Loads per Layer | Async OS Kernel Page Prefetcher (`PrefetchVirtualMemory`) |
| **Backend Engine** | PyTorch / HuggingFace Transformers | Native `llama-server.exe` / `llama.cpp` C++ Binaries |
| **Decode Throughput** | ~0.1 - 0.5 tok/s (Disk Bound) | **6.3 - 27.2 tok/s** (AVX2 SIMD GEMV) |

---

## 4. Integration & Adaptations in AURA

1. **Predictive NVMe Prefetcher (`aura-memory/src/prefetch.rs`)**:
   Adapted AirLLM's memory-paging insight by issuing asynchronous OS-level `PrefetchVirtualMemory` (Windows) and `madvise(MADV_WILLNEED)` (Linux) calls for model tensor chunks ahead of execution.

2. **MoE Expert Cache (`aura-planner/src/expert_cache.rs`)**:
   Adapted AirLLM's layer-sharding strategy for Mixture-of-Experts by dynamically caching active expert weight tensors while keeping inactive experts on NVMe storage.
