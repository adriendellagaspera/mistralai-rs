#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRImageBlockType {
    #[default]
    #[serde(rename = "image")]
    Image,
}
