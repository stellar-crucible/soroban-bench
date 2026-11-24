# Macro Scenarios

Define benchmark scenarios in TOML format:

```toml
name = "dex-liquidity-stress"
description = "Stress test liquidity pool swaps under sequential load"

[[steps]]
name = "seed-liquidity"
contract_method = "deposit"
iterations = 20
expected_max_cpu = 1500000

[[steps]]
name = "sequential-swaps"
contract_method = "swap"
iterations = 100
expected_max_cpu = 900000
```

Run scenarios using the CLI:

```bash
soroban-bench run scenario.toml --output results.json
```
