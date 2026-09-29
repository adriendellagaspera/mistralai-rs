pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSignedUrlResponse {
    #[serde(default)]
    pub url: String,
}

impl GetSignedUrlResponse {
    pub fn builder() -> GetSignedUrlResponseBuilder {
        <GetSignedUrlResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSignedUrlResponseBuilder {
    url: Option<String>,
}

impl GetSignedUrlResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSignedUrlResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](GetSignedUrlResponseBuilder::url)
    pub fn build(self) -> Result<GetSignedUrlResponse, BuildError> {
        Ok(GetSignedUrlResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
