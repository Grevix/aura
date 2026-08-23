use aura_core::turbovec_kernel::{
    gemv_q4_0_reference, gemv_q4_0_turbovec_nibble, verify_kernel_correctness, BlockQ4_0,
};

#[test]
fn test_turbovec_nibble_kernel_numerical_correctness() {
    let rows = 128;
    let cols = 4096; // Realistic LLM dimension
    let blocks_per_row = cols / 32;

    let mut weights = Vec::with_capacity(rows * blocks_per_row);
    for r in 0..rows {
        for b in 0..blocks_per_row {
            let mut qs = [0u8; 16];
            for i in 0..16 {
                let q0 = ((r + b + i) % 15) as u8;
                let q1 = ((r * 2 + b + i) % 15) as u8;
                qs[i] = (q1 << 4) | (q0 & 0x0F);
            }
            // Scale d = 0.125 (0x3000 in IEEE fp16)
            weights.push(BlockQ4_0 { d: 0x3000, qs });
        }
    }

    let mut x = vec![0.0f32; cols];
    for i in 0..cols {
        x[i] = ((i % 17) as f32 - 8.0) * 0.1;
    }

    let mut y_ref = vec![0.0f32; rows];
    let mut y_cand = vec![0.0f32; rows];

    gemv_q4_0_reference(rows, cols, &weights, &x, &mut y_ref);
    gemv_q4_0_turbovec_nibble(rows, cols, &weights, &x, &mut y_cand);

    let metrics = verify_kernel_correctness(&y_ref, &y_cand, 1e-4);

    println!("=== TURBOVEC KERNEL NUMERICAL VERIFICATION ===");
    println!("Max Absolute Error  : {:.6}", metrics.max_absolute_error);
    println!("Mean Absolute Error : {:.6}", metrics.mean_absolute_error);
    println!("Cosine Similarity   : {:.6}", metrics.cosine_similarity);
    println!("Correctness Status  : {}", metrics.is_correct);

    assert!(
        metrics.is_correct,
        "TurboVec kernel failed numerical correctness! Cosine similarity must be >= 0.999"
    );
}
