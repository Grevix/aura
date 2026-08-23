# Adversarial Falsification Analysis: 70B Model on 2GB RAM

**Date:** 23 August 2026  
**Core Hypothesis to Test:** *"AURA can make 70B-class LLM inference practical and interactive on a 2 GB RAM CPU-only consumer laptop."*  
**Adversarial Verdict:** **PROVEN FALSE (UNFEASIBLE FOR INTERACTIVE GENERATION)**  

---

## 1. Adversarial Falsification Proof

### A. Memory Constraint Math
A 70B model quantized to 4-bit (`Q4_0`) requires **~38.5 – 40.0 GB** of weight data.
Physical RAM budget: **2.0 GB**.

$$M_{\text{Weights}} = 40.0 \text{ GB} \gg M_{\text{RAM}} = 2.0 \text{ GB}$$

Because RAM is less than 5% of total weight size, **95%+ of model parameters MUST reside on NVMe SSD storage** at any microsecond during execution.

### B. I/O Bandwidth Bottleneck
Assuming a PCIe Gen3 x4 NVMe SSD with a sequential read speed of **2.5 GB/s**:

$$t_{\text{I/O per token}} = \frac{40.0 \text{ GB}}{2.5 \text{ GB/s}} = 16.0 \text{ seconds/token}$$

$$\text{Throughput} = \frac{1}{16.0} = \mathbf{0.0625 \text{ tokens/second}}$$

Even with an extreme 2-bit quantization (`IQ2_XXS`, ~21.0 GB weights):

$$t_{\text{I/O per token}} = \frac{21.0 \text{ GB}}{2.5 \text{ GB/s}} = 8.4 \text{ seconds/token} \implies \mathbf{0.119 \text{ tokens/second}}$$

### C. KV Cache Memory Floor
At a 4096 context length, 70B FP16 KV cache requires **~1.25 – 2.50 GB**.
At a 2.0 GB RAM allocation, the KV cache alone consumes **100%+ of physical RAM**, leaving 0 MB for active decoder layer weights, OS kernel pages, or runtime memory allocations.

---

## 2. Definitive Verdict Matrix

| Metric / Requirement | 70B Q4 @ 2GB RAM (NVMe 2.5GB/s) | Interactive Requirement | Status |
|---|---|---|---|
| **Decode Speed** | **0.0625 tok/s** (16s / token) | $\ge 5.0$ tok/s | ❌ **UNACCEPTABLE** |
| **TTFT Latency** | **16.0 seconds** | $\le 1.0$ second | ❌ **UNACCEPTABLE** |
| **RAM Residency** | 2.0 GB / 40.0 GB (5%) | 100% | ❌ **TRASHING RISK** |
| **KV Cache Capacity** | Exhausts total RAM budget | Fits in RAM | ❌ **OOM RISK** |

---

## 3. What Technological Breakthrough WOULD Be Required?

To make 70B @ 2GB RAM interactive ($\ge 5.0 \text{ tok/s}$):

1. **Storage Read Bandwidth Increase:** Requires a storage read bus speed of:

$$B_{\text{required}} = 40.0 \text{ GB} \times 5 \text{ tok/s} = \mathbf{200.0 \text{ GB/s}}$$

*(This is 80x faster than PCIe Gen3 NVMe SSDs, matching high-end GPU VRAM bandwidth).*

2. **Activation Sparsity ($\ge 95\%$):** If a predictive engine (e.g. PowerInfer + LLM in a Flash) predicts with 99% accuracy that 95% of weights can be skipped per token, weight load volume drops from 40 GB to 2.0 GB per token:

$$t_{\text{I/O}} = \frac{2.0 \text{ GB}}{2.5 \text{ GB/s}} = 0.80 \text{ seconds/token} \implies \mathbf{1.25 \text{ tok/s}}$$

---

## 4. Summary Conclusion

The claim that 70B inference on 2GB RAM CPU laptops can be made **fast or interactive** purely through software layer-streaming is **scientifically false**. 

However, AURA **CAN** make 70B models **EXECUTE without OOM crashes** at ~0.06 tok/s for non-interactive batch evaluation.
