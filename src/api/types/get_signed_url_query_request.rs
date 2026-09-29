pub use crate::prelude::*;

/// Query parameters for get_signed_url
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSignedUrlQueryRequest {
    /// Number of hours before the URL becomes invalid. Defaults to 24h. Must be between 1h and 168h.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i64>,
}

impl GetSignedUrlQueryRequest {
    pub fn builder() -> GetSignedUrlQueryRequestBuilder {
        <GetSignedUrlQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSignedUrlQueryRequestBuilder {
    expiry: Option<i64>,
}

impl GetSignedUrlQueryRequestBuilder {
    pub fn expiry(mut self, value: i64) -> Self {
        self.expiry = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSignedUrlQueryRequest`].
    pub fn build(self) -> Result<GetSignedUrlQueryRequest, BuildError> {
        Ok(GetSignedUrlQueryRequest {
            expiry: self.expiry,
        })
    }
}
