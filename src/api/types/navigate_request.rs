pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NavigateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    pub direction: NavigationDirection,
    #[serde(default)]
    pub end_offset: i64,
    #[serde(default)]
    pub source_id: String,
    #[serde(default)]
    pub start_offset: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
}

impl NavigateRequest {
    pub fn builder() -> NavigateRequestBuilder {
        <NavigateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NavigateRequestBuilder {
    content_type: Option<String>,
    direction: Option<NavigationDirection>,
    end_offset: Option<i64>,
    source_id: Option<String>,
    start_offset: Option<i64>,
    top_k: Option<i64>,
}

impl NavigateRequestBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn direction(mut self, value: NavigationDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn end_offset(mut self, value: i64) -> Self {
        self.end_offset = Some(value);
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn start_offset(mut self, value: i64) -> Self {
        self.start_offset = Some(value);
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NavigateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`direction`](NavigateRequestBuilder::direction)
    /// - [`end_offset`](NavigateRequestBuilder::end_offset)
    /// - [`source_id`](NavigateRequestBuilder::source_id)
    /// - [`start_offset`](NavigateRequestBuilder::start_offset)
    pub fn build(self) -> Result<NavigateRequest, BuildError> {
        Ok(NavigateRequest {
            content_type: self.content_type,
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
            end_offset: self
                .end_offset
                .ok_or_else(|| BuildError::missing_field("end_offset"))?,
            source_id: self
                .source_id
                .ok_or_else(|| BuildError::missing_field("source_id"))?,
            start_offset: self
                .start_offset
                .ok_or_else(|| BuildError::missing_field("start_offset"))?,
            top_k: self.top_k,
        })
    }
}
