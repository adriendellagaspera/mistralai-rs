pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOptimizationTrialsResponse {
    #[serde(default)]
    pub trials: PaginatedResultOptimizationTrialWithObservations,
}

impl ListOptimizationTrialsResponse {
    pub fn builder() -> ListOptimizationTrialsResponseBuilder {
        <ListOptimizationTrialsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOptimizationTrialsResponseBuilder {
    trials: Option<PaginatedResultOptimizationTrialWithObservations>,
}

impl ListOptimizationTrialsResponseBuilder {
    pub fn trials(mut self, value: PaginatedResultOptimizationTrialWithObservations) -> Self {
        self.trials = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOptimizationTrialsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trials`](ListOptimizationTrialsResponseBuilder::trials)
    pub fn build(self) -> Result<ListOptimizationTrialsResponse, BuildError> {
        Ok(ListOptimizationTrialsResponse {
            trials: self
                .trials
                .ok_or_else(|| BuildError::missing_field("trials"))?,
        })
    }
}
