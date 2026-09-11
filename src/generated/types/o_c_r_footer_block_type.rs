#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRFooterBlockType {
    #[default]
    #[serde(rename = "footer")]
    Footer,
}
