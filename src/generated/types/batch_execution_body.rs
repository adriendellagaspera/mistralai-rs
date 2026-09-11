#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchExecutionBody {
    ///List of execution IDs to process
    ///Constraint: minItems=1, maxItems=100
    pub execution_ids: Vec<String>,
}
