# AURA V7 — Memory Boundary Matrix

> **Target Machine**: Intel Core i5-13420H (8P/12L cores, AVX2, 16.79 GB RAM, NVMe SSD)  
> **Evaluation**: Determining context size, quantization selection, feasibility, and decode throughput across memory budget steps $[1.5\text{GB} \to 16\text{GB}]$.

---

## 1. Memory Boundary Matrix Table

| Model Tag | 1.5 GB Budget | 2.0 GB Budget | 2.5 GB Budget | 3.0 GB Budget | 3.5 GB Budget | 4.0 GB Budget | 6.0 GB Budget | 8.0+ GB Budget |
|---|---|---|---|---|---|---|---|---|
| **`qwen3:0.6b`** (0.49GB) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) |
| **`llama3.2:1b`** (1.23GB) | Feasible (ctx=16k) | Feasible (ctx=64k) | Feasible (ctx=131k)| Feasible (ctx=131k)| Feasible (ctx=131k)| Feasible (ctx=131k)| Feasible (ctx=131k)| Feasible (ctx=131k)|
| **`qwen3:1.7b`** (1.27GB) | Feasible (ctx=8k) | Feasible (ctx=16k) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) | Feasible (ctx=32k) |
| **`llama3.2:3b`** (1.88GB) | Infeasible | Feasible (ctx=1024)| Feasible (ctx=4096)| Feasible (ctx=16k) | Feasible (ctx=64k) | Feasible (ctx=131k)| Feasible (ctx=131k)| Feasible (ctx=131k)|
| **`qwen3:4b`** (2.33GB) | Infeasible | Infeasible | Feasible (ctx=1024)| Feasible (ctx=4096)| Feasible (ctx=16k) | Feasible (ctx=131k)| Feasible (ctx=131k)| Feasible (ctx=131k)|
| **`mistral:latest`** (4.07GB)| Infeasible | Infeasible | Infeasible | Infeasible | Infeasible | Feasible (ctx=1024)| Feasible (ctx=8192) | Feasible (ctx=32k) |
| **`qwen2.5-coder:7b`** (4.36GB)| Infeasible | Infeasible | Infeasible | Infeasible | Infeasible | Feasible (ctx=1024)| Feasible (ctx=8192) | Feasible (ctx=32k) |
| **`llama3:latest`** (4.34GB)| Infeasible | Infeasible | Infeasible | Infeasible | Infeasible | Feasible (ctx=1024)| Feasible (ctx=4096) | Feasible (ctx=8192) |
| **`deepseek-r1:latest`** (4.87GB)| Infeasible | Infeasible | Infeasible | Infeasible | Infeasible | Feasible (ctx=1024)| Feasible (ctx=4096) | Feasible (ctx=32k) |
| **`qwen3:8b`** (4.87GB) | Infeasible | Infeasible | Infeasible | Infeasible | Infeasible | Feasible (ctx=1024)| Feasible (ctx=4096) | Feasible (ctx=32k) |
| **`gemma4:latest`** (8.95GB)| Infeasible | Infeasible | Infeasible | Infeasible | Infeasible | Infeasible | Infeasible | Feasible (ctx=4096)|

---

## 2. Key Observations & Thresholds

1. **Sub-2B Models (`qwen3:0.6b`, `llama3.2:1b`, `qwen3:1.7b`)**:
   - Highly resilient under low memory budgets.
   - Fully feasible at 1.5–2.0 GB budgets with minimal context truncation.
   - Small-model fast-path generates plans in $<1\text{ms}$.

2. **Medium Tier (`llama3.2:3b`, `qwen3:4b`)**:
   - Minimum feasible budget for 3B is $2.0\text{ GB}$ (context reduced to 1024).
   - Minimum feasible budget for 4B is $2.5\text{ GB}$ (context reduced to 1024).
   - At $4.0\text{ GB}$, both models operate with maximum context capacity.

3. **7B–8B Tier (`mistral:7b`, `qwen2.5-coder:7b`, `llama3:8b`, `deepseek-r1:8b`)**:
   - Minimum feasible budget is **$4.0\text{ GB}$** (with context forced to 1024 tokens and Q4_0 / Q4_K_M quant).
   - Below $4.0\text{ GB}$, 7B/8B dense models are mathematically infeasible without ultra-low bit quantization (Q2_K / IQ2_XXS).
   - Performance scales significantly when budget increases to $6.0\text{ GB}$, recovering full context headroom.

4. **Oversized Models (`gemma4:latest` @ 8.95GB GGUF)**:
   - Infeasible under $4.0\text{ GB}$, $6.0\text{ GB}$, or $8.0\text{ GB}$ budgets. Requires $\ge 9.5\text{ GB}$ available RAM.
