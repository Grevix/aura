use crate::traits::{BackendAdapter, BackendOutput};
use aura_core::errors::Result;
use aura_core::types::{ExecutionPlan, FeatureStatus, MetricProvenance, SpeculativeStatus};
use aura_hardware::gpu::detect_gpu;
use aura_hardware::memory::get_process_page_faults;
use aura_memory::enforce_memory_budget;
use std::env;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};
use tracing::{info, warn};

struct ProcessGuard(Option<Child>);

impl Drop for ProcessGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[derive(Default)]
pub struct LlamaCppAdapter;

impl LlamaCppAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn find_llama_binary() -> Option<PathBuf> {
        // 1. Explicit environment variable override
        if let Ok(env_path) = env::var("AURA_LLAMA_SERVER_PATH") {
            let p = PathBuf::from(env_path);
            if p.exists() {
                return Some(p);
            }
        }

        // 2. PATH resolution for llama-cli / llama-server
        if let Ok(path) = which::which("llama-cli") {
            return Some(path);
        }
        if let Ok(path) = which::which("llama-server") {
            return Some(path);
        }

        // 3. Platform-specific default installation paths
        if let Some(user_home) = dirs::home_dir() {
            // Windows Ollama bundled library directory
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

            // Linux standard Ollama paths
            let linux_ollama = PathBuf::from("/usr/share/ollama/lib/ollama/llama-server");
            if linux_ollama.exists() {
                return Some(linux_ollama);
            }

            // macOS standard Ollama paths
            let macos_ollama =
                PathBuf::from("/Applications/Ollama.app/Contents/Resources/llama-server");
            if macos_ollama.exists() {
                return Some(macos_ollama);
            }
        }

        None
    }

    fn find_free_port() -> u16 {
        if let Ok(listener) = TcpListener::bind("127.0.0.1:0") {
            if let Ok(addr) = listener.local_addr() {
                return addr.port();
            }
        }
        8089
    }
}

impl BackendAdapter for LlamaCppAdapter {
    fn name(&self) -> &'static str {
        "llama.cpp"
    }

    fn execute(
        &self,
        model_path: &str,
        prompt: &str,
        plan: &ExecutionPlan,
    ) -> Result<BackendOutput> {
        info!(
            "Launching llama.cpp backend adapter for model: {}",
            model_path
        );

        let resolved_model_path = if Path::new(model_path).exists() {
            PathBuf::from(model_path)
        } else if let Ok(blob_path) = aura_model::resolve_ollama_model_path(model_path) {
            blob_path
        } else {
            PathBuf::from(model_path)
        };

        if let Some(binary_path) = Self::find_llama_binary() {
            info!("Found native llama.cpp binary at: {:?}", binary_path);

            let binary_dir = binary_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();

            let is_server = binary_path
                .file_name()
                .map(|s| s.to_string_lossy().contains("llama-server"))
                .unwrap_or(false);

            if is_server {
                let port = Self::find_free_port();
                let port_str = port.to_string();

                // Check CUDA library paths in binary_dir
                let cuda_v13 = binary_dir.join("cuda_v13");
                let cuda_v12 = binary_dir.join("cuda_v12");

                let current_path = env::var("PATH").unwrap_or_default();
                let mut new_path = format!("{};{}", binary_dir.display(), current_path);

                if cuda_v13.exists() {
                    new_path = format!("{};{}", cuda_v13.display(), new_path);
                } else if cuda_v12.exists() {
                    new_path = format!("{};{}", cuda_v12.display(), new_path);
                }

                let gpu_prof = detect_gpu();

                let mut cmd = Command::new(&binary_path);
                cmd.current_dir(&binary_dir).env("PATH", &new_path);

                let mut args = vec![
                    "-m".to_string(),
                    resolved_model_path.to_string_lossy().to_string(),
                    "-c".to_string(),
                    plan.recommended_context.to_string(),
                    "-t".to_string(),
                    plan.recommended_threads.to_string(),
                    "--port".to_string(),
                    port_str.clone(),
                    "--host".to_string(),
                    "127.0.0.1".to_string(),
                ];

                if gpu_prof.present && plan.gpu_layers_offloaded > 0 {
                    args.push("-ngl".to_string());
                    args.push(plan.gpu_layers_offloaded.to_string());
                }

                cmd.args(args);

                match cmd.spawn() {
                    Ok(child_proc) => {
                        let child_pid = child_proc.id();
                        let mut _guard = ProcessGuard(Some(child_proc));

                        info!(
                            "Spawned llama-server PID={} on port {} with budget {} bytes",
                            child_pid, port, plan.memory_budget_bytes
                        );

                        if let Err(e) = enforce_memory_budget(
                            child_pid,
                            plan.memory_budget_bytes,
                            &plan.enforcement_mechanism,
                        ) {
                            warn!("Failed to apply memory enforcement on child PID: {}", e);
                        }

                        let health_url = format!("http://127.0.0.1:{}/health", port);
                        let client = reqwest::blocking::Client::builder()
                            .timeout(Duration::from_secs(120))
                            .build();

                        if let Ok(cli) = client {
                            let mut server_ready = false;
                            let health_start = Instant::now();

                            while health_start.elapsed() < Duration::from_secs(45) {
                                if let Ok(resp) = cli.get(&health_url).send() {
                                    if resp.status().is_success() {
                                        server_ready = true;
                                        break;
                                    }
                                }
                                std::thread::sleep(Duration::from_millis(150));
                            }

                            if server_ready {
                                info!(
                                    "llama-server PID={} ready after {:.2}s",
                                    child_pid,
                                    health_start.elapsed().as_secs_f64()
                                );

                                let payload = serde_json::json!({
                                    "prompt": prompt,
                                    "n_predict": 128,
                                    "temperature": 0.2
                                });

                                let completion_url =
                                    format!("http://127.0.0.1:{}/completion", port);
                                let req_start = Instant::now();

                                if let Ok(comp_resp) =
                                    cli.post(&completion_url).json(&payload).send()
                                {
                                    let wall_time_ms = req_start.elapsed().as_secs_f64() * 1000.0;
                                    if comp_resp.status().is_success() {
                                        if let Ok(json_body) = comp_resp.json::<serde_json::Value>()
                                        {
                                            let generated_text = json_body["content"]
                                                .as_str()
                                                .unwrap_or("")
                                                .to_string();

                                            let timings = &json_body["timings"];
                                            let prompt_n =
                                                timings["prompt_n"].as_u64().unwrap_or(1) as usize;
                                            let predicted_n = timings["predicted_n"]
                                                .as_u64()
                                                .unwrap_or(0)
                                                as usize;

                                            let prompt_ms =
                                                timings["prompt_ms"].as_f64().unwrap_or(1.0);
                                            let predicted_ms = timings["predicted_ms"]
                                                .as_f64()
                                                .unwrap_or(wall_time_ms);

                                            let ttft_ms = prompt_ms;
                                            let prompt_tok_per_sec =
                                                if prompt_ms > 0.0 {
                                                    (prompt_n as f64 / prompt_ms) * 1000.0
                                                } else {
                                                    0.0
                                                };
                                            let decode_tok_per_sec =
                                                if predicted_ms > 0.0 {
                                                    (predicted_n as f64 / predicted_ms) * 1000.0
                                                } else {
                                                    0.0
                                                };

                                            info!(
                                                "Real inference successful: {} tokens generated at {:.2} tok/s, TTFT={:.1}ms, RSS={:.2}GB",
                                                predicted_n,
                                                decode_tok_per_sec,
                                                ttft_ms,
                                                plan.estimated_peak_rss_bytes as f64 / 1e9
                                            );

                                            return Ok(BackendOutput {
                                                generated_text,
                                                ttft_ms,
                                                prompt_tok_per_sec,
                                                decode_tok_per_sec,
                                                peak_rss_bytes: plan.estimated_peak_rss_bytes,
                                                tokens_prompt: prompt_n,
                                                tokens_predicted: predicted_n,
                                                is_simulated: false,
                                                provenance: MetricProvenance::AuraMeasured,
                                                backend_name: "llama-server".to_string(),
                                                speculative_status: plan.speculative_status.clone(),
                                                fa2_status: plan.fa2_status.clone(),
                                                prefetch_hits: 0,
                                                prefetch_misses: 0,
                                                cache_hits: 0,
                                                cache_misses: 0,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to spawn child process llama-server: {}", e);
                    }
                }
            }
        }

        // Fallback simulation mode
        let simulated_output = format!(
            "[SIMULATED - REAL INFERENCE BACKEND UNREACHABLE]\n\n\
            Model: {}\n\
            Prompt: {}\n\
            Memory Budget: {} MB\n\
            Estimated RSS: {:.2} GB",
            model_path,
            prompt,
            plan.memory_budget_bytes / 1_000_000,
            plan.estimated_peak_rss_bytes as f64 / 1e9
        );

        let current_faults = get_process_page_faults(std::process::id());

        Ok(BackendOutput {
            generated_text: simulated_output,
            ttft_ms: plan.predicted_ttft_ms,
            prompt_tok_per_sec: 40.0,
            decode_tok_per_sec: plan.predicted_decode_tok_per_sec,
            peak_rss_bytes: plan.estimated_peak_rss_bytes,
            tokens_prompt: 16,
            tokens_predicted: 64,
            is_simulated: true,
            provenance: MetricProvenance::Simulated,
            backend_name: "synthetic_fallback".to_string(),
            speculative_status: SpeculativeStatus::Disabled,
            fa2_status: FeatureStatus::Disabled,
            prefetch_hits: 0,
            prefetch_misses: current_faults.0,
            cache_hits: 0,
            cache_misses: 0,
        })
    }
}
