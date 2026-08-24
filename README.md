# AURA
## Adaptive Memory-Aware Runtime for LLM Inference

[![Tests](https://img.shields.io/badge/tests-11%20passing-brightgreen)](https://github.com/Grevix/aura/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange)](https://rustup.rs)

> **Release Statement**:  
> AURA V12 is a release candidate for adaptive, memory-aware local LLM inference. Its currently verified execution path supports real local inference through Ollama/llama.cpp, GPU offload on NVIDIA CUDA hardware, CPU execution, memory-budget enforcement, model discovery, telemetry, and standardized benchmark execution. Frontier-scale models such as GLM-5.2 and Kimi-K3 are currently classified as research/experimental targets rather than verified full-local workloads. Their out-of-core execution architecture is under active development and is not represented as successfully executed until reproducible end-to-end inference is demonstrated.

---

## 1. Verified Architecture & Pipeline

```text
Application / User Request
           │
           ▼
        AURA CLI
           │
           ▼
    Hardware Doctor ─── (Probe CPU SIMD, DRAM Bandwidth, GPU VRAM, NVMe IOPS)
           │
           ▼
  Feasibility Planner ── (Calculates Working Set vs Resident Memory Budget)
           │
           ▼
  Memory Enforcement ── (Win32 Job Objects / Linux cgroup v2)
           │
           ▼
  Execution Backends ── (Native llama-server child process / Ollama REST API)
           │
           ▼
    Terminal Output ─── (Real-time Token Rendering + Provenance Telemetry)
```

---

## 2. Core Capabilities & Verification Status

| Feature / Subsystem | Status | Description & Evidence |
|---|---|---|
| **Process-Enforced Inference** | **VERIFIED** | Spawns `llama-server.exe` child process under Win32 Job Object / Linux cgroup memory limit |
| **GPU Offloading (CUDA)** | **VERIFIED** | Automatic NVIDIA GeForce RTX 4050 detection & layer offload with 2.57x–3.23x speedup |
| **CPU SIMD Execution** | **VERIFIED** | Multi-threaded AVX2/AVX-512 CPU execution path for non-GPU environments |
| **Hardware & Storage Doctor** | **VERIFIED** | `aura hardware-doctor` and `aura storage-doctor` (2313.54 MB/s NVMe read) |
| **Unified Model Discovery** | **VERIFIED** | `aura models` and `aura ollama-list` dynamic discovery across 15 local models |
| **Standardized 70-Prompt Suite** | **VERIFIED** | 70/70 real prompts executed on `qwen3:8b` (14.86 tok/s mean decode throughput) |
| **Frontier Model Safeguards** | **VERIFIED** | `aura frontier inspect` evaluates 2.8T MoE and 753B structures without fatal OOM |
| **Qwen3.8-27B Multimodal** | **ARCHITECTURALLY SUPPORTED** | Target for layer streaming; pending empirical end-to-end verification |
| **Kimi-K3 / GLM-5.2 Full Local**| **EXPERIMENTAL / NOT FEASIBLE LOCAL** | Evaluated via Colab sharding notebooks; ~1.5 TB checkpoints require cluster resources |

---

## 3. CLI Quick Reference

```bash
# 1. System & Storage Diagnostics
./target/release/aura hardware-doctor
./target/release/aura storage-doctor
./target/release/aura gpu-doctor

# 2. Unified Model Discovery & Inspection
./target/release/aura models
./target/release/aura model-inspect --model qwen3:8b
./target/release/aura frontier inspect --model moonshotai/Kimi-K3
./target/release/aura frontier inspect --model zai-org/GLM-5.2

# 3. Model Execution & Inference (Real Token Rendering)
./target/release/aura run --model qwen3:8b --memory 4G --prompt "What is quantum computing?"

# 4. Standardized 70+ Prompt Benchmark Matrix
python benchmarks/runners/run_70_prompt_suite.py
```

---

## 4. Empirical Benchmark Performance (`qwen3:8b` on RTX 4050 Laptop GPU)

```text
Model              : qwen3:8b (8.2B GGUF Q4_K_M)
Total Prompts      : 70 / 70 Standardized Prompts
Pass Rate          : 100% (70 Passed / 0 Failed)
Mean Decode Speed  : 14.86 tok/s
Median Decode Speed: 14.85 tok/s
Mean TTFT Latency  : 166.21 ms
Peak VRAM Consumed : 5.23 GB
Peak RAM Consumed  : 7.12 GB
Sequential Read    : 2313.54 MB/s (PCIe Gen4 NVMe)
Provenance         : OllamaMeasured / AuraMeasured (is_simulated = false)
```

---

## 5. Official Hardware Profiles

- **Profile A**: [`profiles/debian_16gb_small_gpu.json`](file:///c:/Users/Aaryan%20Rawat/Pictures/AURA/profiles/debian_16gb_small_gpu.json) (16 GB RAM + RTX 4050 GPU -> `GPU_OFFLOAD / CUDA`)
- **Profile B**: [`profiles/debian_192gb_cpu.json`](file:///c:/Users/Aaryan%20Rawat/Pictures/AURA/profiles/debian_192gb_cpu.json) (192 GB RAM + No GPU -> `CPU_OFFLOAD / RAM_RESIDENT`)

---

## 6. License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
