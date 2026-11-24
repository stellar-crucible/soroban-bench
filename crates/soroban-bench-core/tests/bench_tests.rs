use soroban_bench_core::{
    BenchmarkHtmlReporter, BenchmarkMetrics, BenchmarkScenario, ContractGasNode, FuzzPercentiles,
    GenerativeInputStrategy, HotLoopDetector, NetworkCongestionProfile, ProtocolVersion,
    ScenarioStep, StorageRentCalculator, StorageTier,
};

#[test]
fn test_metrics_diff_calculation() {
    let baseline = BenchmarkMetrics {
        cpu_instructions: 1_000_000,
        memory_bytes: 50_000,
        ledger_read_entries: 5,
        ledger_write_entries: 2,
        ledger_read_bytes: 400,
        ledger_write_bytes: 200,
        estimated_fee_stroops: 15_000,
    };

    let current = BenchmarkMetrics {
        cpu_instructions: 1_050_000, // +5%
        memory_bytes: 55_000,        // +10%
        ledger_read_entries: 5,
        ledger_write_entries: 2,
        ledger_read_bytes: 400,
        ledger_write_bytes: 200,
        estimated_fee_stroops: 16_000,
    };

    let diff = current.diff(&baseline);
    assert!((diff.cpu_instructions_pct - 5.0).abs() < 0.01);
    assert!((diff.memory_bytes_pct - 10.0).abs() < 0.01);
    assert!(diff.exceeds_threshold(4.0));
    assert!(!diff.exceeds_threshold(12.0));
}

#[test]
fn test_storage_rent_projection() {
    let calc = StorageRentCalculator::default();
    let projection = calc.project_rent(1_024, StorageTier::Persistent, 100_000);

    assert_eq!(projection.entry_bytes, 1024);
    assert_eq!(projection.tier, StorageTier::Persistent);
    assert!(projection.rent_30_days_stroops > 0);
    assert!(projection.rent_90_days_stroops > projection.rent_30_days_stroops);
    assert!(projection.rent_365_days_stroops > projection.rent_90_days_stroops);

    let restore_v20 = calc.project_restoration(1_024, ProtocolVersion::Protocol20);
    let restore_v22 = calc.project_restoration(1_024, ProtocolVersion::Protocol22);
    assert!(restore_v20.total_fee_stroops > restore_v22.total_fee_stroops);
}

#[test]
fn test_network_congestion_simulation() {
    let profile = NetworkCongestionProfile {
        fee_multiplier: 5.0,
        concurrency_level: 10,
        contention_probability: 0.1,
    };

    let metrics = BenchmarkMetrics {
        cpu_instructions: 500_000,
        memory_bytes: 20_000,
        ledger_read_entries: 2,
        ledger_write_entries: 1,
        ledger_read_bytes: 128,
        ledger_write_bytes: 64,
        estimated_fee_stroops: 10_000,
    };

    let outcome = profile.simulate(&metrics, 5);
    assert_eq!(outcome.total_transactions, 50);
    assert!(outcome.successful_transactions > 0);
    assert!(outcome.contested_transactions > 0);
    assert_eq!(outcome.average_fee_stroops, 50_000);
}

#[test]
fn test_scenario_serialization() {
    let scenario = BenchmarkScenario {
        name: "liquidity-pool-stress".to_string(),
        description: "Benchmark swap under high concurrency".to_string(),
        steps: vec![
            ScenarioStep {
                name: "mint-liquidity".to_string(),
                contract_method: "deposit".to_string(),
                iterations: 50,
                expected_max_cpu: Some(2_000_000),
                expected_max_mem: Some(100_000),
            },
            ScenarioStep {
                name: "continuous-swaps".to_string(),
                contract_method: "swap".to_string(),
                iterations: 200,
                expected_max_cpu: Some(1_500_000),
                expected_max_mem: Some(80_000),
            },
        ],
    };

    let toml_str = scenario.to_toml().expect("should serialize");
    assert!(toml_str.contains("liquidity-pool-stress"));
}

#[test]
fn test_gas_node_call_tree_and_attribution() {
    let mut root = ContractGasNode::new("PoolContract", "swap", 0, 100_000, 5_000);
    let mut child1 = ContractGasNode::new("VaultContract", "transfer", 1, 50_000, 2_000);
    let subchild = ContractGasNode::new("TokenLedger", "debit", 2, 25_000, 1_000);
    child1.add_child(subchild);

    let child2 = ContractGasNode::new("OracleContract", "get_price", 1, 25_000, 1_000);

    root.add_child(child1);
    root.add_child(child2);

    // root own: 100k + child1 (50k + 25k) + child2 (25k) = 200k
    assert_eq!(root.total_cpu(), 200_000);
    assert_eq!(root.attribution_pct(200_000), 100.0);
    assert_eq!(root.children[0].total_cpu(), 75_000);
    assert!((root.children[0].attribution_pct(200_000) - 37.5).abs() < 0.01);
}

#[test]
fn test_html_reporter_output() {
    let scenario = BenchmarkScenario {
        name: "amm-benchmark".to_string(),
        description: "Automated Market Maker swap stress test".to_string(),
        steps: vec![ScenarioStep {
            name: "swap-exact-in".to_string(),
            contract_method: "swap".to_string(),
            iterations: 100,
            expected_max_cpu: Some(1_000_000),
            expected_max_mem: Some(50_000),
        }],
    };

    let metrics = vec![BenchmarkMetrics {
        cpu_instructions: 450_000,
        memory_bytes: 18_000,
        ledger_read_entries: 3,
        ledger_write_entries: 2,
        ledger_read_bytes: 200,
        ledger_write_bytes: 100,
        estimated_fee_stroops: 8_500,
    }];

    let html = BenchmarkHtmlReporter::render(&scenario, &metrics);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("amm-benchmark"));
    assert!(html.contains("swap-exact-in"));
    assert!(html.contains("Chart"));
}

#[test]
fn test_fuzz_percentiles_calculation() {
    let cpu_samples = vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];
    let mem_samples = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    let p = FuzzPercentiles::calculate(cpu_samples, mem_samples);
    assert_eq!(p.p50_cpu, 600);
    assert_eq!(p.p95_cpu, 1000);
    assert_eq!(p.p99_cpu, 1000);

    let strat = GenerativeInputStrategy::IntegerRange { min: 1, max: 100 };
    match strat {
        GenerativeInputStrategy::IntegerRange { min, max } => {
            assert_eq!(min, 1);
            assert_eq!(max, 100);
        }
        _ => panic!("wrong strategy"),
    }
}

#[test]
fn test_hot_loop_detector_and_heatmap() {
    let mut root = ContractGasNode::new("PoolContract", "rebalance", 0, 600_000, 20_000);
    let child = ContractGasNode::new("OracleContract", "fetch_twap", 1, 400_000, 10_000);
    root.add_child(child);

    let detector = HotLoopDetector::new(30.0);
    let spots = detector.detect_hotspots(&root);

    // root own is 600k / 1M = 60%, child is 400k / 1M = 40% -> both >= 30%
    assert_eq!(spots.len(), 2);
    assert_eq!(spots[0].contract_id, "PoolContract");
    assert!(spots[0].is_hot_loop); // >= 40%

    let bar = HotLoopDetector::render_heatmap_bar(60.0, 10);
    assert!(bar.contains("60.0%"));
    assert!(bar.contains("█"));
}
