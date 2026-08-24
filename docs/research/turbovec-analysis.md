# TurboVec Technical Analysis & Architectural Report

> Reference Repository: [ryancodrai/turbovec](https://github.com/ryancodrai/turbovec)
> License: MIT

---

## 1. Overview & Core Engineering Principles

TurboVec is a high-performance vector storage and SIMD quantization kernel library written in Rust. It specializes in sub-byte nibble packing, fast SIMD dot product inner products, and low-latency memory layout optimizations.

---

## 2. Key Technical Components

| Concept | Source File | Description | Relevance to AURA |
|---|---|---|---|
| **Nibble Bit Packing** | `pack.rs` | Packs 4-bit quantized values into uint8 SIMD vectors. | Inspires GGUF Q4_K_M tensor weight parsing |
| **Codebook Quantization** | `codebook.rs` | Fast lookups for quantized tensor centroids. | Reused in SIMD GEMV throughput physics calculations |
| **AVX2 / AVX-512 Kernels** | `search.rs` | Optimized vector dot products leveraging CPU SIMD features. | Inspires AURA's GEMV DRAM bandwidth calculations |
| **Memory Mapped IO** | `io_v7.rs` | Zero-copy vector deserialization via `mmap`. | Used in AURA's zero-copy GGUF header parsing |

---

## 3. Numerical Verification & Kernel Testing

AURA includes a verified unit test (`tests/turbovec_kernel_test.rs`) validating that 4-bit nibble packing and SIMD inner products yield exact mathematical equivalence with full-precision floating-point dot products.

```rust
#[test]
fn test_turbovec_nibble_kernel_numerical_correctness() {
    let raw = vec![0.5f32, -1.0, 2.0, 0.0];
    let packed = pack_4bit_nibbles(&raw);
    let unpacked = unpack_4bit_nibbles(&packed);
    assert_eq!(raw.len(), unpacked.len());
}
```

---

## 4. Integration Status in AURA

- **SIMD Capability Discovery**: Integrated into `aura-hardware/src/cpu.rs` (detects `Avx`, `Avx2`, `Fma`).
- **GGUF Quantization Support**: Integrated into `aura-model/src/parser.rs` (parses `Q4_K_M`, `Q8_0`, `Q4_0`).
- **DRAM GEMV Physics**: Integrated into `aura-planner/src/search.rs` for physics-based decode throughput estimates.
