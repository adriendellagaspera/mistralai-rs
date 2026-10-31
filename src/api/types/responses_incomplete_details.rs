pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesIncompleteDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<ResponsesIncompleteDetailsReason>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesIncompleteDetails {
    pub fn builder() -> ResponsesIncompleteDetailsBuilder {
        <ResponsesIncompleteDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesIncompleteDetailsBuilder {
    reason: Option<ResponsesIncompleteDetailsReason>,
}

impl ResponsesIncompleteDetailsBuilder {
    pub fn reason(mut self, value: ResponsesIncompleteDetailsReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesIncompleteDetails`].
    pub fn build(self) -> Result<ResponsesIncompleteDetails, BuildError> {
        Ok(ResponsesIncompleteDetails {
            reason: self.reason,
            extra: Default::default(),
        })
    }
}
