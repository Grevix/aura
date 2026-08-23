# AURA Research Lab & Jupyter Control Plane Architecture

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Document Type:** Experimental Research & Notebook Architecture Standard  
**Status:** Approved Engineering Baseline  

---

## 1. Architecture Topology

The **AURA Experiment Lab** uses Jupyter Notebooks as a reproducible experimental control plane. Notebooks drive CLI benchmarks (`aura` CLI and `ollama` CLI), capturing hardware telemetry, memory footprints, and token throughput metrics into a machine-readable telemetry database (`benchmarks/`).

```
                 AURA EXPERIMENT LAB
                         │
             ┌───────────┼───────────┐
             │           │           │
             ↓           ↓           ↓
        Local PC      Ollama       GPU Cloud
             │         Cloud          │
             │           │            │
       7B/8B models   GLM-5.2       GLM-5.2
       14B/etc.       Kimi K3       Kimi K3
             │           │            │
             └───────────┼────────────┘
                         ↓
                  Jupyter Notebook
                         ↓
                Unified Benchmark
                         ↓
                AURA Benchmark DB
```

---

## 2. Model Classification Protocol: Cloud vs Local

To maintain academic rigor and product credibility, AURA strictly classifies evaluation targets:

| Classification | Deployment Target | Models Tested | Primary Objective |
|---|---|---|---|
| **Local Consumer CPU Target** | Intel Core i5-13420H + 16GB RAM + NVMe SSD | `qwen2.5-coder:7b`, `llama3:8b-instruct-q4_0`, `mistral:latest`, `deepseek-r1:latest`, `gemma4:latest` | Measure actual low-resource CPU inference, RAM peak RSS, page faults, storage mmap bandwidth. |
| **Cloud Reference Baseline** | Remote Cloud Endpoints (`ollama run MODEL:cloud`) | `glm-5.2:cloud` (744B), `kimi-k3:cloud` (2.81T) | Reference capability, reasoning, TTFT, and output comparison. **(NOT LOCAL INFERENCE)** |
| **GPU / Accelerator Research Target** | Dedicated GPU Cloud / Sharded Cluster | Frontier open weights (safetensors shards) | Tensor paging, expert routing sparsity, prefetch queue design. |

---

## 3. Experiment Lab Notebook Index (`experiments/`)

1. **`00_environment.ipynb`**: Hardware Telemetry & Environment Diagnostics (`aura doctor`).
2. **`01_ollama_cloud_baseline.ipynb`**: Cloud Reference Baseline Evaluation (`glm-5.2:cloud`, `kimi-k3:cloud`).
3. **`02_local_models.ipynb`**: Local Model Manifest Inspection (`qwen2.5-coder:7b`, `llama3:8b-instruct-q4_0`, `mistral:latest`, `deepseek-r1:latest`, `gemma4:latest`).
4. **`03_aura_vs_ollama.ipynb`**: Side-by-Side Comparison Matrix (Ollama Default vs AURA Planned).
5. **`04_memory_pressure.ipynb`**: Page Fault Telemetry & Working Set Pressure Analysis.
6. **`05_2gb_experiment.ipynb`**: Mandatory 2 GB RAM Budget Constraint Experiment & Physics Limits.
7. **`06_large_model_analysis.ipynb`**: Large Model Scaling (9B–27B) & Storage Bandwidth Floor Math.
8. **`07_glm52_cloud.ipynb`**: GLM-5.2 (744B) Cloud Reference Evaluation & Tensor Paging Research.
9. **`08_kimik3_cloud.ipynb`**: Kimi K3 (2.81T / 16 of 896 active experts) Cloud Reference & Sparse Routing Study.
10. **`09_gpu_experiment.ipynb`**: Accelerator Offloading Strategy & Layer Split Architecture (`-ngl`).
11. **`10_final_analysis.ipynb`**: Final Synthesis & Telemetry DB Exporter.

---

## 4. Evolved AURA 2.0 System Architecture

```
                 AURA
                  │
        ┌─────────┴─────────┐
        ↓                   ↓
    Local Models        Remote Models
        │                   │
        ↓                   ↓
  Hardware Planner     Cloud Baseline
        │                   │
        └─────────┬─────────┘
                  ↓
             Model Profiler
                  ↓
          Execution Planner
                  ↓
       ┌──────────┼──────────┐
       ↓          ↓          ↓
      RAM        SSD       VRAM
       │          │          │
       └──────────┼──────────┘
                  ↓
          Memory Scheduler
                  ↓
       Expert / Tensor Cache
                  ↓
             Prefetcher
                  ↓
              Runtime
```
