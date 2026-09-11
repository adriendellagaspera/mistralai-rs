#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRTextBlockType {
    #[default]
    #[serde(rename = "text")]
    Text,
}
