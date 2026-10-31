pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PipelineResult {
    #[serde(default)]
    pub duration_ns: i64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub evaluated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub judge_explanations: HashMap<String, String>,
    #[serde(default)]
    pub judge_labels: HashMap<String, String>,
    #[serde(default)]
    pub judge_scores: HashMap<String, f64>,
    #[serde(default)]
    pub span_id: String,
    #[serde(default)]
    pub span_name: String,
    #[serde(default)]
    pub trace_id: String,
}

impl PipelineResult {
    pub fn builder() -> PipelineResultBuilder {
        <PipelineResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineResultBuilder {
    duration_ns: Option<i64>,
    evaluated_at: Option<DateTime<FixedOffset>>,
    judge_explanations: Option<HashMap<String, String>>,
    judge_labels: Option<HashMap<String, String>>,
    judge_scores: Option<HashMap<String, f64>>,
    span_id: Option<String>,
    span_name: Option<String>,
    trace_id: Option<String>,
}

impl PipelineResultBuilder {
    pub fn duration_ns(mut self, value: i64) -> Self {
        self.duration_ns = Some(value);
        self
    }

    pub fn evaluated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.evaluated_at = Some(value);
        self
    }

    pub fn judge_explanations(mut self, value: HashMap<String, String>) -> Self {
        self.judge_explanations = Some(value);
        self
    }

    pub fn judge_labels(mut self, value: HashMap<String, String>) -> Self {
        self.judge_labels = Some(value);
        self
    }

    pub fn judge_scores(mut self, value: HashMap<String, f64>) -> Self {
        self.judge_scores = Some(value);
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn span_name(mut self, value: impl Into<String>) -> Self {
        self.span_name = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`duration_ns`](PipelineResultBuilder::duration_ns)
    /// - [`evaluated_at`](PipelineResultBuilder::evaluated_at)
    /// - [`judge_explanations`](PipelineResultBuilder::judge_explanations)
    /// - [`judge_labels`](PipelineResultBuilder::judge_labels)
    /// - [`judge_scores`](PipelineResultBuilder::judge_scores)
    /// - [`span_id`](PipelineResultBuilder::span_id)
    /// - [`span_name`](PipelineResultBuilder::span_name)
    /// - [`trace_id`](PipelineResultBuilder::trace_id)
    pub fn build(self) -> Result<PipelineResult, BuildError> {
        Ok(PipelineResult {
            duration_ns: self
                .duration_ns
                .ok_or_else(|| BuildError::missing_field("duration_ns"))?,
            evaluated_at: self
                .evaluated_at
                .ok_or_else(|| BuildError::missing_field("evaluated_at"))?,
            judge_explanations: self
                .judge_explanations
                .ok_or_else(|| BuildError::missing_field("judge_explanations"))?,
            judge_labels: self
                .judge_labels
                .ok_or_else(|| BuildError::missing_field("judge_labels"))?,
            judge_scores: self
                .judge_scores
                .ok_or_else(|| BuildError::missing_field("judge_scores"))?,
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            span_name: self
                .span_name
                .ok_or_else(|| BuildError::missing_field("span_name"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
        })
    }
}
