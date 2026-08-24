#!/bin/bash
set -e

echo "=================================================="
echo "AURA Google Colab / Linux Environment Setup Script"
echo "=================================================="

# 1. Update & System Dependencies
apt-get update -qq
apt-get install -y -qq build-essential curl git cmake nvidia-cuda-toolkit pciutils hwloc

# 2. Install Rust Toolchain if missing
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# 3. Verify Rust and Cargo
rustc --version
cargo --version

# 4. Check GPU & CUDA
nvidia-smi || echo "Warning: nvidia-smi failed or no GPU attached."

# 5. Build AURA in release mode
echo "Building AURA Release Binary..."
cargo build --release

echo "=================================================="
echo "✅ Setup Complete. AURA is ready for benchmarking!"
echo "=================================================="
