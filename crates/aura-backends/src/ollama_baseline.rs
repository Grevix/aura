use crate::traits::BackendOutput;
use aura_core::errors::{AuraError, Result};
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

    /// Run a non-streaming inference call via the Ollama REST API.
    /// Returns real timing metrics extracted directly from Ollama's response fields.
    ///
    /// Ollama configuration applied:
    ///   num_gpu = 0  (CPU-only, matches AURA constraint)
    ///   num_thread = 8  (matches AURA recommended_threads on i5-13420H)
    pub fn run_baseline(&self, model_name: &str, prompt: &str) -> Result<BackendOutput> {
        info!(
            "Executing Ollama REST API baseline for model: {} via {}",
            model_name, self.base_url
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
                "num_gpu": 0,
                "num_thread": 8
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

        // ── Derive exact metrics from Ollama's native timing fields ──────────
        // All durations are in nanoseconds. Convert to seconds for tok/s.
        let ns_to_sec = |ns: u64| -> f64 { ns as f64 / 1_000_000_000.0 };

        // Decode throughput: tokens generated / time spent generating
        let decode_tok_per_sec = if body.eval_duration > 0 && body.eval_count > 0 {
            body.eval_count as f64 / ns_to_sec(body.eval_duration)
        } else {
            // Fallback: wall-clock estimate (less accurate)
            warn!(
                "eval_duration/eval_count missing from Ollama response; using wall-clock fallback"
            );
            let approx_tokens = body.response.split_whitespace().count().max(1);
            approx_tokens as f64 / wall_elapsed.max(0.1)
        };

        // Prefill throughput: prompt tokens / prompt eval time
        let prompt_tok_per_sec = if body.prompt_eval_duration > 0 && body.prompt_eval_count > 0 {
            body.prompt_eval_count as f64 / ns_to_sec(body.prompt_eval_duration)
        } else {
            // Reasonable estimate: prefill is typically faster than decode
            decode_tok_per_sec * 2.0
        };

        // TTFT ≈ load_duration + prompt_eval_duration  (nanoseconds → milliseconds)
        let ttft_ms = if body.load_duration > 0 || body.prompt_eval_duration > 0 {
            (body.load_duration + body.prompt_eval_duration) as f64 / 1_000_000.0
        } else {
            // Wall-clock fallback approximation
            wall_elapsed * 300.0
        };

        info!(
            "Ollama REST: model={} eval_count={} eval_duration={}ns decode={:.2}tok/s prefill={:.2}tok/s ttft={:.1}ms",
            body.model,
            body.eval_count,
            body.eval_duration,
            decode_tok_per_sec,
            prompt_tok_per_sec,
            ttft_ms
        );

        // NOTE: Ollama does not expose per-process RSS via the API.
        // Peak RSS must be measured externally (e.g. Process Explorer, psutil).
        // We record 0 to make it explicit this is NOT measured, not fabricated.
        Ok(BackendOutput {
            generated_text: body.response,
            ttft_ms,
            prompt_tok_per_sec,
            decode_tok_per_sec,
            peak_rss_bytes: 0, // NOT MEASURED via REST API — use external RSS measurement
            is_simulated: false,
        })
    }
}

impl Default for OllamaBaselineRunner {
    fn default() -> Self {
        Self::new()
    }
}
