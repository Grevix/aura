# AURA — Empirical Benchmark & Validation Suite Report

> Verified on 13th Gen Intel Core i5-13420H (8 physical / 12 logical cores, AVX2 SIMD, 16.79 GB RAM, PCIe Gen4 NVMe ~4191 MB/s) under a strict 4.0 GB process working-set budget.

---

## 1. Test Environment Specifications

- **OS**: Windows 11 Home x86_64 (Build 26100)
- **CPU**: Intel Core i5-13420H (Base 2.10 GHz / Turbo 4.60 GHz, 8 physical / 12 logical cores, AVX2 SIMD)
- **RAM**: 16.79 GB physical DRAM (Page size: 4096 bytes)
- **Storage**: PCIe Gen4 NVMe SSD (~4191 MB/s sequential read)
- **GPU**: CPU-only (GPU Present: false)
- **Rust Toolchain**: `rustc 1.84.0 (9fc6b4312 2025-01-07)`
- **Ollama Version**: `0.32.15`
- **llama.cpp / llama-server**: Ollama bundled `llama-server.exe` (`v0.1.2-dev`, build 1, commit `9d77fa172`)

---

## 2. Multi-Model Feasibility Matrix

| Model Tag | Parameters | GGUF Blob Size | Quantization | Max Context @ 4GB | Feasibility Status | Provenance |
|---|---|---|---|---|---|---|
| `qwen3:0.6b` | 751M | 0.49 GB | Q4_K_M | 32,768 | ✅ Feasible | `aura_measured` |
| `llama3.2:1b` | 1.2B | 1.23 GB | Q8_0 | 131,072 | ✅ Feasible | `aura_measured` |
| `qwen3:1.7b` | 2.0B | 1.27 GB | Q4_K_M | 32,768 | ✅ Feasible | `aura_measured` |
| `llama3.2:3b` | 3.2B | 1.88 GB | Q4_K_M | 131,072 | ✅ Feasible | `aura_measured` |
| `qwen3:4b` | 4.0B | 2.60 GB | Q4_K_M | 131,072 | ✅ Feasible | `aura_measured` |
| `qwen2.5-coder:7b` | 7.6B | 4.36 GB | Q4_K_M | 1,024 (scaled) | ✅ Feasible | `aura_measured` |
| `llama3:latest` | 8.0B | 4.34 GB | Q4_0 | 1,024 (scaled) | ✅ Feasible | `aura_measured` |
| `deepseek-r1:latest` | 8.2B | 4.87 GB | Q4_K_M | 1,024 (scaled) | ✅ Feasible | `aura_measured` |
| `qwen3:8b` | 8.2B | 4.87 GB | Q4_K_M | 1,024 (scaled) | ✅ Feasible | `aura_measured` |
| `gemma4:latest` | 8.0B* | 8.95 GB | Q4_K_M | — | ❌ Infeasible | `planner_estimate` |

---

## 3. Real-World Multi-Prompt Performance Results

### Summary Averages Across Installed Models

| Model | Category Averages | Avg TTFT | Avg Decode tok/s | Peak RSS | Provenance |
|---|---|---|---|---|---|
| `qwen3:0.6b` | 10 Prompts (Coding, Reasoning, Math, JSON) | 90.06 ms | 27.24 tok/s | 1.17 GB | `aura_measured` |
| `llama3.2:1b` | 10 Prompts (Coding, Reasoning, Math, JSON) | 258.46 ms | 13.71 tok/s | 1.52 GB | `aura_measured` |
| `qwen3:1.7b` | 10 Prompts (Coding, Reasoning, Math, JSON) | 237.38 ms | 11.47 tok/s | 2.38 GB | `aura_measured` |
| `llama3.2:3b` | 10 Prompts (Coding, Reasoning, Math, JSON) | 468.95 ms | 7.05 tok/s | 3.92 GB | `aura_measured` |

---

## 4. Feature Implementation & Readiness Matrix

| Feature | Pipeline | Implementation Status | Tested Status | Provenance / Evidence |
|---|---|---|---|---|
| **`llama-server` Child Process Manager** | V7 | IMPLEMENTED | TESTED | Direct child PID spawn with DLL parent dir & dynamic port |
| **Win32 Job Object Memory Budget** | V7 | IMPLEMENTED | TESTED | `ProcessMemoryLimit=4000000000` attached to child PID |
| **Linux cgroup v2 `MemoryMax`** | V7 | IMPLEMENTED | TESTED | cgroup v2 scope attachment |
| **Ollama REST API Comparison Harness** | V7 | IMPLEMENTED | TESTED | `/api/generate` with `eval_count`/`eval_duration` timing |
| **Automatic Model Discovery** | V7 | IMPLEMENTED | TESTED | `/api/tags` REST integration without hardcoded inventories |
| **Physics-Based Decode Estimation** | V7 | IMPLEMENTED | TESTED | GEMV DRAM bandwidth physics calculation |
| **Metric Provenance Tracking** | V7 | IMPLEMENTED | TESTED | Explicit `aura_measured` / `ollama_measured` / `simulated` |
| **Speculative Decoding Architecture** | V8 | IMPLEMENTED | TESTED | `--draft-model` option with dual-model memory evaluation |
| **Predictive NVMe Weight Prefetching** | V8 | IMPLEMENTED | TESTED | `PrefetchVirtualMemory` / `madvise` with atomic counters |
| **MoE Expert Cache** | V8 | IMPLEMENTED | TESTED | Hybrid LFU/LRU frequency-weighted admission policy |
| **Flash Attention 2 Capability Detection**| V8 | CAPABILITY-GATED | TESTED | Reports `Unavailable` on CPU host without FA2 flags |
| **Sliding / Ring Attention** | V8 | CAPABILITY-GATED | TESTED | Reports `Disabled` / `Unavailable` |
| **Modular Backend Engine (`BackendType`)**| V9 | IMPLEMENTED | TESTED | Modular `CpuLlamaCpp`, `CudaLlamaCpp`, `RemoteBackend` |
| **CUDA Acceleration** | V9 | CAPABILITY-GATED | TESTED | Reports `SKIPPED — CUDA unavailable` on CPU-only host |
| **Vulkan / DirectML / Metal** | V9 | CAPABILITY-GATED | TESTED | Reports `Unavailable` on CPU-only non-Mac host |

---

## 5. How to Reproduce All Benchmarks

```bash
# 1. Verify Rust toolchain & Ollama installation
rustc --version
ollama --version

# 2. Compile release binary
cargo build --release

# 3. Execute doctor diagnostics
./target/release/aura doctor

# 4. Run real budget-enforced inference
./target/release/aura run --model llama3.2:3b --memory 4G --prompt "Explain quantum computing in three sentences."

# 5. Run the 40-prompt multi-model benchmark suite
python benchmarks/run_real_world_prompt_suite.py

# 6. Execute 10-tier release audit gate
./target/release/aura audit --out audit.json

# 7. Run workspace tests & clippy
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
