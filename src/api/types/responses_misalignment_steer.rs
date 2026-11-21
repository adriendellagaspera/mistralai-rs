pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesMisalignmentSteer {
    #[serde(default)]
    pub message: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesMisalignmentSteer {
    pub fn builder() -> ResponsesMisalignmentSteerBuilder {
        <ResponsesMisalignmentSteerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesMisalignmentSteerBuilder {
    message: Option<String>,
}

impl ResponsesMisalignmentSteerBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesMisalignmentSteer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ResponsesMisalignmentSteerBuilder::message)
    pub fn build(self) -> Result<ResponsesMisalignmentSteer, BuildError> {
        Ok(ResponsesMisalignmentSteer {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            extra: Default::default(),
        })
    }
}
