pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "op")]
#[non_exhaustive]
pub enum JsonPatch {
    #[serde(rename = "add")]
    #[non_exhaustive]
    Add {
        #[serde(default)]
        path: String,
        value: serde_json::Value,
    },

    #[serde(rename = "append")]
    #[non_exhaustive]
    Append {
        #[serde(default)]
        path: String,
        value: JsonPatchAppendValue,
    },

    #[serde(rename = "remove")]
    #[non_exhaustive]
    Remove {
        #[serde(default)]
        path: String,
        value: serde_json::Value,
    },

    #[serde(rename = "replace")]
    #[non_exhaustive]
    Replace {
        #[serde(default)]
        path: String,
        value: serde_json::Value,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl JsonPatch {
    pub fn add(path: String, value: serde_json::Value) -> Self {
        Self::Add { path, value }
    }

    pub fn append(path: String, value: JsonPatchAppendValue) -> Self {
        Self::Append { path, value }
    }

    pub fn remove(path: String, value: serde_json::Value) -> Self {
        Self::Remove { path, value }
    }

    pub fn replace(path: String, value: serde_json::Value) -> Self {
        Self::Replace { path, value }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
