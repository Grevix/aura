#!/usr/bin/env python3
"""
AURA V3 Real User Inference Battle & Cross-Runtime Benchmark Suite
Executes head-to-head comparison between Ollama and AURA across 25 real user prompts.
Generates machine-readable JSON artifacts in benchmarks/results/.
"""

import json
import os
import sys
import time
import subprocess
from datetime import datetime, timezone

RESULTS_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "results"))
PROMPTS_FILE = os.path.abspath(os.path.join(os.path.dirname(__file__), "prompts", "prompts.json"))
AURA_EXE = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "debug", "aura.exe"))

os.makedirs(RESULTS_DIR, exist_ok=True)

with open(PROMPTS_FILE, "r", encoding="utf-8") as f:
    PROMPTS_DATA = json.load(f)["categories"]

def run_ollama(model="qwen2.5-coder:7b", prompt="Hello"):
    cmd = ["ollama", "run", model, prompt]
    start_time = time.time()
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, errors="replace", timeout=60)
        elapsed = time.time() - start_time
        return {
            "runtime": "ollama",
            "success": proc.returncode == 0,
            "elapsed_sec": elapsed,
            "output_length": len(proc.stdout)
        }
    except Exception as e:
        return {
            "runtime": "ollama",
            "success": False,
            "elapsed_sec": time.time() - start_time,
            "error": str(e)
        }

def run_aura(model="qwen2.5-coder:7b", budget="4G", prompt="Hello"):
    cmd = [AURA_EXE, "run", "-m", model, "-b", budget, "-p", prompt]
    start_time = time.time()
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, errors="replace", timeout=60)
        elapsed = time.time() - start_time
        return {
            "runtime": "aura",
            "success": proc.returncode == 0,
            "elapsed_sec": elapsed,
            "output_length": len(proc.stdout)
        }
    except Exception as e:
        return {
            "runtime": "aura",
            "success": False,
            "elapsed_sec": time.time() - start_time,
            "error": str(e)
        }

def main():
    print("[AURA V3] Launching Cross-Runtime Inference Battle...")
    results = {
        "timestamp_utc": datetime.now(timezone.utc).isoformat(),
        "system": {
            "os": "windows_11_x86_64",
            "cpu": "13th Gen Intel(R) Core(TM) i5-13420H",
            "ram_gb": 16.79
        },
        "battle_runs": {}
    }

    for cat_name, prompts in PROMPTS_DATA.items():
        print(f"Executing category {cat_name} ({len(prompts)} prompts)...")
        cat_results = []
        for idx, p in enumerate(prompts):
            print(f"  Prompt {idx+1}/{len(prompts)}...")
            # Alternating execution order to eliminate order bias
            if idx % 2 == 0:
                res_aura = run_aura(prompt=p)
                res_ollama = run_ollama(prompt=p)
            else:
                res_ollama = run_ollama(prompt=p)
                res_aura = run_aura(prompt=p)
            
            cat_results.append({
                "prompt": p,
                "aura": res_aura,
                "ollama": res_ollama
            })
        results["battle_runs"][cat_name] = cat_results

    # Save output JSON artifacts
    output_path = os.path.join(RESULTS_DIR, "comparison_results.json")
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(results, f, indent=2)

    # Save aura_results.json
    aura_path = os.path.join(RESULTS_DIR, "aura_results.json")
    with open(aura_path, "w", encoding="utf-8") as f:
        json.dump({"aura_execution": results}, f, indent=2)

    print(f"[AURA V3] Inference Battle Complete. Saved JSON artifacts to: {output_path}")

if __name__ == "__main__":
    main()
