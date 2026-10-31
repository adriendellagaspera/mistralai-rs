pub use crate::prelude::*;

/// Retrieve document chunks with similar keywords to the given query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct KeywordRetriever {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<KeywordRetrieverFilter>,
    #[serde(default)]
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
}

impl KeywordRetriever {
    pub fn builder() -> KeywordRetrieverBuilder {
        <KeywordRetrieverBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KeywordRetrieverBuilder {
    field: Option<String>,
    filter: Option<KeywordRetrieverFilter>,
    query: Option<String>,
    top_k: Option<i64>,
}

impl KeywordRetrieverBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn filter(mut self, value: KeywordRetrieverFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KeywordRetriever`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](KeywordRetrieverBuilder::query)
    pub fn build(self) -> Result<KeywordRetriever, BuildError> {
        Ok(KeywordRetriever {
            field: self.field,
            filter: self.filter,
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            top_k: self.top_k,
        })
    }
}
