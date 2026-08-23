# Physical Bounds & Hardware Bandwidth Mathematics

**Date:** 23 August 2026  
**Target Architecture:** CPU-only, Low-RAM Consumer PC (Intel Core i5-13420H, NVMe SSD @ 2.5 GB/s, DDR5 RAM @ 38.4 GB/s).  

---

## 1. Fundamental Hardware Physics Equations

The generation of each auto-regressive LLM token requires reading the weight tensor matrices $W$ across the memory/storage bus:

$$t_{\text{token}} \ge \frac{\text{Model Weight Size (Bytes)}}{\text{Bus Bandwidth (Bytes/sec)}} + t_{\text{compute}}$$

$$\text{Throughput (tokens/sec)} \le \frac{\text{Bus Bandwidth (Bytes/sec)}}{\text{Model Weight Size (Bytes)}}$$

---

## 2. Hard Storage vs RAM vs VRAM Bandwidth Ceiling

| Bus Interface | Hardware Type | Typical Bandwidth | 7B Q4 (4.68 GB) Max Speed | 70B Q4 (40 GB) Max Speed |
|---|---|---|---|---|
| **PCIe Gen3 x4 NVMe SSD** | Storage | **2.5 GB/s** | 0.53 tok/s | **0.0625 tok/s** (16s / tok) |
| **PCIe Gen4 x4 NVMe SSD** | Fast Storage | **7.0 GB/s** | 1.49 tok/s | **0.175 tok/s** (5.7s / tok) |
| **Dual-Channel DDR5 RAM** | Host Memory | **38.4 GB/s** | 8.20 tok/s | **0.96 tok/s** (1.04s / tok) |
| **PCIe 4.0 x16 Bus** | CPU-to-GPU Link | **31.5 GB/s** | 6.73 tok/s | **0.78 tok/s** |
| **RTX 3050 VRAM (GDDR6)** | Discrete GPU | **192.0 GB/s** | 41.02 tok/s | N/A (Exceeds VRAM) |
| **Apple M3 Max Unified** | Unified RAM | **400.0 GB/s** | 85.47 tok/s | **10.00 tok/s** |

---

## 3. Storage Streaming Breakdown (70B Model on NVMe SSD)

When physical RAM is constrained (e.g. 2 GB RAM), model weights **cannot** remain resident in system memory. They must be streamed from NVMe SSD during every token pass.

- **Weight Footprint (70B Q4_0):** 40,000,000,000 bytes (~40.0 GB).
- **NVMe Sequential Read Speed:** 2,500,000,000 bytes/sec (2.5 GB/s).
- **Required Read Pass Time:**

$$t_{\text{read}} = \frac{40.0 \text{ GB}}{2.5 \text{ GB/s}} = 16.0 \text{ seconds/token}$$

- **Upper Performance Bound:** **0.0625 tokens/second** (~1 token every 16 seconds).

---

## 4. OS Page Fault & Translation Lookaside Buffer (TLB) Overhead

When mmap page eviction occurs at high rates under memory pressure:
1. **Major Page Fault Penalty:** Reading non-cached 4KB OS pages triggers NVMe interrupt handling. Each major fault costs ~10–25 $\mu$s. Reading 40 GB requires 10,000,000 page faults = **250 seconds of pure kernel page fault handling overhead** if non-sequential!
2. **Sequential Block Prefetch Mitigation:** Using 2MB OS HugePages or Linux `MADV_WILLNEED` / Win32 `PrefetchVirtualMemory` reduces page fault count by 512x, restoring read rates back to raw NVMe sequential speed.
