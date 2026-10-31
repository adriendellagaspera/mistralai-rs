pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationsV2 {
    #[serde(default)]
    pub evaluations: PaginatedResultEvaluationV2,
}

impl EvaluationsV2 {
    pub fn builder() -> EvaluationsV2Builder {
        <EvaluationsV2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationsV2Builder {
    evaluations: Option<PaginatedResultEvaluationV2>,
}

impl EvaluationsV2Builder {
    pub fn evaluations(mut self, value: PaginatedResultEvaluationV2) -> Self {
        self.evaluations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationsV2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evaluations`](EvaluationsV2Builder::evaluations)
    pub fn build(self) -> Result<EvaluationsV2, BuildError> {
        Ok(EvaluationsV2 {
            evaluations: self
                .evaluations
                .ok_or_else(|| BuildError::missing_field("evaluations"))?,
        })
    }
}
