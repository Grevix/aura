use aura_backends::{BackendAdapter, LlamaCppAdapter};
use aura_benchmark::{generate_benchmark_report, reproduce_benchmark};
use aura_hardware::probe_hardware;
use aura_memory::enforce_memory_budget;
use aura_model::manifest::load_manifest;
use aura_planner::generate_execution_plan;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn execute_benchmark(model_path: Option<&str>, reproduce_file: Option<&str>, out_path: &str) {
    if let Some(reproduce) = reproduce_file {
        println!("🔄 Reproducing Benchmark Run from JSON: {}...\n", reproduce);
        let path = Path::new(reproduce);
        match reproduce_benchmark(path) {
            Ok(report) => {
                println!("✅ Reproduction Verified for Run ID: {}", report.run_id);
                println!("Model          : {}", report.model.name);
                println!(
                    "Reported Decode: {:.2} tok/s",
                    report.phases.cold_start.decode_tok_per_sec
                );
                println!("Enforcement    : {}", report.plan.enforcement_mechanism);
                println!("Provenance     : {}", report.provenance);
                println!("Simulated      : {}", report.is_simulated);
            }
            Err(e) => {
                eprintln!("❌ Reproduction failed: {}", e);
            }
        }
        return;
    }

    let model = match model_path {
        Some(m) => m,
        None => {
            eprintln!("❌ Error: --model or --reproduce parameter required for aura benchmark");
            return;
        }
    };

    println!("📊 Executing AURA Benchmark Suite on Model: {}...\n", model);

    let hw = probe_hardware();
    let path = Path::new(model);

    let manifest = match load_manifest(path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Failed to load model manifest: {}", e);
            return;
        }
    };

    let plan = generate_execution_plan(&hw, &manifest, 4_000_000_000, None);

    let pid = std::process::id();
    let _ = enforce_memory_budget(pid, plan.memory_budget_bytes, &plan.enforcement_mechanism);

    let adapter = LlamaCppAdapter::new();
    let prompt = "Explain virtual memory versus physical RSS to a beginner.";

    let output = match adapter.execute(model, prompt, &plan) {
        Ok(out) => out,
        Err(e) => {
            eprintln!("❌ Benchmark execution failed: {}", e);
            return;
        }
    };

    let report = generate_benchmark_report(hw, manifest, plan, &output);

    let json_str = serde_json::to_string_pretty(&report).unwrap();
    if let Ok(mut f) = File::create(out_path) {
        let _ = f.write_all(json_str.as_bytes());
        println!("✅ Benchmark Report Saved to: {}", out_path);
    }

    println!("\n=== BENCHMARK SUMMARY ===");
    println!("Run ID         : {}", report.run_id);
    println!(
        "TTFT (Cold)    : {:.2} ms",
        report.phases.cold_start.ttft_ms
    );
    println!(
        "Prefill Speed  : {:.2} tok/s",
        report.phases.cold_start.prompt_tok_per_sec
    );
    println!(
        "Decode tok/s   : {:.2} tok/s",
        report.phases.cold_start.decode_tok_per_sec
    );
    println!(
        "Peak RSS       : {:.2} GB",
        report.phases.cold_start.peak_rss_bytes as f64 / 1e9
    );
    println!("Enforcement    : {}", report.plan.enforcement_mechanism);
    println!("Provenance     : {}", report.provenance);
    println!("Simulated      : {}", report.is_simulated);
}
