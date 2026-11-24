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

/// Discovered hot spot or bottleneck in contract execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotSpot {
    pub contract_id: String,
    pub method: String,
    pub cpu_instructions: u64,
    pub percentage_of_total: f64,
    pub is_hot_loop: bool,
}

/// Detector for performance bottlenecks and excessive CPU instruction loops.
pub struct HotLoopDetector {
    pub threshold_pct: f64,
}

impl Default for HotLoopDetector {
    fn default() -> Self {
        Self {
            threshold_pct: 25.0,
        }
    }
}

impl HotLoopDetector {
    pub fn new(threshold_pct: f64) -> Self {
        Self { threshold_pct }
    }

    /// Traverse the call tree and return all nodes that exceed the threshold share of the transaction.
    pub fn detect_hotspots(&self, root: &ContractGasNode) -> Vec<HotSpot> {
        let total = root.total_cpu();
        let mut spots = Vec::new();
        self.collect_spots(root, total, &mut spots);
        spots.sort_by_key(|b| std::cmp::Reverse(b.cpu_instructions));
        spots
    }

    fn collect_spots(&self, node: &ContractGasNode, total: u64, out: &mut Vec<HotSpot>) {
        let pct = if total == 0 {
            0.0
        } else {
            (node.cpu_instructions as f64 / total as f64) * 100.0
        };

        if pct >= self.threshold_pct {
            out.push(HotSpot {
                contract_id: node.contract_id.clone(),
                method: node.method.clone(),
                cpu_instructions: node.cpu_instructions,
                percentage_of_total: pct,
                is_hot_loop: pct >= 40.0,
            });
        }

        for child in &node.children {
            self.collect_spots(child, total, out);
        }
    }

    /// Generate an ASCII heatmap bar visualization for instruction consumption.
    pub fn render_heatmap_bar(percentage: f64, width: usize) -> String {
        let filled = ((percentage / 100.0) * width as f64).round() as usize;
        let filled = filled.min(width);
        let empty = width.saturating_sub(filled);
        format!(
            "[{}{}] {:.1}%",
            "█".repeat(filled),
            "░".repeat(empty),
            percentage
        )
    }
}
