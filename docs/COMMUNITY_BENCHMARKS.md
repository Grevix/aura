# AURA Community Benchmarks

Submit benchmark results from hardware the AURA project hasn't tested yet.

---

## Why Community Benchmarks?

The AURA reference machine is:
- Intel i5-13420H, 8 cores, 16.79 GB RAM, CPU-only, Windows 11

This covers the **Tiny through Large** model tiers (up to ~8.2B parameters). To benchmark:
- 14B models — needs 8–16 GB RAM
- 32B models — needs 16–32 GB RAM
- 70B models — needs 40+ GB RAM or 24+ GB GPU VRAM
- 100B+ / MoE — needs server hardware or multi-GPU

That's where you come in.

---

## Hardware Tiers

| Tier | RAM / VRAM | Example hardware | Models |
|---|---|---|---|
| Laptop 4 GB budget | 8–16 GB RAM | Any modern laptop | 1B – 8B |
| Workstation 32 GB | 32 GB RAM | Desktop, Mac Studio | 14B – 32B |
| Workstation 64 GB | 64 GB RAM | Threadripper, Xeon | 32B – 72B |
| GPU 24 GB | RTX 3090/4090 | Gaming PC | 7B – 70B |
| GPU 48 GB | RTX 6000 Ada, A6000 | Workstation GPU | 70B – 100B |
| GPU 96 GB+ | H100, A100 x2 | Server | 100B+ / MoE |

---

## Quick Start

```bash
# 1. Start Ollama
ollama serve

# 2. Pull any model (e.g., a 14B for a 32 GB machine)
ollama pull qwen3:14b

# 3. Run the community benchmark
python benchmarks/community_runner/run_community_benchmark.py
```

On Windows:
```powershell
.\benchmarks\community_runner\run_community_benchmark.ps1
```

This produces: `community_result_<hostname>_<timestamp>.json`

---

## Submit Your Result

1. Open a [GitHub Issue](https://github.com/aura-ai/aura/issues/new?template=benchmark.yml)
2. Attach your `community_result_*.json` file
3. Fill in any additional context about your hardware

Your result will be labelled **COMMUNITY REPORTED** and linked from the benchmark table.

---

## What Gets Collected

| Field | Source | Notes |
|---|---|---|
| CPU, RAM, OS | Auto-detected | |
| Ollama version | `ollama --version` | |
| Model digest | Ollama REST API | SHA256 of model blob |
| Ollama decode tok/s | `eval_count`/`eval_duration` from REST API | Real nanosecond precision |
| Ollama TTFT | `load_duration` + `prompt_eval_duration` | |
| AURA wall-clock | `subprocess` timing | |
| AURA is_simulated | Backend flag | `true` if llama.cpp not on PATH |

---

## Labelling Policy

All community results are independently reported and have not been verified by the AURA project team. They are labelled:

> **COMMUNITY REPORTED** — Hardware: [contributor's hardware]. Date: [date]. Not independently verified.

We do not automatically trust community numbers. Schema validation catches obviously invalid results (negative latencies, impossible tok/s, missing fields).

---

## Cloud Models

Models listed in Ollama but served remotely (e.g., **Kimi K3 at 2.81T parameters**) are cloud-routed, not local GGUF files. They **cannot** be included in local benchmark results. The discovery tool automatically excludes them and labels them `CLOUD_ONLY`.

Do not submit results claiming to have run Kimi K3 or other remote-only models locally.

---

## Large Model Suggestions

| Model | Ollama tag | Min RAM | Notes |
|---|---|---|---|
| Qwen3 14B | `qwen3:14b` | ~10 GB | Q4_K_M |
| Qwen3 30B | `qwen3:30b` | ~20 GB | Q4_K_M |
| Llama 3.1 70B | `llama3.1:70b` | ~45 GB | Q4_K_M |
| Qwen2.5 72B | `qwen2.5:72b` | ~45 GB | Q4_K_M |
| Llama 3.1 405B | `llama3.1:405b` | ~240 GB | Not practical on consumer hardware |

Check current availability with `ollama list` or [ollama.com/library](https://ollama.com/library).

Do **not** assume these exact tags exist when you run the benchmark — the tool auto-discovers what is installed.
