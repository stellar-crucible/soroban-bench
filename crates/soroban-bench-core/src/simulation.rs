use crate::metrics::BenchmarkMetrics;
use serde::{Deserialize, Serialize};

/// Configuration for simulating network traffic congestion and fee surges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCongestionProfile {
    /// Surge multiplier on base reserve fees (e.g., 1.0 = normal, 5.0 = surge, 25.0 = peak spike).
    pub fee_multiplier: f64,
    /// Number of concurrent synthetic transactions submitted in parallel.
    pub concurrency_level: u32,
    /// Probability (0.0 to 1.0) of ledger entry lock contention among callers.
    pub contention_probability: f64,
}

impl Default for NetworkCongestionProfile {
    fn default() -> Self {
        Self {
            fee_multiplier: 1.0,
            concurrency_level: 1,
            contention_probability: 0.05,
        }
    }
}

/// Simulated outcome of executing a benchmark scenario under network congestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationOutcome {
    /// Total simulated transactions.
    pub total_transactions: u32,
    /// Transactions successfully included in simulated ledger.
    pub successful_transactions: u32,
    /// Transactions rejected due to contention or fee exhaustion.
    pub contested_transactions: u32,
    /// Average fee per transaction in stroops under the active fee multiplier.
    pub average_fee_stroops: i64,
    /// Peak CPU instruction consumption recorded across concurrent executions.
    pub peak_cpu_instructions: u64,
}

impl NetworkCongestionProfile {
    /// Simulate execution of a set of benchmark metrics under concurrent network pressure.
    pub fn simulate(&self, base_metrics: &BenchmarkMetrics, iterations: u32) -> SimulationOutcome {
        let total_tx = iterations * self.concurrency_level;
        let contested = (total_tx as f64 * self.contention_probability).round() as u32;
        let successful = total_tx.saturating_sub(contested);

        let fee_per_tx =
            (base_metrics.estimated_fee_stroops as f64 * self.fee_multiplier).round() as i64;
        let peak_cpu = (base_metrics.cpu_instructions as f64
            * (1.0 + self.contention_probability * 0.5)) as u64;

        SimulationOutcome {
            total_transactions: total_tx,
            successful_transactions: successful,
            contested_transactions: contested,
            average_fee_stroops: fee_per_tx,
            peak_cpu_instructions: peak_cpu,
        }
    }
}
