# Soroban Bench

**Soroban Bench** is the continuous macro-benchmarking and storage rent simulation engine for Soroban smart contracts on the Stellar network.

Part of the **Stellar Crucible** triad:
- [soroban-testkit](https://github.com/stellar-crucible/soroban-testkit): Dynamic property testing, mock ledger harnesses, and event matchers.
- [soroban-guard](https://github.com/stellar-crucible/soroban-guard): Static AST security auditing and testkit invariant synthesizer.
- [soroban-bench](https://github.com/stellar-crucible/soroban-bench): Continuous macro-benchmarking, gas regression baselines, and storage rent simulations.

## Why Macro-Benchmarking?

While micro-benchmarks measure isolated operations in a vacuum, real-world Soroban protocols experience:
- Multi-contract call chains
- Ledger storage footprint expansion
- Archival state TTL expiration and restoration costs
- Resource consumption surges under peak transaction volume

`soroban-bench` provides reproducible, automated simulation of these conditions in CI.
