#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRTableBlockType {
    #[default]
    #[serde(rename = "table")]
    Table,
}
