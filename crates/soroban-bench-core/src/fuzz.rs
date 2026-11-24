use serde::{Deserialize, Serialize};

/// Strategy for generating randomized input data for stress testing scenarios.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerativeInputStrategy {
    /// Integer values within a range [min, max].
    IntegerRange { min: i128, max: i128 },
    /// Fixed-length pseudorandom byte buffers.
    RandomBytes { length: usize },
    /// Deterministic sequence of mock address strings.
    AddressSequence { prefix: String, count: u32 },
}

/// Statistics calculated across parameterized fuzz iterations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuzzPercentiles {
    pub p50_cpu: u64,
    pub p95_cpu: u64,
    pub p99_cpu: u64,
    pub p50_memory: u64,
    pub p95_memory: u64,
    pub p99_memory: u64,
}

impl FuzzPercentiles {
    /// Calculate 50th, 95th, and 99th percentiles from an execution sample list.
    pub fn calculate(mut cpu_samples: Vec<u64>, mut mem_samples: Vec<u64>) -> Self {
        if cpu_samples.is_empty() {
            return Self {
                p50_cpu: 0,
                p95_cpu: 0,
                p99_cpu: 0,
                p50_memory: 0,
                p95_memory: 0,
                p99_memory: 0,
            };
        }

        cpu_samples.sort_unstable();
        mem_samples.sort_unstable();

        let n = cpu_samples.len();
        let idx_50 = (n as f64 * 0.50) as usize;
        let idx_95 = (n as f64 * 0.95).min((n - 1) as f64) as usize;
        let idx_99 = (n as f64 * 0.99).min((n - 1) as f64) as usize;

        Self {
            p50_cpu: cpu_samples[idx_50],
            p95_cpu: cpu_samples[idx_95],
            p99_cpu: cpu_samples[idx_99],
            p50_memory: mem_samples[idx_50],
            p95_memory: mem_samples[idx_95],
            p99_memory: mem_samples[idx_99],
        }
    }
}
