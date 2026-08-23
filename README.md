# AURA — Adaptive Ultra-Low-Memory Runtime for AI

[![Build](https://github.com/aura-ai/aura/actions/workflows/ci_cd_pipeline.yml/badge.svg)](https://github.com/aura-ai/aura/actions)
[![Tests](https://img.shields.io/badge/tests-11%20passing-brightgreen)](https://github.com/aura-ai/aura/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96%2B-orange)](https://rustup.rs)

**AURA is a hardware-aware memory-budget enforcement and inference orchestration engine for local LLMs on consumer hardware.**

---

## What is AURA?

AURA sits between your application and a local inference backend (Ollama / llama.cpp). It enforces a hard process memory limit using OS-level primitives, automatically selects the best context length and quantization that fits within your budget, and plans model execution before a single byte of model weight is loaded.

AURA does **not** replace llama.cpp or Ollama. It orchestrates them within a defined memory budget.

---

## Why AURA exists

Running a 7B language model on a 16 GB laptop sounds feasible — until you realize:

- The model weights alone are 4.7 GB
- The KV cache adds 0.5–2 GB depending on context length
- The OS, browser, and other processes consume 4–6 GB
- The runtime allocates additional overhead on top of that

Without enforcement, inference swaps to disk, latency becomes unusable, and the system may become unresponsive. AURA makes the memory budget a hard constraint, not a suggestion.

---

## The Problem

| Scenario | Without AURA | With AURA |
|---|---|---|
| 7B model on 16 GB laptop | May exceed 6 GB RSS, slow page-faults | Hard 4 GB limit enforced at OS level |
| Context too large | Silent OOM or crash | Planner reduces context to fit |
| Wrong quantization | User must manually pick | AURA selects Q3_K_S fallback if Q4_K_M won't fit |
| Oversized model (9.6 GB) | Silent failure or OOM | Planner returns INFEASIBLE with clear message |
| Batch automation | Ollama CLI hangs on stdin | AURA runs non-interactively with 100% reliability |

---

## Key Features

- **Hardware-aware planning** — probes CPU, SIMD (AVX2), RAM, and NVMe speed before planning
- **Hard memory enforcement** — Win32 `JOB_OBJECT_LIMIT_PROCESS_MEMORY` on Windows, cgroup v2 on Linux
- **Automatic context tuning** — multi-pass search reduces context 4096→2048→1024 to fit budget
- **Automatic quantization fallback** — suggests Q3_K_S if Q4_K_M won't fit
- **GGUF support** — resolves Ollama blob paths automatically
- **llama.cpp backend** — uses the proven llama.cpp GEMM kernels (not replaced)
- **MoE expert cache** — LRU-based expert eviction for Mixture-of-Experts models
- **Cold-start prefetch** — Win32 `PrefetchVirtualMemory` reduces cold TTFT by ~51%
- **Standalone CLI** — `aura doctor | plan | run | benchmark | audit`

---

## Architecture

```
User prompt
     ↓
aura CLI (clap)
     ↓
Hardware Probe (CPU, AVX2, RAM, NVMe IOPS)
     ↓
Memory Planner (multi-pass context/quant search)
     ↓
Budget Enforcer (Win32 Job Object / cgroup v2)
     ↓
Model Loader (Ollama blob resolver)
     ↓
Expert Cache (LRU, NVMe-backed for MoE)
     ↓
llama.cpp backend (GEMM, AVX2)
     ↓
Generated tokens
```

---

## Supported Hardware

### PROJECT-VERIFIED (this machine)

| Component | Detail |
|---|---|
| CPU | Intel Core i5-13420H |
| Physical cores | 8 |
| Logical cores | 12 |
| SIMD | AVX2 |
| RAM | 16.79 GB |
| Storage | NVMe PCIe Gen4 ~5318 MB/s |
| GPU | None (CPU-only) |
| OS | Windows 11 |

### COMMUNITY-VERIFIED
_Submitted via community benchmark runner — independently reported, not project-verified._
_(Submit yours: see [COMMUNITY_BENCHMARKS.md](docs/COMMUNITY_BENCHMARKS.md))_

### NOT-YET-TESTED
- macOS (code exists, not benchmarked)
- Linux (cgroup v2 code exists, not benchmarked on the reference machine)
- AMD CPUs
- GPU-accelerated inference

---

## Benchmark Results (PROJECT-VERIFIED, CPU-only, 4 GB budget)

> All numbers are from live runs on the verified hardware above.
> `[SIMULATED]` results are planning-layer measurements only — not real inference.
> See [`research/benchmarks/`](research/benchmarks/) for raw data.

### Model Feasibility at 4 GB

| Model | Params | Size | Quant | Max Ctx @ 4GB | 4GB Feasible |
|---|---|---|---|---|---|
| `qwen3:0.6b` | 751M | 0.49 GB | Q4_K_M | 32768 | ✅ |
| `qwen3:1.7b` | 2.0B | 1.27 GB | Q4_K_M | 32768 | ✅ |
| `llama3.2:1b` | 1.2B | 1.23 GB | Q8_0 | 131072 | ✅ |
| `llama3.2:3b` | 3.2B | 1.88 GB | Q4_K_M | 131072 | ✅ |
| `qwen3:4b` | ~4B | ~2.6 GB | Q4_K_M | 131072 | ✅ |
| `mistral:latest` | 7.2B | 4.07 GB | Q4_K_M | 4096 (reduced) | ✅ |
| `qwen2.5-coder:7b` | 7.6B | 4.36 GB | Q4_K_M | 1024 (reduced) | ✅ |
| `llama3:8b` | 8.0B | 4.34 GB | Q4_0 | 1024 (reduced) | ✅ |
| `deepseek-r1:latest` | 8.2B | 4.87 GB | Q4_K_M | 1024 (reduced) | ✅ |
| `qwen3:8b` | 8.2B | 4.87 GB | Q4_K_M | 1024 (reduced) | ✅ |
| `gemma4:latest` | 8.0B* | 8.95 GB | Q4_K_M | — | ❌ INFEASIBLE |

> *gemma4 blob is 8.95 GB. Even at minimum context, the model cannot fit in 4 GB.
> The "8.0B" parameter count is the base model; the GGUF blob is larger due to architecture overhead.

### AURA vs Ollama — Fair REST API Comparison

> Ollama tested via `POST localhost:11434/api/generate` with `num_gpu=0, num_thread=8`.
> Timing extracted from Ollama's `eval_count`/`eval_duration` fields — real nanosecond measurements.
> See [`research/benchmarks/V7_AURA_VS_OLLAMA_FAIR.md`](research/benchmarks/V7_AURA_VS_OLLAMA_FAIR.md).

| Model | Ollama decode (REST) | AURA batch reliability | Winner (use case) |
|---|---|---|---|
| `llama3.2:1b` | Real (see report) | 100% | Ollama (speed) / AURA (automation) |
| `qwen2.5-coder:7b` | Real (see report) | 100% | Ollama (speed) / AURA (4GB guarantee) |
| Any model >5 GB | OOM unconstrained | 100% @ 4GB | **AURA** |

---

## What AURA Does NOT Claim

> This section is intentional. Read it.

- **AURA does not magically run 70B models in 2 GB.** The physics do not permit it.
- **AURA does not beat llama.cpp GEMM kernels** — it uses them.
- **AURA does not provide GPU acceleration** (planned for V9).
- **AURA does not improve model intelligence** — the model's quality is unchanged.
- **Models above ~5 GB are likely infeasible under a strict 4 GB limit** without quantization to Q2/Q3.
- **The 4 GB limit is a virtual commit limit** (Win32 `JOB_OBJECT_LIMIT_PROCESS_MEMORY`), not a physical RAM guarantee. Measured RSS may be 2–5% higher due to memory-mapped GGUF file pages.
- **TurboVec (experimental AVX2 kernel)** was benchmarked at 42.8 GB/s vs llama.cpp's 48.2 GB/s and is **NOT in the production path**.

---

## Reproduce Benchmarks

```bash
# Prerequisites: Rust, Ollama, Python 3.8+

# 1. Build AURA
cargo build

# 2. Start Ollama
ollama serve

# 3. Pull a model
ollama pull llama3.2:3b

# 4. Discover all local models
python benchmarks/discover_models.py

# 5. Run fair AURA vs Ollama benchmark
python benchmarks/run_v7_fair_battle.py

# 6. Run regression tests
cargo test --workspace
```

Full reproduction guide: [docs/REPRODUCIBILITY.md](docs/REPRODUCIBILITY.md)

---

## Community Benchmarks

Contribute results from hardware AURA hasn't been tested on yet:

| Hardware tier | Models of interest |
|---|---|
| 32 GB RAM workstation | 14B – 32B |
| 64 GB RAM workstation | 32B – 72B |
| 24 GB GPU (RTX 3090/4090) | 7B – 70B |
| 48 GB GPU (A6000) | 70B – 100B+ |
| 96 GB+ / multi-GPU | 100B+ / large MoE |

**Single command:**
```bash
python benchmarks/community_runner/run_community_benchmark.py
```

See [docs/COMMUNITY_BENCHMARKS.md](docs/COMMUNITY_BENCHMARKS.md) for instructions.

**Cloud models** (e.g. Kimi K3 at 2.81T parameters) are **not locally benchmarkable** — they are remote services, not local GGUF files. They are labelled `CLOUD_ONLY` and excluded from all local benchmarks.

---

## Roadmap

See [ROADMAP.md](ROADMAP.md).

| Version | Theme |
|---|---|
| **V7** (current) | Fair REST API comparison, multi-tier model coverage, community runner |
| **V8** | Flash Attention, sliding window, speculative decoding |
| **V9** | GPU backend (CUDA/Vulkan/DirectML) |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Development

```bash
cargo build          # debug build
cargo build --release  # release build
cargo test --workspace # run all 11 tests
cargo clippy --workspace
cargo fmt
```

## License

MIT OR Apache-2.0

## FAQ

See [docs/FAQ.md](docs/FAQ.md).
