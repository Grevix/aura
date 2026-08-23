# AURA Testing Architecture Specification

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Document Type:** Test Strategy & Quality Assurance Framework  
**Status:** Approved Baseline  

---

## 1. Test Pyramid & Execution Matrix

```
                 ▲
                / \
               /   \  System & Benchmark Tests (Self-Hosted Hardware)
              /-----\
             /       \  Integration & Memory Tests (Local / Self-Hosted)
            /---------\
           /           \  Unit Tests & Linters (GitHub Actions Matrix)
          /-------------\
```

| Test Level | Scope & Tooling | Execution Environment | CI Gate Stage | Pass Criteria |
|---|---|---|---|---|
| **Unit Tests** | Rust `cargo test`, C++ `ctest` | GitHub Actions (Linux, Win, macOS) | PR merge block | 100% pass rate; code coverage > 80% |
| **Lint & Formatting** | `cargo fmt`, `clippy`, `clang-format` | GitHub Actions | PR merge block | 0 warnings (`--deny warnings`) |
| **Integration Tests** | Synthetic GGUF models, mocked backend APIs | GitHub Actions | PR merge block | 100% pass rate |
| **Memory Leak Tests** | Valgrind / AddressSanitizer (ASan) / LSan | Self-Hosted Linux Runner | PR merge block | 0 byte leaks; 0 illegal accesses |
| **Memory Budget Tests** | Deliberate budget over-allocation stress tests | Self-Hosted Linux/Win/macOS | PR merge block | 100% enforcement; zero kernel OOM escapes |
| **Benchmark Regression** | `aura benchmark` smoke suite | Self-Hosted Reference Hardware | Main merge & Release tag | Latency regression < 3%; RSS leak < 1% |
| **Security Audit** | `cargo audit`, `cargo deny`, CycloneDX | GitHub Actions | Release tag | 0 High/Critical CVEs; 100% license compliance |

---

## 2. Test Category Definitions & Specifications

### 2.1 Unit Tests (`crates/*/src/`)
- **Hardware Prober Tests:** Verify core topology identification, SIMD flag detection, and memory arithmetic without external side-effects.
- **GGUF Parser Tests:** Test parsing of synthetic 1KB GGUF files with valid headers, corrupted headers, missing metadata keys, and truncated files.
- **Planner Analytical Math Tests:** Test memory calculation formulas across 50 distinct combinations of model layers, heads, quant types, and context lengths.

### 2.2 Integration Tests (`tests/integration/`)
- **Backend Mock Adapter Tests:** Mock `llama.cpp` process stdout/stderr streams to verify token parsing, TTFT calculation, and error signal handling.
- **CLI Subcommand Wiring:** Test parsing and routing for `aura doctor`, `aura plan`, `aura run`, and `aura benchmark`.

### 2.3 Memory Budget Enforcement Tests (`tests/memory/`)
- **Linux cgroups v2 Hard Ceiling Test:** Spawn a test process allocating 8 GB RAM inside a 4 GB cgroup. Verify process receives `SIGKILL` or soft throttle, and parent process catches event cleanly without hanging.
- **Windows Job Object Commit Limit Test:** Spawn test allocator exceeding job memory budget. Verify process is terminated with `STATUS_COMMITMENT_LIMIT`.
- **macOS Working-Set Poller Test:** Verify polling loop correctly identifies RSS spike and sends termination signal within 100 ms.

### 2.4 Failure Mode Stress Suite (`tests/failure/`)
Includes 30 mandatory failure scenario tests:
1. Model file missing mid-execution.
2. Model file corrupted (flipped header bytes).
3. Budget requested is smaller than minimum model layer allocation.
4. Storage device disconnected during mmap execution.
5. Storage read speed below 10 MB/s.
6. CPU lacks AVX/AVX2 SIMD extensions.
7. System swap space full.
8. GPU driver crash during execution pass.
9. Invalid GGUF metadata schema version.
10. Unrecognized quantization enum key.
11. Host RAM reduced mid-run by external process.
12. Terminal closed mid-inference generation.
13. Thermal throttling drops CPU clock below 1.0 GHz.
14. Systemd scope creation fails due to permissions.
15. Windows Job Object assignment denied by system policy.
16. Process memory limit changed dynamically.
17. Invalid prompt input format or token overflow.
18. Log directory read-only or out of disk space.
19. Duplicate CLI flags provided.
20. Network disconnection during model fetch phase.
21. Hash mismatch on downloaded model weights.
22. Model file locked by another process.
23. Incompatible library version linked via FFI.
24. Concurrent execution of multiple `aura run` commands on same machine.
25. RAM budget exact match with physical memory (0% headroom).
26. CPU thread allocation exceeds physical thread count.
27. Context size requested exceeds GGUF model maximum.
28. Non-ASCII characters in file paths or prompts.
29. Interrupt signal (Ctrl+C / SIGINT) sent during model load phase.
30. Out of disk space during benchmark JSON serialization.

---

## 3. Automated Test Execution Commands

```bash
# Run standard unit tests
cargo test --workspace --all-targets

# Run linters and format checks
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Run integration tests
cargo test --test integration

# Run memory enforcement integration tests (Linux root / cgroup required)
cargo test --test memory_cgroup -- --ignored

# Run full failure matrix suite
cargo test --test failure_matrix
```
