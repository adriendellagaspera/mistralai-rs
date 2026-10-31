pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetPipelineResults {
    #[serde(default)]
    pub pipeline_results: FeedResultGetPipelineResult,
}

impl GetPipelineResults {
    pub fn builder() -> GetPipelineResultsBuilder {
        <GetPipelineResultsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPipelineResultsBuilder {
    pipeline_results: Option<FeedResultGetPipelineResult>,
}

impl GetPipelineResultsBuilder {
    pub fn pipeline_results(mut self, value: FeedResultGetPipelineResult) -> Self {
        self.pipeline_results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPipelineResults`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pipeline_results`](GetPipelineResultsBuilder::pipeline_results)
    pub fn build(self) -> Result<GetPipelineResults, BuildError> {
        Ok(GetPipelineResults {
            pipeline_results: self
                .pipeline_results
                .ok_or_else(|| BuildError::missing_field("pipeline_results"))?,
        })
    }
}
