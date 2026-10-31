pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineResultsRequest {
    #[serde(default)]
    pub pipeline_config_slugs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_expression: Option<String>,
    #[serde(skip)]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub to: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub page_size: Option<i64>,
    #[serde(skip)]
    pub cursor: Option<String>,
}

impl PipelineResultsRequest {
    pub fn builder() -> PipelineResultsRequestBuilder {
        <PipelineResultsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineResultsRequestBuilder {
    pipeline_config_slugs: Option<Vec<String>>,
    search_expression: Option<String>,
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl PipelineResultsRequestBuilder {
    pub fn pipeline_config_slugs(mut self, value: Vec<String>) -> Self {
        self.pipeline_config_slugs = Some(value);
        self
    }

    pub fn search_expression(mut self, value: impl Into<String>) -> Self {
        self.search_expression = Some(value.into());
        self
    }

    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineResultsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pipeline_config_slugs`](PipelineResultsRequestBuilder::pipeline_config_slugs)
    pub fn build(self) -> Result<PipelineResultsRequest, BuildError> {
        Ok(PipelineResultsRequest {
            pipeline_config_slugs: self
                .pipeline_config_slugs
                .ok_or_else(|| BuildError::missing_field("pipeline_config_slugs"))?,
            search_expression: self.search_expression,
            from: self.from,
            to: self.to,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
