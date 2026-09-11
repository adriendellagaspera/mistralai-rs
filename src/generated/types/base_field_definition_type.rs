#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BaseFieldDefinitionType {
    #[default]
    #[serde(rename = "ENUM")]
    Enum_,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "INT")]
    Int,
    #[serde(rename = "FLOAT")]
    Float,
    #[serde(rename = "BOOL")]
    Bool,
    #[serde(rename = "TIMESTAMP")]
    Timestamp,
    #[serde(rename = "ARRAY")]
    Array,
}
