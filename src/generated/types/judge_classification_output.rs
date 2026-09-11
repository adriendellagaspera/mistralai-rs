#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JudgeClassificationOutput {
    pub options: Vec<JudgeClassificationOutputOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<JudgeClassificationOutputType>,
}
