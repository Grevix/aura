# AURA V5 Reproducibility Instructions

**Date:** 23 August 2026  

---

## Reproducing the V5 Multi-Model 4GB Benchmark

To reproduce the multi-model 4.0 GB process working set memory limit evaluation on host hardware:

1. **Verify Host Requirements:**
   - Host CPU: x86_64 CPU with AVX2 SIMD extensions.
   - Physical RAM: $\ge 8$ GB.
   - Storage: Local NVMe SSD.

2. **Run Workspace Unit Tests:**
   ```bash
   cargo test --workspace
   ```

3. **Execute Adversarial 50-Prompt Multi-Model Battle Suite:**
   ```bash
   python benchmarks/run_v5_multimodel_battle.py
   ```

4. **Inspect Generated JSON Data:**
   - `benchmarks/results/v5_multi_model_results.json`
   - `benchmarks/results/v5_aura_vs_ollama_raw.json`
