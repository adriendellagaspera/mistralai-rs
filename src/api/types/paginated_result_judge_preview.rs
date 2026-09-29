pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedResultJudgePreview {
    #[serde(default)]
    pub count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Judge>>,
}

impl PaginatedResultJudgePreview {
    pub fn builder() -> PaginatedResultJudgePreviewBuilder {
        <PaginatedResultJudgePreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResultJudgePreviewBuilder {
    count: Option<i64>,
    next: Option<String>,
    previous: Option<String>,
    results: Option<Vec<Judge>>,
}

impl PaginatedResultJudgePreviewBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn previous(mut self, value: impl Into<String>) -> Self {
        self.previous = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<Judge>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedResultJudgePreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](PaginatedResultJudgePreviewBuilder::count)
    pub fn build(self) -> Result<PaginatedResultJudgePreview, BuildError> {
        Ok(PaginatedResultJudgePreview {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            next: self.next,
            previous: self.previous,
            results: self.results,
        })
    }
}
