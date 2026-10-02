use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDescriptor {
    pub id: String,
    pub version: String,
    pub purpose: String,
    pub trigger: Vec<String>,
    pub required_tools: Vec<String>,
    pub required_memory: Vec<String>,
    pub steps: Vec<SkillStep>,
    pub verification: Vec<String>,
    pub rollback: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStep {
    pub id: String,
    pub action: String,
    pub tool: Option<String>,
    pub approval_required: bool,
}
