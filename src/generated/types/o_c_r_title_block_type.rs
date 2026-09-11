#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRTitleBlockType {
    #[default]
    #[serde(rename = "title")]
    Title,
}
