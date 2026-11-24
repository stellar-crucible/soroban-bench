//! Soroban Bench Core: Macro-benchmarking, gas profiling, and storage rent engine.

pub mod fuzz;
pub mod history;
pub mod metrics;
pub mod profiler;
pub mod rent;
pub mod report;
pub mod scenario;
pub mod simulation;

pub use fuzz::{FuzzPercentiles, GenerativeInputStrategy};
pub use history::{BenchmarkHistoryStore, HistoricalBenchmarkRecord};
pub use metrics::{BenchmarkMetrics, MetricDiff};
pub use profiler::{ContractGasNode, HotLoopDetector, HotSpot};
pub use rent::{
    ProtocolVersion, RentCalculation, RestorationEstimate, StorageRentCalculator, StorageTier,
};
pub use report::BenchmarkHtmlReporter;
pub use scenario::{BenchmarkScenario, ScenarioStep, StepResult};
pub use simulation::{NetworkCongestionProfile, SimulationOutcome};
