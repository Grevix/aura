import os
import sys
import json
import time
import subprocess
import pandas as pd

def collect_gpu_telemetry():
    """Query nvidia-smi for current GPU utilization, VRAM usage, and driver info."""
    try:
        cmd = [
            "nvidia-smi",
            "--query-gpu=name,memory.total,memory.used,memory.free,utilization.gpu,driver_version",
            "--format=csv,noheader,nounits"
        ]
        res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if res.returncode == 0:
            line = res.stdout.strip().split("\n")[0]
            parts = [p.strip() for p in line.split(",")]
            return {
                "gpu_name": parts[0],
                "vram_total_mb": int(parts[1]),
                "vram_used_mb": int(parts[2]),
                "vram_free_mb": int(parts[3]),
                "gpu_util_pct": float(parts[4]),
                "driver_version": parts[5]
            }
    except Exception as e:
        pass
    return {"gpu_name": "CPU-only / None", "vram_total_mb": 0, "vram_used_mb": 0, "vram_free_mb": 0, "gpu_util_pct": 0.0, "driver_version": "N/A"}

def run_gpu_suite():
    aura_bin = "./target/release/aura"
    if not os.path.exists(aura_bin) and os.path.exists("./target/release/aura.exe"):
        aura_bin = "./target/release/aura.exe"

    telemetry_before = collect_gpu_telemetry()
    print("==================================================")
    print("AURA GPU & CUDA Performance Suite")
    print("==================================================")
    print(f"Detected GPU: {telemetry_before['gpu_name']} | Total VRAM: {telemetry_before['vram_total_mb']} MB")

    prompts = [
        "Explain GPU matrix multiplication speedups over CPU SIMD.",
        "Write a CUDA kernel for vector addition in C++.",
        "Explain memory bandwidth limits in LLM decode phases."
    ]

    results = []
    for idx, prompt in enumerate(prompts):
        print(f"\n[GPU Benchmark Test P{idx+1:02d}] -> Running...")
        start_t = time.time()
        cmd = [aura_bin, "run", "--model", "llama3.2:3b", "--memory", "4G", "--prompt", prompt]
        
        proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, errors="replace")
        elapsed = time.time() - start_t
        out = proc.stdout or ""
        
        gpu_during = collect_gpu_telemetry()
        
        rec = {
            "prompt_id": f"P{idx+1:02d}",
            "prompt": prompt,
            "elapsed_sec": round(elapsed, 2),
            "output_visible": "=== GENERATION OUTPUT ===" in out,
            "is_simulated": "Simulated      : true" in out,
            "backend": "CpuLlamaCpp",
            "vram_used_mb": gpu_during["vram_used_mb"],
            "gpu_util_pct": gpu_during["gpu_util_pct"]
        }
        results.append(rec)
        print(f"  Done in {elapsed:.2f}s | Output Visible: {rec['output_visible']} | VRAM: {rec['vram_used_mb']} MB")

    with open("benchmarks/reports/gpu_benchmark_results.json", "w") as f:
        json.dump(results, f, indent=2)

    print("\n✅ GPU Suite Complete. Results saved to benchmarks/reports/gpu_benchmark_results.json")

if __name__ == "__main__":
    run_gpu_suite()
