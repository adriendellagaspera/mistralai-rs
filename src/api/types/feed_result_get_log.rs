pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeedResultGetLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<GetLog>>,
}

impl FeedResultGetLog {
    pub fn builder() -> FeedResultGetLogBuilder {
        <FeedResultGetLogBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultGetLogBuilder {
    cursor: Option<String>,
    next: Option<String>,
    results: Option<Vec<GetLog>>,
}

impl FeedResultGetLogBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<GetLog>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedResultGetLog`].
    pub fn build(self) -> Result<FeedResultGetLog, BuildError> {
        Ok(FeedResultGetLog {
            cursor: self.cursor,
            next: self.next,
            results: self.results,
        })
    }
}
