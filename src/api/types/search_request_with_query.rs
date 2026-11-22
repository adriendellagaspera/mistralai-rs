pub use crate::prelude::*;

/// Request for search (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchRequest3 {
    #[serde(skip)]
    pub page_size: Option<i64>,
    #[serde(skip)]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<EvaluationRunRecordFilterInput>,
}

impl SearchRequest3 {
    pub fn builder() -> SearchRequest3Builder {
        <SearchRequest3Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchRequest3Builder {
    page_size: Option<i64>,
    page: Option<i64>,
    body: Option<EvaluationRunRecordFilterInput>,
}

impl SearchRequest3Builder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn body(mut self, value: EvaluationRunRecordFilterInput) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchRequest3`].
    pub fn build(self) -> Result<SearchRequest3, BuildError> {
        Ok(SearchRequest3 {
            page_size: self.page_size,
            page: self.page,
            body: self.body,
        })
    }
}
