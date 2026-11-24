# Call-Tree Gas Profiling

Soroban contracts often execute cross-contract calls into external vaults, liquidity routers, or price oracles. Understanding where gas is consumed across nested sub-invocations is critical for performance tuning.

## Cross-Contract Gas Attribution

The `soroban-bench profile` command visualizes execution trees and computes hierarchical resource attribution:

```bash
soroban-bench profile scenarios/dex_stress.toml
```

### Sample Output

```text
Profiling Cross-Contract Gas Attribution: dex-liquidity-stress

Execution Tree for Step `swap`:
▪ [0] PrimaryContract::swap (15000000 CPU instructions, 800000 bytes mem, 100.0%)
  └── [1] TokenVault::transfer_checked (6500000 CPU instructions, 300000 bytes mem, 60.0%)
    └── [2] TokenLedger::debit (2500000 CPU instructions, 120000 bytes mem, 16.7%)
  └── [1] PriceOracle::get_latest_price (3500000 CPU instructions, 150000 bytes mem, 23.3%)
  Cumulative CPU: 27500000 instructions
```

## Attribution Metrics

- **Depth Level**: Depth of contract call nesting (`0` = Root entrypoint, `1` = Direct cross-call, `2` = Deep invocation).
- **Own CPU**: Gas instructions expended directly within the current frame.
- **Subtree Cumulative CPU**: Total instructions consumed by the frame and all child sub-invocations.
- **Attribution Percentage**: Fractional share of the overall transaction budget consumed by the call subtree.

## Integration with HTML Reports

When executing benchmark scenarios with `--format html`, call attribution and step breakdown are automatically rendered as interactive Chart.js bar and radar visualizations.
