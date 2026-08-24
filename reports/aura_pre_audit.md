# AURA Pre-Audit Architectural & Verification Report

> Commit: `c0a9210`
> Target Hardware: 13th Gen Intel Core i5-13420H (12 vCPU), 16.79 GB RAM, NVIDIA GeForce RTX 4050 Laptop GPU (6.00 GB VRAM, Driver 592.82, CUDA 13.1)

---

## 1. Roadmap Classification & Status Matrix

| Milestone | Requirement | Implementation Status | Runtime Status | Real Test Status | Classification |
|---|---|---|---|---|---|
| **V7** | Native llama-server Adapter | `crates/aura-backends/src/llama_cpp.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V7** | Child Process Memory Limits | `crates/aura-memory/src/windows.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V7** | Dynamic Port & /health Polling | `crates/aura-backends/src/llama_cpp.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V7** | Ollama REST Comparison | `crates/aura-backends/src/ollama_baseline.rs`| RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V7** | Dynamic Model Discovery | `crates/aura-model/src/ollama.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V7** | Model Tier Classification | `crates/aura-planner/src/search.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V7** | Provenance & is_simulated | `crates/aura-backends/src/traits.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V8** | Predictive Weight Prefetch | `crates/aura-memory/src/prefetch.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V8** | MoE Expert Cache | `crates/aura-planner/src/expert_cache.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V8** | Speculative Decoding | `crates/aura-planner/src/search.rs` | PLANNER_ONLY | PASSED | **PARTIALLY VERIFIED** |
| **V8** | Flash Attention 2 | `crates/aura-core/src/types.rs` | CAPABILITY_ONLY | PASSED | **CAPABILITY-GATED** |
| **V8** | Sliding / Ring Attention | `crates/aura-core/src/types.rs` | CAPABILITY_ONLY | PASSED | **CAPABILITY-GATED** |
| **V8** | Linux cgroup v2 | `crates/aura-memory/src/linux.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V9** | GPU Hardware Doctor | `crates/aura-hardware/src/gpu.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V9** | CUDA Acceleration | `crates/aura-backends/src/llama_cpp.rs` | RUNTIME_VERIFIED | PASSED | **VERIFIED** |
| **V9** | Vulkan / DirectML / Metal | `crates/aura-core/src/types.rs` | CAPABILITY_ONLY | PASSED | **CAPABILITY-GATED** |
| **V9** | Remote Node Backend | `crates/aura-core/src/types.rs` | NETWORK_GATED | PASSED | **NETWORK-GATED** |
