#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EncodingFormat {
    #[default]
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "base64")]
    Base64,
}
