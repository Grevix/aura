# AirLLM Test Reproduction & Execution Audit Report

**Date:** 23 August 2026  
**Audited Target:** `research/airllm/air_llm/tests`  
**Execution Command:** `pytest research/airllm/air_llm/tests -v`  

---

## 1. PyTest Execution Output & Dependency Matrix

```
=========================== short test summary info ===========================
ERROR research/airllm/air_llm/tests/test_automodel.py - ModuleNotFoundError: No module named 'torch'
ERROR research/airllm/air_llm/tests/test_compression.py - ModuleNotFoundError: No module named 'torch'
ERROR research/airllm/air_llm/tests/test_kimi_k3_split.py - ModuleNotFoundError: No module named 'torch'
ERROR research/airllm/air_llm/tests/test_qwen3_8_split.py - ModuleNotFoundError: No module named 'torch'
ERROR research/airllm/air_llm/tests/test_streaming_gpu.py - ModuleNotFoundError: No module named 'torch'
```

---

## 2. Forensic Test Classification Table

| Test Name | File Location | Intended Target | Result on Test Host | Reason for Result |
|---|---|---|---|---|
| `test_automodel.py` | `air_llm/tests/test_automodel.py` | AutoModel HuggingFace loader | **UNAVAILABLE** | Requires PyTorch (`torch`) and CUDA GPU runtime |
| `test_compression.py` | `air_llm/tests/test_compression.py` | 4-bit bitsandbytes block quant | **UNAVAILABLE** | Requires PyTorch + `bitsandbytes` CUDA library |
| `test_kimi_k3_split.py` | `air_llm/tests/test_kimi_k3_split.py` | Kimi K3 MoE per-expert layer split | **UNAVAILABLE** | Requires PyTorch + HuggingFace transformers |
| `test_qwen3_8_split.py` | `air_llm/tests/test_qwen3_8_split.py` | Qwen3 layer sharding split | **UNAVAILABLE** | Requires PyTorch + HuggingFace transformers |
| `test_streaming_gpu.py` | `air_llm/tests/test_streaming_gpu.py` | CUDA GPU stream prefetch loop | **UNAVAILABLE** | Requires CUDA GPU hardware (`device = "cuda:0"`) |

---

## 3. Critical Scientific Takeaway
AirLLM is **not a standalone C/C++/Rust binary runtime**. It is a PyTorch wrapper library (`AirLLMBaseModel`) requiring PyTorch CUDA infrastructure. It cannot execute natively on GPU-less or PyTorch-less environments. In contrast, AURA is a native Rust/C++ executable runtime (`aura.exe`) requiring zero PyTorch dependencies.
