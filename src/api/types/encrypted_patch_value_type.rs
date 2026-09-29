pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EncryptedPatchValueType {
    #[serde(rename = "__encrypted__")]
    Encrypted,
}
impl fmt::Display for EncryptedPatchValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Encrypted => "__encrypted__",
        };
        write!(f, "{}", s)
    }
}
