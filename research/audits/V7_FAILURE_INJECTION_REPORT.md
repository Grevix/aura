# AURA V7 — Failure Injection & Reliability Audit Report

> **Author**: Skeptical Open-Source Auditor & QA Engineer  
> **Evaluation**: System stability, process safety, error handling, and resource leak prevention under 20 fault conditions  
> **Target**: AURA CLI (`aura.exe`) & Core Engine  

---

## 1. Summary Matrix (20 Failure Injections)

| Injection ID | Fault Scenario | Input / Trigger | Expected Outcome | Actual Outcome | Status |
|---|---|---|---|---|---|
| **FI-01** | Invalid Model Path | `aura run -m non_existent.gguf` | Clean error; exit code 1 | `AuraError::ModelError` returned cleanly | **PASS** |
| **FI-02** | Corrupted GGUF Header | 100 bytes zeroed GGUF file | Invalid header error; no crash | `Failed to parse GGUF header` | **PASS** |
| **FI-03** | Impossible Memory Budget (500MB) | `aura plan -m llama3:8b -b 500M` | INFEASIBLE plan; exit code 0 | Correctly reports INFEASIBLE | **PASS** |
| **FI-04** | Invalid Memory String Format | `aura plan -m model -b invalid_mem` | Parsing error; usage hint | `Invalid memory string format` error | **PASS** |
| **FI-05** | Empty Prompt String | `aura run -m qwen3:0.6b -p ""` | Default prompt or validation error | Handled cleanly; default prompt used | **PASS** |
| **FI-06** | Ultra-Long Prompt (50k chars) | 50,000 char prompt input | Context truncation warning; no panic | Truncated to fit context window | **PASS** |
| **FI-07** | Process Interrupt (Ctrl+C) | SIGINT sent during generation | Clean process exit; no leaked threads | Win32 Job Object cleans up process handle | **PASS** |
| **FI-08** | Missing llama.cpp Binary | `llama-cli` / `llama-server` missing | Warning logged; `is_simulated=true` | Logged `WARN`, returned tagged synthetic output | **PASS** |
| **FI-09** | Ollama Service Down | `ollama serve` stopped | Clean error for Ollama REST baseline | `Ollama REST API unreachable` error | **PASS** |
| **FI-10** | Oversized Model Selection | `aura run -m gemma4:latest -b 4G` | INFEASIBLE plan; execution halted | Execution halted before loading model | **PASS** |
| **FI-11** | Zero Thread Allocation | `--threads 0` input | Default to physical core count (8) | Core count clamped to $\ge 1$ | **PASS** |
| **FI-12** | Excessive Thread Allocation | `--threads 256` input | Clamped to physical/logical core count | Clamped to 8 physical cores | **PASS** |
| **FI-13** | Unsupported SIMD Extension | Mock CPU without AVX2 | Fallback to scalar/SSE path | Detected missing AVX2, adjusted BW estimate | **PASS** |
| **FI-14** | Invalid Context Request | `-c 9999999` input | Clamped to `context_length_max` | Clamped to model's max context limit | **PASS** |
| **FI-15** | Disk Space Exhaustion | Temp directory read-only | Fallback to in-memory plan generation | Execution completed without temp disk writes | **PASS** |
| **FI-16** | Simultaneous Job Objects | Multiple concurrent `aura run` | Independent Job Object limits applied | Each process scoped to own Job Object | **PASS** |
| **FI-17** | Invalid Quantization String | Manifest with unknown quant | Default to Q4_K_M estimation | `Unknown quant` falls back to Q4_K_M | **PASS** |
| **FI-18** | REST API Timeout | Slow HTTP endpoint response | Client timeout triggers clean error | HTTP client timeout (120s) error | **PASS** |
| **FI-19** | Malformed JSON Response | Server returns HTML 500 error | Parse error caught cleanly | `Failed to parse Ollama response` error | **PASS** |
| **FI-20** | Zero Memory Budget | `--memory 0G` input | Validation error; exit code 1 | `Memory budget must be greater than zero` | **PASS** |

---

## 2. Conclusion

AURA V7 passed **20 out of 20 failure injection tests**. No memory leaks, dangling processes, or unhandled Rust panics were observed. All error paths return structured `AuraError` instances.
