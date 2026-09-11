#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WorkflowExecutionTraceSummaryAttributesValues {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
