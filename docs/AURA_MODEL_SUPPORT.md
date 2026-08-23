# AURA Model Compatibility & Support Strategy

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Document Type:** Model Ecosystem & Architecture Integration Strategy  
**Status:** Approved  

---

## 1. Model Landscape Taxonomy (August 2026 Baseline)

AURA classifies target models by architecture, parameter scale, sparse activation factor, and backend execution maturity.

| Architecture Class | Representative Models | Total Params | Active Params | Key Memory Bottleneck | AURA Support Tier |
|---|---|---|---|---|---|
| **Small Dense** | Qwen 2.5 7B, Llama 3.1 8B, Mistral 7B | 7B – 8B | 7B – 8B | RAM Capacity & SIMD Compute | **Tier 1 (MVP Stage A Target)** |
| **Medium Dense** | Qwen 3.6 27B, Llama 3.3 70B (Quantized) | 27B – 70B | 27B – 70B | RAM / NVMe Storage Bandwidth | **Tier 1 (Stage A High Target)** |
| **Small Sparse MoE** | Qwen1.5-MoE-A2.7B, Mixtral 8x7B (Q4) | 14B – 47B | 2.7B – 13B | Expert Thrashing & Locality | **Tier 2 (MVP Stage B Target)** |
| **Frontier MoE** | DeepSeek V4 Flash, Kimi K3, Qwen 3.8 Max | 284B – 2.8T | 13B – 104B | Massive Storage Footprint & IOPS | **Tier 3 (Research Target)** |

---

## 2. Supported File Formats & Quantization Enums

AURA natively ingests **GGUF** (GGML Unified Format) files. GGUF provides lazy mmap tensor loading, structured key-value metadata, and multi-bit quantization support.

### Supported Quantization Enums:
1. `Q4_K_M`: Recommended default balance (4-bit weight quantization with 6-bit scale blocks).
2. `Q4_K_S`: Smaller 4-bit variant for constrained memory budgets.
3. `Q3_K_S`: 3-bit quantization for severe RAM constraints.
4. `IQ3_XS`: 3-bit i-matrix quantization for high compression with minimal perplexity degradation.
5. `Q8_0`: 8-bit precision reserved for high-fidelity embedding/KV operations.

---

## 3. Capability Descriptor Schema

AURA does not rely on hardcoded model filenames. Model files are inspected dynamically by `aura-model` to construct a runtime `ModelManifest`:

```json
{
  "name": "Qwen2.5-7B-Instruct",
  "source_hash_sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "architecture_family": "qwen2",
  "total_parameters": 7615616512,
  "active_parameters": 7615616512,
  "is_moe": false,
  "expert_count": null,
  "active_experts_per_token": null,
  "layer_count": 28,
  "attention_heads": 28,
  "key_value_heads": 4,
  "head_dimension": 128,
  "context_length_max": 32768,
  "quantization_type": "Q4_K_M",
  "required_file_bytes": 4350000000,
  "license_spdx": "Apache-2.0"
}
```

---

## 4. MoE Execution Strategy & llama.cpp Issue #19480

For Mixture-of-Experts (MoE) models:
1. **CPU Execution Path:** AURA tracks upstream `llama.cpp` issue `#19480` regarding CPU MoE routing bandwidth overhead.
2. **Stage B Deployment Strategy:** AURA targets small MoE models (e.g. Qwen1.5-MoE-A2.7B) on machines with modest GPU / iGPU presence for offloading non-expert layers, using `--n-cpu-moe` and `--override-tensor` controls.
3. **Locality Telemetry:** AURA logs expert activation distributions to detect routing thrashing during extended context generation.
