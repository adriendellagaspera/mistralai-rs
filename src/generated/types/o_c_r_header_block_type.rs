#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRHeaderBlockType {
    #[default]
    #[serde(rename = "header")]
    Header,
}
