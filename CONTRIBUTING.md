# Contributing to AURA

Thank you for contributing. Please read this carefully before opening a pull request.

---

## Before You Contribute

AURA is adversarially reviewed. We will reject:
- Fabricated benchmark numbers
- Optimizations without before/after measurements
- Code that silently degrades output quality
- Claims that exceed what the benchmarks show

---

## Types of Contributions

### Bug Reports
Open a GitHub issue. Include:
- OS and version
- AURA version / git commit
- Exact command that failed
- Full error message
- Steps to reproduce

### Benchmark Results
Use the [community benchmark runner](benchmarks/community_runner/).
Your result will be labelled **COMMUNITY REPORTED**, not project-verified.
See [docs/COMMUNITY_BENCHMARKS.md](docs/COMMUNITY_BENCHMARKS.md).

### Code Contributions
1. Fork and create a feature branch
2. Make your change
3. Run `cargo test --workspace` — all tests must pass
4. Run `cargo clippy --workspace` — zero warnings
5. Run `cargo fmt --check` — formatting must match
6. Add a test if you added functionality
7. Open a pull request with a clear description

### Optimization Contributions
Every optimization must include:
- **BASELINE** — measurement before your change
- **CHANGE** — what you changed and why
- **AFTER** — measurement after your change  
- **REGRESSION CHECK** — `cargo test --workspace` results

Optimizations that make things slower will be reverted.

---

## Code Style

- Rust: follow `rustfmt` defaults
- Python: follow PEP 8, 4-space indentation
- JSON: 2-space indentation
- No hardcoded benchmark numbers in source code

## Testing

```bash
cargo test --workspace     # all unit tests
cargo clippy --workspace   # lints
cargo fmt --check          # formatting
cargo build --release      # release build
```

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.
