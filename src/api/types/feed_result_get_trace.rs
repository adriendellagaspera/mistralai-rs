pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedResultGetTrace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<GetTrace>>,
}

impl FeedResultGetTrace {
    pub fn builder() -> FeedResultGetTraceBuilder {
        <FeedResultGetTraceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultGetTraceBuilder {
    cursor: Option<String>,
    next: Option<String>,
    results: Option<Vec<GetTrace>>,
}

impl FeedResultGetTraceBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<GetTrace>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedResultGetTrace`].
    pub fn build(self) -> Result<FeedResultGetTrace, BuildError> {
        Ok(FeedResultGetTrace {
            cursor: self.cursor,
            next: self.next,
            results: self.results,
        })
    }
}
