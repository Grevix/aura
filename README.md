# AURA — Adaptive Ultra-Low-Memory Runtime for AI

[![Tests](https://img.shields.io/badge/tests-11%20passing-brightgreen)](https://github.com/Grevix/aura/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange)](https://rustup.rs)

**AURA is a hardware-aware memory-budget enforcement and inference orchestration engine for local LLMs on consumer hardware.**

---

## What is AURA?

AURA sits between your application and local inference backends (`llama-server` / `llama-cli`). It enforces a hard process memory budget using OS-level kernel primitives, automatically selects the best context length and quantization that fits within your configured memory ceiling, and orchestrates model execution in real-time.

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

## The Problem

| Scenario | Without AURA | With AURA |
|---|---|---|
| 7B model on 16 GB laptop | Exceeds memory budget, triggers heavy disk page faulting | Hard configured memory budget enforced via Win32 Job Object / Linux cgroups |
| Context window too large | Silent process crash or memory overflow | Planner automatically scales context (e.g. 4096 → 2048 → 1024) |
| Quantization fits tightly | Manual user trial-and-error | Planner evaluates quantization fallback (e.g. Q4_K_M → Q3_K_S) |
| Oversized model (8.95 GB) | Uncontrolled allocation attempt | Planner identifies infeasibility before model loading begins |
| Non-interactive automation | Subprocess TTY buffer deadlocks | Native REST API & CLI execution for 100% reliable execution |

---

## Quick Start

### Installation

#### Prebuilt Binaries
Download prebuilt release archives for your platform from [GitHub Releases](https://github.com/Grevix/aura/releases):
- **Linux (x86_64)**: `aura-v0.1.0-linux-x86_64.tar.gz`
- **Windows (x86_64)**: `aura-v0.1.0-windows-x86_64.zip`
- **macOS (x86_64)**: `aura-v0.1.0-macos-x86_64.tar.gz`

#### Building from Source
Prerequisites: Rust 1.80+ installed via [rustup.rs](https://rustup.rs).

```bash
git clone https://github.com/Grevix/aura.git
cd aura
cargo build --release
```
The compiled binary will be placed at `./target/release/aura` (`aura.exe` on Windows).

---

## Usage & CLI Reference

AURA provides a unified command line interface for hardware probing, execution planning, budget-enforced inference, benchmarking, and release audit evaluation.

```bash
# 1. Probe host hardware capabilities, CPU SIMD, RAM, and NVMe IOPS
aura doctor

# 2. Generate a hardware-aware execution plan for a model under a 4GB budget
aura plan --model llama3.2:3b --memory 4G

# 3. Launch budget-enforced real inference execution
aura run --model llama3.2:3b --memory 4G --prompt "Explain quantum computing in three sentences."

# 4. Speculative decoding with draft model
aura run --model qwen3:8b --draft-model qwen3:0.6b --memory 4G --prompt "Explain virtual memory."

# 5. Execute automated benchmark suite and generate JSON telemetry
aura benchmark --model llama3.2:3b --out aura-benchmark.json

# 6. Evaluate the 10-tier release audit gate
aura audit --out audit.json
```

### Example Real Output

```text
🚀 Launching AURA Budget-Enforced Execution Engine...

=== GENERATION OUTPUT ===
Quantum computing is a new way of processing information that uses the principles of quantum mechanics to perform calculations. Unlike classical computers, which use bits to store and process information, quantum computers use quantum bits or qubits, which can exist in multiple states at the same time. This allows quantum computers to solve certain problems much faster than classical computers, making them potentially useful for fields such as cryptography, optimization, and simulation.

=== RUN METRICS & TELEMETRY ===
TTFT Latency   : 374.34 ms
Prefill Speed  : 24.04 tok/s
Decode Speed   : 6.31 tok/s
Peak RSS       : 3.92 GB
Backend        : llama-server
Provenance     : aura_measured
Simulated      : false
Enforcement    : windows_job_object
Speculative    : Disabled
FA2 Status     : Unavailable
Prefetch Hits  : 0 / Misses: 0
Expert Cache   : Hits: 0 / Misses: 0
```

---

## Telemetry Provenance & Integrity

AURA strictly tracks the provenance of every metric to ensure zero fabricated or misleading performance numbers:

| Provenance Label | Description |
|---|---|
| `aura_measured` | Directly measured from a real child `llama-server` process execution |
| `ollama_measured` | Directly extracted from Ollama REST API `/api/generate` response timing fields |
| `planner_estimate` | Analytical memory/decode throughput prediction computed prior to loading weights |
| `simulated` | Synthetic fallback returned only when no backend executable is available on host |

---

## Key Features

- **Real `llama-server` Child Process Manager**: Spawns Ollama bundled `llama-server.exe` / `llama-cli` with working directory DLL loading and dynamic TCP port discovery.
- **Hard OS Memory Enforcement**: Enforces process commit boundaries via Win32 `JOB_OBJECT_LIMIT_PROCESS_MEMORY` on Windows and cgroup v2 `MemoryMax` on Linux directly attached to child process PIDs.
- **Hardware-Aware Planning**: Probes CPU physical cores, AVX2 SIMD, RAM, and NVMe read throughput before allocating execution resources.
- **Dynamic Context Scaling**: Multi-pass planner search automatically scales context windows to stay within configured budgets.
- **Speculative Decoding Architecture**: Evaluates combined target + draft model memory feasibility under the configured budget.
- **Predictive Weight Prefetching**: Win32 `PrefetchVirtualMemory` and Unix `madvise(MADV_WILLNEED)` readahead optimization with telemetry counters.
- **MoE Expert Cache**: Hybrid LFU/LRU frequency-weighted admission policy with hit/miss/eviction metrics.

---

## Architecture

```text
User Request / Prompt
       ↓
   aura CLI
       ↓
 Hardware Prober ──→ Probe CPU / SIMD / RAM / NVMe
       ↓
 Execution Planner ──→ Context & Quantization Search
       ↓
 OS Budget Enforcer ──→ Win32 Job Object / Linux cgroup v2 (Child PID)
       ↓
 Model Resolver ──→ Resolve Ollama GGUF Blob
       ↓
 Expert Cache / VTM ──→ Hybrid LFU/LRU Cache & Async Prefetch
       ↓
 llama-server Child Process ──→ Native AVX2 GEMM Execution via HTTP
```

---

## Supported Platforms

| Platform | Build Status | Enforcement Mechanism | Verified |
|---|---|---|---|
| **Windows 10 / 11 (x86_64)** | ✅ Passing | Win32 `JOB_OBJECT_LIMIT_PROCESS_MEMORY` | **Project Verified** |
| **Ubuntu / Linux (x86_64)** | ✅ Passing | cgroup v2 `MemoryMax` / transient scope | CI Verified |
| **macOS (x86_64 / ARM64)** | ✅ Passing | `mach2` task RSS monitor thread | CI Verified |

---

## Verified 4 GB Benchmark Results

> All tests performed on Intel Core i5-13420H (8 physical / 12 logical cores, AVX2 SIMD, 16.79 GB RAM, PCIe Gen4 NVMe) under a strict 4.0 GB process working-set budget.

### Model Feasibility Matrix

| Model Tag | Parameters | GGUF Size | Quantization | Max Context @ 4GB | Feasible @ 4GB |
|---|---|---|---|---|---|
| `qwen3:0.6b` | 751M | 0.49 GB | Q4_K_M | 32,768 | ✅ Feasible |
| `qwen3:1.7b` | 2.0B | 1.27 GB | Q4_K_M | 32,768 | ✅ Feasible |
| `llama3.2:1b` | 1.2B | 1.23 GB | Q8_0 | 131,072 | ✅ Feasible |
| `llama3.2:3b` | 3.2B | 1.88 GB | Q4_K_M | 131,072 | ✅ Feasible |
| `qwen3:4b` | 4.0B | 2.60 GB | Q4_K_M | 131,072 | ✅ Feasible |
| `mistral:latest` | 7.2B | 4.07 GB | Q4_K_M | 4,096 (scaled) | ✅ Feasible |
| `qwen2.5-coder:7b` | 7.6B | 4.36 GB | Q4_K_M | 1,024 (scaled) | ✅ Feasible |
| `llama3:8b` | 8.0B | 4.34 GB | Q4_0 | 1,024 (scaled) | ✅ Feasible |
| `deepseek-r1:latest` | 8.2B | 4.87 GB | Q4_K_M | 1,024 (scaled) | ✅ Feasible |
| `qwen3:8b` | 8.2B | 4.87 GB | Q4_K_M | 1,024 (scaled) | ✅ Feasible |
| `gemma4:latest` | 8.0B* | 8.95 GB | Q4_K_M | — | ❌ Infeasible |

*Note: GGUF blob for `gemma4:latest` is 8.95 GB due to extended architecture overhead. Even at minimum context, it exceeds the 4.0 GB process budget.*

---

## What AURA Does NOT Claim

- **AURA does not compress arbitrary 70B models into 2 GB.** Memory bounds are governed by hardware limits and model precision.
- **AURA does not replace optimized GEMM kernels** — it wraps and orchestrates llama.cpp's production AVX2 kernels.
- **AURA does not fabricate performance telemetry** — every metric includes explicit provenance tracking.
- **AURA does not alter model output intelligence** — token generation quality is identical to the underlying GGUF model artifact.
- **Process RSS may slightly exceed virtual commit limits** — Win32 `JOB_OBJECT_LIMIT_PROCESS_MEMORY` restricts virtual commit charge. Physical RSS may register 2–4% higher due to read-only shared GGUF memory maps.

---

## Reproducing Benchmarks

To reproduce benchmark results locally:

```bash
# 1. Install prerequisites: Rust 1.80+, Python 3.8+, Ollama
# 2. Build AURA
cargo build --release

# 3. Start local Ollama server
ollama serve

# 4. Pull target model
ollama pull llama3.2:3b

# 5. Run real AURA execution
./target/release/aura run --model llama3.2:3b --memory 4G

# 6. Run workspace tests
cargo test --workspace
```

For complete instructions, see [docs/REPRODUCIBILITY.md](docs/REPRODUCIBILITY.md).

---

## Testing

Run the full workspace unit and integration test suite:

```bash
cargo test --workspace
```

Run Clippy lint analysis with strict warning enforcement:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Verify Rust code formatting:
```bash
cargo fmt --all -- --check
```

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on code style, testing requirements, and pull request procedures.

---

## License

This project is dual-licensed under either of:
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)

at your option.
