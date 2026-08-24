# AURA Benchmark Suite & Empirical Results

---

## 1. Historical vs Current Benchmark Matrix

### Current Release Benchmark (`2026-08-24`)

| Model | Runtime | Hardware Profile | Prompts | Pass Rate | Mean Decode Speed | TTFT Latency | Peak Working Set | Provenance |
|---|---|---|---|---|---|---|---|---|
| `qwen3:8b` | Ollama CUDA Offload | RTX 4050 6GB / i5-13420H | 70 | 100% (70/70) | 14.86 tok/s | 166.21 ms | 5.23 GB | `ollama_measured` |
| `nous-hermes2:latest` | Ollama CUDA Offload | RTX 4050 6GB / i5-13420H | 10 | 100% (10/10) | 14.24 tok/s | 170.40 ms | 6.07 GB | `ollama_measured` |
| `qwen3:8b` | AURA `llama-server` | Win32 Job Object (4GB) | 1 | 100% (1/1) | 3.46 tok/s | 455.51 ms | 4.92 GB | `aura_measured` |
| `nous-hermes2:latest` | AURA `llama-server` | Win32 Job Object (8GB) | 1 | 100% (1/1) | 4.05 tok/s | 470.21 ms | 7.41 GB | `aura_measured` |

---

## 2. Benchmark Artifacts & Schema

Results are saved under `benchmarks/results/`:
- `run_2026-08-24.json`: Complete JSON payload with prompt ID, category, TTFT, decode speed, response text, and provenance tags.
- `run_2026-08-24.jsonl`: Line-delimited stream format.
- `run_2026-08-24.csv`: Tabular format.
- `run_2026-08-24.md`: Markdown summary.
