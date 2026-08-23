# AURA V6 — Output Quality Audit Report

**Date:** 23 August 2026  

> [!IMPORTANT]
> AURA does not perform LLM inference itself — it wraps llama.cpp which runs the model. Therefore **AURA's output quality equals llama.cpp's output quality** for the same model and context. This is both AURA's strength (inherits mature llama.cpp accuracy) and its constraint (cannot surpass llama.cpp quality without changing the backend).

---

## Methodology

For each category prompt, the AURA output was checked against three criteria:
1. **Completeness:** Did the model answer the actual question?
2. **Instruction following:** Were structured output requirements (JSON, code, SQL) respected?
3. **Correctness:** Is code syntactically valid? Is math correct? Is JSON parseable?

---

## Quality Audit Results by Category

| Category | Completeness | Instruction Following | Syntax/JSON Valid | Notes |
|---|---|---|---|---|
| A — Simple Chat | ✅ Complete | ✅ | N/A | No truncation observed |
| B — General Knowledge | ✅ Complete | ✅ | N/A | No hallucinations detected in checked answers |
| C — Coding | ✅ Complete | ✅ | ✅ Compiles | C/Python programs compile correctly |
| D — Debugging | ✅ Complete | ✅ | ✅ | Bugs correctly identified |
| E — Code Generation | ✅ Complete | ✅ | ✅ | FastAPI examples are syntactically valid |
| F — Math Reasoning | ✅ Complete | ✅ | ✅ | Arithmetic answers correct |
| G — Long Context | ⚠️ Truncation risk at Ctx=1024 | ✅ | N/A | Context window capping can truncate very long answers |
| H — Document Style | ✅ Complete | ✅ | N/A | Formatting preserved |
| I — Multi-Turn | ✅ Complete | ✅ | N/A | Context grows per turn; memory pressure managed |
| J — Data Analysis | ✅ Complete | ✅ | ✅ Pandas code valid | |
| K — JSON Structured | ✅ Complete | ✅ | ✅ Valid JSON | qwen2.5-coder especially strong at JSON generation |
| L — Technical Reasoning | ✅ Complete | ✅ | N/A | |
| M — Real Developer | ✅ Complete | ✅ | ✅ | Architecture diagrams and code valid |
| N — Repetitive | ✅ Complete | ✅ | N/A | No degradation detected |
| O — Extreme Context | ⚠️ May truncate | ✅ | N/A | Context cap is the binding constraint, not quality |

---

## Key Finding

> **AURA does NOT degrade output quality vs Ollama for the same model and context length.**
> The only quality difference is indirect: AURA's 4 GB budget may force a shorter context window (Ctx=1024 vs Ctx=4096), which can truncate very long responses.
> This is an **architectural trade-off** of the memory budget constraint, not a model quality regression.
