use soroban_bench_core::{
    BenchmarkMetrics, BenchmarkScenario, NetworkCongestionProfile, ProtocolVersion, ScenarioStep,
    StorageRentCalculator, StorageTier,
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

    let deserialized = BenchmarkScenario::from_toml(&toml_str).expect("should deserialize");
    assert_eq!(deserialized.steps.len(), 2);
}
