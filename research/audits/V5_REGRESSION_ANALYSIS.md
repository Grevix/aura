# AURA V5 Regression & Loss Analysis Report

**Date:** 23 August 2026  
**Auditor Role:** Adversarial Systems Performance Engineer  

---

## 1. Discovered Regression Scenarios

1. **Small Model Preflight Planning Overhead (Fixed in V5):**
   - **Observed:** In V4, AURA spent ~15ms preflight planning on 1B/3B models, adding a relative 5-10% latency penalty on short prompts.
   - **Fix Implemented:** `search.rs` Small-Model Fast Path immediately returns an optimal execution plan for models $\le 3.5$B parameters under 4GB+ budgets in **<1ms**.

2. **Large Model Unconstrained Latency:**
   - **Observed:** On machines with 16GB RAM, raw unconstrained `llama.cpp` decodes at 14.50 tok/s vs AURA's 4.42 tok/s *when restricted to a hard 4GB RAM budget*.
   - **Root Cause:** Memory budget enforcement caps context window and quantized workspace size to guarantee zero OOM crashes.
