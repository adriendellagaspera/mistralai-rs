pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchRequest {
    #[serde(default)]
    pub body: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
}

impl BatchRequest {
    pub fn builder() -> BatchRequestBuilder {
        <BatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchRequestBuilder {
    body: Option<HashMap<String, serde_json::Value>>,
    custom_id: Option<String>,
}

impl BatchRequestBuilder {
    pub fn body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.body = Some(value);
        self
    }

    pub fn custom_id(mut self, value: impl Into<String>) -> Self {
        self.custom_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](BatchRequestBuilder::body)
    pub fn build(self) -> Result<BatchRequest, BuildError> {
        Ok(BatchRequest {
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            custom_id: self.custom_id,
        })
    }
}
