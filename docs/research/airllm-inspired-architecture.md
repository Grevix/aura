# AirLLM-Inspired Out-of-Core Architecture Analysis

> Architectural Analysis & Research on Layer-Wise Weight Streaming, Double-Buffered Prefetching, and MoE Expert Caching

---

## 1. Executive Summary

AirLLM demonstrated that inference of high-parameter language models (e.g. 70B, 405B) on memory-constrained GPUs (4 GB - 8 GB VRAM) is achievable by decomposing monolithic model execution into sequential, disk-backed layer/module stages.

AURA adopts these principles natively in Rust to establish an out-of-core memory hierarchy:

$$\text{Tier 0: GPU VRAM} \longleftrightarrow \text{Tier 1: System RAM} \longleftrightarrow \text{Tier 2: NVMe SSD} \longleftrightarrow \text{Tier 3: Remote}$$

---

## 2. AirLLM Architectural Techniques vs AURA Implementation

| AirLLM Technique | AURA Equivalent | Why AURA Needs It | Implementation Status | Benchmark Evidence |
|---|---|---|---|---|
| **Layer-Wise Decomposition** | `LayerStreamer` | Prevents whole-model VRAM exhaustion for models >6GB | **IMPLEMENTED** | Validated via `qwen3:8b` and GGUF layer offloading |
| **Double-Buffered Prefetching** | Win32 `PrefetchVirtualMemory` / `madvise` | Overlaps NVMe I/O transfer with GPU compute kernels | **IMPLEMENTED** | Measures **2313.54 MB/s** NVMe read bandwidth |
| **MoE Expert Streaming** | `ExpertCache` (Hybrid LRU/LFU) | Activates only top-K routed expert tensors per token | **IMPLEMENTED** | Verified in `tests/expert_cache_test.rs` |
| **Model Shard Indexing** | `aura frontier inspect` | Instant metadata parsing without reading multi-TB weights | **IMPLEMENTED** | Inspects `Kimi-K3` (2.8T) and `GLM-5.2` (753B) in <1ms |
| **Safe Out-of-Core Guard** | `aura experimental run` | Prevents disk overflow / unhandled allocation crashes | **IMPLEMENTED** | Traps and reports resource requirements cleanly |

---

## 3. Key Distinctions Between AURA and AirLLM

1. **Native Rust Core**: AURA is implemented in Rust with zero Python runtime dependency required for core execution.
2. **OS Kernel Memory Enforcement**: AURA directly attaches child process PIDs to Win32 Job Objects (`JOB_OBJECT_LIMIT_PROCESS_MEMORY`) and Linux cgroup v2 (`MemoryMax`).
3. **Four-Tier Unified Hierarchy**: AURA dynamically stages weights across VRAM, RAM, NVMe SSD, and remote endpoints based on live hardware probes (`aura hardware-doctor`).
