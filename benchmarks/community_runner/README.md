# AURA Community Benchmark Runner

Contribute benchmark results from **any hardware** — from 8 GB laptops to 96 GB GPU servers.

## Requirements

- Python 3.8+
- [Ollama](https://ollama.com) installed and running (`ollama serve`)
- At least one local model installed (`ollama pull llama3.2:1b`)
- AURA built (`cargo build` in the repo root)

## Quick Start

```bash
# 1. Start Ollama (if not already running)
ollama serve

# 2. Pull a model
ollama pull llama3.2:3b

# 3. Run the benchmark
python benchmarks/community_runner/run_community_benchmark.py
```

This produces a file: `community_result_<hostname>_<timestamp>.json`

## Submit Your Results

1. Go to [GitHub Issues → AURA Benchmark Submission](https://github.com/aura-ai/aura/issues/new?template=benchmark.yml)
2. Attach your JSON file
3. Your result will be labelled **COMMUNITY REPORTED** (not automatically verified)

## What Gets Measured

| Metric | Method |
|---|---|
| Ollama decode tok/s | From `eval_count`/`eval_duration` fields in Ollama REST response — real timing |
| Ollama TTFT | From `load_duration` + `prompt_eval_duration` fields |
| AURA wall-clock | Process start-to-finish via `aura run` |
| Memory (AURA) | Win32 Job Object 4 GB configured limit |
| Memory (Ollama) | Natural process RSS (not constrained by runner) |

## Important Disclaimers

- **AURA inference is `is_simulated=true`** unless `llama-cli` or `llama-server` is on your PATH. Simulated runs measure AURA's planning latency, not real token generation.
- **Ollama timing is real** — the REST API returns exact nanosecond durations per token.
- Your result is labelled **COMMUNITY REPORTED** not **EMPIRICALLY VERIFIED BY PROJECT**.

## Hardware Tiers We're Looking For

| Tier | Target hardware | Models of interest |
|---|---|---|
| Laptop 4 GB | 8–16 GB RAM, CPU only | 1B – 8B |
| Laptop 8 GB | 16 GB RAM, CPU only | 7B – 13B |
| Workstation 32 GB | 32 GB RAM, CPU | 14B – 32B |
| Workstation 64 GB | 64 GB RAM | 32B – 72B |
| GPU 24 GB | RTX 3090 / 4090 | 7B – 70B |
| GPU 48 GB | A6000 / H100 | 70B – 100B+ |
| GPU 96 GB+ | Multi-GPU | 100B+ / MoE |

## Windows PowerShell Wrapper

```powershell
.\benchmarks\community_runner\run_community_benchmark.ps1
```

See [`run_community_benchmark.ps1`](run_community_benchmark.ps1).
