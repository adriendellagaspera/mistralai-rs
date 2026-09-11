#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRReferencesBlockType {
    #[default]
    #[serde(rename = "references")]
    References,
}
