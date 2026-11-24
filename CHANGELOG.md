# Changelog

All notable changes to `soroban-bench` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-11-24

### Added
- **Benchmark History Store (`BenchmarkHistoryStore`)**: Automatically records scenario execution runs into `.benchmarks/history.json` tracking timestamps, git commit references, and resource metrics (closes #8).
- **History CLI Subcommand (`soroban-bench history <scenario>`)**: View historical performance trends across commits and releases.

## [0.3.0] - 2026-11-24

### Added
- **Instruction Heatmap & Hot-Loop Detector (`HotLoopDetector`)**: Identifies methods exceeding configurable budget thresholds (>25%) and renders ASCII heatmap bars (`[████████░░]`) (closes #4).
- **Benchmark Scenarios**: Added realistic scenarios for Automated Market Makers (`examples/scenarios/liquidity_pool.toml`) and DAO Timelocks (`examples/scenarios/governance_timelock.toml`).
- **End-to-End Triad Tutorial**: Published comprehensive tutorial connecting `soroban-guard`, `soroban-testkit`, and `soroban-bench`.
- Community governance policies: `SECURITY.md` and `CODE_OF_CONDUCT.md`.

## [0.2.0] - 2026-11-24

### Added
- **Interactive HTML Report (`BenchmarkHtmlReporter`)**: Standalone responsive performance dashboard with embedded Chart.js visualizations.
- **Call-Tree Gas Profiling (`ContractGasNode`)**: Hierarchical call attribution across cross-contract invocation depths.
- **Generative Scenario Fuzzing**: `GenerativeInputStrategy` with percentile calculations (`p50`, `p95`, `p99`).
- High-level Mermaid architecture flowchart in `README.md`.

## [0.1.0] - 2026-11-24

### Added
- Initial release of Soroban Bench macro-benchmarking engine.
- Declarative TOML macro scenario runner.
- Storage rent calculator modeling 30, 90, and 365-day TTL retention and Protocol 20–22 state archival restoration.
- Network congestion and fee surge emulator.
- `export-testkit` budget invariant synthesizer.
