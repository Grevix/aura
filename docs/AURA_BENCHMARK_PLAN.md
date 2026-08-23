# AURA Benchmark Specification & Methodology

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Document Type:** Benchmark Methodology & Reproducibility Protocol  
**Schema Version:** 1.0  

---

## 1. Core Principles of Benchmark Integrity

1. **No Magic Numbers:** Every performance claim must be accompanied by a machine-readable `aura-benchmark.json` artifact generated directly by `aura benchmark`.
2. **Mandatory Baseline Comparison:** Every benchmark run must evaluate baseline `llama.cpp` using default CLI flags on the exact same model file and prompt suite.
3. **Explicit Phase Separation:** Benchmarks MUST measure and report **Cold Start**, **Warm Cache**, and **Steady State** phases independently. Collapsing these phases invalidates the benchmark.
4. **Honest Accounting:** Peak Resident Set Size (RSS), mapped virtual memory, major page faults, and storage read volume must be measured via operating system kernel primitives.

---

## 2. Benchmark Metrics Catalog

| Metric Key | Unit | Description | Primary Data Source |
|---|---|---|---|
| `ttft_ms` | Milliseconds | Time To First Token (latency from prompt submit to initial token generation) | High-precision timer (`std::time::Instant`) |
| `prompt_tok_per_sec` | tokens/sec | Prompt processing throughput (prefill speed) | Engine callback telemetry |
| `decode_tok_per_sec` | tokens/sec | Token generation throughput during generation phase | Engine callback telemetry |
| `peak_rss_bytes` | Bytes | Maximum physical RAM occupied by process during run | OS `getrusage` / `GetProcessMemoryInfo` |
| `peak_mapped_bytes` | Bytes | Maximum virtual mapped memory (`mmap`) | `/proc/self/smaps` or platform equivalent |
| `major_page_faults` | Count | Number of page faults requiring disk I/O | OS kernel task statistics |
| `disk_bytes_read` | Bytes | Total bytes read from underlying storage device | `/proc/pid/io` or OS I/O counters |
| `cpu_utilization_pct` | Percent | Average CPU utilization across active logical cores | OS system info counters |
| `temperature_c_max` | Celsius | Maximum CPU core temperature during run | Hardware sensor APIs / sysfs |
| `enforcement_mechanism` | Enum | Enforced memory limit type (`cgroup_v2_hard`, `windows_job_object`, `macos_best_effort`) | `aura-memory` module status |

---

## 3. Test Phases Protocol

### Phase 1: Cold Start Benchmark
- **Pre-condition:** OS page cache for model file purged (`posix_fadvise(DONTNEED)` on Linux, or system reboot / cache flush).
- **Execution:** First run of model loading and 50-token generation pass.
- **Purpose:** Measures worst-case cold disk-streaming performance and initial allocation latency.

### Phase 2: Warm Cache Benchmark
- **Pre-condition:** Immediate re-execution of identical prompt pass within 5 seconds of Cold Start run.
- **Execution:** Second run of 50-token generation pass.
- **Purpose:** Measures performance when OS page cache holds model weight pages in available physical RAM.

### Phase 3: Steady State Benchmark
- **Pre-condition:** Model remains loaded in process.
- **Execution:** 20 consecutive generation passes using standard evaluation prompt suite (e.g. 512 prompt tokens, 128 decode tokens).
- **Purpose:** Computes $P_{50}$, $P_{10}$, mean, and standard deviation of token decode speed under sustained thermal conditions.

---

## 4. Machine-Readable `aura-benchmark.json` Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "schema_version": "1.0",
  "run_id": "c8e2a1b0-7d3e-4b9f-8a12-9876543210fe",
  "timestamp_utc": "2026-08-23T12:00:00Z",
  "aura": {
    "version": "0.1.0",
    "commit_hash": "a1b2c3d4e5f67890123456789abcdef012345678",
    "planner_version": "0.1.0-alpha"
  },
  "hardware": {
    "profile_name": "reference-laptop-8gb",
    "cpu": {
      "model": "Intel(R) Core(TM) i5-1135G7 @ 2.40GHz",
      "physical_cores": 4,
      "logical_cores": 8,
      "simd_features": ["avx2", "avx512f", "avx512_vnni"]
    },
    "ram": {
      "total_bytes": 8589934592,
      "available_bytes_at_start": 6442450944
    },
    "storage": {
      "type": "NVMe Gen3",
      "measured_seq_read_mbps": 2400.0,
      "measured_random_iops": 350000
    },
    "gpu": {
      "present": false,
      "model": null,
      "vram_bytes": null
    },
    "os": {
      "name": "Ubuntu",
      "version": "24.04 LTS",
      "kernel": "6.8.0-40-generic"
    }
  },
  "model": {
    "name": "Qwen2.5-7B-Instruct",
    "source_hash_sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "architecture": "dense",
    "total_params": 7615616512,
    "active_params": 7615616512,
    "quantization": "Q4_K_M",
    "format": "gguf",
    "context_length_configured": 4096
  },
  "backend": {
    "engine": "llama.cpp",
    "engine_commit": "b3421",
    "compile_flags": ["AVX2", "AVX512"]
  },
  "configuration": {
    "memory_budget_bytes": 4294967296,
    "enforcement_mechanism": "cgroup_v2_hard",
    "enforcement_verified": true,
    "thread_count": 4,
    "gpu_layers_offloaded": 0,
    "kv_cache_type": "f16"
  },
  "phases": {
    "cold_start": {
      "ttft_ms": 3200,
      "prompt_tok_per_sec": 42.5,
      "decode_tok_per_sec": 8.2,
      "peak_rss_bytes": 4120000000,
      "peak_mapped_bytes": 4500000000,
      "major_page_faults": 12500,
      "disk_bytes_read": 4250000000
    },
    "warm_cache": {
      "ttft_ms": 450,
      "prompt_tok_per_sec": 120.0,
      "decode_tok_per_sec": 14.5,
      "peak_rss_bytes": 4120000000,
      "peak_mapped_bytes": 4500000000,
      "major_page_faults": 15,
      "disk_bytes_read": 5242880
    },
    "steady_state": {
      "n_samples": 20,
      "decode_tok_per_sec_mean": 14.2,
      "decode_tok_per_sec_p50": 14.3,
      "decode_tok_per_sec_p10": 13.8,
      "cpu_utilization_pct_mean": 85.0,
      "temperature_c_max": 74.0,
      "throttling_detected": false
    }
  },
  "baseline_comparison": {
    "baseline_engine": "llama.cpp default CLI",
    "baseline_result": "OOM",
    "baseline_decode_tok_per_sec": null,
    "aura_win_description": "AURA executed within 4GB budget; baseline crashed with OOM"
  },
  "reproduce_command": "aura run Qwen2.5-7B-Instruct --memory 4G --context 4096"
}
```

---

## 5. Benchmark Reproduction Subsystem (`aura benchmark reproduce`)

The `aura benchmark reproduce <path_to_json>` command validates claims by:
1. Parsing target benchmark JSON artifact.
2. Probing local host hardware to verify hardware signature compatibility.
3. Fetching exact model artifact using cryptographic SHA-256 validation.
4. Executing identical plan configuration under identical memory budget.
5. Emitting comparative reproduction report highlighting performance delta ($\Delta \text{decode tok/s}$, $\Delta \text{peak RSS}$).
