# AURA NVIDIA GPU Hardware & CUDA Validation Report

---

## 1. Hardware Detection Results (`aura gpu-doctor`)

```text
🔍 Running AURA GPU Hardware & Backend Doctor...

GPU Detection
──────────────────────────────────────────────────
NVIDIA GPU        : NVIDIA GeForce RTX 4050 Laptop GPU
CUDA Backend      : NVIDIA CUDA (Driver: 592.82)
VRAM              : 6.00 GB (6141 MiB)
AURA CUDA Backend : READY
```

---

## 2. Technical Investigation & Root Cause Fix

- **Problem**: Previous hardware probes returned `GPU Present: false` because `detect_gpu()` in `aura-hardware/src/gpu.rs` contained an early fallback returning hardcoded `false`.
- **Fix**: Replaced fallback with direct `nvidia-smi` CSV queries and Windows WMI queries.
- **Verification**: `nvidia-smi` correctly detected the NVIDIA GeForce RTX 4050 Laptop GPU with 6141 MiB VRAM and CUDA 13.1 driver.

---

## 3. Bundled Ollama CUDA Libraries

Discovered bundled CUDA libraries in Ollama installation (`C:\Users\Aaryan Rawat\AppData\Local\Programs\Ollama\lib\ollama`):
- `cuda_v13`: `ggml-cuda.dll` (129 MB), `cublas64_13.dll` (50 MB), `cublasLt64_13.dll` (477 MB).
- `cuda_v12`: `ggml-cuda.dll` (344 MB), `cublas64_12.dll` (113 MB), `cublasLt64_12.dll` (692 MB).

When `llama-server.exe` runs with CUDA paths prepended to `PATH`, `ggml-cuda.dll` offloads model layers to the RTX 4050 GPU VRAM.
