import subprocess
import json
import os
import time

BUDGETS = ["2G", "4G", "8G", "12G", "16G"]
MODELS = ["qwen3:0.6b", "qwen3:1.7b", "llama3.2:3b", "qwen3:4b"]

def run_memory_sweep():
    aura_exe = os.path.join(".", "target", "release", "aura.exe")
    sweep_results = []

    print("==================================================")
    print("AURA Multi-Tier Memory Budget Sweep Campaign")
    print("==================================================")

    for m in MODELS:
        for b in BUDGETS:
            print(f"[{m} @ Budget {b}] -> Planning & Executing...")
            
            plan_cmd = [aura_exe, "plan", "--model", m, "--memory", b]
            plan_proc = subprocess.run(plan_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, encoding="utf-8", errors="replace")
            plan_out = plan_proc.stdout or ""
            
            is_feasible = "INFEASIBLE" not in plan_out
            
            details = ""
            for line in plan_out.splitlines():
                if "Details            :" in line or "Feasibility Status :" in line:
                    details += line.strip() + " "

            run_success = False
            ttft = None
            decode_speed = None
            peak_rss = None
            backend = "None"
            simulated = True

            if is_feasible:
                run_cmd = [aura_exe, "run", "--model", m, "--memory", b, "--prompt", "Explain how virtual memory works."]
                try:
                    run_proc = subprocess.run(run_cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, encoding="utf-8", errors="replace", timeout=90)
                    run_out = run_proc.stdout or ""
                    
                    run_success = run_proc.returncode == 0 and "=== GENERATION OUTPUT ===" in run_out
                    simulated = "Simulated      : true" in run_out
                    
                    for line in run_out.splitlines():
                        if "TTFT Latency   :" in line:
                            ttft = line.split(":")[-1].strip()
                        elif "Decode Speed   :" in line:
                            decode_speed = line.split(":")[-1].strip()
                        elif "Peak RSS       :" in line:
                            peak_rss = line.split(":")[-1].strip()
                        elif "Backend        :" in line:
                            backend = line.split(":")[-1].strip()
                except subprocess.TimeoutExpired:
                    print("  Timeout occurred")

            entry = {
                "model": m,
                "memory_budget": b,
                "is_feasible": is_feasible,
                "plan_details": details.strip(),
                "executed": run_success,
                "simulated": simulated,
                "backend": backend,
                "ttft": ttft,
                "decode_speed": decode_speed,
                "peak_rss": peak_rss
            }
            sweep_results.append(entry)
            print(f"  Plan Feasible: {is_feasible} | Executed: {run_success} | Peak RSS: {peak_rss}")

    with open("benchmarks/audit/v7_v8_v9_final/memory_sweep_results.json", "w", encoding="utf-8") as f:
        json.dump(sweep_results, f, indent=2)

    print("\n[SUCCESS] Memory Sweep Campaign Complete. Saved to benchmarks/audit/v7_v8_v9_final/memory_sweep_results.json")

if __name__ == "__main__":
    run_memory_sweep()
