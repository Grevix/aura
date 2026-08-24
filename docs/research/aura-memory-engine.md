# AURA Memory Engine & Large-Model Inference Architecture

> Research Report on Memory Hierarchies, Layer Streaming, Expert Routing, and Storage Offload

---

## 1. Executive Summary

AURA V10 introduces an adaptive memory hierarchy engine designed to optimize large-language-model inference across heterogeneous consumer and enterprise hardware:

$$\text{VRAM} \longrightarrow \text{RAM} \longrightarrow \text{NVMe SSD} \longrightarrow \text{Remote Storage}$$

Unlike naive whole-model memory loaders that trigger unhandled OOM crashes on memory-constrained hardware, AURA dynamically determines component placement, applies double-buffered asynchronous prefetching, and executes layer/expert streaming where required.

---

## 2. Comparative Architectural Matrix

| Metric / Mechanism | AURA V10 Engine | AirLLM Reference | Ollama / llama.cpp |
|---|---|---|---|
| **Primary Stack** | Rust + Native C++ Backends | Python + PyTorch | C++ (llama.cpp) / Go |
| **Layer-Wise Weight Streaming** | Native async NVMe staging with pinned double-buffering | Python generator layer streaming | Whole-model mmap / GPU resident |
| **MoE Expert Routing** | Dynamic Top-K routing with hybrid LRU/LFU ExpertCache | Sequential expert weight staging | In-memory expert execution |
| **KV Cache Offloading** | Memory-budgeted paged KV cache allocator | CPU KV cache tensors | Unified VRAM / RAM KV buffer |
| **OS Memory Budgeting** | Hard Win32 Job Objects / Linux cgroup v2 limits | None (Unenforced Python memory) | Soft context-scaling |
| **Hardware Autotuning** | Automatic probe (`aura hardware-doctor`) | Manual device mapping | CLI / Environment flags |

---

## 3. Storage Bandwidth Bottleneck Analysis

When streaming layers from NVMe SSD to GPU VRAM for models exceeding host RAM:

$$T_{\text{layer}} = T_{\text{read\_nvme}} + T_{\text{H2D\_transfer}} + T_{\text{gpu\_compute}}$$

With double-buffered prefetching:
$$T_{\text{step}} = \max(T_{\text{read\_nvme}} + T_{\text{H2D\_transfer}}, T_{\text{gpu\_compute}})$$

- If $T_{\text{read\_nvme}} > T_{\text{gpu\_compute}}$, the bottleneck is **STORAGE I/O BANDWIDTH**.
- On PCIe Gen4 NVMe (1370+ MB/s), layer staging achieves sustainable throughput for 8B–27B parameter models while preventing system OOM.

---

## 4. Frontier Model Feasibility Analysis

- **Moonshot AI Kimi-K3 (`moonshotai/Kimi-K3`)**: 2.8T Total parameters / 104B Active. Checkpoint size ~1.56 TB. Classified as `NOT_FEASIBLE_FULL_LOCAL` on single machines; supported via `REMOTE_STREAMED` or Colab cluster.
- **ZAI GLM-5.2 (`zai-org/GLM-5.2`)**: 753B Total parameters. Checkpoint size ~1.51 TB. Classified as `NOT_FEASIBLE_FULL_LOCAL`; requires FP8 quantization or multi-node tensor sharding.
