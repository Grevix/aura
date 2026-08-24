# AURA — Roadmap

> No dates are promised. Milestones are ordered by engineering priority, not schedule.

---

## V7 (Current Release — Production-Verified)

**Theme: Real process-managed inference, budget enforcement & fair benchmarking**

- [x] **Native `llama-server` Adapter**: Direct child-process management for Ollama's bundled `llama-server.exe` / `llama-cli`
- [x] **Child Process Memory Enforcement**: Win32 Job Objects (`JOB_OBJECT_LIMIT_PROCESS_MEMORY`) and Linux cgroup v2 (`MemoryMax`) attached directly to child process PIDs
- [x] **Working Directory & DLL Resolution**: Automatic execution environment setup allowing Ollama bundled libraries (`ggml.dll`, `libllama.dll`) to load seamlessly
- [x] **Dynamic Port & `/health` Polling**: Ephemeral TCP port allocation with readiness polling before sending HTTP inference requests
- [x] **Proven Telemetry Provenance**: Strict `MetricProvenance` tracking (`AuraMeasured`, `OllamaMeasured`, `PlannerEstimated`, `Simulated`) preventing synthetic numbers from masking real metrics
- [x] **Ollama REST API Comparison Harness**: `/api/generate` integration with `eval_count`/`eval_duration` timing
- [x] **Auto Model Discovery**: Ollama REST `/api/tags` integration without hardcoded inventories
- [x] **Multi-Tier Model Coverage**: TINY (0.6B) through LARGE (8.2B) feasibility matrix
- [x] **Community Benchmark Runner**: Single-command execution with validated JSON output schema
- [x] **CI/CD Quality Pipeline**: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo audit`, `cargo deny`

---

## V8 (Research & Memory Efficiency)

**Theme: Reduce memory footprint without sacrificing quality**

- [x] **Predictive Weight Prefetching**: Ring-buffered layer prefetching utilizing native Win32 `PrefetchVirtualMemory` and Unix `madvise(MADV_WILLNEED)`
- [x] **Improved MoE Expert Cache**: Hybrid LRU/LFU frequency-weighted admission policy with hit/miss/eviction telemetry
- [x] **Speculative Decoding Architecture**: Planner & CLI support (`aura run --model target:8b --draft-model draft:0.6b --memory 4G`) evaluating combined dual-model memory feasibility
- [x] **Ollama REST Timing Integration**: Exposes native `eval_count` and `eval_duration` fields directly in AURA telemetry outputs
- [ ] **Flash Attention 2 Capability Detection**: Reduces KV cache footprint by ~60% where supported by underlying llama.cpp toolchain (Capability-gated)
- [ ] **Sliding / Ring Attention**: Effective context extension without proportional KV cache growth (Capability-gated)
- [x] **Linux cgroup v2 Enforcement**: Process tree memory enforcement on Linux via cgroup v2 `MemoryMax`

---

## V9 (Exploration & Accelerated Architectures)

**Theme: Hardware-accelerated, heterogeneous, and distributed support**

- [x] **Backend Capability Abstractions**: Modular `BackendType` engine supporting `CpuLlamaCpp`, `CudaLlamaCpp`, `VulkanLlamaCpp`, `DirectMLLlamaCpp`, `MetalLlamaCpp`, and `RemoteBackend`
- [x] **Capability-Driven Hardware Discovery**: Probes host GPU accelerators and returns explicit capability status (`Active` / `Unavailable` / `Disabled`) without faking uninstalled hardware
- [ ] **CUDA Backend Acceleration**: Native CUDA GPU layer offloading when NVIDIA GPUs are detected
- [ ] **Vulkan Backend Acceleration**: Cross-platform GPU acceleration
- [ ] **DirectML Backend Acceleration**: Windows GPU acceleration for AMD/Intel integrated graphics
- [ ] **Metal Backend Acceleration**: Apple Silicon unified memory GPU acceleration
- [ ] **Heterogeneous Multi-Device Placement**: Dynamic layer splitting across CPU and multiple GPUs
- [ ] **Remote AURA Node Orchestration**: Multi-node distributed inference

---

## Research Directions (No Timeline)

- Predictive neuron activation masking (sparse forward pass)
- Weight sparsity exploitation for CPU inference
- Adaptive tensor paging (page-fault driven weight loading)
- Hardware-in-the-loop benchmark automation
- Cross-hardware reproducibility protocol

---

## What Will NOT Be Added

- **Automatic model downloading** — AURA does not manage your model library; use Ollama for that
- **Model quality improvements** — AURA cannot improve what the model knows
- **Proprietary cloud model access** — cloud-routed models (e.g., Kimi K3) are excluded from local benchmarks by definition
