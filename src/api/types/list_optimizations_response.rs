pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOptimizationsResponse {
    #[serde(default)]
    pub optimizations: PaginatedResultOptimization,
}

impl ListOptimizationsResponse {
    pub fn builder() -> ListOptimizationsResponseBuilder {
        <ListOptimizationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOptimizationsResponseBuilder {
    optimizations: Option<PaginatedResultOptimization>,
}

impl ListOptimizationsResponseBuilder {
    pub fn optimizations(mut self, value: PaginatedResultOptimization) -> Self {
        self.optimizations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOptimizationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`optimizations`](ListOptimizationsResponseBuilder::optimizations)
    pub fn build(self) -> Result<ListOptimizationsResponse, BuildError> {
        Ok(ListOptimizationsResponse {
            optimizations: self
                .optimizations
                .ok_or_else(|| BuildError::missing_field("optimizations"))?,
        })
    }
}
