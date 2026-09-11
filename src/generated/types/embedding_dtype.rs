#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EmbeddingDtype {
    #[default]
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "int8")]
    Int8,
    #[serde(rename = "uint8")]
    Uint8,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "ubinary")]
    Ubinary,
}
