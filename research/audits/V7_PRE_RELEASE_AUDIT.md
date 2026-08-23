# AURA V7 — Pre-Release Architectural & Methodology Audit

> **Author**: Skeptical Open-Source Auditor & Principal Systems Engineer  
> **Target**: AURA V7 Release Candidate Codebase & Benchmark Infrastructure  
> **Date**: 2026-08-23  

---

## Executive Summary

Before freezing AURA for the V7 Release Candidate, a comprehensive audit was conducted across all 9 `aura-*` crates, benchmark harnesses, documentation, and historical reports. This audit identified 13 critical areas of technical debt, methodology flaws, and terminology ambiguities. All 13 items have been systematically audited and resolved in V7.

---

## 13 Audit Findings & Resolutions

### 1. Benchmark Methodology: Ollama Subprocess Hang (RESOLVED)
- **Finding**: In V4–V6, Ollama was invoked via `ollama run model prompt` in a Python subprocess. Ollama CLI 0.32.15 expects an interactive TTY on stdin and hangs indefinitely, resulting in a 0% completion rate for Ollama and a false "100% win rate" claim for AURA.
- **Resolution**: V7 completely deprecates subprocess CLI invocation for benchmarks. All Ollama benchmarks use the native REST API (`POST http://localhost:11434/api/generate`) with `stream: false`, `num_gpu: 0`, and `num_thread: 8`. Decode throughput is measured using Ollama's own nanosecond-precision `eval_count` and `eval_duration` fields.

### 2. Memory Terminology Ambiguity: Commit Limit vs RSS (RESOLVED)
- **Finding**: Documentation claimed a "4.0 GB hard physical RSS limit," but benchmarks recorded peak RSS of 4.11–4.15 GB.
- **Resolution**: Audited `crates/aura-memory/src/windows.rs`. AURA uses Win32 `SetInformationJobObject` with `JOB_OBJECT_LIMIT_PROCESS_MEMORY` (`ProcessMemoryLimit`). In Windows kernel architecture:
  - `ProcessMemoryLimit` sets a hard cap on **committed virtual address space**.
  - Physical **Resident Set Size (RSS)** includes memory-mapped GGUF file pages outside the private commit charge.
  - Therefore, measured RSS can naturally be 2–5% above the 4.0 GB commit limit before the OS terminates the process.
  - Terminology updated across README, FAQ, and docs: AURA enforces a **4.0 GB virtual commit limit**.

### 3. Silent Simulated Output Fallback (RESOLVED)
- **Finding**: In `crates/aura-backends/src/llama_cpp.rs`, if `llama-cli` or `llama-server` was missing from system PATH, AURA silently returned synthetic planning text without informing callers, risking simulated planning latencies being mistaken for real inference numbers.
- **Resolution**: Added `is_simulated: bool` to `BackendOutput` trait in `aura-backends`. Whenever the llama.cpp binary is unreachable, AURA logs a `WARN`, tags the output as `is_simulated = true`, and labels generated text with `[SIMULATED — NOT REAL INFERENCE]`.

### 4. Hardcoded Decode Throughput in Fast-Path (RESOLVED)
- **Finding**: `crates/aura-planner/src/search.rs` fast-path hardcoded `predicted_decode_tok_per_sec: 35.0` for all models $\le 3.5\text{B}$. A 0.6B model and a 3.2B model on the same hardware have vastly different memory-bandwidth bounds.
- **Resolution**: Replaced hardcoded constant with a physics-based memory-bandwidth model ($BW_{eff} \approx 45\text{ GB/s}$ for AVX2 DRAM, $bytes\_per\_token = weight\_bytes / params$), clamped to $[4.0, 80.0]\text{ tok/s}$.

### 5. Fabricated Metrics in Ollama Baseline Runner (RESOLVED)
- **Finding**: `crates/aura-backends/src/ollama_baseline.rs` hardcoded `peak_rss_bytes: 5_200_000_000` and estimated TTFT as `elapsed_sec * 300.0`.
- **Resolution**: Rewrote `OllamaBaselineRunner` to query `http://localhost:11434/api/generate` and parse native fields (`eval_count`, `eval_duration`, `prompt_eval_count`, `prompt_eval_duration`, `load_duration`). RSS is set to `0` (not measured via REST) to avoid fabricating numbers.

### 6. Cloud Model Mischaracterization (RESOLVED)
- **Finding**: Ollama catalog lists models like `kimi-k3:cloud` (2.81T params). Previous inventories did not distinguish cloud-routed models from local GGUF artifacts, leading to absurd claims of running 2.81T models locally.
- **Resolution**: Created `benchmarks/discover_models.py` which inspects `remote_host` and `size` via REST API (`GET /api/tags`). `kimi-k3:cloud` is explicitly classified as `CLOUD_ONLY` and excluded from all local inference benchmarks.

### 7. TurboVec Production Status Misrepresentation (RESOLVED)
- **Finding**: TurboVec (custom SIMD nibble kernel) measured 42.8 GB/s vs llama.cpp's 48.2 GB/s (-12.6%).
- **Resolution**: Confirmed TurboVec is kept strictly as an experimental benchmark (`crates/aura-core` / `experiments/turbovec`). llama.cpp remains the primary production inference kernel.

### 8. Hardcoded Model Inventories (RESOLVED)
- **Finding**: Benchmark scripts hardcoded model lists, breaking when models were added or removed from Ollama.
- **Resolution**: Created `benchmarks/discover_models.py` for automated REST discovery, outputting `V7_LOCAL_MODEL_INVENTORY.json` with SHA256 digests, parameter counts, quantization levels, and context sizes.

### 9. MoE 85% I/O Reduction Claim Clarification (RESOLVED)
- **Finding**: Historical docs cited "85% NVMe I/O reduction for MoE models" based on synthetic LRU unit tests (`test_expert_cache_lru_behavior`), not empirical 70B MoE model inference.
- **Resolution**: Re-labeled claim as `[UNIT VERIFIED / SYNTHETIC]` in `V7_MOE_RESULTS.md`. Explicitly noted that full MoE empirical validation requires 48GB+ hardware (Community Track).

### 10. Community Hardware Benchmark Gap (RESOLVED)
- **Finding**: Laptop development hardware (i5-13420H, 16GB RAM) cannot test 14B, 32B, 70B, or 100B+ models locally.
- **Resolution**: Built `benchmarks/community_runner/` with `run_community_benchmark.py`, `schema.json`, and `.github/ISSUE_TEMPLATE/benchmark.yml` so community members with 32GB–96GB+ hardware can submit validated JSON benchmark results.

### 11. Absence of Output Quality Validation (RESOLVED)
- **Finding**: Benchmark harnesses only measured wall-clock execution time, ignoring output correctness or truncation.
- **Resolution**: Created 100-prompt developer test suite (`REAL_WORLD_BENCHMARK.md`) across 20 categories with automated syntax validation (Python `py_compile`, JSON `json.loads`).

### 12. Workspace Regression Safety (RESOLVED)
- **Finding**: Needs confirmation that all workspace crates compile and pass unit tests clean.
- **Resolution**: Verified `cargo test --workspace` passes 11/11 tests with zero failures.

### 13. Documentation Alignment (RESOLVED)
- **Finding**: README contained marketing-style claims and lacked clear disclaimers.
- **Resolution**: Rewrote `README.md` with an explicit **"What AURA Does NOT Claim"** section, accurate model feasibility tables, and distinct project vs community hardware tracks.

---

## Pre-Release Gate Summary

| Category | Status | Notes |
|---|---|---|
| Codebase Integrity | **PASS** | 11/11 workspace unit tests pass |
| Memory Enforcement | **VERIFIED** | Win32 Job Object virtual commit limit clarified |
| Ollama Comparison | **REFACTORED** | Native REST API timing via `eval_count`/`eval_duration` |
| Discovery | **AUTOMATED** | `discover_models.py` via `http://localhost:11434/api/tags` |
| Disclaimer Transparency | **COMPLETE** | README, FAQ, REPRODUCIBILITY updated |
