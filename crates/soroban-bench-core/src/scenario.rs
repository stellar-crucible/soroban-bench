use crate::metrics::BenchmarkMetrics;
use serde::{Deserialize, Serialize};

/// High-level macro benchmark scenario configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkScenario {
    pub name: String,
    pub description: String,
    pub steps: Vec<ScenarioStep>,
}

/// An individual execution step in a benchmark scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioStep {
    pub name: String,
    pub contract_method: String,
    pub iterations: u32,
    pub expected_max_cpu: Option<u64>,
    pub expected_max_mem: Option<u64>,
}

/// Outcome of running a single scenario step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_name: String,
    pub metrics: BenchmarkMetrics,
    pub passed: bool,
    pub notes: Option<String>,
}

impl BenchmarkScenario {
    /// Load a benchmark scenario definition from a TOML string.
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    /// Serialize a scenario to TOML format.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}
