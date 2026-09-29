pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeedResultGetSpan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<GetSpan>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl FeedResultGetSpan {
    pub fn builder() -> FeedResultGetSpanBuilder {
        <FeedResultGetSpanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultGetSpanBuilder {
    results: Option<Vec<GetSpan>>,
    next: Option<String>,
    cursor: Option<String>,
}

impl FeedResultGetSpanBuilder {
    pub fn results(mut self, value: Vec<GetSpan>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedResultGetSpan`].
    pub fn build(self) -> Result<FeedResultGetSpan, BuildError> {
        Ok(FeedResultGetSpan {
            results: self.results,
            next: self.next,
            cursor: self.cursor,
        })
    }
}
