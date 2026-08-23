# AirLLM Reproduction Plan & Forensic Target Metadata

**Audited Repository:** `https://github.com/lyogavin/airllm`  
**Target Commit:** `6ab3b9db9b2fb595e8a4d966f0e1ba600677b1fe`  
**Date:** 23 August 2026  
**Auditor Role:** Scientific Reviewer, Compiler/Runtime Engineer, & Systems Performance Auditor  

---

## 1. Environment & Target Specifications

- **Repository URL:** `https://github.com/lyogavin/airllm`
- **Git Commit:** `6ab3b9db9b2fb595e8a4d966f0e1ba600677b1fe`
- **Python Target:** Python 3.10+ / 3.14
- **Framework Dependencies:** `torch`, `transformers >= 4.36`, `accelerate`, `safetensors`, `bitsandbytes`, `compressed-tensors`
- **Primary Execution Device:** CUDA / Metal GPUs (`device = "cuda:0"`)
- **CPU Execution Support:** Limited (PyTorch CPU fallback; un-optimized for multi-threaded GEMV)
- **Quantization Support:** `bitsandbytes` (4-bit/8-bit), `compressed-tensors` (MXFP4 / FP8)
- **Prefetch Engine:** `ThreadPoolExecutor(max_workers=1)` with PyTorch `pin_memory()`
- **Layer Sharding Format:** `.safetensors` per-layer directory shards (`safetensor_model_persister.py`)

---

## 2. Test & Benchmark Reproduction Protocol

```
                  [PyTest Execution]
            (pytest air_llm/tests/ -xvs)
                         │
                         ▼
        [Reproduction Log & JSON Export]
       (benchmarks/airllm/reproduction/)
                         │
                         ▼
        [CPU Inference Benchmark Script]
            (benchmarks/airllm/run_airllm.py)
```

### Planned Reproduction Artifacts:
1. `benchmarks/airllm/reproduction/pytest_results.log`
2. `benchmarks/airllm/reproduction/airllm_cpu_inference.json`
3. `benchmarks/results/airllm_results.json`
4. `benchmarks/results/aura_results.json`
5. `benchmarks/results/comparison_results.json`
6. `benchmarks/results/ablation_results.json`
