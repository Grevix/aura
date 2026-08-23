# AURA — Reproducing Benchmarks

This document gives exact commands to reproduce every benchmark result in the project.

---

## Requirements

| Tool | Version | Install |
|---|---|---|
| Rust | 1.70+ | https://rustup.rs |
| Python | 3.8+ | https://python.org |
| Ollama | 0.30+ | https://ollama.com |
| Windows | 10/11 | (for Win32 Job Object enforcement) |

---

## Step 1 — Build AURA

```bash
git clone https://github.com/aura-ai/aura
cd aura
cargo build
```

Verify:
```bash
target/debug/aura.exe --version
target/debug/aura.exe doctor
```

---

## Step 2 — Start Ollama

```bash
# Windows
ollama serve

# Or let Ollama run as a background service (default on Windows)
```

Verify the REST API is responding:
```bash
curl http://localhost:11434/api/tags
```

---

## Step 3 — Install Models

```bash
# Tier coverage for the reference machine (i5-13420H, 16.79 GB, CPU-only)
ollama pull qwen3:0.6b        # 0.49 GB — TINY tier
ollama pull qwen3:1.7b        # 1.27 GB — SMALL tier
ollama pull llama3.2:1b       # 1.23 GB — SMALL tier
ollama pull llama3.2:3b       # 1.88 GB — MEDIUM tier
ollama pull qwen3:4b          # ~2.6 GB  — MEDIUM tier
ollama pull mistral:latest    # 4.07 GB — 4GB_BOUNDARY
ollama pull qwen2.5-coder:7b  # 4.36 GB — LARGE
ollama pull qwen3:8b          # 4.87 GB — LARGE
```

---

## Step 4 — Discover Installed Models

```bash
python benchmarks/discover_models.py
```

This queries `http://localhost:11434/api/tags` and writes:
`research/benchmarks/V7_LOCAL_MODEL_INVENTORY.json`

---

## Step 5 — Run the Fair Benchmark (AURA vs Ollama REST)

```bash
python -X utf8 benchmarks/run_v7_fair_battle.py
```

Outputs:
- `research/benchmarks/V7_RAW_RESULTS.json`
- `research/benchmarks/V7_MODEL_BENCHMARK_MATRIX.csv`

---

## Step 6 — Run Regression Tests

```bash
cargo test --workspace
```

Expected: 11 tests, 0 failures.

---

## Memory Budget Configuration

AURA's hard budget is passed via `--memory`:

```bash
target/debug/aura.exe run --model qwen3:1.7b --memory 4G --prompt "Explain recursion."
target/debug/aura.exe run --model mistral:latest --memory 4G --prompt "Write a sorting algorithm."
target/debug/aura.exe plan --model qwen3:8b --memory 4G
```

---

## Memory Limit Semantics

The `--memory 4G` flag sets a **virtual commit limit** via Win32 `JOB_OBJECT_LIMIT_PROCESS_MEMORY`.
This is not a physical RAM cap. Measured RSS may be 2–5% higher than the configured limit.

On Linux, AURA uses cgroup v2 `memory.limit_in_bytes` (not benchmarked on reference hardware yet).

---

## Verifying Ollama REST Timing

The V7 fair battle harness extracts timing from Ollama's response body:

```json
{
  "eval_count": 142,
  "eval_duration": 9876543210,
  "prompt_eval_count": 27,
  "prompt_eval_duration": 1234567890,
  "load_duration": 500000000
}
```

- `decode_tok_per_sec = eval_count / (eval_duration / 1e9)`
- `ttft_ms = (load_duration + prompt_eval_duration) / 1e6`

These are nanosecond-precision measurements from Ollama's internal timer, not wall-clock estimates.

---

## Community Benchmarks

For hardware AURA has not been tested on:

```bash
python benchmarks/community_runner/run_community_benchmark.py
```

See [docs/COMMUNITY_BENCHMARKS.md](COMMUNITY_BENCHMARKS.md).
