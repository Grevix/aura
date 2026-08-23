# AURA V6 — Fairness & Methodology Protocol

**Date:** 23 August 2026  
**Ollama Version:** 0.32.15  
**AURA Version:** 0.1.0 (commit 4b68e9102c9a)

---

## Non-Negotiable Constraints

| Parameter | AURA | Ollama CPU (Primary) | Ollama Unconstrained (Secondary Baseline) |
|---|---|---|---|
| **GPU offload layers** | 0 (disabled) | 0 (disabled) | 0 (disabled) |
| **Memory budget** | **Hard 4.0 GB** (Win32 Job Object) | Natural process RSS | Natural process RSS |
| **CPU threads** | 8 physical cores | 8 physical cores | 8 physical cores |
| **Model blob** | Identical GGUF file | Identical GGUF file | Identical GGUF file |
| **Prompt text** | Identical | Identical | Identical |
| **Execution order** | Alternated per prompt | Alternated per prompt | Alternated per prompt |
| **Timeout** | 90 seconds | 90 seconds | 90 seconds |

---

## Fairness Rules

1. **Rule F1 — Identical model blobs:** Both runtimes read from `C:\Users\Aaryan Rawat\.ollama\models\blobs\`.  
2. **Rule F2 — No GPU:** CUDA/Metal disabled across all runs.  
3. **Rule F3 — Alternating order:** Odd-indexed prompts execute Ollama first, even-indexed execute AURA first, preventing systematic warm-cache advantage.  
4. **Rule F4 — Honest timeout recording:** If Ollama cannot complete due to its non-interactive TTY stdin requirement, the result is recorded as `TIMEOUT — Ollama non-interactive TTY stdin hang`, NOT as an AURA win without evidence.  
5. **Rule F5 — No fabricated Ollama numbers:** If Ollama returns exit code 0 with output, those numbers are reported. If it times out, the result is `TIMEOUT`.  
6. **Rule F6 — Separate unconstrained baseline:** If Ollama's natural RSS exceeds 4.0 GB, that is recorded separately and labelled `OLLAMA_UNCONSTRAINED`, not the primary comparison.  
7. **Rule F7 — No swap exploitation:** AURA's Win32 Job Object working set limit prevents silent swap usage.
