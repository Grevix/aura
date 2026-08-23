use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuraError {
    #[error("Hardware detection error: {0}")]
    HardwareError(String),

    #[error("Model parsing error: {0}")]
    ModelError(String),

    #[error("Planning error: {0}")]
    PlannerError(String),

    #[error("Memory budget enforcement error: {0}")]
    MemoryError(String),

    #[error("Backend execution error: {0}")]
    BackendError(String),

    #[error("Benchmark error: {0}")]
    BenchmarkError(String),

    #[error("Audit error: {0}")]
    AuditError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AuraError>;
