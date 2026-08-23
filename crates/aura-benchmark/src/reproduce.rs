use crate::schema::BenchmarkReport;
use aura_core::errors::{AuraError, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tracing::info;

pub fn reproduce_benchmark(report_path: &Path) -> Result<BenchmarkReport> {
    info!(
        "Loading benchmark report for reproduction: {:?}",
        report_path
    );
    let mut file = File::open(report_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let report: BenchmarkReport = serde_json::from_str(&contents)
        .map_err(|e| AuraError::BenchmarkError(format!("Failed to parse report: {}", e)))?;

    info!("Reproducing run ID: {}", report.run_id);
    info!("Target Model: {}", report.model.name);
    info!(
        "Original Decode Speed: {:.2} tok/s",
        report.phases.cold_start.decode_tok_per_sec
    );

    Ok(report)
}
