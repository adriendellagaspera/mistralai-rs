#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRCodeBlockType {
    #[default]
    #[serde(rename = "code")]
    Code,
}
