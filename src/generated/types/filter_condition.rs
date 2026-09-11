#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterCondition {
    pub field: String,
    pub op: FilterConditionOp,
    pub value: serde_json::Value,
}
