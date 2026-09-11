#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JudgeRegressionOutput {
    ///Constraint: exclusiveMaximum=1000000000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    pub max_description: String,
    ///Constraint: minimum=0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    pub min_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<JudgeRegressionOutputType>,
}
