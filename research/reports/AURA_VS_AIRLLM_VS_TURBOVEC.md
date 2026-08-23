# Architectural Matrix: AURA vs AirLLM vs TurboVec

**Date:** 23 August 2026  
**Focus:** Comparative Systems Evaluation & Technology Synthesis  

---

## Technical Comparison Matrix

| Capability | AURA 2.0 | AirLLM | TurboVec | Best-in-Class Approach |
|---|---|---|---|---|
| **Primary Goal** | Hardware-budgeted LLM inference optimizer | Multi-gigabyte LLM streaming on GPU | Ultra-fast quantized vector search | **AURA Architecture V3** |
| **Language** | Rust / Win32 / C++ / Python | 100% Python | Rust / Assembly | **Rust Runtime + C SIMD** |
| **Hardware Target** | CPU-only, low-RAM laptops | CUDA / Metal GPUs | CPU AVX-512 / AVX2 / NEON | **Heterogeneous CPU/GPU** |
| **Memory Planning** | Preflight analytical search & Job Objects | Meta-device allocation | Fixed block array allocation | **AURA Hardware Preflight** |
| **Layer Streaming** | OS native `mmap` page management | PyTorch forward pre/post hooks | N/A (InMemory Index) | **Async Direct I/O Layer Pager** |
| **Disk-Backed Inference** | Zero-copy `mmap` page cache | Safetensors layer sharding | Disk `.tv` index file load | **Virtual Tensor Memory (VTM)** |
| **Quantization** | GGUF (Q4_K_M, Q3_K_S, IQ2) | bitsandbytes / compressed-tensors | 2-4 bit TurboQuant | **GGUF + TurboQuant Nibble SIMD** |
| **CPU SIMD Kernels** | `llama.cpp` AVX2 backend integration | PyTorch CPU fallback (slow) | Hand-tuned AVX-512BW / NEON | **TurboVec Nibble Lookup Kernels** |
| **MoE Handling** | Active expert tracking | Per-expert forward hook streaming | N/A | **AirLLM Per-Expert Hooking** |
| **Benchmarking** | Machine-readable `aura-benchmark.json` | `LayeredProfiler` stdout prints | Criterion benchmark suites | **AURA Telemetry Database** |

---

## Technology Transfer Blueprint

1. **Adopt from AirLLM:**
   - Per-expert submodule hooking (`_setup_expert_streaming`) to stream only active MoE experts instead of loading entire layer weights.
   - Pinned memory background worker prefetching (`ThreadPoolExecutor` / async ring buffer).

2. **Adopt from TurboVec:**
   - 4-bit nibble-split SIMD lookup tables (`vpshufb` / ARM `tbl1`) for accelerating CPU-side quantized matrix-vector multiplications.
   - Cache-aligned bit-plane data packing and dynamic tile work-stealing thread scheduling.

3. **Retain from AURA:**
   - Hardware-aware preflight feasibility planner and OS job limit memory enforcement (`Job Objects` / `cgroups v2`).
