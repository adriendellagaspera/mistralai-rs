pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeedResultGetPipelineResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<PipelineResult>>,
}

impl FeedResultGetPipelineResult {
    pub fn builder() -> FeedResultGetPipelineResultBuilder {
        <FeedResultGetPipelineResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedResultGetPipelineResultBuilder {
    cursor: Option<String>,
    next: Option<String>,
    results: Option<Vec<PipelineResult>>,
}

impl FeedResultGetPipelineResultBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<PipelineResult>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedResultGetPipelineResult`].
    pub fn build(self) -> Result<FeedResultGetPipelineResult, BuildError> {
        Ok(FeedResultGetPipelineResult {
            cursor: self.cursor,
            next: self.next,
            results: self.results,
        })
    }
}
