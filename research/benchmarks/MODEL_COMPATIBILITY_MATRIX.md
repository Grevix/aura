# Model Compatibility & Cross-Runtime Matrix

**Date:** 23 August 2026  
**Ollama Version:** 0.32.15  

---

## Complete Model Compatibility Table

| Local Model Name | Ollama Support | AURA Support | AirLLM Support | Direct Comparison Possible? | Conversion Required? | Quantization Identical? | Architecture Identical? | Classification |
|---|---|---|---|---|---|---|---|---|
| `qwen2.5-coder:7b` | ✅ Direct | ✅ GGUF Blob | ❌ Safetensors format | **AURA vs Ollama** | No | Yes (Q4_K_M) | Yes | **DIRECTLY COMPARABLE** |
| `llama3:8b-instruct-q4_0` | ✅ Direct | ✅ GGUF Blob | ❌ Safetensors format | **AURA vs Ollama** | No | Yes (Q4_0) | Yes | **DIRECTLY COMPARABLE** |
| `deepseek-r1:latest` | ✅ Direct | ✅ GGUF Blob | ❌ Safetensors format | **AURA vs Ollama** | No | Yes (Q4_K_M) | Yes | **DIRECTLY COMPARABLE** |
| `mistral:latest` | ✅ Direct | ✅ GGUF Blob | ❌ Safetensors format | **AURA vs Ollama** | No | Yes (Q4_0) | Yes | **DIRECTLY COMPARABLE** |
| `gemma4:latest` | ✅ Direct | ✅ GGUF Blob | ❌ Safetensors format | **AURA vs Ollama** | No | Yes (Q4_K_M) | Yes | **DIRECTLY COMPARABLE** |
| `kimi-k3:cloud` | ☁️ Remote Tag | ❌ Local Only | ❌ Local Only | ❌ Remote | N/A | Remote API | Remote API | **NOT COMPARABLE** |

---

## Format & Compatibility Notes
1. **Ollama & AURA:** Both run directly against native GGUF single-file binary blobs stored in `C:\Users\Aaryan Rawat\.ollama\models\blobs\`.
2. **AirLLM:** Requires unquantized or bitsandbytes-quantized HuggingFace PyTorch safetensors directories. It cannot consume GGUF binary blobs directly without conversion.
