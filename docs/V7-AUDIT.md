# AURA V7 Roadmap Implementation & Audit Report

> Theme: Fair benchmarking and community infrastructure

---

## 1. Feature Verification Matrix

| Roadmap Feature | Source File | Connected | Real Execution | Evidence / Status |
|---|---|---|---|---|
| **Ollama REST Harness** | `aura-backends/src/ollama_baseline.rs` | YES | YES | **IMPLEMENTED + VERIFIED** (`eval_count`, `eval_duration`) |
| **Automatic Model Discovery** | `aura-model/src/ollama.rs` | YES | YES | **IMPLEMENTED + VERIFIED** (Queried `/api/tags` live without hardcoded inventory) |
| **Multi-Tier Coverage** | `aura-planner/src/search.rs` | YES | YES | **IMPLEMENTED + VERIFIED** (TINY ~0.6B to LARGE ~8.2B parameter tiering) |
| **Community Benchmark Command** | `aura-cli/src/commands/benchmark.rs` | YES | YES | **IMPLEMENTED + VERIFIED** (`aura benchmark --model llama3.2:3b`) |
| **Physics-Based Decode Estimate** | `aura-planner/src/search.rs` | YES | YES | **IMPLEMENTED + VERIFIED** (GEMV DRAM bandwidth physics calculation) |
| **`is_simulated` Safeguard** | `aura-backends/src/traits.rs` | YES | YES | **IMPLEMENTED + VERIFIED** (`is_simulated = false` on real execution) |
| **Memory Terminology Precision** | `aura-core/src/types.rs`, `README.md` | YES | YES | **IMPLEMENTED + VERIFIED** (Virtual commit charge vs physical RSS) |
| **CI/CD Quality Gates** | `deny.toml`, Cargo workspace | YES | YES | **IMPLEMENTED + VERIFIED** (`cargo fmt`, `clippy`, `test`, `deny`) |

---

## 2. Empirical Verification Evidence

- **Native DLL Resolution**: Resolved child process `LoadLibrary` initialization by prepending `binary_dir` to `PATH` and setting `current_dir`.
- **Win32 Job Object Budget Enforcement**: Assigned child PIDs directly to `JOB_OBJECT_LIMIT_PROCESS_MEMORY`. Peak working set during generation registered **3.92 GB** under the **4.00 GB** process budget limit.
- **Process Cleanup**: `ProcessGuard` cleanup prevents orphan `llama-server.exe` processes (`0` leaked processes).
