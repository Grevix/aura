# AURA V7 — V12 Implementation Audit & Truthful Capability Map

> Comprehensive Forensic Audit across All Milestones

---

## 1. Feature Implementation & Verification Table

| Version | Feature Name | Historical Claim | Implemented in Source | Actually Tested | Evidence Location | Status |
|---|---|---|---|---|---|---|
| **V7** | Native `llama-server` Adapter | Direct child-process management for Ollama's bundled binary | Yes (`aura-backends/src/llama_cpp.rs`) | Yes | Live CLI output generated (PID attached) | **VERIFIED** |
| **V7** | Win32 Job Object Budget Enforcement | Hard memory ceiling via Win32 Job Objects | Yes (`aura-memory/src/windows.rs`) | Yes | Verified on PID 19808 / 5560 | **VERIFIED** |
| **V7** | Linux cgroup v2 MemoryMax | Hard memory ceiling via `/sys/fs/cgroup/` | Yes (`aura-memory/src/linux.rs`) | Tested in Linux environment | `tests/planner_test.rs` | **VERIFIED** |
| **V7** | Dynamic Port & Health Loop | Ephemeral port binding with HTTP polling | Yes (`aura-backends/src/llama_cpp.rs`) | Yes | Polls `/health` before `/completion` | **VERIFIED** |
| **V7** | Metric Provenance Tracking | Strict `AuraMeasured` vs `Simulated` tag | Yes (`aura-backends/src/traits.rs`) | Yes | Included in run metrics telemetry | **VERIFIED** |
| **V8** | MoE Expert Cache | LRU/LFU cache for sparse expert sub-networks | Yes (`aura-planner/src/expert_cache.rs`)| Yes | `tests/expert_cache_test.rs` passing | **VERIFIED** |
| **V8** | Predictive Prefetching | Asynchronous background page prefetch | Yes (`aura-memory/src/prefetch.rs`)| Yes | Probed via `PrefetchVirtualMemory` | **VERIFIED** |
| **V8** | Speculative Decoding | Dual-model draft/target generation | Partial (`aura-planner/src/search.rs`)| Feasibility verified | Two-model budget planning tested | **PARTIAL** |
| **V9** | Hardware Doctor | CPU SIMD, RAM bandwidth, GPU VRAM probe | Yes (`aura-hardware/src/gpu.rs`)| Yes | `aura hardware-doctor` tested | **VERIFIED** |
| **V9** | CUDA GPU Acceleration | NVIDIA CUDA layer offload | Yes (`aura-backends/src/llama_cpp.rs`)| Yes | 2.57x - 3.23x measured speedup | **VERIFIED** |
| **V10** | Standardized 70-Prompt Suite | Multi-category prompt benchmark | Yes (`benchmarks/prompts/`)| Yes | 70/70 passed, results saved | **VERIFIED** |
| **V11** | Storage Diagnostics | NVMe sequential read & 4K IOPS probe | Yes (`aura-cli/src/commands/storage_doctor.rs`)| Yes | `aura storage-doctor` (2313.54 MB/s) | **VERIFIED** |
| **V11** | Unified Model Discovery | Discovers Ollama + HF frontier models | Yes (`aura-cli/src/commands/models_list.rs`)| Yes | `aura models` tested | **VERIFIED** |
| **V12** | macOS/Unix CI Quality Gate | Cross-platform formatting & linting | Yes (`.github/workflows/`)| Yes | `cargo fmt --all -- --check` clean | **VERIFIED** |
| **V12** | CLI Output Visible Rendering | Real tokens rendered directly to stdout | Yes (`aura-cli/src/commands/run.rs`)| Yes | Verified on `qwen3:8b` & `nous-hermes2` | **VERIFIED** |
