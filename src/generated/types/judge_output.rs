#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JudgeOutput {
    pub analysis: String,
    pub answer: JudgeOutputAnswer,
}
