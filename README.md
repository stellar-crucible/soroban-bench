# Soroban Bench

[![CI](https://github.com/stellar-crucible/soroban-bench/actions/workflows/ci.yml/badge.svg)](https://github.com/stellar-crucible/soroban-bench/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue.svg)](https://stellar-crucible.github.io/soroban-bench/)

Continuous macro-benchmarking, storage rent projection, and gas regression baseline tracking for Soroban smart contracts on the Stellar network.

Part of the **Stellar Crucible** triad:
- [soroban-testkit](https://github.com/stellar-crucible/soroban-testkit) - Unit test harnesses, property test strategies, and mock environments.
- [soroban-guard](https://github.com/stellar-crucible/soroban-guard) - AST static security audits and testkit invariant synthesizer.
- [soroban-bench](https://github.com/stellar-crucible/soroban-bench) - Macro-benchmarking, gas regression baselines, and storage rent simulations.

## Capabilities

- **Macro-benchmark Scenarios**: Simulate complex contract interactions under fluctuating ledger load.
- **Storage Rent Projection**: Project ledger rent fees and TTL expiration schedules across 30, 90, and 365 days.
- **Gas Regression Guard**: Baseline comparison catching CPU instruction or memory consumption regressions before merging.
- **Export to soroban-testkit**: Export benchmark workloads as randomized property test profiles.
