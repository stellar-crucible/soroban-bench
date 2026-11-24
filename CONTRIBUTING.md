# Contributing to Soroban Bench

Thank you for your interest in contributing to **Soroban Bench**, part of the **Stellar Crucible** developer tooling ecosystem!

## Development Setup

1. **Prerequisites**:
   - Rust 1.85+ (`rustup toolchain install stable`)
   - Components: `rustfmt`, `clippy`

2. **Clone and Build**:
   ```bash
   git clone https://github.com/stellar-crucible/soroban-bench.git
   cd soroban-bench
   cargo build --workspace
   ```

3. **Run Tests & Lints**:
   ```bash
   cargo fmt --all --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```

## Creating Scenarios

Benchmark scenarios are written in TOML format under `examples/scenarios/`. Test your scenarios with:
```bash
cargo run -p soroban-bench-cli -- run examples/scenarios/liquidity_pool.toml
```
