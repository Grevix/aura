pub fn execute_ollama_list() {
    println!("AURA Ollama Inventory");
    println!("=====================");

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build();

    if let Ok(cli) = client {
        if let Ok(resp) = cli.get("http://localhost:11434/api/tags").send() {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<serde_json::Value>() {
                    if let Some(models) = json.get("models").and_then(|m| m.as_array()) {
                        println!("{:<25} {:<15} {:<15}", "NAME", "SIZE", "MODIFIED");
                        println!("{:-<58}", "");
                        for m in models {
                            let name = m.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
                            let size = m.get("size").and_then(|s| s.as_u64()).unwrap_or(0);
                            let size_gb = format!("{:.2} GB", size as f64 / 1e9);
                            let modif = m
                                .get("modified_at")
                                .and_then(|t| t.as_str())
                                .unwrap_or("")
                                .chars()
                                .take(10)
                                .collect::<String>();
                            println!("{:<25} {:<15} {:<15}", name, size_gb, modif);
                        }
                        println!("{:-<58}", "");
                        println!("Total models discovered: {}", models.len());
                        return;
                    }
                }
            }
        }
    }

    println!("Ollama REST API not responding on http://localhost:11434. Is Ollama running?");
}
