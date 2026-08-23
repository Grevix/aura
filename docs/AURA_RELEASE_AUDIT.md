# AURA Release Audit System Specification

**Project:** AURA (Adaptive Ultra-Low-Memory Runtime for AI)  
**Document Type:** Release Integrity & Quality Audit Standard  
**Status:** Mandatory Release Gate  

---

## 1. Objective

The AURA Release Audit System ensures that **no binary, container, or release artifact is tagged or published without a machine-verifiable, reproducible audit trail.** Every release must generate an `AUDIT.md` and an `audit.json` document as official build assets.

---

## 2. Mandatory Release Audit Checklist

Before tagging any release candidate (e.g. `v0.1.0-rc1`), the automated audit runner evaluates 10 strict audit gates:

| Gate ID | Check Name | Verification Method | Blocker Level |
|---|---|---|---|
| **GATE-01** | Git Working Tree Cleanliness | `git status --porcelain` returns empty | CRITICAL |
| **GATE-02** | Test Suite Pass Guarantee | `cargo test --workspace` returns exit code 0 | CRITICAL |
| **GATE-03** | Zero Security Vulnerabilities | `cargo audit` returns 0 High/Critical CVEs | CRITICAL |
| **GATE-04** | License Compliance Audit | `cargo deny check licenses` confirms MIT/Apache-2.0 compatibility | CRITICAL |
| **GATE-05** | SBOM Artifact Generation | CycloneDX SBOM generated and validated | HIGH |
| **GATE-06** | Memory Budget Hard Limit | Self-hosted runner verifies memory bound under cgroups | CRITICAL |
| **GATE-07** | Performance Regression Gate | Benchmark delta vs prior release: $\Delta \text{TTFT} \le +3\%$, $\Delta \text{Decode} \ge -3\%$ | CRITICAL |
| **GATE-08** | Reproducibility Verification | Clean environment build matches release hash | HIGH |
| **GATE-09** | Platform Binary Build Matrix | Binaries compiled for Linux x86_64, Windows x86_64, macOS ARM64 | CRITICAL |
| **GATE-10** | Documentation Completeness | `README.md`, `CHANGELOG.md`, API docs updated | MEDIUM |

---

## 3. Schema: `audit.json` Specification

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "audit_version": "1.0",
  "release": {
    "version": "0.1.0",
    "git_commit": "a1b2c3d4e5f67890123456789abcdef012345678",
    "build_timestamp_utc": "2026-08-23T12:00:00Z",
    "compiler": "rustc 1.80.0 (051478957 2024-07-21)"
  },
  "audit_summary": {
    "status": "PASSED",
    "total_gates": 10,
    "passed_gates": 10,
    "failed_gates": 0,
    "critical_blockers": 0
  },
  "gates": [
    {
      "gate_id": "GATE-01",
      "name": "Git Working Tree Cleanliness",
      "status": "PASSED",
      "details": "Working tree clean at commit a1b2c3d"
    },
    {
      "gate_id": "GATE-02",
      "name": "Test Suite Pass Guarantee",
      "status": "PASSED",
      "details": "142 unit tests, 38 integration tests passed"
    },
    {
      "gate_id": "GATE-03",
      "name": "Zero Security Vulnerabilities",
      "status": "PASSED",
      "details": "0 vulnerabilities found in 84 dependencies"
    },
    {
      "gate_id": "GATE-04",
      "name": "License Compliance Audit",
      "status": "PASSED",
      "details": "All crates licensed under MIT OR Apache-2.0"
    },
    {
      "gate_id": "GATE-05",
      "name": "SBOM Artifact Generation",
      "status": "PASSED",
      "details": "CycloneDX SBOM attached to release artifacts"
    },
    {
      "gate_id": "GATE-06",
      "name": "Memory Budget Hard Limit",
      "status": "PASSED",
      "details": "cgroup v2 enforcement verified on reference runner"
    },
    {
      "gate_id": "GATE-07",
      "name": "Performance Regression Gate",
      "status": "PASSED",
      "details": "TTFT +0.5%, Decode +1.2% vs v0.0.9"
    },
    {
      "gate_id": "GATE-08",
      "name": "Reproducibility Verification",
      "status": "PASSED",
      "details": "Binary hash matching reproducible build container"
    },
    {
      "gate_id": "GATE-09",
      "name": "Platform Binary Build Matrix",
      "status": "PASSED",
      "details": "Targets x86_64-unknown-linux-gnu, x86_64-pc-windows-msvc, aarch64-apple-darwin compiled"
    },
    {
      "gate_id": "GATE-10",
      "name": "Documentation Completeness",
      "status": "PASSED",
      "details": "Release notes and architecture docs verified"
    }
  ],
  "dependencies": {
    "direct_crates_count": 12,
    "total_crates_count": 84,
    "sbom_sha256": "4b9f8a129876543210fe..."
  },
  "known_limitations": [
    "macOS memory enforcement is best-effort working-set polling, not kernel OS guaranteed",
    "MoE CPU performance tracking llama.cpp issue #19480"
  ]
}
```

---

## 4. Release Asset Bundle Protocol

Upon successful execution of the Audit Gate, the CI pipeline automatically packages and attaches the following immutable assets to the GitHub Release:

1. `aura-v0.1.0-linux-x86_64.tar.gz`
2. `aura-v0.1.0-windows-x86_64.zip`
3. `aura-v0.1.0-macos-arm64.tar.gz`
4. `checksums.txt` (SHA-256 signatures for all archives)
5. `AUDIT.md` (Human-readable release audit summary)
6. `audit.json` (Machine-readable audit artifact)
7. `sbom.json` (CycloneDX Software Bill of Materials)
8. `reference-benchmark.json` (Verified benchmark report on target hardware)
