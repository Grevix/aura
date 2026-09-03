use aura_hardware::probe_hardware;
use aura_model::manifest::load_manifest;
use aura_planner::generate_execution_plan;
use std::path::Path;

fn parse_memory_budget(budget_str: &str) -> u64 {
    let lower = budget_str.trim().to_lowercase();
    if lower.ends_with('g') || lower.ends_with("gb") {
        let num: f64 = lower
            .trim_end_matches("gb")
            .trim_end_matches('g')
            .parse()
            .unwrap_or(4.0);
        (num * 1e9) as u64
    } else if lower.ends_with('m') || lower.ends_with("mb") {
        let num: f64 = lower
            .trim_end_matches("mb")
            .trim_end_matches('m')
            .parse()
            .unwrap_or(4000.0);
        (num * 1e6) as u64
    } else {
        4_000_000_000
    }
}

pub fn execute_plan(model_path: &str, memory: &str, context: Option<usize>) {
    println!("📋 Generating AURA Hardware-Aware Execution Plan...\n");

    let hw = probe_hardware();
    let path = Path::new(model_path);

    let manifest = match load_manifest(path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Failed to parse model: {}", e);
            return;
        }
    };

    let budget_bytes = parse_memory_budget(memory);

    let plan = generate_execution_plan(&hw, &manifest, budget_bytes, context);

    println!("=== AURA EXECUTION DECISION ===");
    println!("Model              : {}", manifest.name);
    println!("Architecture       : {}", manifest.architecture_family);
    println!(
        "Requested Budget   : {} ({:.2} GB)",
        memory,
        budget_bytes as f64 / 1e9
    );
    println!("Enforcement Mode   : {}", plan.enforcement_mechanism);
    println!(
        "Feasibility Status : {}",
        if plan.is_feasible {
            "✅ FEASIBLE"
        } else {
            "⚠️ INFEASIBLE"
        }
    );
    println!("Decision Rationale : {}", plan.feasibility_notes);
    if !plan.is_feasible {
        println!("\n💡 Recommendation:");
        println!("  - Increase budget to {:.1} GB", plan.estimated_peak_rss_bytes as f64 / 1e9 + 0.5);
        println!("  - Or reduce context length (e.g., --context 1024)");
        println!("  - Or select lower-bit quantization (e.g., Q3_K_S or MXFP4)");
    }

    println!("\n=== ESTIMATED MEMORY FOOTPRINT ===");
    println!(
        "Model Weights      : {:.2} GB",
        plan.estimated_weight_bytes as f64 / 1e9
    );
    println!(
        "KV Cache Footprint : {:.2} GB",
        plan.estimated_kv_cache_bytes as f64 / 1e9
    );
    println!(
        "Runtime Overhead   : {:.2} MB",
        plan.estimated_overhead_bytes as f64 / 1e6
    );
    println!(
        "Total Estimated RSS: {:.2} GB",
        plan.estimated_peak_rss_bytes as f64 / 1e9
    );

    println!("\n=== RECOMMENDED CONFIGURATION ===");
    println!("Quantization Variant: {}", plan.recommended_quant);
    println!("Context Length     : {}", plan.recommended_context);
    println!("CPU Threads        : {}", plan.recommended_threads);
    println!("GPU Offload Layers : {}", plan.gpu_layers_offloaded);
    println!("Recommended Flags  : {}", plan.recommended_flags.join(" "));

    println!("\n=== PREDICTED PERFORMANCE ===");
    println!("Est. TTFT Latency  : {:.2} ms", plan.predicted_ttft_ms);
    println!(
        "Est. Decode Speed  : {:.2} tok/s",
        plan.predicted_decode_tok_per_sec
    );
}
