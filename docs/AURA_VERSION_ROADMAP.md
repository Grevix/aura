# AURA Version Roadmap (0.1.0 → 2.0.0)

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Governance Principle:** Continuous Measurable Engineering Improvement. No fake version bumps.  

---

## Roadmap Overview

```
0.1.0 (Foundation & Telemetry) 
  ↳ 0.2.0 (Feasibility Planner)
    ↳ 0.3.0 (Linux Hard cgroups Enforcer)
      ↳ 0.4.0 (Windows Job Objects & macOS Enforcer)
        ↳ 0.5.0 (Dense Model Auto-Tuner & MVP Stage A)
          ↳ 0.6.0 (Cache Telemetry Engine)
            ↳ 0.7.0 (MoE Model Metadata & Tracking)
              ↳ 0.8.0 (CPU-MoE & GPU Split Planner)
                ↳ 0.9.0 (Local Benchmark Database)
                  ↳ 1.0.0 (STABLE AURA RUNTIME & AUDIT GATE)
                    ↳ 1.1.0–1.9.0 (Community Telemetry, KV Quant, iGPU, Thermal Autotune)
                      ↳ 2.0.0 (Unified Virtual Tensor Memory - Research Bet)
```

---

## Detailed Version Specifications

### AURA 0.1.0 — "Foundation & Hardware Telemetry"
- **Primary Objective:** Build core Rust monorepo structure, deterministic hardware profiler, GGUF metadata parser, baseline `llama.cpp` adapter, and `aura doctor` command.
- **User-Visible Improvement:** `aura doctor` reports accurate hardware profile, SIMD capabilities, storage bandwidth, and supported backends.
- **Technical Improvement:** Low-overhead GGUF header parser (<5ms) and storage bandwidth probe (<1.5s).
- **Supported Models:** Dense GGUF models (Llama 3, Qwen 2.5, Mistral 7B).
- **Supported Hardware:** Linux x86_64, Windows x86_64, macOS ARM64.
- **Memory Target:** N/A (prober only).
- **Performance Target:** Prober startup overhead < 15 ms.
- **Exit Criteria:** `aura doctor` output matches physical host configuration across 5 reference test machines.

### AURA 0.2.0 — "Analytical Feasibility Planner"
- **Primary Objective:** Implement analytical memory estimator and feasibility validation engine.
- **User-Visible Improvement:** `aura plan MODEL --memory 4G` displays expected memory breakdown, predicted token generation rate, and feasibility status.
- **Technical Improvement:** Analytical formula for weight bytes + KV cache + execution overhead.
- **Memory Target:** 4 GB – 16 GB budgets.
- **Performance Target:** Planning calculation latency < 10 ms.
- **Exit Criteria:** Memory footprint estimate within ±7% of measured peak RSS on baseline `llama.cpp`.

### AURA 0.3.0 — "Linux Hard Memory Enforcement"
- **Primary Objective:** Integrate Linux `cgroups v2` and `systemd` transient scope memory enforcement (`MemoryMax`, `MemoryHigh`, PSI streaming).
- **User-Visible Improvement:** `aura run MODEL --memory 4G` executes model strictly within kernel-enforced RAM ceiling on Linux.
- **Technical Improvement:** Zero memory leaks outside cgroup boundaries; real-time memory pressure stall (PSI) monitoring.
- **Exit Criteria:** Zero kernel OOM leaks on Linux under 100 stress runs; SIGKILL gracefully handled.

### AURA 0.4.0 — "Cross-Platform Hard Memory Enforcement"
- **Primary Objective:** Windows Job Objects integration (`ProcessMemoryLimit`) and macOS RSS polling wrapper.
- **User-Visible Improvement:** Windows and macOS support budget-enforced execution with explicit platform enforcement labelling (`windows_job_object`, `macos_best_effort`).
- **Technical Improvement:** Hard process termination on Windows commit overflow; proactive RSS backpressure on macOS.
- **Exit Criteria:** Deterministic job limit termination on Windows; RSS logger active on macOS.

### AURA 0.5.0 — "Dense MVP (Stage A)"
- **Primary Objective:** Complete Stage A MVP: automatic quant/context/thread tuning for dense 7B-8B GGUF models on CPU-only laptops.
- **User-Visible Improvement:** `aura run MODEL --memory 4G` automatically out-performs default `llama.cpp` flags or prevents OOM crash.
- **Technical Improvement:** Automated grid search over GGUF quants (`Q4_K_M`, `Q3_K_S`), context (`2048`, `4096`), and threads.
- **Performance Target:** ≥15% peak RSS reduction or ≥20% TTFT improvement vs default `llama.cpp` CLI defaults.
- **Exit Criteria:** 30 consecutive clean runs on reference 8GB laptop without OOM.

### AURA 0.6.0 — "Cache Telemetry Engine"
- **Primary Objective:** Instrument weight page-cache hits/misses, major page faults, and KV cache utilization.
- **User-Visible Improvement:** `--verbose` flag streams live page fault rates, RSS vs mapped memory, and cache hit ratios.
- **Technical Improvement:** Read system OS page fault metrics (`getrusage` on Unix, `GetProcessMemoryInfo` on Windows).
- **Exit Criteria:** Live telemetry overhead < 0.5% CPU utilization.

### AURA 0.7.0 — "MoE Model Metadata & Tracking"
- **Primary Objective:** Extend GGUF parser and registry for Mixture-of-Experts (MoE) models (Mixtral, DeepSeek-MoE, Qwen-MoE).
- **User-Visible Improvement:** `aura models` displays total vs active parameters, expert counts, and active experts per token.
- **Technical Improvement:** Active-expert memory separation calculation.
- **Exit Criteria:** Accurate parsing of all public MoE GGUF formats.

### AURA 0.8.0 — "GPU Split Planner & MoE CPU Gate Tracking"
- **Primary Objective:** Build GPU layer offload planner (`--n-gpu-layers` tuning) and track llama.cpp CPU-MoE issue #19480.
- **User-Visible Improvement:** Hybrid CPU+GPU laptops (iGPU/dGPU) automatically utilize VRAM for non-expert layers.
- **Technical Improvement:** Optimal layer placement algorithm based on VRAM capacity and PCIe bandwidth.
- **Exit Criteria:** 100% stable layer split execution on integrated Intel/AMD/Apple GPUs.

### AURA 0.9.0 — "Local Telemetry Database & Calibration"
- **Primary Objective:** Build local SQLite/JSON benchmark history database to refine planning heuristics based on past runs.
- **User-Visible Improvement:** `aura plan` cites empirical data from previous runs on the user's host machine.
- **Technical Improvement:** Historical telemetry lookup and Bayesian heuristic adjustment.
- **Exit Criteria:** Planner error decreases by >25% after 10 calibration runs on target host.

### AURA 1.0.0 — "STABLE AURA RUNTIME"
- **Primary Objective:** Production-ready local inference planner & runtime.
- **User-Visible Improvement:** Single-binary distribution with signed releases, robust installer, full documentation, and audit suite.
- **Technical Improvement:** Comprehensive release audit gate passing on all reference hardware profiles.
- **Exit Criteria:** All release gates pass; machine-readable audit report published.

---

## 1.1.0 → 2.0.0 — POST-1.0 EVOLUTION ROADMAP

- **1.1.0:** Opt-In Shared Telemetry Database (anonymous, privacy-preserving benchmark sharing).
- **1.2.0:** Expert Locality Telemetry & Diagnostics (per-prompt expert routing stability score).
- **1.3.0:** KV Cache Quantization & Paging Integration (Q8_0 and Q4_0 KV cache options).
- **1.4.0:** Heterogeneous CPU + iGPU Graph Split Optimization (Metal / Vulkan split execution).
- **1.5.0:** Sustained Thermal Throttling Feedback Loop (automatic thread reduction upon thermal CPU clock drops).
- **1.6.0 – 1.9.0:** Iterative Planner Optimization via Machine Learning telemetry curves.
- **2.0.0:** AURA Virtual Tensor Memory (VTM) unified paging layer across weights, KV cache, and expert buffers (conditioned on empirical telemetry proving superiority over standard OS mmap).
