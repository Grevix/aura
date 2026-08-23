# AURA Security Architecture & Model Supply Chain Policy

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Document Type:** Security Model & Supply Chain Governance  
**Status:** Approved  

---

## 1. Threat Model & Boundaries

AURA operates as a local AI model runtime. The security perimeter must protect the host operating system from untrusted model files, malicious third-party dependencies, corrupted downloads, and privilege escalation vulnerabilities.

```
[Untrusted Model File / Hugging Face]
                   │
                   ▼ (SHA-256 Hash Verification & Format Check)
[AURA Model Ingestion Gate (aura-model)]
                   │
                   ▼ (No Arbitrary Executable Code / Safe GGUF Parsing)
[AURA Planner & OS Budget Enforcer (aura-memory)]
                   │
                   ▼ (Unprivileged Subprocess Execution)
[Backend Process (llama.cpp Engine)]
```

### Primary Threat Vectors:
1. **Malicious Model Weights / Remote Code Execution:** Executable code embedded inside Python pickle or arbitrary weights formats.
2. **Buffer Overflows in Model Parsers:** Malformed GGUF file headers designed to exploit memory unsafety in C/C++ parsers.
3. **Dependency Supply-Chain Attacks:** Compromised third-party crates or libraries.
4. **Privilege Escalation:** Insecure OS memory management interface invocations (`cgroups`/Job Objects).

---

## 2. Security Defense Controls

### 2.1 Model Artifact Verification & Zero Remote Code
- **Non-Executable Formats Default:** AURA exclusively ingests static, non-executable tensor formats (GGUF, `safetensors`).
- **Strict Prohibition of Remote Code:** AURA rejects model repositories requiring `trust_remote_code=True` or custom Python code execution.
- **Cryptographic Hash Validation:** Every downloaded model weight artifact is validated against its published SHA-256 digest before registration or execution.

### 2.2 Fuzzing & Memory-Safe Parsing
- **Rust-Native GGUF Header Parser:** Header metadata parsing is implemented in safe Rust (`aura-model`) with strict bounds checking.
- **Continuous Fuzz Testing:** `aura-model` parser is continuously fuzzed using `cargo fuzz` (libFuzzer) against malformed, truncated, and random byte streams.

### 2.3 Dependency Governance & SBOM
- **Automated CVE Auditing:** `cargo audit` runs on every Pull Request and release build, blocking compilation on any High or Critical CVE.
- **License & Dependency Control:** `cargo deny` enforces strict license filtering (denying GPL/AGPL copyleft infection where incompatible) and blocks duplicate crate versions.
- **Software Bill of Materials (SBOM):** Every release automatically generates a machine-readable CycloneDX JSON SBOM (`sbom.json`).

### 2.4 Unprivileged Subprocess Execution
- **Least Privilege Principle:** Backend execution processes (`llama.cpp`) are spawned under unprivileged user scopes.
- **No System-Wide Mutex/Write Access:** AURA CLI never requires `sudo` or Administrator privileges for standard inference runs. OS `cgroups` access uses unprivileged `systemd --user` scopes where available.

### 2.5 Signed Release Binaries & Updates
- **Cryptographic Release Signatures:** Release binaries are signed using Cosign / Minisign.
- **Immutable Release Hashes:** Every GitHub release publishes SHA-256 checksums signed by project maintainers.
