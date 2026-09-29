pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NetworkEncodedInput {
    /// The encoded payload
    #[serde(default)]
    pub b64payload: String,
    /// Whether the payload is empty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    /// The encoding of the payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_options: Option<Vec<EncodedPayloadOptions>>,
}

impl NetworkEncodedInput {
    pub fn builder() -> NetworkEncodedInputBuilder {
        <NetworkEncodedInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NetworkEncodedInputBuilder {
    b64payload: Option<String>,
    empty: Option<bool>,
    encoding_options: Option<Vec<EncodedPayloadOptions>>,
}

impl NetworkEncodedInputBuilder {
    pub fn b64payload(mut self, value: impl Into<String>) -> Self {
        self.b64payload = Some(value.into());
        self
    }

    pub fn empty(mut self, value: bool) -> Self {
        self.empty = Some(value);
        self
    }

    pub fn encoding_options(mut self, value: Vec<EncodedPayloadOptions>) -> Self {
        self.encoding_options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NetworkEncodedInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`b64payload`](NetworkEncodedInputBuilder::b64payload)
    pub fn build(self) -> Result<NetworkEncodedInput, BuildError> {
        Ok(NetworkEncodedInput {
            b64payload: self
                .b64payload
                .ok_or_else(|| BuildError::missing_field("b64payload"))?,
            empty: self.empty,
            encoding_options: self.encoding_options,
        })
    }
}
