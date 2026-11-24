//! Soroban Bench Core: Macro-benchmarking, gas profiling, and storage rent engine.

pub mod metrics;
pub mod rent;
pub mod scenario;
pub mod simulation;

pub use metrics::{BenchmarkMetrics, MetricDiff};
pub use rent::{
    ProtocolVersion, RentCalculation, RestorationEstimate, StorageRentCalculator, StorageTier,
};
pub use scenario::{BenchmarkScenario, ScenarioStep, StepResult};
pub use simulation::{NetworkCongestionProfile, SimulationOutcome};
