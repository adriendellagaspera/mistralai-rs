#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FilePurpose {
    #[default]
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "ocr")]
    Ocr,
}
