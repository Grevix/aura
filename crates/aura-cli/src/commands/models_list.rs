pub fn execute_models_list() {
    println!("AURA UNIFIED MODEL DISCOVERY REGISTRY");
    println!("=====================================");

    println!("{:<12} {:<28} {:<12} {:<20}", "SOURCE", "MODEL NAME / IDENTIFIER", "SIZE", "AURA EXECUTION MODE");
    println!("{:-<76}", "");

    // 1. Ollama Discovered Models
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build();

    if let Ok(cli) = client {
        if let Ok(resp) = cli.get("http://localhost:11434/api/tags").send() {
            if let Ok(json) = resp.json::<serde_json::Value>() {
                if let Some(models) = json.get("models").and_then(|m| m.as_array()) {
                    for m in models {
                        let name = m.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
                        let size = m.get("size").and_then(|s| s.as_u64()).unwrap_or(0);
                        let size_gb = format!("{:.2} GB", size as f64 / 1e9);
                        let mode = if name.contains("0.6b") || name.contains("1b") || name.contains("1.7b") || name.contains("3b") {
                            "LOCAL_GPU (Full Resident)"
                        } else if name.contains("8b") || name.contains("7b") || name.contains("9b") {
                            "GPU_OFFLOAD / CUDA"
                        } else if name.contains("cloud") {
                            "CLOUD_ROUTED (Segregated)"
                        } else {
                            "OUT_OF_CORE / STREAMING"
                        };
                        println!("{:<12} {:<28} {:<12} {:<20}", "Ollama", name, size_gb, mode);
                    }
                }
            }
        }
    }

    // 2. Frontier Model Registry
    println!("{:<12} {:<28} {:<12} {:<20}", "HF Frontier", "moonshotai/Kimi-K3", "1.56 TB", "MOE_EXPERT_STREAMING");
    println!("{:<12} {:<28} {:<12} {:<20}", "HF Frontier", "zai-org/GLM-5.2", "1.51 TB", "OUT_OF_CORE_STREAMING");
    println!("{:<12} {:<28} {:<12} {:<20}", "HF Vision", "Qwen/Qwen3.8-27B", "16.50 GB", "GPU_OFFLOAD + STREAMING");
    println!("{:-<76}", "");
}
