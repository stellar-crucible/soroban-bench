use crate::metrics::BenchmarkMetrics;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// An individual execution snapshot in the benchmark history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalBenchmarkRecord {
    pub timestamp: String,
    pub scenario: String,
    pub git_ref: String,
    pub total_cpu: u64,
    pub total_memory: u64,
    pub total_fee_stroops: i64,
    pub step_metrics: Vec<BenchmarkMetrics>,
}

/// Registry of historical benchmark runs across commits, branches, and releases.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BenchmarkHistoryStore {
    pub records: Vec<HistoricalBenchmarkRecord>,
}

impl BenchmarkHistoryStore {
    /// Load from a JSON file, or initialize an empty store if the file does not exist.
    pub fn load_or_init<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let p = path.as_ref();
        if p.exists() {
            let content = std::fs::read_to_string(p).map_err(|e| e.to_string())?;
            let store: Self = serde_json::from_str(&content).map_err(|e| e.to_string())?;
            Ok(store)
        } else {
            Ok(Self::default())
        }
    }

    /// Record a scenario run and persist the updated history to disk.
    pub fn record_and_save<P: AsRef<Path>>(
        &mut self,
        path: P,
        scenario: &str,
        git_ref: &str,
        metrics: &[BenchmarkMetrics],
    ) -> Result<(), String> {
        let total_cpu: u64 = metrics.iter().map(|m| m.cpu_instructions).sum();
        let total_mem: u64 = metrics.iter().map(|m| m.memory_bytes).sum();
        let total_fee: i64 = metrics.iter().map(|m| m.estimated_fee_stroops).sum();

        let rec = HistoricalBenchmarkRecord {
            timestamp: "2026-11-24T12:00:00Z".to_string(),
            scenario: scenario.to_string(),
            git_ref: git_ref.to_string(),
            total_cpu,
            total_memory: total_mem,
            total_fee_stroops: total_fee,
            step_metrics: metrics.to_vec(),
        };

        self.records.push(rec);

        if let Some(parent) = path.as_ref().parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let serialized = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, serialized).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Retrieve all records for a specific scenario name.
    pub fn query_scenario(&self, scenario_name: &str) -> Vec<&HistoricalBenchmarkRecord> {
        self.records
            .iter()
            .filter(|r| r.scenario == scenario_name)
            .collect()
    }
}
