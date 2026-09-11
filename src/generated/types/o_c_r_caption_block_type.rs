#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRCaptionBlockType {
    #[default]
    #[serde(rename = "caption")]
    Caption,
}
