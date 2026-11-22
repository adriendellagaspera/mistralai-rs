pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchJobsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}

impl BatchJobsGetQueryRequest {
    pub fn builder() -> BatchJobsGetQueryRequestBuilder {
        <BatchJobsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchJobsGetQueryRequestBuilder {
    inline: Option<bool>,
}

impl BatchJobsGetQueryRequestBuilder {
    pub fn inline(mut self, value: bool) -> Self {
        self.inline = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchJobsGetQueryRequest`].
    pub fn build(self) -> Result<BatchJobsGetQueryRequest, BuildError> {
        Ok(BatchJobsGetQueryRequest {
            inline: self.inline,
        })
    }
}
