#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutJudgeInSchema {
    ///Constraint: maxLength=500
    pub description: String,
    ///Constraint: maxLength=10000
    pub instructions: String,
    ///Constraint: maxLength=500
    pub model_name: String,
    ///Constraint: minLength=5, maxLength=50
    pub name: String,
    pub output: PutJudgeInSchemaOutput,
    pub tools: Vec<String>,
}
