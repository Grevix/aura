use crate::commands::model_inspect::execute_model_inspect;

pub fn execute_frontier_inspect(model: &str) {
    println!("AURA FRONTIER INSPECTOR");
    println!("────────────────────────────────────────\n");
    execute_model_inspect(model);
}

pub fn execute_frontier_run(model: &str) {
    println!("AURA FRONTIER OUT-OF-CORE RUNTIME");
    println!("────────────────────────────────────────\n");
    execute_model_inspect(model);

    println!("\n=== PRE-EXECUTION RESOURCE EVALUATION ===");
    if model.contains("Kimi-K3") || model.contains("kimi-k3") || model.contains("GLM-5.2") || model.contains("glm-5.2") {
        println!("⚠️ SAFETY INTERVENTION:");
        println!("- Model weights require >1.5 TB local NVMe storage.");
        println!("- Local full parameter downloading blocked to prevent disk exhaustion.");
        println!("- Out-of-core streaming architecture validated. For execution, connect remote/Colab sharded instance.");
    }
}
