# AURA Reproducibility & Telemetry Schema Standard (V2 Pass)

**Date:** 23 August 2026  
**Artifact Path:** `benchmarks/results/aura_benchmark_run_latest.json`  

---

## 1. Machine-Readable Schema Specification

Every benchmark run produces a JSON document matching the specification below:

```json
{
  "timestamp_utc": "2026-08-23T08:50:55.034470+00:00",
  "system": {
    "os": "windows_11_x86_64",
    "cpu": "13th Gen Intel(R) Core(TM) i5-13420H",
    "cores": "8 physical / 12 logical",
    "ram_gb": 16.79
  },
  "workload_results": {
    "WORKLOAD_A_SHORT_CHAT": [
      {
        "success": true,
        "elapsed_sec": 1.9535768032073975,
        "output_snippet": "AURA Response for prompt: 'What is Python?'"
      }
    ]
  }
}
```

---

## 2. Reproducibility Instructions
To independently reproduce the complete benchmark suite:
```bash
python benchmarks/run_all.py
```
Output artifacts are saved directly to `benchmarks/results/aura_benchmark_run_latest.json`.
