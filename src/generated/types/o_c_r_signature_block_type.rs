#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRSignatureBlockType {
    #[default]
    #[serde(rename = "signature")]
    Signature,
}
