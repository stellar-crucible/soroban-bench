# Soroban Bench

**Soroban Bench** is the continuous macro-benchmarking, gas regression tracking, and storage rent simulation engine for Soroban smart contracts on the Stellar network.

Part of the **Stellar Crucible** smart contract engineering triad:
- [soroban-testkit](https://github.com/stellar-crucible/soroban-testkit): Dynamic property testing, mock ledger harnesses, and event matchers.
- [soroban-guard](https://github.com/stellar-crucible/soroban-guard): Static AST security auditing and testkit invariant synthesizer.
- [soroban-bench](https://github.com/stellar-crucible/soroban-bench): Continuous macro-benchmarking, gas regression baselines, and storage rent simulations.

---

## Why Soroban Bench?

Building high-throughput, mission-critical smart contracts on Stellar requires strict resource predictability:

1. **Host-Metered Resource Caps**: Soroban enforces hard limits on CPU instructions, memory allocations, and ledger footprint sizes per transaction. A minor code change or dependency upgrade can cause a 10% gas increase, pushing contract interactions over network limits and causing user transactions to fail unexpectedly.
2. **State Archival & Storage Rent Economics**: Unlike perpetual storage blockchains, Soroban charges continuous ledger rent based on entry size, storage tier (**Persistent**, **Instance**, or **Temporary**), and Time-To-Live (TTL). Without automated rent forecasting, protocols risk deploying contracts with unsustainable operating expenses or sudden entry expirations.
3. **Cross-Contract Call Opacity**: Modern decentralized finance protocols route calls across multiple contracts (e.g. liquidity routers calling token vaults and price oracles). Standard test runners only report total transaction gas, leaving developers unable to identify which sub-invocation caused a CPU bottleneck.
4. **Network Congestion & Fee Surges**: Under heavy ledger traffic, network fee multipliers spike (up to 25x), and write contention on shared keys triggers transaction delays. Isolated single-threaded tests cannot simulate performance under realistic concurrent traffic.

---

## How It Works

Soroban Bench provides an integrated benchmarking, profiling, and rent estimation pipeline:

1. **Declarative Workload Modeling (`TOML`)**: Scenarios define multi-step contract workflows specifying method sequences, iteration counts, and maximum expected CPU/memory bounds.
2. **Call-Tree Gas Profiling (`ContractGasNode`)**: Tracks nested sub-invocations, attributing CPU instructions and memory consumption across call depths (`0` = root, `1` = direct call, `2` = deep invocation) and calculating exact percentage attribution.
3. **Network Congestion Simulation**: Emulates ledger fee surges (1.0x to 25.0x) and concurrent caller contention to estimate latency, peak CPU, and collision rates under peak traffic.
4. **Protocol-Aware Storage Rent Calculator**: Evaluates ledger entry sizes across Persistent, Instance, and Temporary tiers to project 30, 90, and 365-day retention fees, as well as Protocol 20, 21, and 22 archival restoration costs.
5. **CI Regression Guard**: Compares current metrics against a committed baseline (`.benchmarks/baseline.json`), failing CI when CPU or memory consumption regresses beyond a specified threshold (e.g., 5.0%).
6. **Symbiotic Invariant Export (`export-testkit`)**: Automatically converts scenario budgets and execution invariants into executable `soroban-testkit` property test harnesses.
