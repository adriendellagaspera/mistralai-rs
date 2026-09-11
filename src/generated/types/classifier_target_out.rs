#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassifierTargetOut {
    pub labels: Vec<String>,
    pub loss_function: FTClassifierLossFunction,
    pub name: String,
    pub weight: f64,
}
