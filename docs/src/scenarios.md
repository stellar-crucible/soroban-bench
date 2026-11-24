# Macro Scenarios

Macro scenarios define realistic, multi-step contract execution workflows simulating actual on-chain user journeys under configurable iterations.

## Scenario Schema

Define benchmark scenarios in TOML format:

```toml
name = "dex-liquidity-stress"
description = "Stress test liquidity pool swaps under sequential load"

[[steps]]
name = "seed-liquidity"
contract_method = "deposit"
iterations = 20
expected_max_cpu = 1500000
expected_max_mem = 80000

[[steps]]
name = "sequential-swaps"
contract_method = "swap"
iterations = 100
expected_max_cpu = 900000
expected_max_mem = 45000
```

## Running Scenarios

Run scenarios and output JSON metrics for automated CI diffing:

```bash
soroban-bench run scenarios/dex_stress.toml --output results.json
```

Generate an interactive standalone HTML report with Chart.js visualization:

```bash
soroban-bench run scenarios/dex_stress.toml --format html --output report.html
```

## Network Congestion Simulation

Simulate how scenarios behave during ledger fee surges and concurrent invocation contention:

```bash
soroban-bench simulate scenarios/dex_stress.toml --multiplier 10.0 --concurrency 50
```
