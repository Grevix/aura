use aura_core::errors::{AuraError, Result};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Deserialize)]
struct OllamaLayer {
    #[serde(rename = "mediaType")]
    media_type: String,
    digest: String,
}

#[derive(Debug, Deserialize)]
struct OllamaManifest {
    layers: Vec<OllamaLayer>,
}

pub fn resolve_ollama_model_path(model_name: &str) -> Result<PathBuf> {
    let user_home = dirs::home_dir().ok_or_else(|| {
        AuraError::ModelError("Could not determine user home directory".to_string())
    })?;

    let parts: Vec<&str> = model_name.split(':').collect();
    let (name, tag) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        (model_name, "latest")
    };

    let manifest_path = user_home
        .join(".ollama")
        .join("models")
        .join("manifests")
        .join("registry.ollama.ai")
        .join("library")
        .join(name)
        .join(tag);

    if !manifest_path.exists() {
        return Err(AuraError::ModelError(format!(
            "Ollama model manifest not found at: {:?}",
            manifest_path
        )));
    }

    let mut file = File::open(&manifest_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let manifest: OllamaManifest = serde_json::from_str(&contents)
        .map_err(|e| AuraError::ModelError(format!("Failed to parse Ollama manifest: {}", e)))?;

    for layer in manifest.layers {
        if layer.media_type.contains("model") {
            let hash_clean = layer.digest.replace(':', "-");
            let blob_path = user_home
                .join(".ollama")
                .join("models")
                .join("blobs")
                .join(hash_clean);

            if blob_path.exists() {
                info!("Resolved Ollama model '{}' -> GGUF Blob: {:?}", model_name, blob_path);
                return Ok(blob_path);
            }
        }
    }

    Err(AuraError::ModelError(format!(
        "Could not find model weight layer blob for Ollama model '{}'",
        model_name
    )))
}
