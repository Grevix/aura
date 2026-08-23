"""
AURA V6 — Full Local Ollama Model 4GB Adversarial Benchmark Harness
Phases 0-20: Auto-discovery, real-workload evaluation, AURA vs Ollama comparison.
"""
import os
import sys
import json
import time
import subprocess
import datetime
import csv
import statistics
import re

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.abspath(os.path.join(SCRIPT_DIR, ".."))
RESULTS_DIR = os.path.join(ROOT_DIR, "research", "benchmarks", "v6")
AUDITS_DIR = os.path.join(ROOT_DIR, "research", "audits", "v6")
PROMPT_SUITE = os.path.join(RESULTS_DIR, "V6_PROMPT_SUITE.json")
AURA_EXE = os.path.join(ROOT_DIR, "target", "debug", "aura.exe")

TIMEOUT_SEC = 90

# ─── 1. Auto-discover local Ollama models ─────────────────────────────────────
def discover_ollama_models():
    """Run 'ollama list' and parse every line. Excludes cloud-only entries."""
    try:
        r = subprocess.run(
            ["ollama", "list"],
            stdout=subprocess.PIPE, stderr=subprocess.PIPE,
            text=True, encoding="utf-8", errors="ignore", timeout=30
        )
        models = []
        for line in r.stdout.splitlines()[1:]:   # skip header
            parts = line.split()
            if len(parts) < 3:
                continue
            name, mid, size_str = parts[0], parts[1], parts[2]
            # kimi-k3 has SIZE "-" → cloud-only, exclude
            if size_str == "-":
                print(f"  [SKIP] {name} — cloud-routed model, no local GGUF blob.")
                continue
            try:
                size_gb = float(size_str.replace("GB","").replace("MB",""))
                if "MB" in size_str:
                    size_gb /= 1024
            except Exception:
                size_gb = 0.0
            models.append({"tag": name, "id": mid, "size_gb": size_gb})
        return models
    except Exception as e:
        print(f"[ERROR] ollama list failed: {e}")
        return []

# ─── 2. Load prompt suite ──────────────────────────────────────────────────────
def load_prompts():
    with open(PROMPT_SUITE, "r", encoding="utf-8") as f:
        return json.load(f).get("categories", {})

# ─── 3. Run AURA @ 4GB ────────────────────────────────────────────────────────
def run_aura(model_tag, prompt, budget="4G"):
    if not os.path.exists(AURA_EXE):
        return {"runtime": "aura", "success": False, "error": "aura.exe not found", "elapsed_sec": 0.0, "stdout_len": 0}
    t0 = time.time()
    try:
        cmd = [AURA_EXE, "run", "--model", model_tag, "--prompt", prompt, "--memory", budget]
        r = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                           text=True, encoding="utf-8", errors="ignore", timeout=TIMEOUT_SEC)
        elapsed = time.time() - t0
        return {
            "runtime": "aura",
            "success": r.returncode == 0,
            "elapsed_sec": round(elapsed, 3),
            "stdout_len": len(r.stdout),
            "stderr_snippet": r.stderr[:200]
        }
    except subprocess.TimeoutExpired:
        return {"runtime": "aura", "success": False, "error": f"TIMEOUT >{TIMEOUT_SEC}s", "elapsed_sec": TIMEOUT_SEC, "stdout_len": 0}
    except Exception as e:
        return {"runtime": "aura", "success": False, "error": str(e), "elapsed_sec": round(time.time()-t0, 3), "stdout_len": 0}

# ─── 4. Run Ollama CPU ────────────────────────────────────────────────────────
def run_ollama(model_tag, prompt):
    t0 = time.time()
    try:
        cmd = ["ollama", "run", model_tag, prompt]
        r = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                           text=True, encoding="utf-8", errors="ignore", timeout=TIMEOUT_SEC)
        elapsed = time.time() - t0
        return {
            "runtime": "ollama",
            "success": r.returncode == 0,
            "elapsed_sec": round(elapsed, 3),
            "stdout_len": len(r.stdout),
            "stderr_snippet": r.stderr[:200]
        }
    except subprocess.TimeoutExpired:
        return {"runtime": "ollama", "success": False,
                "error": f"TIMEOUT >{TIMEOUT_SEC}s — Ollama non-interactive TTY stdin hang",
                "elapsed_sec": TIMEOUT_SEC, "stdout_len": 0}
    except Exception as e:
        return {"runtime": "ollama", "success": False, "error": str(e), "elapsed_sec": round(time.time()-t0,3), "stdout_len": 0}

# ─── 5. Prompt size ladder ────────────────────────────────────────────────────
PROMPT_LADDER = [
    ("S1_TINY",   "Explain recursion."),
    ("S2_SMALL",  "Explain what an API is to a first-year CS student. Include a real example showing how a mobile app talks to a server backend."),
    ("S3_MEDIUM", "Write a complete Python program that reads a CSV file, cleans missing values by filling numeric columns with the column mean and string columns with the string 'unknown', detects duplicate rows and removes them, then writes the cleaned CSV to a new output file. Include error handling and a brief explanation of every step."),
    ("S4_LARGE",  ("Design and implement a production-ready Python FastAPI service that manages users. "
                   "Include: JWT authentication middleware, PostgreSQL connection pooling via asyncpg, "
                   "Pydantic models for request/response validation, proper HTTP status codes, "
                   "rate limiting, health check endpoint, and OpenAPI docs auto-generation. "
                   "Explain every design decision. Also list 5 security vulnerabilities to watch for. " * 4).strip()),
    ("S5_XLARGE", ("Explain the Linux kernel virtual memory subsystem in extreme depth. "
                   "Cover: page tables, TLB, huge pages, NUMA topology, page fault handling, "
                   "demand paging, copy-on-write, memory mapping (mmap), anonymous vs file-backed pages, "
                   "ksmd/khugepaged, OOM killer policy, cgroup memory limits, and swap. "
                   "Then contrast this with the Windows NT memory manager and explain how "
                   "Win32 Job Objects enforce process working set limits. "
                   "Finally design a cross-platform memory budget enforcement library in Rust. " * 3).strip()),
]

# ─── 6. Output size ladder prompts ───────────────────────────────────────────
OUTPUT_LADDER = [32, 64, 128, 256, 512]   # target token counts (approximate via word count)

# ─── 7. Multi-turn session ───────────────────────────────────────────────────
MULTI_TURN_SESSION = [
    "I am building a student management system. What tech stack do you recommend?",
    "Now add JWT authentication to the backend.",
    "Explain the 3 biggest security risks in this design.",
    "Now write the database schema for students, courses, and grades.",
    "Write the FastAPI endpoint to enroll a student in a course.",
    "Find bugs in the endpoint you just wrote.",
    "Optimize the database queries for performance.",
]

# ─── 8. Cold vs warm cache test ──────────────────────────────────────────────
COLD_WARM_PROMPT = "Write a C program that implements a singly linked list with insert, delete, and print operations."

# ─── 9. 4GB boundary budgets ─────────────────────────────────────────────────
MEMORY_BUDGETS = ["2G", "2.5G", "3G", "3.5G", "3.75G", "4G"]

# ─── Main campaign ───────────────────────────────────────────────────────────
def main():
    os.makedirs(RESULTS_DIR, exist_ok=True)
    os.makedirs(AUDITS_DIR, exist_ok=True)

    print("=" * 60)
    print(" AURA V6 — Full Local Ollama 4GB Adversarial Campaign")
    print("=" * 60)

    # Phase 0: env already frozen via V6_ENVIRONMENT.json

    # Phase 1: auto-discover
    print("\n[Phase 1] Discovering local Ollama models...")
    models = discover_ollama_models()
    print(f"  Found {len(models)} testable local models:")
    for m in models:
        print(f"    {m['tag']}  ({m['size_gb']} GB)")

    # Phase 3: load prompt suite
    categories = load_prompts()
    all_cat_prompts = [(cat, p) for cat, prompts in categories.items() for p in prompts]
    print(f"\n[Phase 3] Loaded {len(all_cat_prompts)} prompts across {len(categories)} categories.")

    raw_results = {
        "timestamp_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "memory_budget_gb": 4.0,
        "models": [m["tag"] for m in models],
        "prompt_count": len(all_cat_prompts),
        "runs": {}
    }

    csv_rows = []

    for m_info in models:
        m_tag = m_info["tag"]
        print(f"\n{'='*60}")
        print(f"  MODEL: {m_tag}  ({m_info['size_gb']} GB)")
        print(f"{'='*60}")

        m_data = {"category_runs": {}, "prompt_ladder": [], "cold_warm": [], "multi_turn": [], "memory_boundary": []}

        # ── Phase 6 + 10: Category prompts, AURA vs Ollama ─────────────────
        for cat_name, prompts in categories.items():
            cat_runs = []
            for idx, p in enumerate(prompts):
                print(f"  [{cat_name}] {idx+1}/{len(prompts)}: {p[:40]}...")
                # Alternate order for fairness
                if idx % 2 == 0:
                    aura_r = run_aura(m_tag, p)
                    oll_r  = run_ollama(m_tag, p)
                else:
                    oll_r  = run_ollama(m_tag, p)
                    aura_r = run_aura(m_tag, p)

                entry = {"prompt": p, "aura": aura_r, "ollama": oll_r}
                cat_runs.append(entry)
                csv_rows.append({
                    "model": m_tag, "size_gb": m_info["size_gb"],
                    "category": cat_name, "prompt_snippet": p[:60],
                    "aura_success": aura_r["success"],
                    "aura_elapsed_sec": aura_r["elapsed_sec"],
                    "aura_stdout_len": aura_r.get("stdout_len", 0),
                    "ollama_success": oll_r["success"],
                    "ollama_elapsed_sec": oll_r["elapsed_sec"],
                    "ollama_error": oll_r.get("error", ""),
                    "memory_budget_gb": 4.0,
                })
            m_data["category_runs"][cat_name] = cat_runs

        # ── Phase 4: Prompt size ladder ────────────────────────────────────
        print(f"  [Prompt Ladder]")
        for size_name, p in PROMPT_LADDER:
            print(f"    {size_name}: {p[:35]}...")
            aura_r = run_aura(m_tag, p)
            m_data["prompt_ladder"].append({"size": size_name, "prompt_len": len(p), "aura": aura_r})

        # ── Phase 8: Cold vs Warm (3 warm runs) ────────────────────────────
        print(f"  [Cold/Warm Cache]")
        cold_r = run_aura(m_tag, COLD_WARM_PROMPT)
        m_data["cold_warm"].append({"run": "COLD", "aura": cold_r})
        for w in range(1, 4):
            warm_r = run_aura(m_tag, COLD_WARM_PROMPT)
            m_data["cold_warm"].append({"run": f"WARM_{w}", "aura": warm_r})

        # ── Phase 7: Multi-turn session ────────────────────────────────────
        print(f"  [Multi-Turn Session]")
        for turn_idx, turn_prompt in enumerate(MULTI_TURN_SESSION):
            turn_r = run_aura(m_tag, turn_prompt)
            m_data["multi_turn"].append({"turn": turn_idx+1, "prompt": turn_prompt, "aura": turn_r})

        # ── Phase 9: Memory boundary sweep (AURA only) ────────────────────
        print(f"  [Memory Boundary Sweep]")
        boundary_prompt = "Explain sorting algorithms."
        for budget in MEMORY_BUDGETS:
            bnd_r = run_aura(m_tag, boundary_prompt, budget=budget)
            m_data["memory_boundary"].append({"budget": budget, "aura": bnd_r})

        raw_results["runs"][m_tag] = m_data

    # ── Persist raw JSON ───────────────────────────────────────────────────────
    raw_path = os.path.join(RESULTS_DIR, "V6_RAW_RESULTS.json")
    with open(raw_path, "w", encoding="utf-8") as f:
        json.dump(raw_results, f, indent=2)
    print(f"\n[V6] Saved raw results: {raw_path}")

    # ── Persist CSV matrix ────────────────────────────────────────────────────
    csv_path = os.path.join(RESULTS_DIR, "V6_MODEL_BENCHMARK_MATRIX.csv")
    if csv_rows:
        with open(csv_path, "w", newline="", encoding="utf-8") as f:
            writer = csv.DictWriter(f, fieldnames=csv_rows[0].keys())
            writer.writeheader()
            writer.writerows(csv_rows)
    print(f"[V6] Saved CSV matrix: {csv_path}")

    # ── Persist JSON matrix ───────────────────────────────────────────────────
    json_matrix_path = os.path.join(RESULTS_DIR, "V6_MODEL_BENCHMARK_MATRIX.json")
    with open(json_matrix_path, "w", encoding="utf-8") as f:
        json.dump(csv_rows, f, indent=2)
    print(f"[V6] Saved JSON matrix: {json_matrix_path}")

    # ── Compute and print summary statistics ──────────────────────────────────
    print("\n" + "="*60)
    print("  AURA V6 — Per-Model Summary Statistics")
    print("="*60)
    for m_tag, m_data in raw_results["runs"].items():
        aura_times = []
        aura_successes = 0
        ollama_timeouts = 0
        total_prompts = 0
        for cat_name, runs in m_data["category_runs"].items():
            for entry in runs:
                total_prompts += 1
                if entry["aura"]["success"]:
                    aura_times.append(entry["aura"]["elapsed_sec"])
                    aura_successes += 1
                err = entry["ollama"].get("error", "")
                if "TIMEOUT" in err or not entry["ollama"]["success"]:
                    ollama_timeouts += 1

        if aura_times:
            med = statistics.median(aura_times)
            mn  = statistics.mean(aura_times)
            sd  = statistics.stdev(aura_times) if len(aura_times)>1 else 0.0
        else:
            med = mn = sd = None
        aura_pct = 100.0 * aura_successes / total_prompts if total_prompts else 0
        oll_fail_pct = 100.0 * ollama_timeouts / total_prompts if total_prompts else 0
        print(f"\n  {m_tag}")
        print(f"    AURA success rate : {aura_pct:.1f}%  ({aura_successes}/{total_prompts})")
        print(f"    AURA median latency: {med:.3f}s" if med else "    AURA median latency: N/A")
        print(f"    AURA mean±std     : {mn:.3f}±{sd:.3f}s" if mn else "    AURA mean±std: N/A")
        print(f"    Ollama timeout/fail: {oll_fail_pct:.1f}%")

    print("\n[V6] Campaign complete.")

if __name__ == "__main__":
    main()
