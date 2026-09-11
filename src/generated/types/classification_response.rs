#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassificationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<ClassificationResponseResultsItem>,
}
