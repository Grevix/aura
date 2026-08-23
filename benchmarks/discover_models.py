"""
AURA V7 — Model Discovery Tool
Uses the Ollama REST API (GET /api/tags) to automatically enumerate all local models
with exact parameter counts, quantization, context length, and digest.
Produces research/benchmarks/V7_LOCAL_MODEL_INVENTORY.json.
"""
import json
import urllib.request
import urllib.error
import os
import datetime

ROOT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
OUTPUT_PATH = os.path.join(ROOT_DIR, "research", "benchmarks", "V7_LOCAL_MODEL_INVENTORY.json")

OLLAMA_API = "http://localhost:11434"

TIER_RULES = [
    # (max_params_B,  tier_label,  note)
    (0.7,   "TINY",           "Sub-1B — extreme low-memory target"),
    (2.0,   "SMALL",          "1–2B — comfortable in 2–3 GB RAM"),
    (4.5,   "MEDIUM",         "2–4.5B — fits 4 GB with context headroom"),
    (6.0,   "4GB_BOUNDARY",   "4.5–6B — borderline 4 GB; context reduction may be required"),
    (9.5,   "LARGE",          "6–9.5B — typically needs 5–6 GB; borderline with aggressive quant"),
    (16.0,  "VERY_LARGE",     "9.5–16B — community/8–16 GB RAM track"),
    (35.0,  "LARGE_COMMUNITY","16–35B — community 16–32 GB RAM track"),
    (75.0,  "XL_COMMUNITY",   "35–75B — community 32–64 GB RAM track"),
    (1e9,   "GIANT",          "75B+ or MoE — large GPU / multi-GPU track"),
]

def classify(params_b: float, size_bytes: int, is_cloud: bool) -> str:
    if is_cloud:
        return "CLOUD_ONLY"
    if size_bytes < 1000:
        return "CLOUD_ONLY"
    for max_b, label, _ in TIER_RULES:
        if params_b <= max_b:
            return label
    return "GIANT"

def parse_params(s: str) -> float:
    """Parse '7.6B', '1.2B', '2.81T' → float in billions."""
    if not s:
        return 0.0
    s = s.strip().upper()
    try:
        if s.endswith("T"):
            return float(s[:-1]) * 1000.0
        if s.endswith("B"):
            return float(s[:-1])
        if s.endswith("M"):
            return float(s[:-1]) / 1000.0
        return float(s)
    except Exception:
        return 0.0

def get_tags():
    url = f"{OLLAMA_API}/api/tags"
    try:
        with urllib.request.urlopen(url, timeout=10) as resp:
            return json.loads(resp.read().decode())
    except Exception as e:
        print(f"[ERROR] Cannot reach Ollama REST API at {url}: {e}")
        print("  Is 'ollama serve' running?")
        return None

def main():
    print("=" * 60)
    print(" AURA V7 — Model Discovery via Ollama REST API")
    print(f" Endpoint: {OLLAMA_API}/api/tags")
    print("=" * 60)

    data = get_tags()
    if not data:
        return

    raw_models = data.get("models", [])
    print(f"\n  {len(raw_models)} model entries found.\n")

    inventory = {
        "generated_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "discovery_method": f"Ollama REST GET {OLLAMA_API}/api/tags",
        "model_count_total": len(raw_models),
        "models": []
    }

    local_count = 0
    cloud_count = 0

    for m in raw_models:
        name      = m.get("name", "")
        size_b    = m.get("size", 0)
        digest    = m.get("digest", "")
        details   = m.get("details", {})
        remote    = m.get("remote_model") is not None or m.get("remote_host") is not None

        family      = details.get("family", "")
        families    = details.get("families", [])
        param_str   = details.get("parameter_size", "")
        quant       = details.get("quantization_level", "")
        ctx         = details.get("context_length", 0)
        embed_dim   = details.get("embedding_length", 0)
        caps        = m.get("capabilities", [])

        params_b    = parse_params(param_str)
        tier        = classify(params_b, size_b, remote)
        size_gb     = round(size_b / 1_073_741_824, 3)

        entry = {
            "model_tag":          name,
            "digest_sha256":      digest,
            "size_bytes":         size_b,
            "size_gb":            size_gb,
            "family":             family,
            "families":           families,
            "parameter_size_str": param_str,
            "parameter_size_B":   round(params_b, 3),
            "quantization":       quant,
            "context_length":     ctx,
            "embedding_length":   embed_dim,
            "capabilities":       caps,
            "is_cloud_only":      remote,
            "tier":               tier,
            "include_in_local_benchmark": not remote and size_b > 1000,
        }

        if remote:
            cloud_count += 1
            entry["exclusion_reason"] = (
                f"Cloud-routed model (remote_host detected). No local GGUF blob. "
                f"Parameter count reported as {param_str} but this is a remote service, "
                "NOT a locally executed model weight."
            )
        else:
            local_count += 1

        status = "LOCAL" if not remote else "CLOUD-ONLY (EXCLUDED)"
        print(f"  {status:25s} {name:40s} {param_str:8s} {quant:12s} {size_gb:.2f} GB  [{tier}]")
        inventory["models"].append(entry)

    inventory["model_count_local"]  = local_count
    inventory["model_count_cloud"]  = cloud_count
    inventory["model_count_testable"] = local_count

    os.makedirs(os.path.dirname(OUTPUT_PATH), exist_ok=True)
    with open(OUTPUT_PATH, "w", encoding="utf-8") as f:
        json.dump(inventory, f, indent=2)
    print(f"\n  [OK] Inventory written: {OUTPUT_PATH}")
    print(f"  Testable local models: {local_count}")
    print(f"  Cloud-only (excluded): {cloud_count}")
    return inventory

if __name__ == "__main__":
    main()
