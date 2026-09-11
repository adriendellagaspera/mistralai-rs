#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JudgePreviews {
    pub judges: PaginatedResultJudgePreview,
}
