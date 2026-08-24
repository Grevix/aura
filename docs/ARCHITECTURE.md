# AURA Engineering & Architecture Guide

```mermaid
flowchart TD
    CLI[AURA CLI\n'aura run' / 'aura models'] --> HD[Hardware Doctor\nCPU SIMD, VRAM, NVMe IOPS]
    CLI --> FP[Feasibility Planner\nWorking Set vs Memory Budget]
    FP --> ME[Memory Enforcement\nWin32 Job Objects / Linux cgroup v2]
    ME --> BE[Execution Backends]
    
    subgraph MH [Memory Hierarchy]
        T0[Tier 0: GPU VRAM\nCurrent Layer / Working Set]
        T1[Tier 1: System RAM\nDouble-Buffered Staging Cache]
        T2[Tier 2: NVMe SSD\nModel Weight Shards]
        T3[Tier 3: Remote / Cloud\nColab / Object Storage]
        T0 <--> T1
        T1 <--> T2
        T2 <--> T3
    end
    
    BE --> Llama[Native llama-server Child Process]
    BE --> Ollama[Ollama REST API Adapter]
    BE --> OutOfCore[Out-of-Core Layer Streamer]
    
    Llama --> Out[Terminal Output & Telemetry]
    Ollama --> Out
    OutOfCore --> Out
```

---

## Component Implementation Status

- **`crates/aura-core`**: Core data structures, `MetricProvenance`, `BackendType`, and error types. (**IMPLEMENTED**)
- **`crates/aura-hardware`**: Multi-platform hardware telemetry probing via CPUID, sysinfo, and WMI/nvidia-smi. (**IMPLEMENTED**)
- **`crates/aura-memory`**: Hard process budget limits via Win32 Job Objects and Linux cgroup v2. (**IMPLEMENTED**)
- **`crates/aura-model`**: Ollama blob resolution and GGUF header parsing. (**IMPLEMENTED**)
- **`crates/aura-backends`**: Process-managed `llama-server` adapter and Ollama baseline client. (**IMPLEMENTED**)
- **`crates/aura-planner`**: Budget feasibility calculation, context auto-tuning, and `ExpertCache`. (**IMPLEMENTED**)
- **`crates/aura-cli`**: Complete command suite (`hardware-doctor`, `storage-doctor`, `models`, `frontier`, `run`, `benchmark`). (**IMPLEMENTED**)
