# AURA — Roadmap

> No dates are promised. Milestones are ordered by engineering priority, not schedule.

---

## V7 (Current)

**Theme: Fair benchmarking and community infrastructure**

- [x] Ollama REST API comparison harness (`/api/generate`, `eval_count`/`eval_duration` timing)
- [x] Auto model discovery via Ollama REST (`/api/tags`) — no hardcoded inventories
- [x] Multi-tier model coverage: TINY (0.6B) through LARGE (8.2B)
- [x] Community benchmark runner — single command, validated JSON output
- [x] Physics-based decode estimate replacing hardcoded `35.0 tok/s`
- [x] `is_simulated` flag in `BackendOutput` — prevents fabricated numbers reaching reports
- [x] Memory terminology precision: virtual commit limit vs physical RSS documented
- [x] Professional README with explicit "What AURA Does NOT Claim" section
- [x] CI/CD pipeline: format, clippy, test, build, deny

---

## V8 (Research)

**Theme: Reduce memory footprint without sacrificing quality**

- [ ] **Flash Attention 2 integration** — reduces KV cache footprint by ~60%, enabling longer context at same RAM budget (requires llama.cpp version with FA2)
- [ ] **Sliding / Ring Attention** — effective context extension without proportional KV cache growth
- [ ] **Speculative decoding** — 2–3× decode speedup using a small draft model (e.g., 0.6B as draft, 8B as target)
- [ ] **Predictive weight prefetch** — overlap NVMe read with active layer compute
- [ ] **Improved MoE expert cache** — admission policy beyond simple LRU (frequency-weighted, semantic routing hint)
- [ ] **Linux cgroup v2 benchmark** — verify memory enforcement on Ubuntu/Fedora with empirical data
- [ ] **Ollama REST API timing integration in AURA CLI** — expose Ollama's `eval_count`/`eval_duration` directly in `aura benchmark` output

---

## V9 (Exploration)

**Theme: GPU and distributed support**

- [ ] **CUDA backend** — GPU offload via llama.cpp CUDA path
- [ ] **Vulkan backend** — cross-platform GPU acceleration
- [ ] **DirectML backend** — Windows GPU acceleration (AMD/Intel integrated)
- [ ] **Metal backend** — macOS GPU (Apple Silicon)
- [ ] **Multi-device execution** — heterogeneous CPU+GPU tensor placement
- [ ] **Remote model execution** — AURA as orchestrator for distributed inference

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
