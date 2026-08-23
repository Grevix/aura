#!/usr/bin/env python3
"""
AURA Automated Benchmark & Telemetry Test Harness (Phase 5 & 6)
Executes real-world workload classes and exports machine-readable JSON artifacts to benchmarks/results/.
"""

import json
import os
import sys
import time
import subprocess
from datetime import datetime, timezone

RESULTS_DIR = os.path.join(os.path.dirname(__file__), "results")
os.makedirs(RESULTS_DIR, exist_ok=True)

AURA_EXE = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "debug", "aura.exe"))

WORKLOAD_CLASSES = {
    "WORKLOAD_A_SHORT_CHAT": [
        "What is Python?",
        "Explain recursion simply.",
        "Write a C program for a basic stack."
    ],
    "WORKLOAD_B_CODING": [
        "Write a Python script that calculates prime numbers.",
        "Write a C++ function for binary search.",
        "Generate a SQL query to select top 10 users by spend."
    ],
    "WORKLOAD_C_LONG_CONTEXT": [
        "Summarize the architectural differences between dense LLMs and sparse MoE models in 200 words."
    ],
    "WORKLOAD_D_MULTI_TURN": [
        "Turn 1: Explain sorting algorithms.",
        "Turn 2: Compare QuickSort vs MergeSort."
    ],
    "WORKLOAD_E_REPETITIVE": [
        "Generate 100 random words."
    ]
}

def run_aura_benchmark(model="qwen2.5-coder:7b", memory_budget="4G", prompt="Hello", context=1024):
    cmd = [
        AURA_EXE, "run",
        "-m", model,
        "-b", memory_budget,
        "-p", prompt
    ]
    start_time = time.time()
    try:
        proc = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            errors="replace",
            cwd=os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
        )
        elapsed = time.time() - start_time
        success = proc.returncode == 0
        output = proc.stdout + proc.stderr
    except Exception as e:
        elapsed = time.time() - start_time
        success = False
        output = str(e)
    
    return {
        "success": success,
        "elapsed_sec": elapsed,
        "output_snippet": output[-500:] if len(output) > 500 else output
    }

def main():
    print("[AURA] Starting Automated Benchmark Suite...")
    benchmark_data = {
        "timestamp_utc": datetime.now(timezone.utc).isoformat(),
        "system": {
            "os": "windows_11_x86_64",
            "cpu": "13th Gen Intel(R) Core(TM) i5-13420H",
            "cores": "8 physical / 12 logical",
            "ram_gb": 16.79
        },
        "workload_results": {}
    }

    for name, prompts in WORKLOAD_CLASSES.items():
        print(f"Executing {name} ({len(prompts)} prompts)...")
        results = []
        for p in prompts:
            res = run_aura_benchmark(prompt=p)
            results.append(res)
        benchmark_data["workload_results"][name] = results

    output_file = os.path.join(RESULTS_DIR, "aura_benchmark_run_latest.json")
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(benchmark_data, f, indent=2)

    print(f"[AURA] Benchmark run completed. Artifacts saved to: {output_file}")

if __name__ == "__main__":
    main()
