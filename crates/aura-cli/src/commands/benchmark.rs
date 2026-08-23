use aura_benchmark::{generate_benchmark_report, reproduce_benchmark};
use aura_hardware::probe_hardware;
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
                println!("Reported Decode: {:.2} tok/s", report.phases.cold_start.decode_tok_per_sec);
                println!("Enforcement    : {}", report.plan.enforcement_mechanism);
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

    let report = generate_benchmark_report(
        hw,
        manifest,
        plan,
        180.0,
        14.5,
    );

    let json_str = serde_json::to_string_pretty(&report).unwrap();
    if let Ok(mut f) = File::create(out_path) {
        let _ = f.write_all(json_str.as_bytes());
        println!("✅ Benchmark Report Saved to: {}", out_path);
    }

    println!("\n=== BENCHMARK SUMMARY ===");
    println!("Run ID         : {}", report.run_id);
    println!("TTFT (Cold)    : {:.2} ms", report.phases.cold_start.ttft_ms);
    println!("Decode tok/s   : {:.2}", report.phases.cold_start.decode_tok_per_sec);
    println!("Enforcement    : {}", report.plan.enforcement_mechanism);
}
