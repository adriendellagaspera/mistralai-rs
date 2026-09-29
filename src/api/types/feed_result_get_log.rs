pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeedResultGetLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<GetLog>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl FeedResultGetLog {
    pub fn builder() -> FeedResultGetLogBuilder {
        <FeedResultGetLogBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultGetLogBuilder {
    results: Option<Vec<GetLog>>,
    next: Option<String>,
    cursor: Option<String>,
}

impl FeedResultGetLogBuilder {
    pub fn results(mut self, value: Vec<GetLog>) -> Self {
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

    /// Consumes the builder and constructs a [`FeedResultGetLog`].
    pub fn build(self) -> Result<FeedResultGetLog, BuildError> {
        Ok(FeedResultGetLog {
            results: self.results,
            next: self.next,
            cursor: self.cursor,
        })
    }
}
