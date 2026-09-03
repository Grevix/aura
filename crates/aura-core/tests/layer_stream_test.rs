use aura_core::LayerStreamPipeline;

#[test]
fn test_double_buffered_layer_stream_pipeline() {
    let total_layers = 32;
    let total_model_bytes = 4_000_000_000; // 4 GB model

    let mut pipeline = LayerStreamPipeline::new(total_layers, total_model_bytes);

    for layer in 0..total_layers {
        let (active_slot, next_slot) = pipeline
            .step_layer(layer, 6000.0, 15.0) // 6000 MB/s PCIe, 15 TFLOPS GPU
            .expect("Step valid");

        assert!(active_slot == 0 || active_slot == 1);
        assert_eq!(next_slot, 1 - active_slot);
    }

    let metrics = pipeline.get_metrics();
    assert_eq!(metrics.layers_streamed, 32);
    assert_eq!(metrics.active_double_buffer_slots, 2);
    assert!(metrics.overlap_efficiency_pct > 0.0);
}
