use crate::schema::BenchmarkReport;
use aura_core::errors::{AuraError, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use tracing::info;

pub fn save_telemetry_record(report: &BenchmarkReport, db_path: &Path) -> Result<()> {
    if let Some(parent) = db_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut records: Vec<BenchmarkReport> = if db_path.exists() {
        let mut file = File::open(db_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        Vec::new()
    };

    records.push(report.clone());

    let json_str = serde_json::to_string_pretty(&records)
        .map_err(|e| AuraError::BenchmarkError(e.to_string()))?;

    let mut file = File::create(db_path)?;
    file.write_all(json_str.as_bytes())?;

    info!("Saved benchmark telemetry to local database: {:?}", db_path);
    Ok(())
}

pub fn load_historical_telemetry(db_path: &Path) -> Vec<BenchmarkReport> {
    if db_path.exists() {
        if let Ok(mut file) = File::open(db_path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                return serde_json::from_str(&contents).unwrap_or_default();
            }
        }
    }
    Vec::new()
}
