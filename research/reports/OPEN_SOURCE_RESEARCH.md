# Open-Source Ecosystem Forensic Survey

**Date:** 23 August 2026  
**Scope:** Survey of open-source LLM inference engines and low-memory execution techniques relevant to AURA.

---

## Ecosystem Analysis Matrix

| Project | Core Technique | Performance / Memory Benefit | AURA Relevance | License | Adaptation Feasibility |
|---|---|---|---|---|---|
| **llama.cpp / GGML** | GGUF quantized mmap execution, AVX2/AVX-512 GEMM | High CPU speed, zero copy load, multi-quant (Q2–Q8) | **CRITICAL** (Core backend) | MIT | **INSTALLED** |
| **PowerInfer** | High-locality neuron activation prediction | Offloads only hot neurons to VRAM (saves 70–80% VRAM) | **HIGH** (Predictive weight paging) | Apache-2.0 | **ADAPTABLE** |
| **FlexGen** | Throughput-oriented linear programming offloading (RAM + SSD) | High throughput batch offloading for 175B models | **HIGH** (Multi-tier tensor placement) | Apache-2.0 | **ADAPTABLE** |
| **LLM in a Flash** (Apple) | Flash-memory sparse weight streaming & activation sparsity | Streams weights directly from NVMe with activation prediction | **CRITICAL** (Low-RAM NVMe streaming) | Research | **ADAPTABLE** |
| **vLLM / PagedAttention** | Non-contiguous virtual memory allocation for KV cache | Eliminates KV cache fragmentation (96%+ memory utilization) | **HIGH** (KV Cache Virtualization) | Apache-2.0 | **ADAPTABLE** |
| **llamafile** | Single-file executable with embedded weights & cosmic libc | Zero-dependency portable execution | **MEDIUM** (Packaging) | Apache-2.0 | **INSPIRATIONAL** |
| **MLX (Apple Silicon)** | Unified memory model arrays & Metal kernel compilation | Zero-copy host/GPU array sharing on M-series | **HIGH** (Unified memory path) | MIT | **ADAPTABLE** |

---

## Detailed Tech Breakdown: PowerInfer & LLM in a Flash

1. **PowerInfer (Predictive Activation Sparsity):**
   - **Mechanism:** Exploits non-uniform activation distributions in LLMs. ~20% of neurons ("hot neurons") process 80%+ of token activations.
   - **Application to AURA:** Pre-load hot neurons into physical RAM while leaving cold neurons on NVMe storage, reducing memory bandwidth demand by up to 75%.

2. **LLM in a Flash (Apple Research):**
   - **Mechanism:** Combines sliding-window weight retention with activation-guided prefetching directly from NVMe flash storage.
   - **Application to AURA:** Avoids reading unchanged feed-forward weights across prompt tokens, cutting NVMe read volume in half.
