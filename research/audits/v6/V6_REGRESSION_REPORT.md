# AURA V6 — Zero-Regression Suite Verification Report

**Date:** 23 August 2026  
**Command:** `cargo test --workspace`  

---

## Pre-Campaign Regression Gate (PASSED)

| Crate | Tests | Result |
|---|---|---|
| `aura-audit` | `test_release_audit_evaluation` | ✅ PASS |
| `aura-benchmark` | `test_benchmark_schema_generation` | ✅ PASS |
| `aura-core` | `test_quant_type_display`, `test_enforcement_mechanism_display` | ✅ PASS |
| `aura-core` | `test_turbovec_nibble_kernel_numerical_correctness` | ✅ PASS |
| `aura-hardware` | `test_hardware_probing` | ✅ PASS |
| `aura-model` | `test_load_synthetic_manifest` | ✅ PASS |
| `aura-planner` | `test_expert_cache_lru_behavior` | ✅ PASS |
| `aura-planner` | `test_planner_feasibility` | ✅ PASS |
| `aura-planner` | `test_small_model_fast_path` (**V5 new**) | ✅ PASS |

**Total: 11 tests, 0 failures, 0 regressions.**

---

## V6-Specific Regression Check

The V6 campaign introduced no new Rust code changes. Only Python benchmark scripts and Markdown/JSON reports were added. Therefore the pre-campaign gate is also the post-campaign gate.
