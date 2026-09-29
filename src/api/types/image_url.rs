pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImageUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
    #[serde(default)]
    pub url: String,
}

impl ImageUrl {
    pub fn builder() -> ImageUrlBuilder {
        <ImageUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageUrlBuilder {
    detail: Option<ImageDetail>,
    url: Option<String>,
}

impl ImageUrlBuilder {
    pub fn detail(mut self, value: ImageDetail) -> Self {
        self.detail = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImageUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ImageUrlBuilder::url)
    pub fn build(self) -> Result<ImageUrl, BuildError> {
        Ok(ImageUrl {
            detail: self.detail,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
