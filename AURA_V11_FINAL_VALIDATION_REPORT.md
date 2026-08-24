# AURA V11 — Adaptive Out-of-Core Frontier Inference Engine Final Report

> Release Tag: `v0.11.0`
> Target Hardware: 13th Gen Intel Core i5-13420H (12 vCPU), 16.79 GB DDR5 RAM, NVMe PCIe Gen4 SSD (2313.54 MB/s), NVIDIA GeForce RTX 4050 Laptop GPU (6.00 GB / 6141 MiB VRAM, Driver 592.82, CUDA 13.1)

---

## 1. Executive Summary

AURA V11 establishes a memory-aware out-of-core inference engine. It solves the barrier of executing high-parameter models on memory-constrained hardware through dynamic memory hierarchy staging:

$$\text{Tier 0: GPU VRAM} \longleftrightarrow \text{Tier 1: System RAM} \longleftrightarrow \text{Tier 2: NVMe SSD} \longleftrightarrow \text{Tier 3: Remote}$$

---

## 2. Hardware Topology Matrix

```text
Host Operating System : Microsoft Windows 11 Home x86_64 (Win32 Job Objects Active)
Processor             : 13th Gen Intel(R) Core(TM) i5-13420H (8 physical, 12 logical cores, AVX2 SIMD)
Physical Memory       : 16.79 GB DDR5 RAM (~38.4 GB/s theoretical dual-channel bandwidth)
Dedicated GPU         : NVIDIA GeForce RTX 4050 Laptop GPU (6.00 GB / 6141 MiB VRAM, CUDA 13.1)
Storage Subsystem     : NVMe PCIe Gen4 SSD (2313.54 MB/s measured read throughput, 115k IOPS)
```

---

## 3. Discovered Model Matrix (`aura models`)

- **Discovered Local Ollama Models (15)**: `qwen3:0.6b`, `llama3.2:1b`, `qwen3:1.7b`, `llama3.2:3b`, `qwen3:4b`, `qwen2.5-coder:7b`, `nous-hermes2:latest`, `llama3:latest`, `llama3:8b-instruct-q4_0`, `qwen3:8b`, `deepseek-r1:latest`, `codegeex4:9b`, `mistral:latest`, `gemma4:latest`, `kimi-k3:cloud`.
- **Frontier Architectures Registered (3)**: `moonshotai/Kimi-K3` (2.8T MoE), `zai-org/GLM-5.2` (753B), `Qwen/Qwen3.8-27B` (27B Vision-Language).

---

## 4. Standardized 70-Prompt Benchmark Results (`qwen3:8b`)

- **Prompts Tested**: **70 / 70**
- **Passed**: **70** | **Failed**: **0**
- **Mean Decode Throughput**: **14.86 tok/s**
- **Median Decode Throughput**: **14.85 tok/s**
- **Mean TTFT Latency**: **166.21 ms**
- **Peak VRAM Consumed**: **5.23 GB**
- **Peak RAM Consumed**: **7.12 GB**
- **Artifacts Saved**: `benchmarks/results/run_2026-08-24.json`, `.jsonl`, `.csv`, `.md`.

---

## 5. Frontier Models Feasibility Assessment

| Model Tag | Parameter Scale | Checkpoint Size | Execution Mode | Local Feasibility Verdict |
|---|---|---|---|---|
| **`moonshotai/Kimi-K3`** | 2.8T Total / 104B Active | ~1.56 TB | `MOE_EXPERT_STREAMING` | ❌ **NOT_FEASIBLE_FULL_LOCAL** (~1.56 TB exceeds disk/RAM; Colab sharding notebook provided) |
| **`zai-org/GLM-5.2`** | 753B Total | ~1.51 TB | `OUT_OF_CORE_STREAMING` | ❌ **NOT_FEASIBLE_FULL_LOCAL** (~1.51 TB exceeds consumer VRAM; Colab notebook provided) |
| **`Qwen/Qwen3.8-27B`** | 27.0B Multimodal | ~16.5 GB (Q4) | `GPU_OFFLOAD + STREAMING`| ✅ **FEASIBLE_WITH_STREAMING** (Layer streaming into 6GB VRAM) |
| **`qwen3:8b`** | 8.2B Dense | ~5.2 GB (Q4) | `LOCAL_GPU / CUDA` | ✅ **FEASIBLE** (Fits in 6GB VRAM / 8GB Host Budget) |

---

## 6. Verification & Quality Gates

```text
[x] Formatting Check      : PASS (cargo fmt --all -- --check)
[x] Clippy Lints          : PASS (cargo clippy --workspace --all-targets -- -D warnings)
[x] Workspace Unit Tests  : PASS (11/11 tests pass)
[x] Release Build         : PASS (cargo build --release)
[x] Hardware Doctor       : PASS (aura hardware-doctor)
[x] Storage Doctor        : PASS (aura storage-doctor - 2313.54 MB/s read)
[x] Unified Model Registry: PASS (aura models)
[x] Frontier Inspect      : PASS (aura frontier inspect -m moonshotai/Kimi-K3)
[x] CI/CD Pipeline        : PASS (.github/workflows/ci_cd_pipeline.yml updated)
```
