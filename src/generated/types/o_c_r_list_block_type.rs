#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRListBlockType {
    #[default]
    #[serde(rename = "list")]
    List,
}
