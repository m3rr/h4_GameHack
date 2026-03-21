use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueType {
    Byte,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    String,
    AOB,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanResult {
    pub address: usize,
    pub value_type: ValueType,
    pub label: Option<String>,
    #[serde(default = "default_category")]
    pub category: String, // "Games", "System", "Windows", "Necessary", "Third Party"
}

fn default_category() -> String { "Third Party".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableFolder {
    pub name: String,
    pub entries: Vec<ScanResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEntry {
    pub pid: u32,
    pub name: String,
    pub status: String, // "Active", "Sleeping", "Inactive"
    pub is_system: bool,
    pub category: String, // "System", "Windows", "Necessary", "Third Party", "Games"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalEntry {
    pub timestamp: String,
    pub content: String,
}
