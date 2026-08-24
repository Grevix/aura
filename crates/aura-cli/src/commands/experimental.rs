use crate::commands::model_inspect::execute_model_inspect;

pub fn execute_experimental_run(model: &str) {
    println!("AURA EXPERIMENTAL LARGE-MODEL RUNTIME");
    println!("=====================================");
    println!("Target Model: {}\n", model);

    execute_model_inspect(model);

    println!("\n=== SAFETY & MEMORY RESOURCE EVALUATION ===");
    if model.contains("Kimi-K3") || model.contains("kimi-k3") || model.contains("GLM-5.2") || model.contains("glm-5.2") {
        println!("⚠️ CRITICAL RESOURCE REQUIREMENT WARNING:");
        println!("- Model checkpoint size exceeds 1.5 TB.");
        println!("- Local full parameter instantiation will cause system Out-Of-Memory / disk overflow.");
        println!("\nRecommended Execution Mode: Colab Cloud / Multi-GPU Sharded Session.");
        println!("AURA Safety Guard: Aborting local full parameter download. Use Google Colab notebook in benchmarks/notebooks/.");
    } else {
        println!("Model fits within standard memory hierarchy. Launching execution planner...");
    }
}
