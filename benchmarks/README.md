# AURA Automated Benchmark Suite & Harness

**Date:** 23 August 2026  
**Target:** Reproducible Machine-Readable Benchmark Exporter (`aura_benchmark_run_latest.json`)  

---

## Benchmark Workload Classification Matrix

- **WORKLOAD A — SHORT CHAT:** Short single-sentence prompts ("What is Python?", "Explain recursion.").
- **WORKLOAD B — CODING:** Python/C++/SQL code generation prompts.
- **WORKLOAD C — LONG CONTEXT:** 1K, 2K, 4K, 8K token context length scaling prompts.
- **WORKLOAD D — MULTI-TURN CONVERSATION:** 10, 20, 50 turn simulated dialogue streams for KV growth & expert locality tracking.
- **WORKLOAD E — REPETITIVE GENERATION:** Long token generation runs (100, 500, 1000 tokens).
- **WORKLOAD F — COLD START:** Cold-start page cache readahead evaluation.
- **WORKLOAD G — WARM CACHE:** Hot page cache residency evaluation.
- **WORKLOAD H — MEMORY PRESSURE:** RAM budget scaling (2GB, 3GB, 4GB, 4.2GB, 6GB, 8GB, 16GB).
- **WORKLOAD I — SYSTEM UNDER LOAD:** Background CPU stress evaluation.
- **WORKLOAD J — AIR-GAPPED / OFFLINE:** Wi-Fi & Ethernet disabled local execution verification.

---

## Execution Command
```bash
python benchmarks/run_all.py
```
Outputs machine-readable JSON files into `benchmarks/results/`.
