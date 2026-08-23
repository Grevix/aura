# AURA Implementation Plan (0.1.0 → 2.0.0)

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Document Type:** Master Implementation Specification  
**Status:** Approved  

---

## Program Execution Workflow

```
PHASE → TASK → SUBTASK → FILES → MODULE → API → TEST → BENCHMARK → EXIT CRITERIA
```

Every release gate requires:
1. All unit and integration tests passing.
2. Zero memory budget leaks or unhandled memory panics.
3. Clean build across target OS matrix (Linux x86_64, Windows x86_64, macOS ARM64).
4. Machine-readable `aura-benchmark.json` published and verified.

---

## 0.1.0 — FOUNDATION & HARNESS

### Phase 1: Workspace & Core Types Setup
- **Task 1.1:** Initialize Cargo Monorepo structure.
  - Subtask: Create `Cargo.toml` workspace manifest with core crates (`aura-core`, `aura-hardware`, `aura-model`, `aura-planner`, `aura-memory`, `aura-backends`, `aura-cli`, `aura-benchmark`).
  - Subtask: Configure standard linting (`clippy.toml`, `rustfmt.toml`).
  - **Files:** `Cargo.toml`, `crates/aura-core/src/lib.rs`, `crates/aura-core/src/types.rs`
  - **Module:** `aura_core::types`
  - **API:** `pub struct HardwareProfile`, `pub struct ModelManifest`, `pub struct ExecutionPlan`
  - **Test:** `crates/aura-core/tests/types_test.rs`
  - **Exit Criteria:** Clean build with `cargo build --workspace`.

### Phase 2: Hardware Profiling Subsystem
- **Task 2.1:** Implement CPU & RAM probing.
  - Subtask: Query CPU topology (physical cores, logical threads, base/max clock).
  - Subtask: Query SIMD extension availability (`AVX`, `AVX2`, `AVX512F`, `AVX512_VNNI`, `NEON`, `AMX`).
  - Subtask: Read physical RAM, available RAM, swap space.
  - **Files:** `crates/aura-hardware/src/cpu.rs`, `crates/aura-hardware/src/memory.rs`
  - **Module:** `aura_hardware::cpu`, `aura_hardware::memory`
  - **API:** `pub fn detect_cpu() -> CpuProfile`, `pub fn detect_memory() -> MemoryProfile`
  - **Test:** `crates/aura-hardware/tests/probe_test.rs`
  - **Benchmark:** Probe execution time < 15 ms.
  - **Exit Criteria:** Returns valid deterministic hardware profile on Linux, Windows, macOS.

- **Task 2.2:** Implement Storage Bandwidth Profiler.
  - Subtask: Perform non-destructive read benchmark (64 MB sequential read, 4K random read IOPS).
  - **Files:** `crates/aura-hardware/src/storage.rs`
  - **Module:** `aura_hardware::storage`
  - **API:** `pub fn benchmark_storage(path: &Path) -> StorageProfile`
  - **Test:** Storage benchmark cleanup verification.
  - **Benchmark:** Bandwidth measurement completes in < 1.5 seconds.
  - **Exit Criteria:** Reports storage sequential MB/s and IOPS within ±10% accuracy.

### Phase 3: Model Header Parser & GGUF Registry
- **Task 3.1:** Build GGUF Metadata Parser.
  - Subtask: Parse GGUF magic bytes, version, header metadata without loading weight tensors into RAM.
  - Subtask: Extract layer count, parameter count, architecture (`llama`, `qwen2`, `deepseek2`), quantization type, context length.
  - **Files:** `crates/aura-model/src/gguf.rs`, `crates/aura-model/src/manifest.rs`
  - **Module:** `aura_model::gguf`
  - **API:** `pub fn parse_gguf_manifest(path: &Path) -> Result<ModelManifest, ModelError>`
  - **Test:** Test parsing on valid GGUF files and invalid corrupted files.
  - **Exit Criteria:** Header parsing overhead < 5 ms for 50 GB GGUF file.

### Phase 4: CLI Skeleton & Baseline Backend Integration
- **Task 4.1:** Build `aura-cli` commands.
  - Subtask: Implement `aura doctor`, `aura models`, `aura plan`, `aura run`, `aura benchmark`.
  - **Files:** `crates/aura-cli/src/main.rs`, `crates/aura-cli/src/commands/*.rs`
  - **Module:** `aura_cli`
  - **API:** CLI entrypoint parsing via `clap`.
  - **Test:** CLI argument integration tests.
  - **Exit Criteria:** CLI executes all subcommands with formatted terminal tables.

- **Task 4.2:** Integrate `llama.cpp` Execution Backend Adapter.
  - Subtask: Build process-level adapter launching `llama-cli` / `llama-server` binary.
  - Subtask: Stream output tokens and compute TTFT and decode throughput.
  - **Files:** `crates/aura-backends/src/llama_cpp.rs`
  - **Module:** `aura_backends::llama_cpp`
  - **API:** `pub trait BackendAdapter`, `pub struct LlamaCppAdapter`
  - **Test:** Integration test executing 7B model generation.
  - **Exit Criteria:** Successful completion of 50-token inference generation pass.

### Phase 5: Benchmark & Audit Engine
- **Task 5.1:** Implement JSON Schema Exporter.
  - Subtask: Collect hardware, model, configuration, phases (cold start, warm cache, steady state), baseline comparison, and quality metrics.
  - Subtask: Export to standardized `aura-benchmark.json`.
  - **Files:** `crates/aura-benchmark/src/schema.rs`, `crates/aura-benchmark/src/exporter.rs`
  - **Module:** `aura_benchmark`
  - **API:** `pub fn generate_benchmark_report(...) -> BenchmarkReport`
  - **Test:** Schema validation test against JSON schema.
  - **Exit Criteria:** Benchmark exporter outputs valid schema compliant JSON.

---

## 0.2.0 — FEASIBILITY PLANNER & MEMORY BUDGETING

### Phase 1: Analytical Feasibility Planner
- **Task 1.1:** Build Memory Footprint Estimator.
  - Subtask: Compute weight footprint $W = \sum \text{tensor\_bytes}$.
  - Subtask: Compute KV cache memory $KV = 2 \times n_{layers} \times n_{heads} \times d_{head} \times CtxSize \times PrecisionBytes$.
  - Subtask: Compute runtime overhead allocation buffer $Buffer = 512 \text{ MB}$.
  - **Files:** `crates/aura-planner/src/estimator.rs`
  - **Module:** `aura_planner::estimator`
  - **API:** `pub fn estimate_memory_footprint(manifest: &ModelManifest, ctx: usize, quant: QuantType) -> MemoryEstimate`
  - **Test:** Unit tests comparing analytical estimates against measured RSS.
  - **Exit Criteria:** Estimate within ±7% of measured llama.cpp peak RSS.

- **Task 1.2:** Build Feasibility Rule Engine.
  - Subtask: Compare estimated memory against budget.
  - Subtask: Rejection logic when storage bandwidth imposes > 30 sec/token cold start.
  - **Files:** `crates/aura-planner/src/rules.rs`
  - **Module:** `aura_planner::rules`
  - **API:** `pub fn evaluate_plan(hw: &HardwareProfile, manifest: &ModelManifest, budget: &MemoryBudget) -> PlanResult`
  - **Test:** Feasibility evaluation matrix unit tests.
  - **Exit Criteria:** Correctly rejects impossible execution plans (e.g. 70B FP16 on 4GB budget).

---

## 0.3.0 — HARD OS MEMORY ENFORCEMENT

### Phase 1: OS-Specific Hard Memory Enforcement
- **Task 1.1:** Linux `cgroups v2` Enforcer.
  - Subtask: Spawn llama.cpp inside transient systemd scope (`MemoryMax`, `MemoryHigh`, `MemorySwapMax=0`).
  - Subtask: Stream `memory.pressure` (PSI) telemetry.
  - **Files:** `crates/aura-memory/src/linux.rs`
  - **Module:** `aura_memory::linux`
  - **API:** `pub fn apply_linux_cgroup(pid: u32, budget_bytes: u64) -> Result<CgroupHandle, MemoryError>`
  - **Test:** Trigger deliberate allocation overflow test; confirm SIGKILL / soft backpressure.
  - **Exit Criteria:** Linux cgroup guarantees process RSS never exceeds `MemoryMax`.

- **Task 1.2:** Windows Job Object Enforcer.
  - Subtask: Create Windows Job Object with `ProcessMemoryLimit`.
  - Subtask: Assign backend process handle to Job Object.
  - **Files:** `crates/aura-memory/src/windows.rs`
  - **Module:** `aura_memory::windows`
  - **API:** `pub fn apply_windows_job_object(process_handle: HANDLE, budget_bytes: u64) -> Result<(), MemoryError>`
  - **Test:** Memory allocation limit integration test on Windows.
  - **Exit Criteria:** Process terminated immediately upon crossing hard commit limit.

- **Task 1.3:** macOS Best-Effort RSS Polling Enforcer.
  - Subtask: Poll process RSS via `proc_pidinfo` at 50ms intervals.
  - Subtask: Issue SIGSTOP/SIGTERM when crossing 95% of memory threshold.
  - **Files:** `crates/aura-memory/src/macos.rs`
  - **Module:** `aura_memory::macos`
  - **API:** `pub fn spawn_macos_rss_monitor(pid: i32, budget_bytes: u64) -> MonitorHandle`
  - **Test:** Polling loop threshold test.
  - **Exit Criteria:** Output clearly flags enforcement as `macos_best_effort` in benchmark JSON.

---

## 0.4.0 — AUTOMATIC CONFIGURATION SEARCH

- **Task 4.1:** Automated Parameter Search.
  - Subtask: Search quantization space (`Q4_K_M`, `Q3_K_S`, `IQ3_XS`), context window (`2048`, `4096`, `8192`), and CPU thread allocation to maximize predicted decode throughput under memory budget.
  - **Files:** `crates/aura-planner/src/search.rs`
  - **Exit Criteria:** Search completes in < 50 ms and produces non-OOM execution plan.

---

## 0.5.0 → 1.0.0 — CACHE, PREFETCH, MOE & STABLE RELEASE

- **0.5.0:** MoE model GGUF metadata parsing (Mixtral, Qwen-MoE, DeepSeek-MoE). Expert parameter count separation.
- **0.6.0:** Weight & KV cache management policies (LRU, page cache warming).
- **0.7.0:** Storage-aware asynchronous prefetching engine (`aura-storage`).
- **0.8.0:** Hardware acceleration tuning (AVX-512, Metal, Vulkan split offloading).
- **0.9.0:** Benchmark database telemetry aggregation & Bayesian plan calibration.
- **1.0.0:** Production Release candidate: Full cross-platform signed release binaries, complete documentation, automated CI/CD benchmark audit gate.

---

## 1.1.0 → 2.0.0 — ADVANCED OPTIMIZATIONS & RESEARCH

- **1.1.0–1.9.0:** Opt-in shared community benchmark database, expert locality activation tracing, KV compression, iGPU/dGPU dynamic graph splitting, thermal throttling feedback loops.
- **2.0.0:** AURA Virtual Tensor Memory (VTM) unified paging layer across weights, KV cache, and expert buffers — evaluated strictly if empirical benchmark telemetry demonstrates superior performance over standard OS mmap.
