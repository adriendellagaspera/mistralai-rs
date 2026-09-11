#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRAsideTextBlockType {
    #[default]
    #[serde(rename = "aside_text")]
    AsideText,
}
