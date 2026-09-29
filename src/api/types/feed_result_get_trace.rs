pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedResultGetTrace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<GetTrace>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl FeedResultGetTrace {
    pub fn builder() -> FeedResultGetTraceBuilder {
        <FeedResultGetTraceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultGetTraceBuilder {
    results: Option<Vec<GetTrace>>,
    next: Option<String>,
    cursor: Option<String>,
}

impl FeedResultGetTraceBuilder {
    pub fn results(mut self, value: Vec<GetTrace>) -> Self {
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

    /// Consumes the builder and constructs a [`FeedResultGetTrace`].
    pub fn build(self) -> Result<FeedResultGetTrace, BuildError> {
        Ok(FeedResultGetTrace {
            results: self.results,
            next: self.next,
            cursor: self.cursor,
        })
    }
}
