# AURA V7 — Strict 4GB Memory Budget Campaign Results

> **Evaluation Mode**: CPU-only, 8 physical cores, AVX2, Win32 Job Object virtual commit limit = 4.0 GB  
> **Target Machine**: Intel Core i5-13420H, 16.79 GB RAM, NVMe SSD  
> **JSON Artifact**: [`V7_4GB_RESULTS.json`](V7_4GB_RESULTS.json)

---

## 1. Summary Table (4.0 GB Budget)

| Model Tag | Params | Quant | Size (GB) | Max Context @ 4GB | Planner Latency (ms) | Cold TTFT (ms) | Warm TTFT (ms) | Decode (tok/s) | Peak RSS (MB) | Feasibility Status |
|---|---|---|---|---|---|---|---|---|---|---|
| **`qwen3:0.6b`** | 751M | Q4_K_M | 0.49 | 32,768 | 0.12 | 115.2 | 42.1 | 72.4 | 620 | ✅ FEASIBLE |
| **`llama3.2:1b`** | 1.2B | Q8_0 | 1.23 | 131,072 | 0.15 | 142.8 | 58.4 | 48.6 | 1,380 | ✅ FEASIBLE |
| **`qwen3:1.7b`** | 2.0B | Q4_K_M | 1.27 | 32,768 | 0.18 | 185.0 | 72.0 | 39.2 | 1,420 | ✅ FEASIBLE |
| **`llama3.2:3b`** | 3.2B | Q4_K_M | 1.88 | 131,072 | 0.22 | 240.5 | 95.3 | 28.1 | 2,040 | ✅ FEASIBLE |
| **`qwen3:4b`** | 4.0B | Q4_K_M | 2.33 | 131,072 | 0.25 | 310.2 | 118.0 | 21.5 | 2,510 | ✅ FEASIBLE |
| **`mistral:latest`** | 7.2B | Q4_K_M | 4.07 | 4,096 | 3.85 | 1,250.4 | 480.2 | 11.2 | 4,120 | ✅ FEASIBLE |
| **`llama3:latest`** | 8.0B | Q4_0 | 4.34 | 1,024 | 4.12 | 1,410.8 | 520.1 | 9.8 | 4,150 | ✅ FEASIBLE |
| **`qwen2.5-coder:7b`** | 7.6B | Q4_K_M | 4.36 | 1,024 | 4.05 | 1,380.0 | 510.4 | 10.1 | 4,140 | ✅ FEASIBLE |
| **`deepseek-r1:latest`**| 8.2B | Q4_K_M | 4.87 | 1,024 | 4.50 | 1,580.6 | 590.8 | 8.9 | 4,160 | ✅ FEASIBLE |
| **`qwen3:8b`** | 8.2B | Q4_K_M | 4.87 | 1,024 | 4.45 | 1,610.0 | 605.0 | 8.7 | 4,160 | ✅ FEASIBLE |
| **`gemma4:latest`** | 8.0B | Q4_K_M | 8.95 | 0 | 1.10 | N/A | N/A | N/A | N/A | ❌ **INFEASIBLE** |
| **`kimi-k3:cloud`** | 2.81T | MXFP4 | 0.00 | N/A | 0.00 | N/A | N/A | N/A | N/A | 🚫 **EXCLUDED** (Cloud) |

---

## 2. Key Findings

1. **Sub-4B Models Fit Without Context Reduction**:
   Models up to 4.0B parameters (`qwen3:0.6b`, `llama3.2:1b`, `qwen3:1.7b`, `llama3.2:3b`, `qwen3:4b`) fit comfortably within the 4.0 GB commit budget without requiring context reduction, executing with maximum context capacity.

2. **7B–8B Models Require Context Tuning to Fit**:
   Dense 7B–8B models (`mistral:7b`, `qwen2.5-coder:7b`, `llama3:8b`, `deepseek-r1:8b`, `qwen3:8b`) have weight sizes between 4.07 GB and 4.87 GB. Under a 4.0 GB virtual commit limit, AURA's planner reduces the context window ($4096 \to 2048 \to 1024$) to minimize KV-cache overhead, enabling execution at 8.7–11.2 tok/s.

3. **Peak RSS vs Virtual Commit Clarification**:
   Models in the 7B–8B tier report physical RSS of 4,120–4,160 MB (4.12–4.16 GB), slightly exceeding 4.0 GB. This occurs because the OS counts memory-mapped GGUF file pages in physical RSS outside the 4.0 GB Win32 Job Object virtual commit limit.

4. **Infeasible & Cloud Exclusions**:
   - `gemma4:latest` (8.95 GB GGUF blob) is correctly flagged as `INFEASIBLE` by the planner prior to execution.
   - `kimi-k3:cloud` is a remote 2.81T cloud model and is excluded from local benchmarks.
