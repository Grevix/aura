# Contributing to AURA

We welcome contributions from systems engineers, runtime researchers, and benchmark contributors.

## Pull Request Checklist

Before submitting a pull request, ensure that your branch passes all required quality checks:

```bash
# 1. Format check
cargo fmt --all -- --check

# 2. Lint check
cargo clippy --workspace --all-targets -- -D warnings

# 3. Workspace tests
cargo test --workspace

# 4. Release compilation
cargo build --release --workspace
```

## Performance & Benchmark Evidence Rules

1. **No Simulated Numbers as Measured**: Any benchmark contribution must clearly specify its hardware environment and use real measurement provenance (`aura_measured` / `ollama_measured`).
2. **Memory Limit Verification**: Always test with process memory limits enabled.
3. **No False Feasibility Claims**: Do not mark large frontier models as runnable on consumer laptops unless verified through real token generation.
