pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdateInvocationBodyInput {
    NetworkEncodedInput(NetworkEncodedInput),

    StringToValueMap(HashMap<String, serde_json::Value>),
}

impl UpdateInvocationBodyInput {
    pub fn is_network_encoded_input(&self) -> bool {
        matches!(self, Self::NetworkEncodedInput(_))
    }

    pub fn is_string_to_value_map(&self) -> bool {
        matches!(self, Self::StringToValueMap(_))
    }

    pub fn as_network_encoded_input(&self) -> Option<&NetworkEncodedInput> {
        match self {
            Self::NetworkEncodedInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_network_encoded_input(self) -> Option<NetworkEncodedInput> {
        match self {
            Self::NetworkEncodedInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string_to_value_map(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
            Self::StringToValueMap(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_to_value_map(self) -> Option<HashMap<String, serde_json::Value>> {
        match self {
            Self::StringToValueMap(value) => Some(value),
            _ => None,
        }
    }
}
