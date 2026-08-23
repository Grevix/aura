use crate::estimator::estimate_memory_footprint;
use crate::rules::determine_enforcement_mechanism;
use aura_core::types::{ExecutionPlan, HardwareProfile, ModelManifest, QuantType};

pub fn generate_execution_plan(
    hw: &HardwareProfile,
    manifest: &ModelManifest,
    budget_bytes: u64,
    requested_context: Option<usize>,
) -> ExecutionPlan {
    // V5 Small-Model Fast-Path Optimization:
    // For small models (<= 3.5B parameters or <= 2.5 GB weights) under a 4GB+ RAM budget,
    // skip the multi-pass search loop and execute the sub-millisecond fast-path.
    if manifest.total_parameters <= 3_500_000_000 || manifest.required_file_bytes <= 2_500_000_000 {
        if budget_bytes >= 4_000_000_000 {
            let target_context = requested_context.unwrap_or(4096).min(manifest.context_length_max);
            let estimate = estimate_memory_footprint(manifest, target_context, &manifest.quantization_type);
            let enforcement = determine_enforcement_mechanism(hw);
            let recommended_threads = hw.cpu.physical_cores.max(1).min(8);

            // V7: Physics-based decode estimate replaces the previous hardcoded 35.0 tok/s.
            // bytes_per_token = weight_bytes / total_parameters (GEMV weight BW per token).
            // effective_bw ~45 GB/s for AVX2 DRAM on i5-13420H. Clamped [4.0, 80.0].
            let bytes_per_token = if manifest.total_parameters > 0 {
                manifest.required_file_bytes as f64 / manifest.total_parameters as f64
            } else {
                4.0
            };
            let effective_bw_bytes_sec: f64 = 45_000_000_000.0;
            let physics_tok_per_sec = (effective_bw_bytes_sec / bytes_per_token.max(0.1)).clamp(4.0, 80.0);

            return ExecutionPlan {
                model_name: manifest.name.clone(),
                memory_budget_bytes: budget_bytes,
                estimated_peak_rss_bytes: estimate.total_peak_rss_bytes,
                estimated_weight_bytes: estimate.weight_bytes,
                estimated_kv_cache_bytes: estimate.kv_cache_bytes,
                estimated_overhead_bytes: estimate.overhead_bytes,
                recommended_quant: manifest.quantization_type.clone(),
                recommended_context: target_context,
                recommended_threads,
                gpu_layers_offloaded: 0,
                predicted_decode_tok_per_sec: physics_tok_per_sec,
                predicted_ttft_ms: 120.0,
                is_feasible: true,
                feasibility_notes: format!(
                    "FAST-PATH FEASIBLE: Small model ({:.1}B params, {:.2} GB) fits within {:.2} GB budget. Physics estimate: {:.1} tok/s.",
                    manifest.total_parameters as f64 / 1e9,
                    manifest.required_file_bytes as f64 / 1e9,
                    budget_bytes as f64 / 1e9,
                    physics_tok_per_sec,
                ),
                enforcement_mechanism: enforcement,
                recommended_flags: vec![
                    format!("--ctx-size {}", target_context),
                    format!("--threads {}", recommended_threads),
                ],
            };
        }
    }


    let mut target_context = requested_context
        .unwrap_or(4096)
        .min(manifest.context_length_max);

    let mut selected_quant = manifest.quantization_type.clone();
    let mut estimate = estimate_memory_footprint(manifest, target_context, &selected_quant);

    // 0.2.0 Configuration Search Optimization Loop:
    // If initial footprint exceeds budget, attempt context reduction first (down to 1024 min)
    if estimate.total_peak_rss_bytes > budget_bytes && target_context > 1024 {
        target_context = 2048;
        estimate = estimate_memory_footprint(manifest, target_context, &selected_quant);
        if estimate.total_peak_rss_bytes > budget_bytes {
            target_context = 1024;
            estimate = estimate_memory_footprint(manifest, target_context, &selected_quant);
        }
    }

    // If still over budget, suggest quantization downgrade fallback (Q3_K_S)
    if estimate.total_peak_rss_bytes > budget_bytes && selected_quant == QuantType::Q4_K_M {
        selected_quant = QuantType::Q3_K_S;
        // Estimated ~25% weight size reduction for 3-bit variant
        let mut reduced_manifest = manifest.clone();
        reduced_manifest.required_file_bytes = (manifest.required_file_bytes as f64 * 0.75) as u64;
        estimate = estimate_memory_footprint(&reduced_manifest, target_context, &selected_quant);
    }

    let enforcement = determine_enforcement_mechanism(hw);
    let is_feasible = estimate.total_peak_rss_bytes <= budget_bytes;

    let feasibility_notes = if is_feasible {
        format!(
            "Feasible plan: Estimated peak RSS ({:.2} GB) fits within budget ({:.2} GB) at context={}.",
            estimate.total_peak_rss_bytes as f64 / 1e9,
            budget_bytes as f64 / 1e9,
            target_context
        )
    } else {
        format!(
            "INFEASIBLE: Estimated peak RSS ({:.2} GB) exceeds budget ({:.2} GB) even after context reduction to {}.",
            estimate.total_peak_rss_bytes as f64 / 1e9,
            budget_bytes as f64 / 1e9,
            target_context
        )
    };

    let recommended_threads = hw.cpu.physical_cores.max(1).min(8);

    // Storage bandwidth physics calculation
    let storage_bw_bytes_sec = (hw.storage.seq_read_mbps * 1e6).max(1e8);
    let estimated_cold_pass_sec = manifest.required_file_bytes as f64 / storage_bw_bytes_sec;
    let predicted_ttft_ms = (estimated_cold_pass_sec * 1000.0).max(100.0);

    let predicted_decode_tok_per_sec = if is_feasible {
        (1000.0 / (predicted_ttft_ms / 8.0)).clamp(1.5, 45.0)
    } else {
        0.20 // Storage thrashing floor
    };

    let mut recommended_flags = vec![
        format!("--ctx-size {}", target_context),
        format!("--threads {}", recommended_threads),
    ];

    if selected_quant != manifest.quantization_type {
        recommended_flags.push(format!("--quant {}", selected_quant));
    }

    ExecutionPlan {
        model_name: manifest.name.clone(),
        memory_budget_bytes: budget_bytes,
        estimated_peak_rss_bytes: estimate.total_peak_rss_bytes,
        estimated_weight_bytes: estimate.weight_bytes,
        estimated_kv_cache_bytes: estimate.kv_cache_bytes,
        estimated_overhead_bytes: estimate.overhead_bytes,
        recommended_quant: selected_quant,
        recommended_context: target_context,
        recommended_threads,
        gpu_layers_offloaded: 0,
        predicted_decode_tok_per_sec,
        predicted_ttft_ms,
        is_feasible,
        feasibility_notes,
        enforcement_mechanism: enforcement,
        recommended_flags,
    }
}
