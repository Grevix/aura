# AURA — Frequently Asked Questions

## General

**Can AURA run 7B models on 4 GB?**
Yes, with reduced context (typically 1024 tokens instead of 4096). The model weights for a 7.2B Q4_K_M model are ~4.1 GB. With context=1024 the KV cache is small enough to fit within a 4 GB process memory limit. Inference will be slower than with 6+ GB available. See benchmark results for measured speeds.

**Can AURA run 8B models on 4 GB?**
Usually yes, at context=1024. Models like `llama3:8b` (4.34 GB Q4_0) and `deepseek-r1:latest` (4.87 GB Q4_K_M) fit within a 4 GB virtual commit limit with aggressive context reduction. Measured feasibility varies by exact architecture.

**Can AURA run 14B models on 4 GB?**
No. A 14B Q4_K_M model is approximately 8.5 GB. This is mathematically infeasible under a 4 GB limit even with Q3_K_S quantization and minimum context.

**Can AURA run 32B models on 4 GB?**
No. Physically impossible.

**Can AURA run 70B models on 4 GB?**
No. 70B Q4_K_M is ~40 GB. Not feasible on any consumer hardware with a 4 GB constraint.

**Can AURA run 70B models on larger hardware?**
This is the community benchmark track. With 48+ GB RAM or GPU VRAM, yes. AURA itself can orchestrate larger budgets — the 4 GB limit is configurable, not fixed.

---

## Memory

**How is memory measured?**
AURA enforces a **virtual commit limit** using Win32 `JOB_OBJECT_LIMIT_PROCESS_MEMORY` (Windows) or cgroup v2 memory.limit_in_bytes (Linux). This limits the total virtual address space committed by the AURA process. Measured physical RSS (Resident Set Size) may be 2–5% higher because memory-mapped GGUF file pages are counted by the OS outside the commit charge.

**Why does reported RSS sometimes exceed the configured budget?**
Because `JOB_OBJECT_LIMIT_PROCESS_MEMORY` controls virtual commit, not physical RSS. The OS may keep recently accessed GGUF pages in physical RAM beyond what the commit limit tracks. This is expected behavior and is documented in the Windows SDK. It is not a bug — it is a measurement precision issue.

**Does AURA use swap memory?**
The Win32 Job Object limit constrains commit (which includes page file usage). AURA explicitly does not rely on swap as extra model memory. If the commit limit is reached, the OS terminates the process cleanly.

---

## Comparison

**Is AURA faster than Ollama?**
Depends on the scenario:
- In **batch/automation** mode (non-interactive subprocess): AURA completes 100% of prompts. Ollama CLI 0.32.15 hangs waiting for an interactive TTY, completing 0%.
- In **interactive terminal** mode with no memory constraint: Ollama's native runtime is faster by ~40% for 7B models because it uses more RAM.
- At **equal 4 GB constrained** conditions: AURA succeeds; Ollama would OOM without a constraint mechanism.

**Does AURA replace Ollama?**
No. AURA's default backend for local inference is Ollama (via llama.cpp). AURA plans and enforces memory budgets; Ollama/llama.cpp executes the model.

**Does AURA improve model intelligence?**
No. AURA wraps a backend. Output quality is determined by the model and the backend (llama.cpp), not by AURA.

---

## Hardware

**Does AURA use the GPU?**
Not in the current release. GPU offload layers = 0. GPU support is planned for V9.

**Can I benchmark on Linux?**
Yes — cgroup v2 memory enforcement is implemented. Not yet benchmarked on the reference machine. Community contributions welcome.

**Can I benchmark on macOS?**
Code exists (`macos.rs`), but it uses soft RSS monitoring, not a hard OS-level limit. macOS doesn't expose a per-process hard memory cap equivalent to cgroup v2 or Win32 Job Objects.

---

## Models

**Why is kimi-k3:cloud excluded from local benchmarks?**
Kimi K3 is a 2.81 trillion parameter cloud model served via `ollama.com`. It has no local GGUF blob. Running it requires a remote network call to Moonshot AI's servers. It is not a locally executed model and cannot be evaluated as a local inference benchmark.

**Why are some models marked INFEASIBLE?**
Because the minimum resident memory exceeds the configured budget even at minimum context and minimum quantization. `gemma4:latest` at 8.95 GB is the clearest example on this machine — the blob is simply too large for a 4 GB process limit.

---

## Contributing Benchmarks

**Can I contribute a 70B benchmark?**
Yes — use the [community benchmark runner](benchmarks/community_runner/). Your result will be labelled `COMMUNITY REPORTED`, not `PROJECT VERIFIED`.

**Can I benchmark with an NVIDIA GPU?**
Not yet — AURA sets GPU offload = 0 in all current tests. You can modify this for your community benchmark, but label results clearly as `GPU ACCELERATED`.
