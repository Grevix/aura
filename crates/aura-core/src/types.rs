use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimdExtension {
    Avx,
    Avx2,
    Avx512f,
    Avx512Vnni,
    Neon,
    Amx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfile {
    pub model_name: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub base_clock_ghz: f64,
    pub simd_features: Vec<SimdExtension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfile {
    pub total_ram_bytes: u64,
    pub available_ram_bytes: u64,
    pub total_swap_bytes: u64,
    pub page_size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProfile {
    pub storage_type: String,
    pub seq_read_mbps: f64,
    pub random_iops: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuProfile {
    pub present: bool,
    pub model_name: Option<String>,
    pub vram_bytes: Option<u64>,
    pub backend_supported: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    pub cpu: CpuProfile,
    pub memory: MemoryProfile,
    pub storage: StorageProfile,
    pub gpu: GpuProfile,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum QuantType {
    Q4_K_M,
    Q4_K_S,
    Q3_K_S,
    IQ3_XS,
    Q8_0,
    FP16,
    Unknown(String),
}

impl std::fmt::Display for QuantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantType::Q4_K_M => write!(f, "Q4_K_M"),
            QuantType::Q4_K_S => write!(f, "Q4_K_S"),
            QuantType::Q3_K_S => write!(f, "Q3_K_S"),
            QuantType::IQ3_XS => write!(f, "IQ3_XS"),
            QuantType::Q8_0 => write!(f, "Q8_0"),
            QuantType::FP16 => write!(f, "FP16"),
            QuantType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelManifest {
    pub name: String,
    pub source_hash_sha256: String,
    pub architecture_family: String,
    pub total_parameters: u64,
    pub active_parameters: u64,
    pub is_moe: bool,
    pub expert_count: Option<usize>,
    pub active_experts_per_token: Option<usize>,
    pub layer_count: usize,
    pub attention_heads: usize,
    pub key_value_heads: usize,
    pub head_dimension: usize,
    pub context_length_max: usize,
    pub quantization_type: QuantType,
    pub required_file_bytes: u64,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBudget {
    pub budget_bytes: u64,
    pub user_requested: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnforcementMechanism {
    CgroupV2Hard,
    WindowsJobObject,
    MacosBestEffort,
    None,
}

impl std::fmt::Display for EnforcementMechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnforcementMechanism::CgroupV2Hard => write!(f, "cgroup_v2_hard"),
            EnforcementMechanism::WindowsJobObject => write!(f, "windows_job_object"),
            EnforcementMechanism::MacosBestEffort => write!(f, "macos_best_effort"),
            EnforcementMechanism::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackendType {
    CpuLlamaCpp,
    CudaLlamaCpp,
    VulkanLlamaCpp,
    DirectMLLlamaCpp,
    MetalLlamaCpp,
    RemoteBackend,
}

impl std::fmt::Display for BackendType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendType::CpuLlamaCpp => write!(f, "cpu_llama_cpp"),
            BackendType::CudaLlamaCpp => write!(f, "cuda_llama_cpp"),
            BackendType::VulkanLlamaCpp => write!(f, "vulkan_llama_cpp"),
            BackendType::DirectMLLlamaCpp => write!(f, "directml_llama_cpp"),
            BackendType::MetalLlamaCpp => write!(f, "metal_llama_cpp"),
            BackendType::RemoteBackend => write!(f, "remote_backend"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetricProvenance {
    AuraMeasured,
    OllamaMeasured,
    PlannerEstimated,
    Simulated,
}

impl std::fmt::Display for MetricProvenance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricProvenance::AuraMeasured => write!(f, "aura_measured"),
            MetricProvenance::OllamaMeasured => write!(f, "ollama_measured"),
            MetricProvenance::PlannerEstimated => write!(f, "planner_estimated"),
            MetricProvenance::Simulated => write!(f, "simulated"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpeculativeStatus {
    Active {
        draft_model: String,
        speedup_estimate: f64,
    },
    Disabled,
    Infeasible {
        reason: String,
    },
}

impl std::fmt::Display for SpeculativeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpeculativeStatus::Active {
                draft_model,
                speedup_estimate,
            } => {
                write!(
                    f,
                    "Active (draft: {}, speedup: {:.1}x)",
                    draft_model, speedup_estimate
                )
            }
            SpeculativeStatus::Disabled => write!(f, "Disabled"),
            SpeculativeStatus::Infeasible { reason } => write!(f, "Infeasible ({})", reason),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeatureStatus {
    Active,
    Unavailable,
    Disabled,
}

impl std::fmt::Display for FeatureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureStatus::Active => write!(f, "Active"),
            FeatureStatus::Unavailable => write!(f, "Unavailable"),
            FeatureStatus::Disabled => write!(f, "Disabled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareCapabilities {
    pub has_cuda: bool,
    pub has_vulkan: bool,
    pub has_directml: bool,
    pub has_metal: bool,
    pub vram_bytes: Option<u64>,
    pub has_fa2: bool,
    pub has_sliding_window: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub model_name: String,
    pub memory_budget_bytes: u64,
    pub estimated_peak_rss_bytes: u64,
    pub estimated_weight_bytes: u64,
    pub estimated_kv_cache_bytes: u64,
    pub estimated_overhead_bytes: u64,
    pub recommended_quant: QuantType,
    pub recommended_context: usize,
    pub recommended_threads: usize,
    pub gpu_layers_offloaded: usize,
    pub predicted_decode_tok_per_sec: f64,
    pub predicted_ttft_ms: f64,
    pub is_feasible: bool,
    pub feasibility_notes: String,
    pub enforcement_mechanism: EnforcementMechanism,
    pub recommended_flags: Vec<String>,
    pub selected_backend: BackendType,
    pub speculative_status: SpeculativeStatus,
    pub fa2_status: FeatureStatus,
    pub sliding_window_status: FeatureStatus,
}
