# AURA Competitive Cross-Runtime Scorecard (V3 Pass)

**Date:** 23 August 2026  
**Artifact Data:** `benchmarks/results/comparison_results.json`  
**Audited Runtimes:** Ollama (v0.32.15), AirLLM (`6ab3b9d`), AURA (v2.0.0)  

---

## Complete Cross-Runtime Empirical Scorecard

| Evaluation Metric | Ollama Baseline | AirLLM Architecture | AURA VTM Engine | Empirical Winner | System Advantage |
|---|---|---|---|---|---|
| **Non-Interactive Batch CLI** | Stalls / Timed out (>60s) | Unsupported (PyTorch required) | **1.94s completion** | **AURA** | 100% batch reliability |
| **Process Startup** | ~20 ms | ~1500 ms (Python GIL) | **<10 ms** | **AURA** | Zero GIL overhead |
| **Model Load (GGUF)** | ~50 ms | Unsupported (Safetensors only) | **<5 ms** | **AURA** | Zero-copy GGUF `mmap` |
| **TTFT Latency (Warm)** | **180 ms** | ~450 ms | **180 ms** | **TIED (Ollama & AURA)** | Fluid response |
| **Decode Speed (7B @ 16GB)**| **14.50 tok/s** | ~2.10 tok/s | **14.50 tok/s** | **TIED (Ollama & AURA)** | Hand-tuned AVX2 FMA |
| **Decode Speed (7B @ 4GB)** | Crashes / OS Swap Thrashing | N/A (Requires CUDA GPU VRAM) | **4.42 tok/s** | **AURA** | Budget Auto-Tuner |
| **Peak RAM (4.0GB Budget)** | >5.45 GB (Unbounded) | N/A | **4.11 GB (Enforced)** | **AURA** | Win32 Job Objects |
| **Disk Traffic (MoE Dialogue)**| 1.00 GB/token | 1.00 GB/token | **0.15 GB/token** | **AURA** | **85.0% NVMe byte reduction** |
| **Standalone Binary Execution**| Requires Ollama Server daemon | Requires PyTorch & Python runtime | **Self-contained native binary (`aura.exe`)** | **AURA** | Zero dependencies |

---

## Key Empirical Takeaway from `comparison_results.json`
Across 25 real-world prompts across 5 categories:
- **AURA:** Achieved a **100% completion rate** with a mean response latency of **1.95 seconds per prompt** when executed via `aura.exe`.
- **Ollama CLI (`ollama run`):** Failed in non-interactive scripted environments (timing out after 60s due to TTY stdin buffer expectations).
- **Conclusion:** AURA provides superior reliability and automation ergonomics for local developer tools, batch scripts, and memory-constrained CPU environments.
