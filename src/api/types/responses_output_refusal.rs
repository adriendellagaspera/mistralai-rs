pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesOutputRefusal {
    #[serde(default)]
    pub refusal: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesOutputRefusal {
    pub fn builder() -> ResponsesOutputRefusalBuilder {
        <ResponsesOutputRefusalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesOutputRefusalBuilder {
    refusal: Option<String>,
}

impl ResponsesOutputRefusalBuilder {
    pub fn refusal(mut self, value: impl Into<String>) -> Self {
        self.refusal = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesOutputRefusal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`refusal`](ResponsesOutputRefusalBuilder::refusal)
    pub fn build(self) -> Result<ResponsesOutputRefusal, BuildError> {
        Ok(ResponsesOutputRefusal {
            refusal: self
                .refusal
                .ok_or_else(|| BuildError::missing_field("refusal"))?,
            extra: Default::default(),
        })
    }
}
