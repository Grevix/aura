import os
import sys
import json
import time
import urllib.request
import pandas as pd

def run_70_prompt_suite():
    with open("benchmarks/prompts/aura_70_prompts.json", "r") as f:
        prompts = json.load(f)

    print("============================================================")
    print("AURA 70+ STANDARDIZED BENCHMARK SUITE EXECUTION")
    print("============================================================")
    print(f"Total Prompts Loaded: {len(prompts)}")
    
    target_model = "qwen3:8b"
    results = []
    
    start_all = time.time()
    
    for idx, item in enumerate(prompts):
        pid = item["id"]
        category = item["category"]
        prompt_text = item["prompt"]
        
        payload = {
            "model": target_model,
            "prompt": prompt_text,
            "stream": False,
            "options": {
                "num_gpu": 99,
                "num_thread": 8
            }
        }
        
        req = urllib.request.Request(
            "http://localhost:11434/api/generate",
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json"}
        )
        
        t0 = time.time()
        try:
            with urllib.request.urlopen(req, timeout=60) as resp:
                body = json.loads(resp.read().decode("utf-8"))
                wall_sec = time.time() - t0
                
                eval_count = body.get("eval_count", 0)
                eval_dur_ns = body.get("eval_duration", 1)
                decode_tok_s = eval_count / (eval_dur_ns / 1e9) if eval_dur_ns > 0 else 0.0
                
                load_ns = body.get("load_duration", 0)
                prompt_eval_ns = body.get("prompt_eval_duration", 0)
                ttft_ms = (load_ns + prompt_eval_ns) / 1e6
                
                response_text = body.get("response", "").strip()
                preview = response_text.replace("\n", " ")[:60] + "..."
                
                print(f"[{idx+1:02d}/70] [{category:<18}] {pid} -> Decode: {decode_tok_s:5.2f} tok/s | TTFT: {ttft_ms:6.1f} ms | Ans: {preview}")
                
                results.append({
                    "id": pid,
                    "category": category,
                    "prompt": prompt_text,
                    "model": target_model,
                    "backend": "Ollama CUDA (RTX 4050)",
                    "response": response_text,
                    "eval_count": eval_count,
                    "decode_tok_s": round(decode_tok_s, 2),
                    "ttft_ms": round(ttft_ms, 2),
                    "wall_sec": round(wall_sec, 2),
                    "provenance": "OllamaMeasured",
                    "status": "PASS"
                })
        except Exception as e:
            print(f"[{idx+1:02d}/70] [{category:<18}] {pid} -> FAILED: {e}")
            results.append({
                "id": pid,
                "category": category,
                "prompt": prompt_text,
                "model": target_model,
                "backend": "Ollama CUDA",
                "response": f"ERROR: {e}",
                "eval_count": 0,
                "decode_tok_s": 0.0,
                "ttft_ms": 0.0,
                "wall_sec": 0.0,
                "provenance": "OllamaMeasured",
                "status": "FAIL"
            })

    total_time = time.time() - start_all
    
    # Save artifacts in 4 formats (JSON, JSONL, CSV, Markdown)
    os.makedirs("benchmarks/results", exist_ok=True)
    timestamp_str = "2026-08-24"
    
    # 1. JSON
    with open(f"benchmarks/results/run_{timestamp_str}.json", "w") as f:
        json.dump(results, f, indent=2)
        
    # 2. JSONL
    with open(f"benchmarks/results/run_{timestamp_str}.jsonl", "w") as f:
        for r in results:
            f.write(json.dumps(r) + "\n")
            
    # 3. CSV
    df = pd.DataFrame(results)
    df.to_csv(f"benchmarks/results/run_{timestamp_str}.csv", index=False)
    
    # 4. Markdown
    with open(f"benchmarks/results/run_{timestamp_str}.md", "w") as f:
        f.write("# AURA 70-Prompt Benchmark Results\n\n")
        f.write(f"- **Target Model**: `{target_model}`\n")
        f.write(f"- **Backend**: `NVIDIA CUDA (RTX 4050 Laptop GPU)`\n")
        f.write(f"- **Total Prompts**: {len(results)}\n")
        f.write(f"- **Mean Decode Throughput**: {df['decode_tok_s'].mean():.2f} tok/s\n")
        f.write(f"- **Median Decode Throughput**: {df['decode_tok_s'].median():.2f} tok/s\n")
        f.write(f"- **Mean TTFT**: {df['ttft_ms'].mean():.2f} ms\n\n")
        f.write(df[["id", "category", "ttft_ms", "decode_tok_s", "status"]].to_markdown(index=False))

    print("\n============================================================")
    print(f"BENCHMARK COMPLETE ({total_time:.1f}s total)")
    print(f"Total: {len(results)} | Passed: {sum(1 for r in results if r['status'] == 'PASS')} | Failed: {sum(1 for r in results if r['status'] == 'FAIL')}")
    print(f"Mean Decode Speed: {df['decode_tok_s'].mean():.2f} tok/s | Median: {df['decode_tok_s'].median():.2f} tok/s")
    print(f"Mean TTFT Latency: {df['ttft_ms'].mean():.2f} ms")
    print(f"Saved results to benchmarks/results/run_{timestamp_str}.*")

if __name__ == "__main__":
    run_70_prompt_suite()
