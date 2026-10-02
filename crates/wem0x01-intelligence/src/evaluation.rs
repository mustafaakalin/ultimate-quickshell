use crate::EvaluationResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationSuite {
    pub id: String,
    pub version: String,
    pub evaluators: Vec<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationReport {
    pub run_id: String,
    pub results: Vec<EvaluationResult>,
    pub passed: bool,
}
