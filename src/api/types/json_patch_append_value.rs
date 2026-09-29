pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum JsonPatchAppendValue {
    String(String),

    EncryptedPatchValue(EncryptedPatchValue),
}

impl JsonPatchAppendValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_encrypted_patch_value(&self) -> bool {
        matches!(self, Self::EncryptedPatchValue(_))
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_encrypted_patch_value(&self) -> Option<&EncryptedPatchValue> {
        match self {
            Self::EncryptedPatchValue(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_encrypted_patch_value(self) -> Option<EncryptedPatchValue> {
        match self {
            Self::EncryptedPatchValue(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for JsonPatchAppendValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::EncryptedPatchValue(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
