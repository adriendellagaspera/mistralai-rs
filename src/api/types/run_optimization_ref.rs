pub use crate::prelude::*;

/// Typed run→optimization back-ref, derived at read from the optimization_run link.
///
/// Not stored on the run (single source of truth = optimization_run); lets a run row show
/// its optimization (id + name) without an extra request per run.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunOptimizationRef {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub optimization_id: String,
}

impl RunOptimizationRef {
    pub fn builder() -> RunOptimizationRefBuilder {
        <RunOptimizationRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunOptimizationRefBuilder {
    name: Option<String>,
    optimization_id: Option<String>,
}

impl RunOptimizationRefBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn optimization_id(mut self, value: impl Into<String>) -> Self {
        self.optimization_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunOptimizationRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RunOptimizationRefBuilder::name)
    /// - [`optimization_id`](RunOptimizationRefBuilder::optimization_id)
    pub fn build(self) -> Result<RunOptimizationRef, BuildError> {
        Ok(RunOptimizationRef {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            optimization_id: self
                .optimization_id
                .ok_or_else(|| BuildError::missing_field("optimization_id"))?,
        })
    }
}
