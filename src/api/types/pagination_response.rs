pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaginationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub page_size: i64,
}

impl PaginationResponse {
    pub fn builder() -> PaginationResponseBuilder {
        <PaginationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginationResponseBuilder {
    next_cursor: Option<String>,
    page_size: Option<i64>,
}

impl PaginationResponseBuilder {
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`page_size`](PaginationResponseBuilder::page_size)
    pub fn build(self) -> Result<PaginationResponse, BuildError> {
        Ok(PaginationResponse {
            next_cursor: self.next_cursor,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
        })
    }
}
