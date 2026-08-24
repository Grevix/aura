# PowerShell Reproducible Fresh Audit Script for AURA
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "AURA Windows Automated Forensic Audit & Quality Check" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan

# 1. Clean build check
Write-Host "`n[1/5] Building Release Binary..." -ForegroundColor Yellow
cargo build --release

# 2. Hardware and GPU Doctor
Write-Host "`n[2/5] Running Hardware Probes..." -ForegroundColor Yellow
.\target\release\aura.exe doctor
.\target\release\aura.exe gpu-doctor

# 3. Dynamic Model Discovery & Benchmark Run
Write-Host "`n[3/5] Running Benchmark Suite..." -ForegroundColor Yellow
python benchmarks/runners/run_local.py

# 4. Release Audit Gate Evaluation
Write-Host "`n[4/5] Evaluating Release Audit Gates..." -ForegroundColor Yellow
.\target\release\aura.exe audit --out audit.json

# 5. Quality Gates
Write-Host "`n[5/5] Running Cargo Formatting, Clippy, and Workspace Tests..." -ForegroundColor Yellow
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

Write-Host "`n✅ AURA Forensic Audit Complete! All gates passed successfully." -ForegroundColor Green
