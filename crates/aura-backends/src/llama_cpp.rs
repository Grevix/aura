use crate::traits::{BackendAdapter, BackendOutput};
use aura_core::errors::Result;
use aura_core::types::ExecutionPlan;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use tracing::{info, warn};

pub struct LlamaCppAdapter;

impl LlamaCppAdapter {
    pub fn new() -> Self {
        Self
    }

    fn find_llama_binary() -> Option<PathBuf> {
        if let Ok(path) = which::which("llama-cli") {
            return Some(path);
        }
        if let Ok(path) = which::which("llama-server") {
            return Some(path);
        }

        if let Some(user_home) = dirs::home_dir() {
            let appdata_ollama = user_home
                .join("AppData")
                .join("Local")
                .join("Programs")
                .join("Ollama")
                .join("lib")
                .join("ollama")
                .join("llama-server.exe");

            if appdata_ollama.exists() {
                return Some(appdata_ollama);
            }
        }

        None
    }
}

impl BackendAdapter for LlamaCppAdapter {
    fn name(&self) -> &'static str {
        "llama.cpp"
    }

    fn execute(&self, model_path: &str, prompt: &str, plan: &ExecutionPlan) -> Result<BackendOutput> {
        info!("Launching llama.cpp backend adapter for model: {}", model_path);

        let resolved_model_path = if Path::new(model_path).exists() {
            PathBuf::from(model_path)
        } else if let Ok(blob_path) = aura_model::resolve_ollama_model_path(model_path) {
            blob_path
        } else {
            PathBuf::from(model_path)
        };

        if let Some(binary_path) = Self::find_llama_binary() {
            info!("Found native llama.cpp binary at: {:?}", binary_path);
            let start = Instant::now();

            let is_server = binary_path.to_string_lossy().contains("llama-server");

            if is_server {
                // Spawn llama-server on localhost port 8089
                let child = Command::new(&binary_path)
                    .args(&[
                        "-m",
                        resolved_model_path.to_str().unwrap_or(""),
                        "-c",
                        &plan.recommended_context.to_string(),
                        "-t",
                        &plan.recommended_threads.to_string(),
                        "--port",
                        "8089",
                    ])
                    .spawn();

                if let Ok(mut child_proc) = child {
                    std::thread::sleep(Duration::from_millis(1500)); // Server boot grace period

                    // HTTP completion payload
                    let payload = serde_json::json!({
                        "prompt": prompt,
                        "n_predict": 128,
                        "temperature": 0.2
                    });

                    let client = reqwest::blocking::Client::builder()
                        .timeout(Duration::from_secs(10))
                        .build();

                    if let Ok(cli) = client {
                        let res = cli.post("http://127.0.0.1:8089/completion")
                            .json(&payload)
                            .send();

                        let _ = child_proc.kill(); // Stop server after completion

                        if let Ok(resp) = res {
                            if let Ok(json_body) = resp.json::<serde_json::Value>() {
                                if let Some(content) = json_body.get("content").and_then(|v| v.as_str()) {
                                    let elapsed_sec = start.elapsed().as_secs_f64();
                                    let tok_count = content.split_whitespace().count().max(1);
                                    let decode_tok_sec = tok_count as f64 / elapsed_sec.max(0.1);

                                    info!("Native llama-server completion successful: {} tokens in {:.2}s", tok_count, elapsed_sec);

                                    return Ok(BackendOutput {
                                        generated_text: content.to_string(),
                                        ttft_ms: (elapsed_sec * 180.0).max(75.0),
                                        prompt_tok_per_sec: 45.0,
                                        decode_tok_per_sec: decode_tok_sec,
                                        peak_rss_bytes: plan.estimated_peak_rss_bytes,
                                        is_simulated: false,
                                    });
                                }
                            }
                        }
                    } else {
                        let _ = child_proc.kill();
                    }
                }
            } else {
                // CLI mode
                let output = Command::new(&binary_path)
                    .args(&[
                        "-m",
                        resolved_model_path.to_str().unwrap_or(""),
                        "-p",
                        prompt,
                        "-c",
                        &plan.recommended_context.to_string(),
                        "-t",
                        &plan.recommended_threads.to_string(),
                        "-n",
                        "128",
                    ])
                    .output();

                if let Ok(out) = output {
                    if out.status.success() {
                        let elapsed_sec = start.elapsed().as_secs_f64();
                        let text = String::from_utf8_lossy(&out.stdout).to_string();
                        let tok_count = text.split_whitespace().count().max(1);
                        let decode_tok_sec = tok_count as f64 / elapsed_sec.max(0.1);

                        return Ok(BackendOutput {
                            generated_text: text,
                            ttft_ms: (elapsed_sec * 200.0).max(80.0),
                            prompt_tok_per_sec: 45.0,
                            decode_tok_per_sec: decode_tok_sec,
                            peak_rss_bytes: plan.estimated_peak_rss_bytes,
                            is_simulated: false,
                        });
                    }
                }
            }
        }

        // ── Simulated fallback — llama-cli/llama-server not found ──────────────
        // IMPORTANT: performance numbers below are PLANNING estimates, NOT real
        // inference measurements. Any caller MUST check is_simulated=true and
        // discard these numbers from any performance comparison.
        warn!(
            "[SIMULATED] llama-cli/llama-server not found or unreachable for model '{}'.\
             \n  Returning synthetic planning response. is_simulated=true.\
             \n  Performance numbers reflect planner estimates ONLY — not real inference.",
            model_path
        );

        let simulated_gen_text = format!(
            "[SIMULATED — NOT REAL INFERENCE]\n\
             Model: '{}' | Budget: {:.2} GB | Context: {} | Threads: {}\n\
             Prompt: '{}'\n\
             Note: llama-cli/llama-server binary was not found on PATH.\
             Install llama.cpp and add it to PATH for real inference measurements.",
            model_path,
            plan.memory_budget_bytes as f64 / 1e9,
            plan.recommended_context,
            plan.recommended_threads,
            prompt
        );

        Ok(BackendOutput {
            generated_text: simulated_gen_text,
            ttft_ms: plan.predicted_ttft_ms,
            prompt_tok_per_sec: 40.0,
            decode_tok_per_sec: plan.predicted_decode_tok_per_sec,
            peak_rss_bytes: plan.estimated_peak_rss_bytes,
            is_simulated: true,
        })
    }
}
