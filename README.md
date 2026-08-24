# AURA — Adaptive Ultra-Low-Memory Runtime for AI

[![Tests](https://img.shields.io/badge/tests-11%20passing-brightgreen)](https://github.com/Grevix/aura/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange)](https://rustup.rs)

**AURA is a hardware-aware memory-budget enforcement and inference orchestration engine for local LLMs on consumer hardware.**

---

## What is AURA?

AURA sits between your application and local inference backends (`llama-server` / `llama-cli` / Ollama). It enforces a hard process memory budget using OS-level kernel primitives, automatically selects the best context length and quantization that fits within your configured memory ceiling, and orchestrates model execution in real-time.

AURA does **not** replace llama.cpp or Ollama. It orchestrates them as child processes within a defined, non-negotiable process memory boundary.

---

## Why AURA Exists

Running a 7B language model on a 16 GB laptop sounds feasible — until you realize:

- The GGUF model weights alone take 4.0–4.8 GB
- The KV cache adds 0.5–2.0 GB depending on context size
- Operating system, browser, and background processes consume 6–8 GB
- Standard runtimes allocate overhead on top of model weights

Without enforcement, inference triggers OS page swapping to NVMe disk, destroys system responsiveness, and leads to unhandled OOM crashes. AURA makes the memory budget a hard constraint enforced at the operating system kernel level.

---

## Quick Start

### Installation

#### Building from Source
Prerequisites: Rust 1.80+ installed via [rustup.rs](https://rustup.rs).

```bash
git clone https://github.com/Grevix/aura.git
cd aura
cargo build --release
```

---

## Usage & CLI Reference

### 1. Hardware & GPU Diagnostics (`aura doctor` / `aura gpu-doctor`)

Probe physical host hardware, CPU SIMD features, RAM, storage throughput, and GPU acceleration readiness:

```bash
# General CPU, RAM, SIMD, and NVMe IOPS Probe
./target/release/aura doctor

# NVIDIA GPU, VRAM, Driver, and CUDA Readiness Probe
./target/release/aura gpu-doctor
```

#### Example `aura gpu-doctor` Output:
```text
🔍 Running AURA GPU Hardware & Backend Doctor...

GPU Detection
──────────────────────────────────────────────────
NVIDIA GPU        : NVIDIA GeForce RTX 4050 Laptop GPU
CUDA Backend      : NVIDIA CUDA (Driver: 592.82)
VRAM              : 6.00 GB (6141 MiB)
AURA CUDA Backend : READY
```

---

### 2. Execution Planning (`aura plan`)

Generate a memory-budgeted execution plan for a local Ollama or GGUF model:

```bash
./target/release/aura plan --model llama3.2:3b --memory 4G
```

---

### 3. Real Inference Execution (`aura run`)

Launch budget-enforced model execution engine. Model generation text prints directly to terminal:

```bash
./target/release/aura run --model llama3.2:3b --memory 4G --prompt "Explain quantum computing in three sentences."
```

#### Terminal Output Example:
```text
🚀 Launching AURA Budget-Enforced Execution Engine...

=== GENERATION OUTPUT ===
Quantum computing is a new way of processing information that uses the principles of quantum mechanics...

=== RUN METRICS & TELEMETRY ===
TTFT Latency   : 374.34 ms
Prefill Speed  : 24.04 tok/s
Decode Speed   : 6.31 tok/s
Peak RSS       : 3.92 GB
Backend        : llama-server
Provenance     : aura_measured
Simulated      : false
Enforcement    : windows_job_object
```

---

### 4. Automated Benchmarking (`aura benchmark`)

Run real model inference benchmarks and generate schema-validated JSON reports:

```bash
./target/release/aura benchmark --model llama3.2:3b --out aura-benchmark.json
```

---

### 5. Automated Python & Jupyter Audit Infrastructure

Run the complete multi-model multi-prompt benchmark suite:

```bash
# Run local dynamic discovery benchmark suite
python benchmarks/runners/run_local.py

# Or launch Jupyter / Colab Audit Notebook
jupyter notebook benchmarks/notebooks/aura_full_audit.ipynb
```

---

## Architecture & Roadmap Status

| Milestone | Theme | Features | Status |
|---|---|---|---|
| **V7** | Fair Benchmarking | Ollama REST harness (`/api/generate`, `/api/tags`), dynamic discovery, Win32 Job Object / cgroup v2 memory enforcement, `is_simulated` tracking, physics decode estimate | ✅ **100% COMPLETE** |
| **V8** | Memory Footprint Reduction | Speculative decoding (`--draft-model`), predictive NVMe weight prefetching, MoE expert cache (LFU/LRU), FA2 & Sliding Window capability detection | ✅ **VERIFIED** |
| **V9** | Heterogeneous GPU Backends | GPU hardware detection (`aura gpu-doctor`), CUDA offloading engine, modular `BackendType` abstractions (`CpuLlamaCpp`, `CudaLlamaCpp`) | ✅ **VERIFIED** |

---

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
