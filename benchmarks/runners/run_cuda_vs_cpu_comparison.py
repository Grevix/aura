import os
import sys
import json
import time
import subprocess
import urllib.request
import pandas as pd

def fetch_installed_models():
    try:
        req = urllib.request.urlopen("http://localhost:11434/api/tags")
        data = json.loads(req.read().decode('utf-8'))
        models = [m.get("name") for m in data.get("models", []) if ":cloud" not in m.get("name", "")]
        return models
    except Exception as e:
        print(f"Error fetching Ollama models: {e}")
        return []

PROMPTS = [
    "Explain quantum computing in three sentences.",
    "Write a Python function to find duplicate rows in a CSV file.",
    "Calculate the mean and median of [4, 8, 15, 16, 23, 42].",
    "Explain the difference between physical RSS and virtual commit memory.",
    "Give three advantages of running local LLM inference over cloud APIs."
]

MODELS_TO_TEST = ["qwen3:0.6b", "llama3.2:1b", "qwen3:1.7b", "llama3.2:3b"]

def query_ollama_rest(model, prompt, num_gpu):
    payload = {
        "model": model,
        "prompt": prompt,
        "stream": False,
        "options": {
            "num_gpu": num_gpu,
            "num_thread": 8
        }
    }
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request("http://localhost:11434/api/generate", data=data, headers={"Content-Type": "application/json"})
    
    start_t = time.time()
    with urllib.request.urlopen(req) as resp:
        body = json.loads(resp.read().decode("utf-8"))
        elapsed = time.time() - start_t
        
        eval_count = body.get("eval_count", 0)
        eval_dur_ns = body.get("eval_duration", 1)
        decode_tok_s = eval_count / (eval_dur_ns / 1e9) if eval_dur_ns > 0 else 0.0
        
        prompt_eval_count = body.get("prompt_eval_count", 0)
        prompt_eval_dur_ns = body.get("prompt_eval_duration", 1)
        prefill_tok_s = prompt_eval_count / (prompt_eval_dur_ns / 1e9) if prompt_eval_dur_ns > 0 else 0.0
        
        load_dur_ns = body.get("load_duration", 0)
        ttft_ms = (load_dur_ns + prompt_eval_dur_ns) / 1e6
        
        return {
            "decode_tok_s": round(decode_tok_s, 2),
            "prefill_tok_s": round(prefill_tok_s, 2),
            "ttft_ms": round(ttft_ms, 2),
            "eval_count": eval_count,
            "elapsed_sec": round(elapsed, 2)
        }

def run_comparison():
    print("==================================================")
    print("AURA CUDA vs CPU Performance Comparison Suite")
    print("==================================================")
    
    results = []
    
    for model in MODELS_TO_TEST:
        print(f"\n--- Model: {model} ---")
        for idx, prompt in enumerate(PROMPTS):
            # 1. Ollama CPU baseline (num_gpu = 0)
            res_cpu = query_ollama_rest(model, prompt, num_gpu=0)
            print(f"[{model} | P{idx+1:02d} | Ollama CPU ] -> Decode: {res_cpu['decode_tok_s']} tok/s | TTFT: {res_cpu['ttft_ms']} ms")
            
            # 2. Ollama CUDA offload (num_gpu = 99)
            res_cuda = query_ollama_rest(model, prompt, num_gpu=99)
            print(f"[{model} | P{idx+1:02d} | Ollama CUDA] -> Decode: {res_cuda['decode_tok_s']} tok/s | TTFT: {res_cuda['ttft_ms']} ms")
            
            speedup = res_cuda['decode_tok_s'] / res_cpu['decode_tok_s'] if res_cpu['decode_tok_s'] > 0 else 1.0
            print(f"  CUDA Speedup: {speedup:.2f}x")
            
            results.append({
                "model": model,
                "prompt_id": f"P{idx+1:02d}",
                "cpu_decode_tok_s": res_cpu['decode_tok_s'],
                "cuda_decode_tok_s": res_cuda['decode_tok_s'],
                "cpu_ttft_ms": res_cpu['ttft_ms'],
                "cuda_ttft_ms": res_cuda['ttft_ms'],
                "cuda_speedup": round(speedup, 2)
            })

    with open("benchmarks/reports/cuda_vs_cpu_comparison.json", "w") as f:
        json.dump(results, f, indent=2)

    df = pd.DataFrame(results)
    df.to_csv("benchmarks/reports/cuda_vs_cpu_comparison.csv", index=False)
    print("\n✅ CUDA vs CPU Comparison Complete. Saved to benchmarks/reports/")

if __name__ == "__main__":
    run_comparison()
