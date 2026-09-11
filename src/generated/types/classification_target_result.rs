#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassificationTargetResult {
    pub scores: ClassificationTargetResultScores,
}
