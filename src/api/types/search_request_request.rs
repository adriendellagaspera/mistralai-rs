pub use crate::prelude::*;

/// Request for search (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchRequest2 {
    #[serde(skip)]
    pub page_size: Option<i64>,
    #[serde(skip)]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<EvaluationRunFilterInput>,
}

impl SearchRequest2 {
    pub fn builder() -> SearchRequest2Builder {
        <SearchRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchRequest2Builder {
    page_size: Option<i64>,
    page: Option<i64>,
    body: Option<EvaluationRunFilterInput>,
}

impl SearchRequest2Builder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn body(mut self, value: EvaluationRunFilterInput) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchRequest2`].
    pub fn build(self) -> Result<SearchRequest2, BuildError> {
        Ok(SearchRequest2 {
            page_size: self.page_size,
            page: self.page,
            body: self.body,
        })
    }
}
