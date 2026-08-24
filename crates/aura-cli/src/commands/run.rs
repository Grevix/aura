use aura_backends::{BackendAdapter, LlamaCppAdapter};
use aura_hardware::probe_hardware;
use aura_memory::enforce_memory_budget;
use aura_model::manifest::load_manifest;
use aura_planner::generate_execution_plan_with_options;
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

pub fn execute_run(model_path: &str, memory: &str, draft_model_path: Option<&str>, prompt: &str) {
    println!("🚀 Launching AURA Budget-Enforced Execution Engine...\n");

    let hw = probe_hardware();
    let path = Path::new(model_path);

    let manifest = match load_manifest(path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Failed to load model: {}", e);
            return;
        }
    };

    let draft_info = if let Some(dm_path) = draft_model_path {
        match load_manifest(Path::new(dm_path)) {
            Ok(dm) => Some((dm, 2.2)), // estimated 2.2x speedup if feasible
            Err(e) => {
                warn_log(&format!("Could not load draft model manifest: {}", e));
                None
            }
        }
    } else {
        None
    };

    let budget_bytes = parse_memory_budget(memory);
    let draft_ref = draft_info.as_ref().map(|(dm, speedup)| (dm, *speedup));
    let plan = generate_execution_plan_with_options(&hw, &manifest, budget_bytes, None, draft_ref);

    if !plan.is_feasible {
        eprintln!("⚠️ WARNING: Execution Plan exceeds requested budget!");
        eprintln!("Details: {}", plan.feasibility_notes);
    }

    let pid = std::process::id();
    if let Err(e) =
        enforce_memory_budget(pid, plan.memory_budget_bytes, &plan.enforcement_mechanism)
    {
        eprintln!("⚠️ Memory enforcement warning: {}", e);
    }

    let adapter = LlamaCppAdapter::new();
    match adapter.execute(model_path, prompt, &plan) {
        Ok(output) => {
            println!("=== GENERATION OUTPUT ===");
            println!("{}", output.generated_text);

            println!("\n=== RUN METRICS & TELEMETRY ===");
            println!("TTFT Latency   : {:.2} ms", output.ttft_ms);
            println!("Prefill Speed  : {:.2} tok/s", output.prompt_tok_per_sec);
            println!("Decode Speed   : {:.2} tok/s", output.decode_tok_per_sec);
            println!(
                "Peak RSS       : {:.2} GB",
                output.peak_rss_bytes as f64 / 1e9
            );
            println!("Backend        : {}", output.backend_name);
            println!("Provenance     : {}", output.provenance);
            println!("Simulated      : {}", output.is_simulated);
            println!("Enforcement    : {}", plan.enforcement_mechanism);
            println!("Speculative    : {}", output.speculative_status);
            println!("FA2 Status     : {}", output.fa2_status);
            println!(
                "Prefetch Hits  : {} / Misses: {}",
                output.prefetch_hits, output.prefetch_misses
            );
            println!(
                "Expert Cache   : Hits: {} / Misses: {}",
                output.cache_hits, output.cache_misses
            );
        }
        Err(e) => {
            eprintln!("❌ Execution failed: {}", e);
        }
    }
}

fn warn_log(msg: &str) {
    eprintln!("⚠️ {}", msg);
}
