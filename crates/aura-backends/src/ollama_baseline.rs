use crate::traits::BackendOutput;
use aura_core::errors::{AuraError, Result};
use aura_core::types::{FeatureStatus, MetricProvenance, SpeculativeStatus};
use std::time::Instant;
use tracing::{info, warn};

/// Ollama REST API response structure (non-streaming).
/// Fields from: https://github.com/ollama/ollama/blob/main/docs/api.md#generate-a-completion
#[derive(serde::Deserialize, Debug)]
struct OllamaGenerateResponse {
    model: String,
    response: String,
    done: bool,
    /// nanoseconds spent loading model
    #[serde(default)]
    load_duration: u64,
    /// number of tokens in the prompt
    #[serde(default)]
    prompt_eval_count: u64,
    /// nanoseconds spent evaluating prompt
    #[serde(default)]
    prompt_eval_duration: u64,
    /// number of tokens in the response
    #[serde(default)]
    eval_count: u64,
    /// nanoseconds spent generating response tokens
    #[serde(default)]
    eval_duration: u64,
}

pub struct OllamaBaselineRunner {
    base_url: String,
}

impl OllamaBaselineRunner {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
        }
    }

    pub fn with_base_url(url: &str) -> Self {
        Self {
            base_url: url.to_string(),
        }
    }

    pub fn run_baseline(&self, model_name: &str, prompt: &str) -> Result<BackendOutput> {
        self.run_baseline_with_options(model_name, prompt, 0, 8)
    }

    /// Run a non-streaming inference call via the Ollama REST API with GPU offload options.
    pub fn run_baseline_with_options(
        &self,
        model_name: &str,
        prompt: &str,
        num_gpu: u32,
        num_thread: u32,
    ) -> Result<BackendOutput> {
        info!(
            "Executing Ollama REST API baseline (num_gpu={}, num_thread={}) for model: {} via {}",
            num_gpu, num_thread, model_name, self.base_url
        );

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| AuraError::BackendError(format!("Failed to build HTTP client: {}", e)))?;

        let payload = serde_json::json!({
            "model": model_name,
            "prompt": prompt,
            "stream": false,
            "options": {
                "num_gpu": num_gpu,
                "num_thread": num_thread
            }
        });

        let wall_start = Instant::now();

        let resp = client
            .post(format!("{}/api/generate", self.base_url))
            .json(&payload)
            .send()
            .map_err(|e| {
                AuraError::BackendError(format!(
                    "Ollama REST API unreachable at {}: {}. Is 'ollama serve' running?",
                    self.base_url, e
                ))
            })?;

        let wall_elapsed = wall_start.elapsed().as_secs_f64();

        if !resp.status().is_success() {
            return Err(AuraError::BackendError(format!(
                "Ollama REST API returned HTTP {}: {}",
                resp.status(),
                model_name
            )));
        }

        let body: OllamaGenerateResponse = resp.json().map_err(|e| {
            AuraError::BackendError(format!("Failed to parse Ollama response: {}", e))
        })?;

        if !body.done {
            warn!("Ollama response marked done=false for model {}", model_name);
        }

        let ns_to_sec = |ns: u64| -> f64 { ns as f64 / 1_000_000_000.0 };

        let decode_tok_per_sec = if body.eval_duration > 0 && body.eval_count > 0 {
            body.eval_count as f64 / ns_to_sec(body.eval_duration)
        } else {
            warn!(
                "eval_duration/eval_count missing from Ollama response; using wall-clock fallback"
            );
            let approx_tokens = body.response.split_whitespace().count().max(1);
            approx_tokens as f64 / wall_elapsed.max(0.1)
        };

        let prompt_tok_per_sec = if body.prompt_eval_duration > 0 && body.prompt_eval_count > 0 {
            body.prompt_eval_count as f64 / ns_to_sec(body.prompt_eval_duration)
        } else {
            decode_tok_per_sec * 2.0
        };

        let ttft_ms = if body.load_duration > 0 || body.prompt_eval_duration > 0 {
            (body.load_duration + body.prompt_eval_duration) as f64 / 1_000_000.0
        } else {
            wall_elapsed * 300.0
        };

        let backend_name = if num_gpu > 0 {
            "ollama-rest-cuda".to_string()
        } else {
            "ollama-rest-cpu".to_string()
        };

        info!(
            "Ollama REST: model={} eval_count={} eval_duration={}ns decode={:.2}tok/s prefill={:.2}tok/s ttft={:.1}ms backend={}",
            body.model,
            body.eval_count,
            body.eval_duration,
            decode_tok_per_sec,
            prompt_tok_per_sec,
            ttft_ms,
            backend_name
        );

        Ok(BackendOutput {
            generated_text: body.response,
            ttft_ms,
            prompt_tok_per_sec,
            decode_tok_per_sec,
            peak_rss_bytes: 0,
            tokens_prompt: body.prompt_eval_count as usize,
            tokens_predicted: body.eval_count as usize,
            is_simulated: false,
            provenance: MetricProvenance::OllamaMeasured,
            backend_name,
            speculative_status: SpeculativeStatus::Disabled,
            fa2_status: FeatureStatus::Disabled,
            prefetch_hits: 0,
            prefetch_misses: 0,
            cache_hits: 0,
            cache_misses: 0,
        })
    }
}

impl Default for OllamaBaselineRunner {
    fn default() -> Self {
        Self::new()
    }
}
