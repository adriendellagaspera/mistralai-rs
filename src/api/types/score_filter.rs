pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreFilter {
    #[serde(default)]
    pub evaluator_name: String,
    #[serde(default)]
    pub metric: String,
    pub operator: ScoreOperator,
    pub value: ScoreFilterValue,
}

impl ScoreFilter {
    pub fn builder() -> ScoreFilterBuilder {
        <ScoreFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScoreFilterBuilder {
    evaluator_name: Option<String>,
    metric: Option<String>,
    operator: Option<ScoreOperator>,
    value: Option<ScoreFilterValue>,
}

impl ScoreFilterBuilder {
    pub fn evaluator_name(mut self, value: impl Into<String>) -> Self {
        self.evaluator_name = Some(value.into());
        self
    }

    pub fn metric(mut self, value: impl Into<String>) -> Self {
        self.metric = Some(value.into());
        self
    }

    pub fn operator(mut self, value: ScoreOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn value(mut self, value: ScoreFilterValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScoreFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evaluator_name`](ScoreFilterBuilder::evaluator_name)
    /// - [`metric`](ScoreFilterBuilder::metric)
    /// - [`operator`](ScoreFilterBuilder::operator)
    /// - [`value`](ScoreFilterBuilder::value)
    pub fn build(self) -> Result<ScoreFilter, BuildError> {
        Ok(ScoreFilter {
            evaluator_name: self
                .evaluator_name
                .ok_or_else(|| BuildError::missing_field("evaluator_name"))?,
            metric: self
                .metric
                .ok_or_else(|| BuildError::missing_field("metric"))?,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
