# AURA Google Colab GPU & Benchmark Validation Suite

> Run AURA's budget-enforced inference engine, multi-model prompt matrix, and GPU telemetry collectors directly in Google Colab (Linux / T4 / V100 / A100 GPU instances).

---

## 1. Quick Start in Google Colab

1. Open `benchmarks/colab/aura_v7_v8_v9_gpu_audit.ipynb` in Google Colab.
2. Select Runtime -> Change runtime type -> **T4 GPU** or **A100 GPU**.
3. Run Step 1 cell: `!bash benchmarks/colab/setup_colab.sh`
4. Run Step 2 cell: `!python benchmarks/colab/run_gpu_benchmark.py`
5. Inspect generated CSV, JSON, and charts in `benchmarks/reports/`.

---

## 2. Included Infrastructure

- `aura_v7_v8_v9_gpu_audit.ipynb`: Full automated audit notebook.
- `setup_colab.sh`: Automated Linux dependency installer (Rust, CMake, NVIDIA CUDA toolkit).
- `run_gpu_benchmark.py`: GPU VRAM and execution performance telemetry collector.
- `collect_gpu_telemetry.py`: Live `nvidia-smi` hardware sampling.
