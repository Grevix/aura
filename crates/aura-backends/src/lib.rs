pub mod llama_cpp;
pub mod ollama_baseline;
pub mod traits;

pub use llama_cpp::LlamaCppAdapter;
pub use ollama_baseline::OllamaBaselineRunner;
pub use traits::{BackendAdapter, BackendOutput};
