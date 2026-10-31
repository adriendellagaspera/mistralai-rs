pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PipelinesResponse {
    #[serde(default)]
    pub pipelines: PaginatedResultPipeline,
}

impl PipelinesResponse {
    pub fn builder() -> PipelinesResponseBuilder {
        <PipelinesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelinesResponseBuilder {
    pipelines: Option<PaginatedResultPipeline>,
}

impl PipelinesResponseBuilder {
    pub fn pipelines(mut self, value: PaginatedResultPipeline) -> Self {
        self.pipelines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelinesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pipelines`](PipelinesResponseBuilder::pipelines)
    pub fn build(self) -> Result<PipelinesResponse, BuildError> {
        Ok(PipelinesResponse {
            pipelines: self
                .pipelines
                .ok_or_else(|| BuildError::missing_field("pipelines"))?,
        })
    }
}
