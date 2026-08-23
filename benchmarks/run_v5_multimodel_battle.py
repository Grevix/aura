import os
import sys
import json
import time
import subprocess
import datetime
import statistics

PROMPTS_FILE = os.path.join(os.path.dirname(__file__), "prompts", "prompts.json")
RESULTS_DIR = os.path.join(os.path.dirname(__file__), "results")
OUTPUT_JSON = os.path.join(RESULTS_DIR, "v5_multi_model_results.json")
RAW_JSON = os.path.join(RESULTS_DIR, "v5_aura_vs_ollama_raw.json")
AURA_EXE = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "target", "debug", "aura.exe"))

MODELS_TO_TEST = [
    {"tag": "qwen2.5-coder:7b", "params": "7.6B", "tier": "C_4GB_BOUNDARY"},
    {"tag": "llama3:8b-instruct-q4_0", "params": "8.0B", "tier": "D_GENERAL_PURPOSE"}
]

def load_prompts():
    with open(PROMPTS_FILE, "r", encoding="utf-8") as f:
        data = json.load(f)
        return data.get("categories", {})

def run_aura(model_tag, prompt, budget_str="4G"):
    start = time.time()
    try:
        cmd = [AURA_EXE, "run", "--model", model_tag, "--prompt", prompt, "--memory", budget_str]
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

def run_ollama(model_tag, prompt):
    start = time.time()
    try:
        cmd = ["ollama", "run", model_tag, prompt]
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
        "models_evaluated": MODELS_TO_TEST,
        "battle_runs": {}
    }

    print("[AURA V5] Launching Multi-Model 4GB CPU-Only Adversarial Campaign...")
    
    for m_info in MODELS_TO_TEST:
        m_tag = m_info["tag"]
        print(f"\n==========================================")
        print(f" EVALUATING MODEL: {m_tag} ({m_info['params']})")
        print(f"==========================================")
        
        m_results = {}
        for cat_name, prompts in categories.items():
            print(f"  Category {cat_name} ({len(prompts)} prompts)...")
            cat_runs = []
            for idx, p in enumerate(prompts):
                print(f"    Prompt {idx+1}/{len(prompts)}: {p[:35]}...")
                
                # Alternating execution order for fairness
                if idx % 2 == 0:
                    aura_res = run_aura(m_tag, p, budget_str="4G")
                    ollama_res = run_ollama(m_tag, p)
                else:
                    ollama_res = run_ollama(m_tag, p)
                    aura_res = run_aura(m_tag, p, budget_str="4G")
                    
                cat_runs.append({
                    "prompt": p,
                    "aura": aura_res,
                    "ollama": ollama_res
                })
            m_results[cat_name] = cat_runs
        results["battle_runs"][m_tag] = m_results

    with open(OUTPUT_JSON, "w", encoding="utf-8") as f:
        json.dump(results, f, indent=2)
        
    with open(RAW_JSON, "w", encoding="utf-8") as f:
        json.dump(results, f, indent=2)
        
    print(f"\n[AURA V5] Saved multi-model results to: {OUTPUT_JSON}")

if __name__ == "__main__":
    main()
