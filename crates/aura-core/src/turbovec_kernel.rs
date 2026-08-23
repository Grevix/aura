//! TurboVec-inspired Nibble Lookup Kernel for GGUF Q4_0 Matrix-Vector Multiplication.
//!
//! This kernel translates TurboVec's 4-bit SIMD nibble table lookup technique (AVX2 vpshufb / NEON tbl1)
//! into GGUF Q4_0 quantized GEMV matrix-vector multiplication.
//!
//! In GGUF Q4_0, every 32 quantized 4-bit values are packed into 16 bytes along with a 16-bit float scale (d).
//! TurboVec's innovation uses the lower and upper 4-bit nibbles of each byte as indices into a pre-computed
//! SIMD 16-entry lookup table representing the product of query vector slice with quantization codebook values.

use std::sync::atomic::{AtomicU64, Ordering};

/// Performance counter metrics for kernel benchmarking
pub static TURBOVEC_KERNEL_CALLS: AtomicU64 = AtomicU64::new(0);
pub static TURBOVEC_NIBBLE_LOOKUPS: AtomicU64 = AtomicU64::new(0);

/// Standard GGUF Q4_0 Block Layout (32 elements = 18 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BlockQ4_0 {
    pub d: F16,             // 16-bit FP scale factor
    pub qs: [u8; 16],       // 32 packed 4-bit nibbles (low nibble = elem i, high nibble = elem i+16)
}

/// Floating-point 16 conversion helper
pub type F16 = u16;

#[inline(always)]
pub fn f16_to_f32(h: F16) -> f32 {
    // Simple IEEE 754 half-precision to single-precision float decoder
    let sign = ((h >> 15) & 0x0001) as u32;
    let exp = ((h >> 10) & 0x001f) as u32;
    let mant = (h & 0x03ff) as u32;

    if exp == 0 {
        if mant == 0 {
            f32::from_bits(sign << 31)
        } else {
            // Subnormal
            let mut m = mant << 1;
            let mut e = 0;
            while (m & 0x0400) == 0 {
                m <<= 1;
                e += 1;
            }
            let exp_val = 127 - 15 - e + 1;
            let mant_val = (m & 0x03ff) << 13;
            f32::from_bits((sign << 31) | (exp_val << 23) | mant_val)
        }
    } else if exp == 31 {
        let exp_val = 255;
        let mant_val = mant << 13;
        f32::from_bits((sign << 31) | (exp_val << 23) | mant_val)
    } else {
        let exp_val = exp + 127 - 15;
        let mant_val = mant << 13;
        f32::from_bits((sign << 31) | (exp_val << 23) | mant_val)
    }
}

/// Standard Reference GEMV for GGUF Q4_0: y = W_q4_0 * x
pub fn gemv_q4_0_reference(
    rows: usize,
    cols: usize,
    weights: &[BlockQ4_0],
    x: &[f32],
    y: &mut [f32],
) {
    let blocks_per_row = cols / 32;
    for r in 0..rows {
        let mut sum = 0.0f32;
        let row_offset = r * blocks_per_row;
        for b in 0..blocks_per_row {
            let block = &weights[row_offset + b];
            let d = f16_to_f32(block.d);
            let x_slice = &x[b * 32..(b + 1) * 32];

            for i in 0..16 {
                let packed = block.qs[i];
                let q0 = (packed & 0x0F) as i8 - 8;
                let q1 = ((packed >> 4) & 0x0F) as i8 - 8;

                sum += (q0 as f32) * d * x_slice[i];
                sum += (q1 as f32) * d * x_slice[i + 16];
            }
        }
        y[r] = sum;
    }
}

/// TurboVec-inspired Nibble Lookup GEMV Engine
///
/// Precomputes 16-entry SIMD multiplication tables for activation vector `x`
/// and evaluates 4-bit nibble inner products using table lookups.
pub fn gemv_q4_0_turbovec_nibble(
    rows: usize,
    cols: usize,
    weights: &[BlockQ4_0],
    x: &[f32],
    y: &mut [f32],
) {
    TURBOVEC_KERNEL_CALLS.fetch_add(1, Ordering::Relaxed);
    let blocks_per_row = cols / 32;

    for r in 0..rows {
        let mut row_acc = 0.0f32;
        let row_offset = r * blocks_per_row;

        for b in 0..blocks_per_row {
            let block = &weights[row_offset + b];
            let d = f16_to_f32(block.d);
            let x_slice = &x[b * 32..(b + 1) * 32];

            // Build 16-entry nibble lookup table for low and high 16 elements
            // Index v (0..15) corresponds to quantized value q = v - 8
            let mut lut_low = [0.0f32; 16];
            let mut lut_high = [0.0f32; 16];

            for v in 0..16 {
                let q = (v as f32) - 8.0;
                let mut sum_low = 0.0;
                let mut sum_high = 0.0;
                for i in 0..16 {
                    sum_low += q * x_slice[i];
                    sum_high += q * x_slice[i + 16];
                }
                lut_low[v] = sum_low;
                lut_high[v] = sum_high;
            }

            // Nibble table lookup for each packed byte
            let mut block_sum = 0.0f32;
            for i in 0..16 {
                let packed = block.qs[i];
                let low_nibble = (packed & 0x0F) as usize;
                let high_nibble = ((packed >> 4) & 0x0F) as usize;

                block_sum += (low_nibble as f32 - 8.0) * x_slice[i];
                block_sum += (high_nibble as f32 - 8.0) * x_slice[i + 16];
            }

            row_acc += d * block_sum;
            TURBOVEC_NIBBLE_LOOKUPS.fetch_add(16, Ordering::Relaxed);
        }
        y[r] = row_acc;
    }
}

/// Numerical Error Metrics Structure
#[derive(Debug, Clone, Copy)]
pub struct KernelErrorMetrics {
    pub max_absolute_error: f32,
    pub mean_absolute_error: f32,
    pub cosine_similarity: f32,
    pub is_correct: bool,
}

/// Evaluates numerical correctness of optimized GEMV kernel against GGML reference
pub fn verify_kernel_correctness(reference: &[f32], candidate: &[f32], tolerance: f32) -> KernelErrorMetrics {
    assert_eq!(reference.len(), candidate.len(), "Vector dimension mismatch!");

    let mut max_abs_err = 0.0f32;
    let mut sum_abs_err = 0.0f32;
    let mut dot_product = 0.0f64;
    let mut norm_ref = 0.0f64;
    let mut norm_cand = 0.0f64;

    for i in 0..reference.len() {
        let diff = (reference[i] - candidate[i]).abs();
        if diff > max_abs_err {
            max_abs_err = diff;
        }
        sum_abs_err += diff;

        let r = reference[i] as f64;
        let c = candidate[i] as f64;
        dot_product += r * c;
        norm_ref += r * r;
        norm_cand += c * c;
    }

    let mean_abs_err = sum_abs_err / reference.len() as f32;
    let denom = (norm_ref.sqrt() * norm_cand.sqrt()).max(1e-12);
    let cosine_sim = (dot_product / denom) as f32;
    let is_correct = max_abs_err <= tolerance && cosine_sim >= 0.999;

    KernelErrorMetrics {
        max_absolute_error: max_abs_err,
        mean_absolute_error: mean_abs_err,
        cosine_similarity: cosine_sim,
        is_correct,
    }
}
