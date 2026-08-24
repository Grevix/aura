import subprocess
import json
import time
import os
import sys

PROMPTS = [
    {
        "id": "P01_Rust_Deadlock",
        "category": "General Reasoning",
        "prompt": "Explain why a Rust program using Arc<Mutex<T>> can deadlock and provide a safe alternative."
    },
    {
        "id": "P02_Python_CSV",
        "category": "Coding",
        "prompt": "Write a Python function that detects duplicate rows in a CSV and explain its time complexity."
    },
    {
        "id": "P03_Python_Debug",
        "category": "Debugging",
        "prompt": "Debug this Python code snippet: `def avg(nums): return sum(nums)/len(nums)` when nums can be an empty list."
    },
    {
        "id": "P04_Math_Stats",
        "category": "Mathematical Reasoning",
        "prompt": "Given dataset [4, 8, 15, 16, 23, 42], calculate the mean and median step by step."
    },
    {
        "id": "P05_JSON_Extract",
        "category": "Structured JSON",
        "prompt": "Extract details from text 'Alice is a 29 year old Software Engineer living in Tokyo' strictly as JSON with keys name, age, role, city."
    },
    {
        "id": "P06_VMem_Vs_RSS",
        "category": "Technical Explanation",
        "prompt": "Explain the difference between virtual memory commit limit and physical resident set size (RSS)."
    },
    {
        "id": "P07_Local_Vs_Cloud",
        "category": "Instruction Following",
        "prompt": "Give three advantages and three disadvantages of running local LLM inference over cloud APIs."
    },
    {
        "id": "P08_Memory_Math",
        "category": "Multi-Step Reasoning",
        "prompt": "If process A uses 1.2 GB RAM and process B uses 800 MB RAM on a 4 GB system, calculate the exact remaining free RAM."
    },
    {
        "id": "P09_Quantum_Intro",
        "category": "Short Conversational",
        "prompt": "Explain quantum computing in three sentences."
    },
    {
        "id": "P10_SQL_Query",
        "category": "Data Extraction",
        "prompt": "Write an SQL query to find the top 5 customers by total order amount from tables 'customers' and 'orders'."
    }
]

MODELS_TO_TEST = [
    "qwen3:0.6b",
    "llama3.2:1b",
    "qwen3:1.7b",
    "llama3.2:3b"
]

def run_test():
    aura_exe = os.path.join(".", "target", "release", "aura.exe")
    if not os.path.exists(aura_exe):
        aura_exe = "aura"

    results = []

    print("==================================================")
    print("AURA Real-World Multi-Model Multi-Prompt Test Suite")
    print("==================================================")

    for model in MODELS_TO_TEST:
        print(f"\n--- Testing Model: {model} ---")
        for item in PROMPTS:
            pid = item["id"]
            prompt = item["prompt"]
            cat = item["category"]

            print(f"[{model} | {pid}] ({cat}) -> Running...")
            start_t = time.time()
            
            cmd = [aura_exe, "run", "--model", model, "--memory", "4G", "--prompt", prompt]
            
            try:
                proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, timeout=90)
                elapsed = time.time() - start_t
                out = proc.stdout
                err = proc.stderr
                
                output_visible = "=== GENERATION OUTPUT ===" in out
                simulated_false = "Simulated      : false" in out
                backend_llama = "Backend        : llama-server" in out
                provenance_measured = "Provenance     : aura_measured" in out

                # Parse metrics from output
                ttft = None
                decode_speed = None
                peak_rss = None
                
                for line in out.splitlines():
                    if "TTFT Latency   :" in line:
                        ttft = line.split(":")[-1].strip()
                    elif "Decode Speed   :" in line:
                        decode_speed = line.split(":")[-1].strip()
                    elif "Peak RSS       :" in line:
                        peak_rss = line.split(":")[-1].strip()

                status = "SUCCESS" if proc.returncode == 0 and output_visible and simulated_false else "FAILED"
                
                res = {
                    "model": model,
                    "prompt_id": pid,
                    "category": cat,
                    "status": status,
                    "output_received": output_visible,
                    "is_simulated": not simulated_false,
                    "backend": "llama-server" if backend_llama else "unknown",
                    "provenance": "aura_measured" if provenance_measured else "other",
                    "ttft": ttft,
                    "decode_speed": decode_speed,
                    "peak_rss": peak_rss,
                    "elapsed_sec": round(elapsed, 2)
                }
                
                results.append(res)
                print(f"  Result: {status} | TTFT: {ttft} | Speed: {decode_speed} | RSS: {peak_rss} | Elapsed: {elapsed:.2f}s")
                
            except subprocess.TimeoutExpired:
                print(f"  Result: TIMEOUT after 90s")
                results.append({
                    "model": model,
                    "prompt_id": pid,
                    "category": cat,
                    "status": "TIMEOUT",
                    "output_received": False,
                    "is_simulated": True,
                    "elapsed_sec": 90.0
                })

    with open("benchmarks/real_world_prompt_results.json", "w") as f:
        json.dump(results, f, indent=2)

    print("\n[SUCCESS] Real-World Test Suite Complete. Results saved to benchmarks/real_world_prompt_results.json")

if __name__ == "__main__":
    run_test()
