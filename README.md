# AURA
## Adaptive Out-of-Core Runtime for Frontier AI

[![Tests](https://img.shields.io/badge/tests-11%20passing-brightgreen)](https://github.com/Grevix/aura/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange)](https://rustup.rs)

**AURA is a hardware-aware, out-of-core memory hierarchy and inference orchestration runtime that treats VRAM, RAM, and NVMe storage as a unified memory system.**

---

## 1. What is AURA?

AURA enables large language models (from 7B up to 70B, 753B, and 2.8T architectures) to run on consumer and mid-tier hardware that cannot hold the full model weights in VRAM or RAM simultaneously.

Instead of crashing with Out-Of-Memory (OOM) or triggering unhandled operating system page swapping, AURA orchestrates a four-tier memory pipeline:

$$\text{Tier 0: GPU VRAM} \longleftrightarrow \text{Tier 1: System RAM} \longleftrightarrow \text{Tier 2: NVMe SSD} \longleftrightarrow \text{Tier 3: Remote / Cloud}$$

---

## 2. Architecture & Pipeline

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
  Feasibility Planner ── (Calculates Working Set vs Resident Memory)
           │
           ▼
   Memory Hierarchy
  ┌────────────────────────────────────────────────────────┐
  │  Tier 0: GPU VRAM   ── (Current Layer + Active Experts)│
  │  Tier 1: System RAM ── (Double-Buffered Staging Cache) │
  │  Tier 2: NVMe SSD   ── (Model Weight Shard Repository) │
  │  Tier 3: Remote     ── (Hugging Face / Sharded Bucket) │
  └────────────────────────────────────────────────────────┘
           │
           ▼
  Layer / Expert Streamer ── (Async Prefetch + LRU/LFU ExpertCache)
           │
           ▼
  Execution Backends ────── (CUDA Offload / CPU SIMD / llama.cpp / Ollama)
```

---

## 3. Core Features

- **Adaptive Memory Hierarchy**: Dynamic layer/expert staging between NVMe SSD, DDR5 RAM, and GPU VRAM.
- **MoE Expert Streaming**: Top-K dynamic expert routing that loads only activated expert sub-networks into VRAM.
- **Hardware & Storage Diagnostics**:
  - `aura hardware-doctor`: CPU SIMD, RAM bandwidth, GPU VRAM, and storage bandwidth.
  - `aura storage-doctor`: Measures NVMe sequential read throughput, random 4K IOPS, and prefetch sizing.
  - `aura models`: Discovers models across Ollama, Hugging Face cache, and local GGUF/Safetensors directories.
- **Kernel-Level Memory Enforcement**: Hard process memory budget limits via Win32 Job Objects and Linux cgroup v2.
- **Frontier Model Inspection**: `aura frontier inspect` inspects parameters, active sub-networks, and storage feasibility for models like `moonshotai/Kimi-K3` (2.8T) and `zai-org/GLM-5.2` (753B).

---

## 4. CLI Quick Reference

```bash
# 1. System & Storage Diagnostics
./target/release/aura hardware-doctor
./target/release/aura storage-doctor
./target/release/aura gpu-doctor

# 2. Model Discovery & Inspection
./target/release/aura models
./target/release/aura model-inspect --model qwen3:8b
./target/release/aura frontier inspect --model moonshotai/Kimi-K3
./target/release/aura frontier inspect --model zai-org/GLM-5.2

# 3. Model Execution & Inference
./target/release/aura run --model qwen3:8b --memory 4G --prompt "Explain quantum computing in three sentences."

# 4. Standardized 70+ Prompt Benchmark Matrix
python benchmarks/runners/run_70_prompt_suite.py
```

---

## 5. Benchmark Performance Summary (`qwen3:8b` on RTX 4050 Laptop GPU)

| Metric | Measured Value | Provenance |
|---|---|---|
| **Mean Decode Throughput** | **14.86 tok/s** | `OllamaMeasured` |
| **Median Decode Throughput** | **14.85 tok/s** | `OllamaMeasured` |
| **Mean Time-to-First-Token (TTFT)** | **166.21 ms** | `OllamaMeasured` |
| **Peak VRAM Consumed** | **5.23 GB** | `AuraMeasured` |
| **Peak System RAM Consumed** | **7.12 GB** | `AuraMeasured` |
| **Sequential NVMe Read Speed** | **2313.54 MB/s** | `AuraMeasured` |

---

## 6. License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
