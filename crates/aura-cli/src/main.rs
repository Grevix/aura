mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "aura",
    author = "AURA Systems Engineering Team",
    version = "0.11.0",
    about = "AURA — Adaptive Out-of-Core Runtime for Frontier AI",
    long_about = "AURA is a hardware-aware, out-of-core memory hierarchy and inference orchestration engine for large language models."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Probe physical host hardware, SIMD extensions, RAM, and storage IOPS
    Doctor,

    /// Comprehensive hardware, CPU SIMD, RAM bandwidth, GPU VRAM, and storage doctor
    HardwareDoctor,

    /// Probe GPU hardware acceleration, VRAM, drivers, and CUDA capabilities
    GpuDoctor,

    /// Probe NVMe storage sequential/random bandwidth, latency, and streaming recommendations
    StorageDoctor,

    /// Unified discovery across local Ollama models, Hugging Face cache, and Frontier architectures
    Models,

    /// List all discovered local models from Ollama repository
    OllamaList,

    /// Inspect model architecture, layers, experts, and memory feasibility
    ModelInspect {
        /// Model identifier or tag (e.g. qwen3:8b, moonshotai/Kimi-K3, zai-org/GLM-5.2)
        #[arg(short = 'm', long)]
        model: String,
    },

    /// Frontier model inspection and out-of-core streaming execution suite
    Frontier {
        #[command(subcommand)]
        subcmd: FrontierCommands,
    },

    /// Launch experimental large-model execution or feasibility evaluation
    Experimental {
        #[command(subcommand)]
        subcmd: ExperimentalCommands,
    },

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

#[derive(Subcommand)]
enum FrontierCommands {
    /// Inspect frontier model architecture, expert count, and memory feasibility
    Inspect {
        #[arg(short = 'm', long)]
        model: String,
    },
    /// Launch frontier model out-of-core streaming execution path
    Run {
        #[arg(short = 'm', long)]
        model: String,
    },
}

#[derive(Subcommand)]
enum ExperimentalCommands {
    /// Evaluate resource feasibility and run experimental large-model path
    Run {
        /// Target model identifier
        #[arg(short = 'm', long)]
        model: String,
    },
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Doctor => {
            commands::doctor::execute_doctor();
        }
        Commands::HardwareDoctor => {
            commands::hardware_doctor::execute_hardware_doctor();
        }
        Commands::GpuDoctor => {
            commands::gpu_doctor::execute_gpu_doctor();
        }
        Commands::StorageDoctor => {
            commands::storage_doctor::execute_storage_doctor();
        }
        Commands::Models => {
            commands::models_list::execute_models_list();
        }
        Commands::OllamaList => {
            commands::ollama_list::execute_ollama_list();
        }
        Commands::ModelInspect { model } => {
            commands::model_inspect::execute_model_inspect(&model);
        }
        Commands::Frontier { subcmd } => match subcmd {
            FrontierCommands::Inspect { model } => {
                commands::frontier::execute_frontier_inspect(&model);
            }
            FrontierCommands::Run { model } => {
                commands::frontier::execute_frontier_run(&model);
            }
        },
        Commands::Experimental { subcmd } => match subcmd {
            ExperimentalCommands::Run { model } => {
                commands::experimental::execute_experimental_run(&model);
            }
        },
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
