use serde::{Deserialize, Serialize};

/// Performance and resource consumption metrics from contract execution.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    /// Total CPU instructions consumed by the VM.
    pub cpu_instructions: u64,
    /// Total memory allocated or referenced in bytes.
    pub memory_bytes: u64,
    /// Number of ledger entries read from storage.
    pub ledger_read_entries: u32,
    /// Number of ledger entries created, modified, or deleted.
    pub ledger_write_entries: u32,
    /// Total size in bytes of storage keys and values read.
    pub ledger_read_bytes: u32,
    /// Total size in bytes of storage keys and values written.
    pub ledger_write_bytes: u32,
    /// Estimated transaction fee in stroops.
    pub estimated_fee_stroops: i64,
}

impl BenchmarkMetrics {
    /// Compute percentage change against a baseline run.
    pub fn diff(&self, baseline: &Self) -> MetricDiff {
        MetricDiff {
            cpu_instructions_pct: calculate_pct(self.cpu_instructions, baseline.cpu_instructions),
            memory_bytes_pct: calculate_pct(self.memory_bytes, baseline.memory_bytes),
            read_bytes_pct: calculate_pct(
                self.ledger_read_bytes as u64,
                baseline.ledger_read_bytes as u64,
            ),
            write_bytes_pct: calculate_pct(
                self.ledger_write_bytes as u64,
                baseline.ledger_write_bytes as u64,
            ),
        }
    }
}

fn calculate_pct(current: u64, baseline: u64) -> f64 {
    if baseline == 0 {
        return if current > 0 { 100.0 } else { 0.0 };
    }
    ((current as f64 - baseline as f64) / baseline as f64) * 100.0
}

/// Percentage difference between baseline and current benchmark execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDiff {
    pub cpu_instructions_pct: f64,
    pub memory_bytes_pct: f64,
    pub read_bytes_pct: f64,
    pub write_bytes_pct: f64,
}

impl MetricDiff {
    /// True if any metric regressed beyond the specified tolerance percentage.
    pub fn exceeds_threshold(&self, threshold_pct: f64) -> bool {
        self.cpu_instructions_pct > threshold_pct
            || self.memory_bytes_pct > threshold_pct
            || self.read_bytes_pct > threshold_pct
            || self.write_bytes_pct > threshold_pct
    }
}
