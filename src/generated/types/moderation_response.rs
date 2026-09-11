#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<ModerationObject>,
}
