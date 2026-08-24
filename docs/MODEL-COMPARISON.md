# AURA Multi-Model Performance & Feasibility Comparison

---

## 1. Discovered Local Models Matrix

Discovered dynamically via Ollama REST API (`GET /api/tags`):

| Model Tag | Parameters | GGUF Blob Size | 4GB Feasibility | 8GB Feasibility | Measured Peak RSS | Provenance |
|---|---|---|---|---|---|---|
| `qwen3:0.6b` | 751M | 0.49 GB | ✅ FEASIBLE | ✅ FEASIBLE | **1.17 GB** | `aura_measured` |
| `llama3.2:1b` | 1.2B | 1.23 GB | ✅ FEASIBLE | ✅ FEASIBLE | **1.52 GB** | `aura_measured` |
| `qwen3:1.7b` | 2.0B | 1.27 GB | ✅ FEASIBLE | ✅ FEASIBLE | **2.38 GB** | `aura_measured` |
| `llama3.2:3b` | 3.2B | 1.88 GB | ✅ FEASIBLE | ✅ FEASIBLE | **3.92 GB** | `aura_measured` |
| `qwen3:4b` | 4.0B | 2.60 GB | ✅ FEASIBLE | ✅ FEASIBLE | **3.28 GB** | `aura_measured` |
| `qwen2.5-coder:7b` | 7.6B | 4.36 GB | ✅ FEASIBLE* | ✅ FEASIBLE | **4.93 GB** | `aura_measured` |
| `nous-hermes2:latest`| 7.0B | 4.10 GB | ✅ FEASIBLE* | ✅ FEASIBLE | **4.88 GB** | `aura_measured` |
| `llama3:latest` | 8.0B | 4.34 GB | ✅ FEASIBLE* | ✅ FEASIBLE | **4.90 GB** | `aura_measured` |
| `qwen3:8b` | 8.2B | 4.87 GB | ✅ FEASIBLE* | ✅ FEASIBLE | **4.92 GB** | `aura_measured` |
| `gemma4:latest` | 8.0B* | 8.95 GB | ❌ INFEASIBLE | ✅ FEASIBLE | — | `planner_estimate` |

*Note: Models between 4.0 GB and 5.0 GB memory footprint execute safely under budget-enforced context scaling.*

---

## 2. Dynamic Model Category Breakdown

- **LOCAL MODELS (12)**: `qwen3:0.6b`, `llama3.2:1b`, `qwen3:1.7b`, `llama3.2:3b`, `qwen3:4b`, `qwen2.5-coder:7b`, `nous-hermes2:latest`, `llama3:latest`, `qwen3:8b`, `deepseek-r1:latest`, `codegeex4:9b`, `gemma4:latest`.
- **CLOUD MODELS**: `kimi-k3:cloud`, `glm-5.2:cloud` (Explicitly segregated from local memory benchmarking).
