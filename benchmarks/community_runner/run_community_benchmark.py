"""
AURA V7 — Community Benchmark Runner
Single-command tool for contributors to benchmark AURA on their hardware
and generate a validated JSON result for submission.

Usage:
    python run_community_benchmark.py

Produces:
    community_result_<hostname>_<timestamp>.json

Submit via:
    GitHub Issue → AURA Benchmark Submission template
"""
import json
import os
import sys
import time
import datetime
import platform
import subprocess
import urllib.request
import socket
import hashlib

OLLAMA_API = "http://localhost:11434"
TIMEOUT = 180  # Longer for large community models
SCHEMA_VERSION = "v7.0"

BENCHMARK_PROMPTS = {
    "quick_chat":   "What are three practical ways to reduce memory when running LLMs on consumer hardware?",
    "coding_py":    "Write a Python function that implements binary search. Include type hints and a docstring.",
    "reasoning":    "4 workers, 9 tasks, each task takes 1 hour, parallel execution allowed. Minimum completion time?",
    "json_output":  "Return a JSON object for a REST API user-auth endpoint: path, method, request_body, response_body, status_codes.",
}


def collect_hardware():
    hw = {
        "hostname": socket.gethostname(),
        "os": platform.platform(),
        "cpu": platform.processor(),
        "python_version": platform.python_version(),
    }
    try:
        import psutil
        hw["physical_cores"] = psutil.cpu_count(logical=False)
        hw["logical_cores"] = psutil.cpu_count(logical=True)
        hw["total_ram_gb"] = round(psutil.virtual_memory().total / 1e9, 2)
        hw["available_ram_gb"] = round(psutil.virtual_memory().available / 1e9, 2)
    except ImportError:
        hw["ram_note"] = "psutil not installed; RAM not measured"
    return hw


def collect_aura_version(aura_exe):
    try:
        r = subprocess.run([aura_exe, "--version"], capture_output=True, text=True, timeout=5)
        return r.stdout.strip() or r.stderr.strip()
    except Exception:
        return "unknown"


def ollama_generate(model, prompt):
    payload = json.dumps({
        "model": model, "prompt": prompt, "stream": False,
        "options": {"num_gpu": 0, "num_thread": 8}
    }).encode()
    req = urllib.request.Request(
        f"{OLLAMA_API}/api/generate", data=payload,
        headers={"Content-Type": "application/json"}, method="POST"
    )
    t0 = time.time()
    try:
        with urllib.request.urlopen(req, timeout=TIMEOUT) as resp:
            body = json.loads(resp.read().decode())
            wall = time.time() - t0
            ec, ed = body.get("eval_count", 0), body.get("eval_duration", 0)
            pc, pd_ = body.get("prompt_eval_count", 0), body.get("prompt_eval_duration", 0)
            ld = body.get("load_duration", 0)
            decode_tps = (ec / (ed / 1e9)) if (ec and ed) else None
            ttft_ms = ((ld + pd_) / 1e6) if (ld or pd_) else wall * 300
            return {
                "success": True,
                "eval_count": ec, "eval_duration_ns": ed,
                "decode_tok_per_sec": round(decode_tps, 2) if decode_tps else None,
                "ttft_ms": round(ttft_ms, 1),
                "wall_sec": round(wall, 3),
                "timing_source": "ollama_eval_fields" if decode_tps else "wall_clock_fallback",
            }
    except Exception as e:
        return {"success": False, "error": str(e), "wall_sec": round(time.time()-t0, 3)}


def aura_run(aura_exe, model, prompt, memory="4G"):
    if not os.path.exists(aura_exe):
        return {"success": False, "error": "aura.exe not found", "is_simulated": True}
    t0 = time.time()
    try:
        r = subprocess.run(
            [aura_exe, "run", "--model", model, "--memory", memory, "--prompt", prompt],
            capture_output=True, text=True, encoding="utf-8", errors="ignore", timeout=TIMEOUT
        )
        wall = time.time() - t0
        simulated = "[SIMULATED" in r.stdout
        return {
            "success": r.returncode == 0,
            "wall_sec": round(wall, 3),
            "memory_budget": memory,
            "is_simulated": simulated,
            "note": "SIMULATED — llama.cpp not found" if simulated else "real",
        }
    except subprocess.TimeoutExpired:
        return {"success": False, "error": f"TIMEOUT>{TIMEOUT}s", "wall_sec": TIMEOUT}
    except Exception as e:
        return {"success": False, "error": str(e), "wall_sec": round(time.time()-t0, 3)}


def validate_result(result):
    required = ["schema_version", "timestamp_utc", "hardware", "ollama_version", "results"]
    for field in required:
        if field not in result:
            return False, f"Missing required field: {field}"
    if not result["results"]:
        return False, "No model results present"
    return True, "OK"


def main():
    ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    AURA_EXE = os.path.join(ROOT, "target", "debug", "aura.exe")
    if not os.path.exists(AURA_EXE):
        AURA_EXE = os.path.join(ROOT, "target", "release", "aura.exe")

    print("=" * 65)
    print("  AURA V7 — Community Benchmark Runner")
    print(f"  Schema version: {SCHEMA_VERSION}")
    print("=" * 65)

    hw = collect_hardware()
    print(f"\n  Hardware: {hw.get('cpu', 'unknown')}")
    print(f"  RAM: {hw.get('total_ram_gb', '?')} GB")
    print(f"  OS: {hw.get('os', '?')}")

    # Ollama version
    try:
        r = subprocess.run(["ollama", "--version"], capture_output=True, text=True, timeout=5)
        ollama_ver = r.stdout.strip()
    except Exception:
        ollama_ver = "unknown"

    # Discover models
    try:
        with urllib.request.urlopen(f"{OLLAMA_API}/api/tags", timeout=10) as resp:
            tags = json.loads(resp.read())
    except Exception as e:
        print(f"\n[FATAL] Ollama unreachable: {e}")
        sys.exit(1)

    models = [m for m in tags.get("models", [])
              if m.get("size", 0) > 1000 and not m.get("remote_model")]
    print(f"\n  {len(models)} local models found.\n")

    result = {
        "schema_version": SCHEMA_VERSION,
        "timestamp_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "contributor_note": "COMMUNITY REPORTED — independently submitted; not empirically verified by AURA project",
        "hardware": hw,
        "aura_version": collect_aura_version(AURA_EXE),
        "ollama_version": ollama_ver,
        "results": {}
    }

    for m in models:
        tag = m["name"]
        details = m.get("details", {})
        model_results = {
            "parameter_size": details.get("parameter_size"),
            "quantization": details.get("quantization_level"),
            "context_length": details.get("context_length"),
            "size_gb": round(m.get("size", 0) / 1e9, 2),
            "digest": m.get("digest", ""),
            "prompts": {}
        }
        print(f"  {tag} ({details.get('parameter_size', '?')}):")
        for cat, prompt in BENCHMARK_PROMPTS.items():
            oll = ollama_generate(tag, prompt)
            aur = aura_run(AURA_EXE, tag, prompt, "4G")
            s = "✓" if oll.get("success") else "✗"
            tps = oll.get("decode_tok_per_sec")
            print(f"    [{cat}] Ollama={s} {f'{tps:.1f}tok/s' if tps else 'N/A':>12}  AURA={'✓' if aur.get('success') else '✗'}{' (SIMULATED)' if aur.get('is_simulated') else ''}")
            model_results["prompts"][cat] = {"ollama": oll, "aura_4gb": aur}
        result["results"][tag] = model_results

    # Validate
    ok, msg = validate_result(result)
    result["validation"] = {"passed": ok, "message": msg}
    print(f"\n  Validation: {'PASS' if ok else 'FAIL'} — {msg}")

    # Write output
    ts = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    host = socket.gethostname().replace(" ", "_")
    fname = f"community_result_{host}_{ts}.json"
    out_path = os.path.join(os.path.dirname(__file__), fname)
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(result, f, indent=2)

    print(f"\n  [OK] Result written: {out_path}")
    print("\n  To submit:")
    print("  1. Go to https://github.com/aura-ai/aura/issues/new?template=benchmark.yml")
    print("  2. Attach the JSON file above")
    print("  3. Your result will be labelled COMMUNITY REPORTED")


if __name__ == "__main__":
    main()
