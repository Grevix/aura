# AURA Research Gaps & Technical Open Questions

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Document Type:** Technical Gap Analysis & Research Agenda  
**Status:** Open Engineering Tracking  

---

## 1. Unresolved Technical Questions & Experimental Agenda

The following 6 core technical questions cannot be answered by assertion and require empirical spike experiments:

### GAP-01: llama.cpp CPU-MoE Issue #19480 Performance Tracking
- **Question:** What is the actual CPU token generation throughput gap on modern MoE GGUF models (Mixtral 8x7B, Qwen-MoE) running via `llama.cpp` on CPU-only hosts, and when will issue `#19480` resolve the ~5× bandwidth degradation?
- **Provisional Decision:** Scope Stage A MVP strictly to dense 7B-8B GGUF models. Stage B MoE models require GPU/iGPU presence for offloading non-expert layers until issue `#19480` is resolved upstream.
- **Required Experiment:** Benchmark `llama-cli` on Qwen1.5-MoE-A2.7B across CPU-only vs iGPU offload tiers.

### GAP-02: Universal Virtual Tensor Memory (VTM) Unification Value
- **Question:** Does unifying weight paging, KV cache paging, and expert buffer paging into a single custom VTM engine yield a statistically significant performance gain over standard OS mmap + `llama.cpp` buffer management?
- **Provisional Decision:** Reject custom VTM for 0.x releases. Implement AURA as an adaptive planner above `llama.cpp`. Re-evaluate VTM for 2.0.0 only if telemetry proves OS mmap thrashing cannot be mitigated via planner heuristics.
- **Required Experiment:** Measure major page fault frequency and storage I/O stalls on a 4GB RAM host streaming a 27B Q4 model via mmap vs chunked prefetch.

### GAP-03: Windows Soft Memory Backpressure Throttling
- **Question:** Since Windows Job Objects terminate processes abruptly upon hitting `ProcessMemoryLimit` without a graceful `MemoryHigh` equivalent, how effectively can AURA's RSS polling loop prevent hard job object crashes?
- **Provisional Decision:** Implement a 50ms polling loop using `GetProcessMemoryInfo` on Windows to trigger early context reduction before the Job Object hard limit is reached.
- **Required Experiment:** Stress-test allocation loops under Windows Job Objects to determine optimal soft warning thresholds (e.g. 85% or 90% of commit limit).

### GAP-04: Expert Locality Stability Under Real User Prompts
- **Question:** Does modern MoE expert routing demonstrate high temporal locality and expert reuse during multi-turn conversation prompts, or does routing entropy destroy cache prefetching efficiency?
- **Provisional Decision:** Treat expert prefetching as a secondary research track (0.7.0+). Rely on deterministic layer loading for baseline execution.
- **Required Experiment:** Instrument `llama.cpp` expert routing callbacks to record expert ID sequences over 100 benchmark prompts and compute routing entropy and LRU cache hit rates.

### GAP-05: Storage Bandwidth Profiler Accuracy vs OS Caching
- **Question:** How can `aura-hardware` measure true physical SSD sequential read bandwidth without being skewed by OS dirty page cache or pre-fetched storage buffers?
- **Provisional Decision:** Use direct I/O flags (`O_DIRECT` on Linux, `FILE_FLAG_NO_BUFFERING` on Windows) during non-destructive 64MB storage read tests.
- **Required Experiment:** Compare Direct I/O storage benchmark measurements against raw `fio` benchmarks on NVMe and SATA SSDs.

### GAP-06: Perplexity Degradation of Extreme Low-Bit Quantization
- **Question:** At what point does ultra-low-bit quantization (`IQ3_XS`, `Q2_K`) degrade task quality beyond usability on 7B-27B models?
- **Provisional Decision:** Set `Q4_K_M` as the recommended default. Permit `Q3_K_S` only under severe memory constraints, accompanied by an explicit user warning.
- **Required Experiment:** Run WikiText-2 perplexity evaluations and MMLU benchmark suites across GGUF `Q4_K_M`, `Q3_K_S`, and `IQ3_XS` variants for target MVP models.
