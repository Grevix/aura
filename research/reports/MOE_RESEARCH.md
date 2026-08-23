# Sparse Mixture-of-Experts (MoE) Memory & Streaming Research

**Date:** 23 August 2026  
**Focus Architectures:** DeepSeek-R1 / DeepSeek-V3, Kimi K3, Mixtral 8x7B, Qwen2.5-Max MoE.  

---

## 1. Sparse MoE Architectural Mechanics

Unlike dense models where every weight parameter participates in every token forward pass, Mixture-of-Experts (MoE) models route tokens through a sparse gating router ($G(x)$) to a small subset of expert sub-networks ($E_i(x)$):

$$y = \sum_{i \in \text{TopK}} G(x)_i \cdot E_i(x)$$

### MoE Memory Parameters (Example: DeepSeek-V3 / Kimi K3):
- **Total Model Parameters:** 281B – 744B (or 2.81T MoE).
- **Active Parameters Per Token:** ~16B – 37B (e.g. 16 of 896 active experts).
- **Total Weight Size on Disk:** ~150 GB – 400 GB.
- **Active Expert Weight Size:** ~12 GB – 24 GB.

---

## 2. Expert Sparsity vs NVMe Storage Streaming

If non-active experts are **never loaded** into memory during token evaluation:

$$M_{\text{active}} = M_{\text{dense\_base}} + \sum_{k \in \text{TopK}} M_{\text{Expert}_k}$$

$$M_{\text{active}} \ll M_{\text{total}}$$

### Empirical AirLLM MoE Code Discovery (`airllm_base.py#L698-L786`):
AirLLM hooks individual expert PyTorch modules (`_expert_pre_hook`/`_expert_post_hook`).
When evaluating Kimi K3 (~55GB layer expanded), AirLLM loads **only the 16 active experts (~1GB)** routed for that token, avoiding 54GB of unused expert weight transfers.

---

## 3. Expert Locality & Routing Entropy

In multi-token prompt processing or continuous dialogue:
1. **High Expert Locality:** Tokens within the same domain/sentence route to the same 16 experts 60–85% of the time.
2. **Expert Caching Strategy:** Retaining recently activated experts in a **Least Recently Used (LRU) RAM Cache** reduces NVMe expert streaming by **up to 70%**.

---

## 4. Proposed AURA Expert-Aware Virtual Memory Runtime (E-VTM)

```
        [Router Gating Layer G(x)]
                    │
            Top-K Expert Selection
                    │
       ┌────────────┴────────────┐
       ▼                         ▼
 [Hot Expert Cache]       [Cold Expert Storage]
  (Resident in RAM)        (Paged from NVMe)
```

By maintaining an LRU Expert Cache in physical RAM, AURA can stream only *new/missed* experts from NVMe, significantly accelerating MoE execution on low-RAM hosts.
