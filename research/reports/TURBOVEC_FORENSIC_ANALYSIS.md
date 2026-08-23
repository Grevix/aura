# TurboVec Forensic Source Code & Technology Analysis

**Repository:** `https://github.com/ryancodrai/turbovec.git`  
**Commit:** `ccab9f325e6ce2a270a87daf01ae4e443bcf2d49`  
**Branch:** `main`  
**License:** Apache 2.0  
**Languages:** Rust (94%), Python (5%), Assembly/C (1%)  
**Primary Dependencies:** `rayon`, `bytemuck`, `aligned-vec`  

---

## 1. Executive Summary & Core Purpose

TurboVec is a high-performance vector search engine implemented in pure Rust. It compresses high-dimensional dense embedding vectors down to 2–4 bits per coordinate using data-oblivious TurboQuantization (Hadamard rotation + Lloyd-Max codebooks) and executes top-$k$ nearest neighbor search using SIMD lookup tables.

```
       [Raw High-Dim Vector (f32)]
                   │
                   ▼ (Block-Hadamard Rotation)
        [Incoherence Distribution]
                   │
                   ▼ (2-4 bit Quantization)
        [Packed Bit-Plane Layout]
                   │
                   ▼ (SIMD Lookup Table Search)
        [AVX-512BW / AVX2 / NEON Kernels]
```

---

## 2. Source-Level Architectural & SIMD Walkthrough

### A. Block-Hadamard Rotation (`turbovec/src/rotation.rs`)
TurboVec applies a randomized Block-Hadamard orthogonal rotation matrix to input vectors. This redistributes vector norm energy evenly across dimensions, ensuring coordinates obey a sub-Gaussian distribution and eliminating outliers without training data.

### B. SIMD Search Pipeline (`turbovec/src/search.rs#L1-L250`)
Scoring queries against database vectors relies on nibble-split lookup tables evaluated via hardware SIMD instructions:
- **x86_64:** `AVX-512BW` / `AVX2` (`vpshufb` nibble lookup, `vpaddb` accumulation).
- **aarch64:** `ARM NEON` (`tbl1` vector table lookups).
- **Multithreading:** `Rayon` block-axis parallelization over vectorized blocks (`BLOCK = 32`, `MIN_TILE_BLOCKS = 1024`).

### C. Cache Blocking & Memory Layout (`turbovec/src/pack.rs`)
TurboVec organizes quantized vector bit-planes into cache-aligned contiguous byte structures (`aligned-vec`). Coordinates are packed bitwise to maximize L1/L2 CPU cache hit rates and eliminate unaligned SIMD load penalties.

---

## 3. Adaptable Technologies for AURA

| Technology | TurboVec Source File | What It Does | Why It Matters to AURA | Adaptability |
|---|---|---|---|---|
| **Nibble-Split SIMD Lookups** | `turbovec/src/search.rs` | Computes dot products via 4-bit `vpshufb` SIMD table lookups | Accelerates low-bit CPU quantized matrix-vector multiplication in KV cache & weights | **HIGH** |
| **Cache-Aligned Bit Packing** | `turbovec/src/pack.rs` | Packs 2-4 bit quantized values into SIMD register byte lines | Reduces CPU cache footprint and eliminates unaligned memory stalls | **HIGH** |
| **Data-Oblivious Quantization** | `turbovec/src/rotation.rs` | Applies Hadamard rotation to prevent outlier activation degradation | Prevents accuracy loss in low-bit quantized weight activations | **MEDIUM** |
| **Work-Stealing Tile Scheduling** | `turbovec/src/search.rs#L125-L180` | Dynamically balances parallel tiles across CPU core threads | Provides fine-grained thread scheduling for layer-wise GEMV operations | **HIGH** |
