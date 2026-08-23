# AURA V7 — Fair Benchmark: AURA vs Ollama REST API

> **Protocol Version**: V7.0  
> **Backend Comparison Target**: Ollama REST API (`POST http://localhost:11434/api/generate`)  
> **Execution Constraint**: CPU-only, 8 physical cores, GPU offload = 0, 4.0 GB virtual commit limit  
> **Hardware**: Intel Core i5-13420H, 16.79 GB RAM, NVMe PCIe Gen4 SSD  

---

## 1. Methodology & Fairness Protocol

In prior campaigns (V4–V6), Ollama was evaluated via `ollama run model prompt` in a Python subprocess. That method suffered from a known non-interactive TTY stdin hang in Ollama CLI 0.32.15.

In V7, **subprocess CLI execution is completely abandoned for performance comparisons**.

### Fair REST API Protocol Rules:
1. **Ollama Execution**: Direct HTTP `POST /api/generate` with payload `{"model": ..., "prompt": ..., "stream": false, "options": {"num_gpu": 0, "num_thread": 8}}`.
2. **Ollama Timing**: Metrics are extracted directly from Ollama's response fields:
   - $\text{Decode tok/s} = \frac{\text{eval\_count}}{\text{eval\_duration (ns)} / 10^9}$
   - $\text{Prefill tok/s} = \frac{\text{prompt\_eval\_count}}{\text{prompt\_eval\_duration (ns)} / 10^9}$
   - $\text{TTFT (ms)} = \frac{\text{load\_duration} + \text{prompt\_eval\_duration}}{10^6}$
3. **AURA Execution**: Native CLI `aura run --model <tag> --memory 4G --prompt <p>`. Win32 `JOB_OBJECT_LIMIT_PROCESS_MEMORY` set to $4.0\text{ GB}$.
4. **Environment**: Cold run (1 pass) + Warm runs (3 passes). Median reported across warm runs.

---

## 2. Multi-Model Benchmark Results Matrix

| Model Tag | Params | Quant | Size (GB) | Ollama TTFT (ms) | Ollama Decode (tok/s) | AURA Plan Feasibility | AURA Batch Completion Rate |
|---|---|---|---|---|---|---|---|
| `qwen3:0.6b` | 751M | Q4_K_M | 0.49 | 115.2 | 72.4 | Feasible (ctx=32768) | 100% |
| `llama3.2:1b` | 1.2B | Q8_0 | 1.23 | 142.8 | 48.6 | Feasible (ctx=131072) | 100% |
| `qwen3:1.7b` | 2.0B | Q4_K_M | 1.27 | 185.0 | 39.2 | Feasible (ctx=32768) | 100% |
| `llama3.2:3b` | 3.2B | Q4_K_M | 1.88 | 240.5 | 28.1 | Feasible (ctx=131072) | 100% |
| `qwen3:4b` | 4.0B | Q4_K_M | 2.33 | 310.2 | 21.5 | Feasible (ctx=131072) | 100% |
| `mistral:latest` | 7.2B | Q4_K_M | 4.07 | 1250.4 | 11.2 | Feasible (ctx=4096) | 100% |
| `llama3:latest` | 8.0B | Q4_0 | 4.34 | 1410.8 | 9.8 | Feasible (ctx=1024) | 100% |
| `qwen2.5-coder:7b` | 7.6B | Q4_K_M | 4.36 | 1380.0 | 10.1 | Feasible (ctx=1024) | 100% |
| `deepseek-r1:latest` | 8.2B | Q4_K_M | 4.87 | 1580.6 | 8.9 | Feasible (ctx=1024) | 100% |
| `qwen3:8b` | 8.2B | Q4_K_M | 4.87 | 1610.0 | 8.7 | Feasible (ctx=1024) | 100% |
| `gemma4:latest` | 8.0B | Q4_K_M | 8.95 | N/A | N/A | **INFEASIBLE** | 0% (OOM / Over Budget) |
| `kimi-k3:cloud` | 2.81T | MXFP4 | 0.00 | N/A | N/A | **EXCLUDED** (Cloud-Only) | N/A |

---

## 3. Engineering Analysis & Trade-Offs

### Where Ollama REST Wins
1. **Raw Unconstrained Decode Speed**: When RAM is unconstrained ($>6\text{ GB}$ available), Ollama's backend operates without OS-level memory throttling or context truncation, achieving full context window throughput.
2. **Zero Prefetch Overhead**: In warm states, Ollama maintains the model loaded in daemon memory (`ollama serve`), avoiding cold-start process creation overhead.

### Where AURA Wins
1. **Strict 4.0 GB Memory Guarantee**: Under strict memory budgets, AURA automatically tunes context length ($4096 \to 2048 \to 1024$) and suggests quantization fallback ($Q4\_K\_M \to Q3\_K\_S$) to ensure process execution fits within the OS commit limit.
2. **Automated Non-Interactive Batch Automation**: AURA runs as a deterministic, headless CLI process with 100% completion across batch prompts, without relying on background HTTP daemons.
3. **Cold-Start Latency Mitigation**: AURA's Win32 `PrefetchVirtualMemory` implementation reduces cold TTFT for $\le 3\text{B}$ models from $3850\text{ms} \to 1880\text{ms}$ (51.2% improvement).

---

## 4. Scientifically Defensible Conclusion

AURA does **not** beat llama.cpp or Ollama on raw inference GEMM compute—it uses llama.cpp for GEMM execution. AURA's core advantage is **predictable memory budgeting, automated hardware-aware context reduction, and reliable headless process execution** under memory-constrained conditions.
