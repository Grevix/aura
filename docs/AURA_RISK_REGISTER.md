# AURA Risk Register & Mitigation Strategy

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Document Type:** Risk Governance & Contingency Planning  
**Status:** Approved  

---

## 1. Risk Matrix Overview

| Severity \ Likelihood | Low | Medium | High |
|---|---|---|---|
| **Critical** | R-04 | R-01, R-02 | R-03 |
| **High** | R-07 | R-05, R-06 | R-08 |
| **Medium** | R-11 | R-09, R-10 | R-12 |

---

## 2. Detailed Technical & Product Risk Register

### R-01: Upstream llama.cpp CPU-MoE Issue #19480 Unresolved
- **Category:** Technical / Dependency Risk
- **Severity:** Critical | **Likelihood:** Medium
- **Description:** `llama.cpp` CPU-MoE execution path currently delivers ~5× lower throughput than storage bandwidth predicts due to issue `#19480`.
- **Mitigation:** Pivot Stage A MVP strictly to dense 7B-8B GGUF models. Scope MoE Stage B to GPU/iGPU offloading paths where issue `#19480` does not block execution. Pin upstream tracking.
- **Residual Risk:** Weak-CPU-only MoE performance remains uncompetitive until upstream fix lands.

### R-02: Storage Read Bandwidth Imposes Interactive Latency Wall
- **Category:** Physics / Hardware Constraint Risk
- **Severity:** Critical | **Likelihood:** Medium
- **Description:** Cold model streaming on SATA SSDs (<500 MB/s) results in multi-second per-token decode speeds, making interactive chat impossible.
- **Mitigation:** `aura-planner` calculates storage latency floor preflight. If predicted decode speed < 1.0 tok/sec, AURA rejects execution or forces context reduction before downloading.
- **Residual Risk:** Users on SATA SSDs cannot run models larger than physical RAM interactively.

### R-03: macOS Lacks Unprivileged Per-Process Hard Memory Ceiling API
- **Category:** OS Integration Risk
- **Severity:** Critical | **Likelihood:** High
- **Description:** macOS Jetsam memory enforcer is system-wide and does not expose per-process hard RAM limits to unprivileged CLI tools.
- **Mitigation:** Implement working-set RSS polling loop in `aura-memory` (50ms interval). Explicitly label macOS enforcement as `macos_best_effort` in CLI output and benchmark JSON.
- **Residual Risk:** macOS OS memory pressure may invoke Jetsam kill before AURA soft-throttle reacts.

### R-04: Process Termination Under Windows Job Objects on Over-Allocation
- **Category:** OS Integration Risk
- **Severity:** Critical | **Likelihood:** Low
- **Description:** Exceeding `ProcessMemoryLimit` under Windows Job Objects results in immediate process termination without graceful cleanup.
- **Mitigation:** `aura-memory` polls `GetProcessMemoryInfo` and signals backend engine to drop KV context or stop generation when approaching 90% of commit limit.
- **Residual Risk:** Abrupt process termination if allocations spike faster than 50ms polling loop.

### R-05: Model Quality Collapse Under Extreme Quantization (Q2_K / IQ3_XS)
- **Category:** Model Quality Risk
- **Severity:** High | **Likelihood:** Medium
- **Description:** Ultra-low-bit quantization reduces memory footprint but may introduce severe perplexity spikes or incoherent generation.
- **Mitigation:** Set `Q4_K_M` as standard default. Restrict `Q3_K_S` to optional fallback with explicit quality warning. Require MMLU/WikiText validation gates.
- **Residual Risk:** Certain outlier-sensitive models perform poorly under 3-bit quantization.

### R-06: AURA Value Proposition Misperceived as "Thin Wrapper"
- **Category:** Product & Positioning Risk
- **Severity:** High | **Likelihood:** Medium
- **Description:** Users may perceive AURA as merely a wrapper script invoking `llama.cpp` unless distinct planning wins are demonstrated.
- **Mitigation:** Build release strategy around `aura benchmark reproduce` and head-to-head comparisons against default `llama.cpp` CLI flags, demonstrating zero-OOM execution and automated optimization.
- **Residual Risk:** Requires continuous messaging focus on honest planning and reproducibility rather than marketing hyperbole.

### R-07: Unsafe Model Files / Executable Code Injection
- **Category:** Security Supply Chain Risk
- **Severity:** High | **Likelihood:** Low
- **Description:** Untrusted model files or custom remote code could execute malicious code on host system.
- **Mitigation:** Enforce static GGUF parsing only; strictly prohibit `trust_remote_code=True`; validate SHA-256 digests; run unprivileged subprocesses.
- **Residual Risk:** Potential un-discovered buffer overflow vulnerabilities in native C/C++ GGUF parsing libraries.

### R-08: Thermal Throttling Degrades Sustained Token Throughput
- **Category:** Hardware Physics Risk
- **Severity:** High | **Likelihood:** High
- **Description:** Consumer laptops under sustained CPU load throttle clock speeds, causing benchmark throughput to drop during long runs.
- **Mitigation:** Measure and log CPU core temperatures; separate Cold Start, Warm Cache, and Steady State phases; record `throttling_detected` flag in `aura-benchmark.json`.
- **Residual Risk:** Steady-state performance on thin-and-light laptops will decrease after 5+ minutes of continuous generation.
