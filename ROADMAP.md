# AURA — Engineering Roadmap

> No dates are promised. Milestones are ordered by engineering priority and validated capabilities.

---

## Completed Releases

### V7 (Production-Verified Base)
- [x] **Native `llama-server` Child Process Adapter**: Automatic port binding, DLL resolution, and readiness `/health` polling.
- [x] **Child Process Memory Enforcement**: Win32 Job Objects (`JOB_OBJECT_LIMIT_PROCESS_MEMORY`) and Linux cgroup v2 (`MemoryMax`).
- [x] **Metric Provenance Tracking**: Strict telemetry tagging (`AuraMeasured`, `OllamaMeasured`, `PlannerEstimated`, `Simulated`).
- [x] **Ollama Model Discovery**: Dynamic resolution of local manifests and GGUF blobs (`/api/tags`).

### V8 (Memory Efficiency & Speculative Architecture)
- [x] **MoE Expert Cache**: Hybrid LRU/LFU frequency-weighted cache for sparse expert sub-networks (`crates/aura-planner/src/expert_cache.rs`).
- [x] **Predictive Prefetching**: Memory prefetch abstractions (`Win32 PrefetchVirtualMemory` / Unix `madvise`).
- [x] **Speculative Decoding Architecture**: Multi-model memory budget planning and feasibility evaluation.

### V9 (Hardware Detection & CUDA Acceleration)
- [x] **Hardware Doctor (`aura hardware-doctor`)**: Real-time probing of CPU SIMD, RAM bandwidth, GPU VRAM, and storage IOPS.
- [x] **CUDA GPU Layer Offloading**: NVIDIA CUDA support (empirically measured 2.57x–3.23x speedup on RTX 4050 Laptop GPU).
- [x] **Modular Backend Architecture**: Support for CPU, CUDA, and baseline Ollama adapters.

### V10 (Standardized 70-Prompt Benchmark Suite)
- [x] **70-Prompt Multi-Category Suite**: 70 real prompts across Reasoning, Math, Coding, Debugging, SQL, JSON, Multilingual, and Instruction Following.
- [x] **Multi-Format Results Generation**: Export to JSON, JSONL, CSV, and Markdown with full provenance tagging.

### V11 & V12 (Out-of-Core Memory Hierarchy & Frontier Safeguards)
- [x] **Storage Doctor (`aura storage-doctor`)**: Sequential read throughput and random 4K IOPS benchmarking.
- [x] **Unified Model Discovery (`aura models`)**: Dynamic discovery across Ollama and Frontier model architectures.
- [x] **Frontier Model Feasibility Inspector (`aura frontier inspect`)**: Pre-execution resource evaluation for 2.8T MoE and 753B models preventing fatal system OOMs.
- [x] **CLI Output Rendering Stabilization**: Verified end-to-end stdout token rendering without dropped pipes or output truncation.
- [x] **Cross-Platform CI Matrix**: Stable, deterministic quality gates passing across Windows, Ubuntu, and macOS.

---

## Active & Upcoming Engineering Milestones

### V13 (Out-of-Core Native Layer Streaming)
- [ ] **Asynchronous Double-Buffered Layer Streamer**: Pipelined NVMe SSD → RAM Staging → GPU VRAM transfer overlapping layer computation with next-layer I/O.
- [ ] **Persistent Layer Shard Cache**: Checkpoint decomposition into indexed `.safetensors` layer slices with CRC64 checksums.
- [ ] **Adaptive Prefetch Depth**: Dynamic adjustment of prefetch queue depth (0, 1, 2, 4) based on host RAM headroom and storage latency.

### V14 (Heterogeneous & Distributed Backend Expansion)
- [ ] **Vulkan Backend Acceleration**: Cross-vendor GPU acceleration for AMD and Intel discrete graphics.
- [ ] **Metal Backend Acceleration**: Unified memory acceleration for Apple Silicon (M1/M2/M3/M4).
- [ ] **Remote Shard Streaming Adapter**: Streaming layer tensors over high-speed local network / NVMe-oF from remote storage nodes.
- [ ] **Flash Attention 2 & Ring Attention Integration**: Upgraded KV-cache management when supported by underlying backends.

---

## Research Directions (Long-Term)

- Predictive expert activation routing for ultra-sparse MoE models.
- Hardware-in-the-loop continuous benchmarking across heterogeneous nodes.
- Paged KV cache tiering between GPU VRAM, host RAM, and NVMe swap.

---

## Non-Goals

- **Automatic model downloading / hosting**: AURA does not run a proprietary model hub; it integrates with standard Hugging Face and Ollama formats.
- **Model quality fabrication**: AURA does not alter model weights or claim synthetic accuracy numbers.
- **Fictitious local execution**: Models exceeding physical host storage/RAM limits are never marked as runnable without real token generation evidence.
