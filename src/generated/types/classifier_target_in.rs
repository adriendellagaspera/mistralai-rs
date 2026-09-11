#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassifierTargetIn {
    pub labels: Vec<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub loss_function: Option<Option<FTClassifierLossFunction>>,
    pub name: String,
    ///Constraint: minimum=0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}
