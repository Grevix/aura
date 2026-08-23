import os
import sys
import json
import time
import subprocess
import datetime
import statistics

PROMPTS_FILE = os.path.join(os.path.dirname(__file__), "prompts", "prompts.json")
RESULTS_DIR = os.path.join(os.path.dirname(__file__), "results")
OUTPUT_JSON = os.path.join(RESULTS_DIR, "v4_4gb_raw_results.json")
STATS_JSON = os.path.join(RESULTS_DIR, "v4_4gb_statistics.json")
AURA_EXE = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "debug", "aura.exe"))

def load_prompts():
    with open(PROMPTS_FILE, "r", encoding="utf-8") as f:
        data = json.load(f)
        return data.get("categories", {})

def run_aura_4gb(prompt, budget_str="4G"):
    start = time.time()
    try:
        cmd = [AURA_EXE, "run", "--model", "qwen2.5-coder:7b", "--prompt", prompt, "--memory", budget_str]
        res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, encoding="utf-8", errors="ignore", timeout=60)
        elapsed = time.time() - start
        return {
            "runtime": "aura",
            "success": res.returncode == 0,
            "elapsed_sec": elapsed,
            "stdout_len": len(res.stdout),
            "stderr": res.stderr
        }
    except Exception as e:
        return {
            "runtime": "aura",
            "success": False,
            "elapsed_sec": time.time() - start,
            "error": str(e)
        }

def run_ollama_4gb(prompt):
    start = time.time()
    try:
        cmd = ["ollama", "run", "qwen2.5-coder:7b", prompt]
        res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, encoding="utf-8", errors="ignore", timeout=60)
        elapsed = time.time() - start
        return {
            "runtime": "ollama",
            "success": res.returncode == 0,
            "elapsed_sec": elapsed,
            "stdout_len": len(res.stdout),
            "stderr": res.stderr
        }
    except Exception as e:
        return {
            "runtime": "ollama",
            "success": False,
            "elapsed_sec": time.time() - start,
            "error": str(e)
        }

def main():
    os.makedirs(RESULTS_DIR, exist_ok=True)
    categories = load_prompts()
    
    results = {
        "timestamp_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "memory_budget_gb": 4.0,
        "runs": {}
    }

    print("[AURA V4] Launching Adversarial 4GB CPU-Only Battle Suite...")
    
    for cat_name, prompts in categories.items():
        print(f"Executing category {cat_name} ({len(prompts)} prompts)...")
        cat_results = []
        for idx, p in enumerate(prompts):
            print(f"  Prompt {idx+1}/{len(prompts)}: {p[:40]}...")
            
            # Alternating execution order for statistical fairness
            if idx % 2 == 0:
                aura_res = run_aura_4gb(p, budget_str="4G")
                ollama_res = run_ollama_4gb(p)
            else:
                ollama_res = run_ollama_4gb(p)
                aura_res = run_aura_4gb(p, budget_str="4G")
                
            cat_results.append({
                "prompt": p,
                "aura": aura_res,
                "ollama": ollama_res
            })
        results["runs"][cat_name] = cat_results

    with open(OUTPUT_JSON, "w", encoding="utf-8") as f:
        json.dump(results, f, indent=2)
        
    print(f"[AURA V4] Saved raw 4GB results to: {OUTPUT_JSON}")

    # Compute statistical metrics
    aura_latencies = []
    ollama_latencies = []
    
    for cat_name, runs in results["runs"].items():
        for item in runs:
            if item["aura"]["success"]:
                aura_latencies.append(item["aura"]["elapsed_sec"])
            if item["ollama"]["success"]:
                ollama_latencies.append(item["ollama"]["elapsed_sec"])

    stats = {
        "aura": {
            "sample_count": len(aura_latencies),
            "median_sec": statistics.median(aura_latencies) if aura_latencies else None,
            "mean_sec": statistics.mean(aura_latencies) if aura_latencies else None,
            "stdev_sec": statistics.stdev(aura_latencies) if len(aura_latencies) > 1 else None
        },
        "ollama": {
            "sample_count": len(ollama_latencies),
            "median_sec": statistics.median(ollama_latencies) if ollama_latencies else None,
            "mean_sec": statistics.mean(ollama_latencies) if ollama_latencies else None,
            "stdev_sec": statistics.stdev(ollama_latencies) if len(ollama_latencies) > 1 else None
        }
    }

    with open(STATS_JSON, "w", encoding="utf-8") as f:
        json.dump(stats, f, indent=2)
        
    print(f"[AURA V4] Saved statistical analysis to: {STATS_JSON}")

if __name__ == "__main__":
    main()
