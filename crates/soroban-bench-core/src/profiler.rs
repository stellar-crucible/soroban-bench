use serde::{Deserialize, Serialize};

/// Represents an execution node in a cross-contract invocation call tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractGasNode {
    pub contract_id: String,
    pub method: String,
    pub depth: usize,
    pub cpu_instructions: u64,
    pub memory_bytes: u64,
    pub children: Vec<ContractGasNode>,
}

impl ContractGasNode {
    pub fn new(contract_id: &str, method: &str, depth: usize, cpu: u64, mem: u64) -> Self {
        Self {
            contract_id: contract_id.to_string(),
            method: method.to_string(),
            depth,
            cpu_instructions: cpu,
            memory_bytes: mem,
            children: Vec::new(),
        }
    }

    /// Add a child sub-invocation to this node.
    pub fn add_child(&mut self, child: ContractGasNode) {
        self.children.push(child);
    }

    /// Compute cumulative CPU instructions consumed across this node and all descendants.
    pub fn total_cpu(&self) -> u64 {
        let child_total: u64 = self.children.iter().map(|c| c.total_cpu()).sum();
        self.cpu_instructions + child_total
    }

    /// Compute attribution percentage for this specific node relative to a root transaction total.
    pub fn attribution_pct(&self, root_total: u64) -> f64 {
        if root_total == 0 {
            return 0.0;
        }
        (self.total_cpu() as f64 / root_total as f64) * 100.0
    }
}
