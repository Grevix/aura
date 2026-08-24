mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "aura",
    author = "AURA Systems Engineering Team",
    version = "0.1.0",
    about = "AURA — Adaptive Ultra-Low-Memory Runtime for AI",
    long_about = "AURA is a hardware-aware, memory-budgeted local inference optimizer and orchestration engine."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Probe physical host hardware, SIMD extensions, RAM, and storage IOPS
    Doctor,

    /// Generate an optimized hardware-aware execution plan for a model under a memory budget
    Plan {
        /// Path to model artifact (GGUF or manifest)
        #[arg(short = 'm', long)]
        model: String,

        /// Memory budget limit (e.g. 4G, 8G, 500M)
        #[arg(short = 'b', long, default_value = "4G")]
        memory: String,

        /// Target context window size
        #[arg(short, long)]
        context: Option<usize>,
    },

    /// Launch budget-enforced model execution engine
    Run {
        /// Path to model artifact (GGUF or manifest)
        #[arg(short = 'm', long)]
        model: String,

        /// Memory budget limit (e.g. 4G, 8G)
        #[arg(short = 'b', long, default_value = "4G")]
        memory: String,

        /// Optional draft model for speculative decoding
        #[arg(short = 'd', long)]
        draft_model: Option<String>,

        /// Prompt string for generation
        #[arg(
            short,
            long,
            default_value = "Explain quantum computing in three sentences."
        )]
        prompt: String,
    },

    /// Execute benchmark suite or reproduce a previous benchmark JSON artifact
    Benchmark {
        /// Path to model artifact
        #[arg(short = 'm', long)]
        model: Option<String>,

        /// Path to historical aura-benchmark.json for reproduction
        #[arg(short, long)]
        reproduce: Option<String>,

        /// Output path for benchmark JSON artifact
        #[arg(short, long, default_value = "aura-benchmark.json")]
        out: String,
    },

    /// Evaluate 10-tier release audit gates and generate audit.json artifact
    Audit {
        /// Output path for audit JSON artifact
        #[arg(short, long, default_value = "audit.json")]
        out: String,
    },
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Doctor => {
            commands::doctor::execute_doctor();
        }
        Commands::Plan {
            model,
            memory,
            context,
        } => {
            commands::plan::execute_plan(&model, &memory, context);
        }
        Commands::Run {
            model,
            memory,
            draft_model,
            prompt,
        } => {
            commands::run::execute_run(&model, &memory, draft_model.as_deref(), &prompt);
        }
        Commands::Benchmark {
            model,
            reproduce,
            out,
        } => {
            commands::benchmark::execute_benchmark(model.as_deref(), reproduce.as_deref(), &out);
        }
        Commands::Audit { out } => {
            commands::audit::execute_audit(&out);
        }
    }
}
