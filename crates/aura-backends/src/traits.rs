use aura_core::errors::Result;
use aura_core::types::{ExecutionPlan, FeatureStatus, MetricProvenance, SpeculativeStatus};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackendOutput {
    pub generated_text: String,
    pub ttft_ms: f64,
    pub prompt_tok_per_sec: f64,
    pub decode_tok_per_sec: f64,
    pub peak_rss_bytes: u64,
    pub tokens_prompt: usize,
    pub tokens_predicted: usize,
    /// true when llama-cli/llama-server was not found and AURA returned
    /// a synthetic planning response instead of real model inference.
    /// Any benchmark that observes is_simulated=true MUST discard the
    /// performance numbers — they reflect planning latency, not inference.
    pub is_simulated: bool,
    pub provenance: MetricProvenance,
    pub backend_name: String,
    pub speculative_status: SpeculativeStatus,
    pub fa2_status: FeatureStatus,
    pub prefetch_hits: u64,
    pub prefetch_misses: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

pub trait BackendAdapter {
    fn name(&self) -> &str;
    fn execute(
        &self,
        model_path: &str,
        prompt: &str,
        plan: &ExecutionPlan,
    ) -> Result<BackendOutput>;
}
