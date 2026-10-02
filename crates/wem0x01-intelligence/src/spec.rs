use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecState {
    Draft,
    Validated,
    Approved,
    Active,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecInvariant {
    pub id: String,
    pub expression: String,
    pub mandatory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub id: String,
    pub version: String,
    pub state: SpecState,
    pub goal: String,
    pub constraints: Vec<String>,
    pub invariants: Vec<SpecInvariant>,
    pub acceptance_tests: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub allowed_tools: Vec<String>,
}
