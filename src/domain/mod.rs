use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Confidence {
    Exact,
    Range,
    Floating,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "value")]
pub enum VersionValue {
    Exact(String),
    Range(String),
    Floating(String),
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VersionObservation {
    pub value: VersionValue,
    pub source: SourceLocation,
    pub confidence: Confidence,
    pub environment: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Evidence {
    pub file: PathBuf,
    pub line: Option<u32>,
    pub snippet: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Finding {
    pub code: String,
    pub severity: Severity,
    pub title: String,
    pub evidence: Evidence,
    pub why_it_matters: String,
    pub remediation: String,
    pub references: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ProjectSummary {
    pub versions: Vec<VersionObservation>,
    pub topology: Vec<String>,
    pub drift: Vec<EnvironmentDrift>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentDrift {
    pub environment: String,
    pub versions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub root: PathBuf,
    pub summary: ProjectSummary,
    pub findings: Vec<Finding>,
}

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid version: {0}")]
    InvalidVersion(String),
    #[error("invalid advisory: {0}")]
    InvalidAdvisory(String),
}

pub trait Detector {
    fn id(&self) -> &'static str;
    fn detect(&self, report: &mut AnalysisReport);
}
pub trait AdvisoryStore {
    fn advisory(&self, id: &str) -> Option<&Advisory>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct Advisory {
    pub id: String,
    pub aliases: Vec<String>,
    pub severity: String,
    pub mitigations: Vec<String>,
    pub references: Vec<String>,
    pub affected: Vec<AffectedBranch>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AffectedBranch {
    pub branch: String,
    pub range: String,
    pub first_fixed: Option<String>,
    pub status: Option<String>,
}
