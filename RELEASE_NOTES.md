# AURA v0.1.0 Release Notes

**AURA (Adaptive Ultra-Low-Memory Runtime for AI)** is a hardware-aware memory-budget enforcement and inference orchestration engine for local LLMs on consumer hardware.

---

## Highlights

- **Hardware-Aware Memory Planning**: Probes host CPU physical cores, SIMD capabilities (AVX2/AVX-512), installed/available RAM, and NVMe IOPS before executing inference.
- **Hard Kernel Budget Enforcement**: Enforces hard process commit limits using native OS primitives — Win32 `JOB_OBJECT_LIMIT_PROCESS_MEMORY` on Windows and cgroup v2 `MemoryMax` on Linux.
- **Dynamic Context Window Scaling**: Multi-pass search algorithm scales context length (e.g. 4096 → 2048 → 1024) to fit specified memory ceilings without OOM crashes.
- **Quantization Fallback Engine**: Evaluates and suggests lower precision quants (e.g. Q3_K_S) when standard Q4_K_M weights exceed target memory budgets.
- **Cross-Platform Compatibility**: Full compilation and verified execution on Windows 10/11, Ubuntu Linux, and macOS (x86_64 / Apple Silicon).
- **llama.cpp Production Integration**: Uses production llama.cpp AVX2 GEMM kernels for high-throughput CPU inference.
- **MoE Expert LRU Eviction Cache**: Manages Mixture-of-Experts active memory footprints with configurable LRU cache policies.
- **Cold-Start Prefetch Optimization**: Uses Win32 `PrefetchVirtualMemory` and Unix `madvise(MADV_WILLNEED)` to reduce cold-start Time-To-First-Token (TTFT).

---

## Verified 4 GB Benchmark Results

*Target machine: Intel Core i5-13420H (8 physical / 12 logical cores, AVX2 SIMD, 16.79 GB RAM, PCIe Gen4 NVMe).*

| Model Tag | Parameters | GGUF Size | Quantization | Feasible @ 4GB | Max Context @ 4GB |
|---|---|---|---|---|---|
| `qwen3:0.6b` | 751M | 0.49 GB | Q4_K_M | ✅ Feasible | 32,768 |
| `qwen3:1.7b` | 2.0B | 1.27 GB | Q4_K_M | ✅ Feasible | 32,768 |
| `llama3.2:1b` | 1.2B | 1.23 GB | Q8_0 | ✅ Feasible | 131,072 |
| `llama3.2:3b` | 3.2B | 1.88 GB | Q4_K_M | ✅ Feasible | 131,072 |
| `qwen3:4b` | 4.0B | 2.60 GB | Q4_K_M | ✅ Feasible | 131,072 |
| `mistral:latest` | 7.2B | 4.07 GB | Q4_K_M | ✅ Feasible | 4,096 |
| `qwen2.5-coder:7b` | 7.6B | 4.36 GB | Q4_K_M | ✅ Feasible | 1,024 |
| `llama3:8b` | 8.0B | 4.34 GB | Q4_0 | ✅ Feasible | 1,024 |
| `deepseek-r1:latest` | 8.2B | 4.87 GB | Q4_K_M | ✅ Feasible | 1,024 |
| `qwen3:8b` | 8.2B | 4.87 GB | Q4_K_M | ✅ Feasible | 1,024 |
| `gemma4:latest` | 8.0B* | 8.95 GB | Q4_K_M | ❌ Infeasible | — |

---

## CLI Commands Included in Release

The compiled `aura` binary supports five primary subcommands:

1. `aura doctor`: Probes CPU, SIMD, physical memory, storage IOPS, and OS environment.
2. `aura plan --model <MODEL> --memory <BUDGET>`: Generates a hardware-aware execution plan.
3. `aura run --model <MODEL> --memory <BUDGET> --prompt <PROMPT>`: Executes budget-enforced model inference.
4. `aura benchmark [--out <OUTPUT>]`: Runs performance telemetry benchmarks.
5. `aura audit [--out <OUTPUT>]`: Validates release audit criteria and outputs audit telemetry.

---

## Security & Licensing Audit

- **Vulnerability Audit**: Passed `cargo audit` cleanly with 0 known vulnerabilities.
- **License Audit**: Passed `cargo deny check licenses` cleanly under Apache-2.0 / MIT policy.
- **Allowed Licenses**: MIT, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-2-Clause, BSD-3-Clause, MPL-2.0, Unicode-DFS-2016, Unicode-3.0, 0BSD.

---

## Installation & Verification

### From Source
```bash
git clone https://github.com/Grevix/aura.git
cd aura
cargo build --release
```

### Verification Commands
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

---

## Known Limitations

- Discrete GPU offloading is not supported in v0.1.0 (CPU-only execution).
- Models larger than ~5 GB are mathematically infeasible under a strict 4.0 GB budget without sub-Q4 quantization.
- Virtual commit limit (Win32 Job Objects) restricts virtual memory charge; measured RSS may be 2–4% higher due to read-only shared GGUF mmap pages.
