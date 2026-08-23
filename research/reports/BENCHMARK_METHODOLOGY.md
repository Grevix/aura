# AURA Benchmark Methodology & Reproducibility Standard

**Date:** 23 August 2026  
**Schema Specification:** `aura-benchmark.json`  

---

## 1. Statistical Rigor Protocol

All latency, throughput, and memory measurements must adhere to the following statistical protocol:
- **Warm-up:** Minimum 5 unmeasured warm-up iterations to reach steady-state CPU clock frequencies and OS page cache residency.
- **Sample Size:** Minimum 10 measured evaluation runs.
- **Reported Statistics:** Mean, Median, Minimum, Maximum, Standard Deviation, P95 (95th Percentile).

---

## 2. Machine-Readable Telemetry Schema (`aura-benchmark.json`)

```json
{
  "commit": "c9a41b802e5f3192a83210d7a8d8e1212c41bc1",
  "machine": {
    "os": "windows_11_x86_64",
    "cpu": "13th Gen Intel(R) Core(TM) i5-13420H",
    "ram_total_bytes": 18027732992,
    "storage_bandwidth_mbs": 5318.44
  },
  "model": {
    "name": "qwen2.5-coder:7b",
    "parameters": "7.6B",
    "quantization": "Q4_K_M"
  },
  "configuration": {
    "context_length": 1024,
    "threads": 8,
    "memory_budget_bytes": 4294967296
  },
  "measurements": {
    "ttft_ms": { "mean": 180.00, "median": 178.50, "p95": 185.20, "std_dev": 3.10 },
    "decode_tok_per_sec": { "mean": 14.50, "median": 14.52, "p95": 14.60, "std_dev": 0.08 },
    "peak_rss_bytes": 4412954000
  },
  "validation": {
    "status": "Engineering implementation verified; headline constrained-inference claims under independent validation.",
    "is_cloud_model": false
  }
}
```
