use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AssetKind {
    Agent,
    Tool,
    Skill,
    McpServer,
    Model,
    MemoryNamespace,
    Spec,
    Capability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relation {
    Uses,
    Calls,
    Provides,
    DependsOn,
    Reads,
    Writes,
    GovernedBy,
    VerifiedBy,
    ParentOf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub kind: AssetKind,
    pub revision: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub relation: Relation,
    pub to: String,
}

#[derive(Debug, Default)]
pub struct GovernanceGraph {
    nodes: BTreeMap<String, GraphNode>,
    edges: Vec<GraphEdge>,
}

impl GovernanceGraph {
    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.insert(node.id.clone(), node);
    }
    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }
    pub fn node(&self, id: &str) -> Option<&GraphNode> {
        self.nodes.get(id)
    }
    pub fn edges(&self) -> &[GraphEdge] {
        &self.edges
    }
}
