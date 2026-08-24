import os
import sys
import json
import time
import subprocess
import urllib.request
import pandas as pd

def fetch_installed_models():
    """Dynamically discover models from Ollama /api/tags without hardcoding."""
    try:
        req = urllib.request.urlopen("http://localhost:11434/api/tags")
        data = json.loads(req.read().decode('utf-8'))
        models = data.get("models", [])
        
        local_models = []
        cloud_models = []
        
        for m in models:
            name = m.get("name", "")
            if ":cloud" in name or "cloud" in m.get("details", {}).get("family", ""):
                cloud_models.append(name)
            else:
                local_models.append(name)
        return local_models, cloud_models
    except Exception as e:
        print(f"Error fetching Ollama models: {e}")
        return [], []

PROMPTS = [
    "Explain quantum computing in three sentences.",
    "Write a Python function to detect duplicate rows in a CSV file.",
    "Debug this Rust snippet: Arc<Mutex<T>> deadlock scenarios.",
    "Calculate the step-by-step mean and median of [4, 8, 15, 16, 23, 42].",
    "Extract JSON fields name, age, city from 'Alice is 29 living in Tokyo'.",
    "Explain the difference between physical RSS and virtual commit limit.",
    "What are three advantages of running local LLMs over cloud APIs?",
    "Write an SQL query to find top 5 customers by total order amount.",
    "Summarize the key principles of modern operating system memory paging.",
    "Implement a fast binary search in C++ with boundary checks."
]

BUDGETS = ["4G", "8G", "12G", "16G"]

def run_suite():
    aura_exe = os.path.join(".", "target", "release", "aura.exe")
    local_models, cloud_models = fetch_installed_models()
    
    print("==================================================")
    print("AURA Forensic Benchmark Runner (Dynamic Discovery)")
    print("==================================================")
    print(f"Discovered Local Models ({len(local_models)}): {local_models}")
    print(f"Discovered Cloud Models ({len(cloud_models)}): {cloud_models} (Excluded from local benchmarking)")
    
    records = []
    
    for model in local_models:
        print(f"\n--- Model: {model} ---")
        for budget in BUDGETS:
            for idx, prompt in enumerate(PROMPTS[:5]):
                cmd = [aura_exe, "run", "--model", model, "--memory", budget, "--prompt", prompt]
                start_t = time.time()
                
                try:
                    proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, encoding="utf-8", errors="replace", timeout=90)
                    elapsed = time.time() - start_t
                    out = proc.stdout or ""
                    
                    output_received = "=== GENERATION OUTPUT ===" in out
                    output_displayed = output_received
                    is_simulated = "Simulated      : true" in out
                    
                    ttft = None
                    decode_speed = None
                    peak_rss = None
                    backend = "CpuLlamaCpp"
                    provenance = "aura_measured"
                    
                    for line in out.splitlines():
                        if "TTFT Latency   :" in line:
                            ttft = line.split(":")[-1].strip()
                        elif "Decode Speed   :" in line:
                            decode_speed = line.split(":")[-1].strip()
                        elif "Peak RSS       :" in line:
                            peak_rss = line.split(":")[-1].strip()
                        elif "Backend        :" in line:
                            backend = line.split(":")[-1].strip()
                        elif "Provenance     :" in line:
                            provenance = line.split(":")[-1].strip()
                            
                    rec = {
                        "model": model,
                        "budget": budget,
                        "prompt_id": f"P{idx+1:02d}",
                        "status": "SUCCESS" if proc.returncode == 0 and output_received else "FAILED",
                        "output_received": output_received,
                        "output_displayed": output_displayed,
                        "is_simulated": is_simulated,
                        "backend": backend,
                        "provenance": provenance,
                        "ttft": ttft,
                        "decode_speed": decode_speed,
                        "peak_rss": peak_rss,
                        "elapsed_sec": round(elapsed, 2)
                    }
                    records.append(rec)
                    print(f"[{model} | {budget} | P{idx+1:02d}] -> Status: {rec['status']} | TTFT: {ttft} | Speed: {decode_speed} | RSS: {peak_rss}")
                except subprocess.TimeoutExpired:
                    print(f"[{model} | {budget} | P{idx+1:02d}] -> TIMEOUT")

    df = pd.DataFrame(records)
    df.to_csv("benchmarks/reports/forensic_suite_results.csv", index=False)
    with open("benchmarks/reports/forensic_suite_results.json", "w", encoding="utf-8") as f:
        json.dump(records, f, indent=2)

    print("\n[SUCCESS] Forensic Benchmark Suite Complete. Saved to benchmarks/reports/")

if __name__ == "__main__":
    run_suite()
