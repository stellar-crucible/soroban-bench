# Integration with soroban-testkit

`soroban-bench` and `soroban-testkit` work hand-in-hand:

1. **Benchmark to Property Test**: Export macro-scenarios directly into `soroban-testkit` property tests:
   ```bash
   soroban-bench export-testkit scenario.toml --output tests/benchmark_invariant.rs
   ```
2. **Budget Diff Verification**: Testkit assertions verify CPU instruction and memory limits derived from bench profiles:
   ```rust
   soroban_testkit::budget::assert_cpu_instructions_under(&env, 1_500_000);
   ```
3. **Continuous Performance Baselines**: Catch silent resource regressions before PR merge.
