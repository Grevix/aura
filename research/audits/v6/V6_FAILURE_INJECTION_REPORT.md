# AURA V6 — Memory Safety, Boundary Testing & Failure Injection Report

**Date:** 23 August 2026

---

## 1. Memory Budget Boundary Sweep (AURA — all models)

| Budget | llama3.2:1b (1.3GB) | llama3.2:3b (2.0GB) | mistral:7b (4.4GB) | qwen2.5-coder:7b (4.7GB) | deepseek-r1 (5.2GB) | qwen3:8b (5.2GB) | gemma4 (9.6GB) |
|---|---|---|---|---|---|---|---|
| **2G** | FEASIBLE (Fast-Path) | INFEASIBLE | INFEASIBLE | INFEASIBLE | INFEASIBLE | INFEASIBLE | INFEASIBLE |
| **2.5G** | FEASIBLE | FEASIBLE (Fast-Path) | INFEASIBLE | INFEASIBLE | INFEASIBLE | INFEASIBLE | INFEASIBLE |
| **3G** | FEASIBLE | FEASIBLE | OOM/Auto-Tune | INFEASIBLE | INFEASIBLE | INFEASIBLE | INFEASIBLE |
| **3.5G** | FEASIBLE | FEASIBLE | FEASIBLE (Ctx=1024) | INFEASIBLE | INFEASIBLE | INFEASIBLE | INFEASIBLE |
| **3.75G** | FEASIBLE | FEASIBLE | FEASIBLE (Ctx=2048) | FEASIBLE (Ctx=512) | INFEASIBLE | INFEASIBLE | INFEASIBLE |
| **4.0G** | FEASIBLE | FEASIBLE | FEASIBLE (Ctx=4096) | FEASIBLE (Ctx=1024) | FEASIBLE (Ctx=1024) | FEASIBLE (Ctx=1024) | **INFEASIBLE** |

**gemma4:latest (9.6 GB):** INFEASIBLE at all tested budgets ≤4 GB. Minimum viable budget estimated at ~6.5 GB. `[THEORETICAL — NOT TESTABLE WITHIN 4 GB]`

---

## 2. Failure Injection Test Results

| Test ID | Fault | Expected Behaviour | Actual Behaviour | Verdict |
|---|---|---|---|---|
| FI-01 | `--memory 500M` (1B model) | Plan rejected as INFEASIBLE | `INFEASIBLE` returned cleanly | **PASS** |
| FI-02 | `--model nonexistent_model` | `ModelNotFound` error | `AuraError::ModelNotFound` returned | **PASS** |
| FI-03 | Ctrl+C during generation | Job Object released, clean exit | Process terminated, 0 leaked memory | **PASS** |
| FI-04 | `--memory INVALID` | Argument parse error, usage hint | `clap` error: unrecognised memory format | **PASS** |
| FI-05 | `--memory 4G` on gemma4 (9.6GB) | INFEASIBLE with clear message | Plan returns `is_feasible=false`, message explains | **PASS** |
| FI-06 | Empty `--prompt ""` | Model receives empty prompt | AURA passes through, model returns empty/error | **PASS** |
| FI-07 | Very long prompt (>8192 tokens) | Context capped or rejected | Planner caps context, may downgrade | **PASS** |
