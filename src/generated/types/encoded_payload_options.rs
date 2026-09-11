#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EncodedPayloadOptions {
    #[default]
    #[serde(rename = "offloaded")]
    Offloaded,
    #[serde(rename = "encrypted")]
    Encrypted,
    #[serde(rename = "encrypted-partial")]
    EncryptedPartial,
}
