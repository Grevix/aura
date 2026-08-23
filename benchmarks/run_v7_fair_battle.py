"""
AURA V7 — Fair Benchmark Harness
Uses Ollama REST API for honest AURA vs Ollama comparison.

Key fairness rules:
  - Ollama: POST /api/generate with num_gpu=0, num_thread=8, stream=false
  - AURA: aura.exe run --model <tag> --memory <budget> --prompt <p>
  - Timing from Ollama's own eval_count/eval_duration fields (not wall-clock estimates)
  - Memory budgets tested: 4G (primary), 6G, 8G, 16G
  - Cold (first) run + 3 warm runs per model/prompt pair
"""
import json
import os
import sys
import time
import subprocess
import urllib.request
import urllib.error
import datetime
import csv
import statistics

ROOT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
RESULTS_DIR = os.path.join(ROOT_DIR, "research", "benchmarks")
AURA_EXE = os.path.join(ROOT_DIR, "target", "debug", "aura.exe")
OLLAMA_API = "http://localhost:11434"
TIMEOUT = 120

MEMORY_BUDGETS = ["4G", "6G", "8G", "16G"]

BENCHMARK_PROMPTS = {
    "coding":       "Write a Python function that implements binary search on a sorted list. Include type hints, docstring, and a test case.",
    "debugging":    "Here is a buggy C program:\n\nint main() {\n  int arr[5] = {1,2,3,4,5};\n  for(int i=0; i<=5; i++) printf(\"%d\\n\", arr[i]);\n}\n\nFind the bug and fix it.",
    "sql":          "Write a PostgreSQL query that returns the top 5 customers by total purchase amount, including customer name, email, and total. Use a CTE.",
    "json_output":  "Return a JSON object describing a REST API endpoint for user authentication. Fields: path, method, request_body, response_body, status_codes, description.",
    "reasoning":    "I have 4 workers and 9 tasks. Each task takes 1 hour. Workers can work in parallel. What is the minimum time to complete all tasks? Show your reasoning.",
    "summarise":    "Explain the difference between a process and a thread in an operating system, including memory layout, scheduling, and communication primitives.",
    "chat":         "What are three practical ways to reduce memory usage when running large language models on consumer hardware?",
}


# ── Ollama REST API helpers ───────────────────────────────────────────────────

def ollama_generate(model: str, prompt: str) -> dict:
    """POST /api/generate with num_gpu=0, stream=false. Returns parsed JSON or error dict."""
    payload = json.dumps({
        "model": model,
        "prompt": prompt,
        "stream": False,
        "options": {"num_gpu": 0, "num_thread": 8}
    }).encode()

    req = urllib.request.Request(
        f"{OLLAMA_API}/api/generate",
        data=payload,
        headers={"Content-Type": "application/json"},
        method="POST"
    )
    t0 = time.time()
    try:
        with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
            body = json.loads(resp.read().decode())
            wall = time.time() - t0

            ec  = body.get("eval_count", 0)
            ed  = body.get("eval_duration", 0)   # nanoseconds
            pc  = body.get("prompt_eval_count", 0)
            pd_ = body.get("prompt_eval_duration", 0)
            ld  = body.get("load_duration", 0)

            def ns2s(ns): return ns / 1_000_000_000 if ns > 0 else None

            decode_tps   = (ec / ns2s(ed))   if (ec and ed)   else None
            prefill_tps  = (pc / ns2s(pd_))  if (pc and pd_)  else None
            ttft_ms      = ((ld + pd_) / 1_000_000) if (ld or pd_) else wall * 300

            return {
                "success": True, "runtime": "ollama",
                "model": model,
                "eval_count": ec, "eval_duration_ns": ed,
                "prompt_eval_count": pc, "prompt_eval_duration_ns": pd_,
                "load_duration_ns": ld,
                "decode_tok_per_sec": round(decode_tps, 3) if decode_tps else None,
                "prefill_tok_per_sec": round(prefill_tps, 3) if prefill_tps else None,
                "ttft_ms": round(ttft_ms, 1),
                "wall_sec": round(wall, 3),
                "response_len": len(body.get("response", "")),
                "note": "Real timing from Ollama eval_count/eval_duration fields"
                        if decode_tps else "wall-clock fallback (eval fields missing)",
            }
    except urllib.error.URLError as e:
        return {"success": False, "runtime": "ollama", "error": str(e), "wall_sec": time.time()-t0}
    except Exception as e:
        return {"success": False, "runtime": "ollama", "error": str(e), "wall_sec": time.time()-t0}


def aura_run(model: str, prompt: str, memory: str = "4G") -> dict:
    """Run aura.exe and measure wall-clock timing."""
    if not os.path.exists(AURA_EXE):
        return {"success": False, "runtime": "aura", "error": "aura.exe not found",
                "wall_sec": 0, "is_simulated": True}
    cmd = [AURA_EXE, "run", "--model", model, "--memory", memory, "--prompt", prompt]
    t0 = time.time()
    try:
        r = subprocess.run(cmd, capture_output=True, text=True,
                           encoding="utf-8", errors="ignore", timeout=TIMEOUT)
        wall = time.time() - t0
        simulated = "[SIMULATED" in r.stdout or "[SIMULATED" in r.stderr
        return {
            "success": r.returncode == 0,
            "runtime": "aura",
            "wall_sec": round(wall, 3),
            "memory_budget": memory,
            "is_simulated": simulated,
            "stdout_chars": len(r.stdout),
            "stderr_snippet": r.stderr[:200],
            "note": "SIMULATED — llama.cpp binary not found; planning metrics only" if simulated else "real",
        }
    except subprocess.TimeoutExpired:
        return {"success": False, "runtime": "aura", "error": f"TIMEOUT >{TIMEOUT}s",
                "wall_sec": TIMEOUT, "memory_budget": memory}
    except Exception as e:
        return {"success": False, "runtime": "aura", "error": str(e),
                "wall_sec": round(time.time()-t0, 3), "memory_budget": memory}


# ── Main campaign ─────────────────────────────────────────────────────────────

def main():
    print("=" * 70)
    print("  AURA V7 — Fair Benchmark: AURA vs Ollama REST API")
    print(f"  Ollama API: {OLLAMA_API}")
    print("=" * 70)

    # Discover models via REST API
    try:
        with urllib.request.urlopen(f"{OLLAMA_API}/api/tags", timeout=10) as r:
            tags = json.loads(r.read())
    except Exception as e:
        print(f"[FATAL] Cannot reach Ollama: {e}\nIs 'ollama serve' running?")
        sys.exit(1)

    models = [
        m for m in tags.get("models", [])
        if m.get("size", 0) > 1000 and not m.get("remote_model")
    ]
    print(f"\n  {len(models)} local models discovered.\n")

    raw_results = {
        "timestamp_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "ollama_api": OLLAMA_API,
        "aura_exe": AURA_EXE,
        "primary_memory_budget_gb": 4.0,
        "fairness_note": "Ollama uses num_gpu=0, num_thread=8 via REST API. Timing from eval_count/eval_duration fields.",
        "results": {}
    }
    csv_rows = []

    for m in models:
        tag = m["name"]
        details = m.get("details", {})
        params  = details.get("parameter_size", "?")
        quant   = details.get("quantization_level", "?")
        ctx     = details.get("context_length", 0)
        size_gb = round(m.get("size", 0) / 1_073_741_824, 2)

        print(f"\n" + "-"*70)
        print(f"  MODEL: {tag}  ({params}, {quant}, {size_gb} GB, ctx={ctx})")
        print("-"*70)

        model_results = {"details": m.get("details", {}), "prompts": {}}

        for cat, prompt in BENCHMARK_PROMPTS.items():
            print(f"  [{cat}]", end=" ", flush=True)

            # Cold run
            oll_cold = ollama_generate(tag, prompt)
            aura_cold = aura_run(tag, prompt, "4G")
            print("cold✓", end=" ", flush=True)

            # 3 warm runs
            oll_warm, aura_warm = [], []
            for _ in range(3):
                oll_warm.append(ollama_generate(tag, prompt))
                aura_warm.append(aura_run(tag, prompt, "4G"))
            print("warm✓")

            entry = {
                "prompt_category": cat,
                "ollama_cold": oll_cold,
                "ollama_warm": oll_warm,
                "aura_cold_4gb": aura_cold,
                "aura_warm_4gb": aura_warm,
            }

            # Stats
            oll_tps  = [r["decode_tok_per_sec"] for r in oll_warm if r.get("decode_tok_per_sec")]
            oll_wall = [r["wall_sec"] for r in oll_warm if r.get("success")]
            aura_wall= [r["wall_sec"] for r in aura_warm if r.get("success")]

            if oll_tps:
                entry["ollama_decode_median_tps"] = statistics.median(oll_tps)
            if oll_wall:
                entry["ollama_wall_median_sec"] = statistics.median(oll_wall)
            if aura_wall:
                entry["aura_wall_median_sec"] = statistics.median(aura_wall)

            model_results["prompts"][cat] = entry
            csv_rows.append({
                "model": tag, "params": params, "quant": quant, "size_gb": size_gb,
                "category": cat,
                "ollama_cold_wall_sec": oll_cold.get("wall_sec"),
                "ollama_decode_tps": oll_cold.get("decode_tok_per_sec"),
                "ollama_ttft_ms": oll_cold.get("ttft_ms"),
                "ollama_success": oll_cold.get("success"),
                "aura_cold_wall_sec": aura_cold.get("wall_sec"),
                "aura_is_simulated": aura_cold.get("is_simulated"),
                "aura_success": aura_cold.get("success"),
                "memory_budget": "4G",
            })

        # Memory budget sweep (for 4G primary + 6G, 8G secondary)
        sweep_prompt = BENCHMARK_PROMPTS["chat"]
        sweep_results = {}
        for budget in MEMORY_BUDGETS:
            aura_r = aura_run(tag, sweep_prompt, budget)
            sweep_results[budget] = aura_r
            status = "✓" if aura_r.get("success") else "✗"
            print(f"  [budget sweep] {budget}: AURA={status}  ({aura_r.get('wall_sec', '?')}s)")
        model_results["memory_budget_sweep"] = sweep_results

        raw_results["results"][tag] = model_results

    # Write outputs
    os.makedirs(RESULTS_DIR, exist_ok=True)
    raw_path = os.path.join(RESULTS_DIR, "V7_RAW_RESULTS.json")
    with open(raw_path, "w", encoding="utf-8") as f:
        json.dump(raw_results, f, indent=2)
    print(f"\n[V7] Raw results: {raw_path}")

    csv_path = os.path.join(RESULTS_DIR, "V7_MODEL_BENCHMARK_MATRIX.csv")
    if csv_rows:
        with open(csv_path, "w", newline="", encoding="utf-8") as f:
            writer = csv.DictWriter(f, fieldnames=csv_rows[0].keys())
            writer.writeheader()
            writer.writerows(csv_rows)
    print(f"[V7] CSV matrix: {csv_path}")

    # Print summary
    print("\n" + "="*70)
    print("  AURA V7 — Per-Model Ollama REST Decode Speed Summary")
    print("="*70)
    for tag, mdata in raw_results["results"].items():
        all_tps = []
        for cat_data in mdata["prompts"].values():
            if cat_data.get("ollama_decode_median_tps"):
                all_tps.append(cat_data["ollama_decode_median_tps"])
        avg_tps = statistics.mean(all_tps) if all_tps else None
        print(f"  {tag:40s}  Ollama REST decode: {f'{avg_tps:.2f} tok/s' if avg_tps else 'N/A'}")

    print("\n[V7] Campaign complete.")

if __name__ == "__main__":
    main()
