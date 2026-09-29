pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TempoTraceScope {
    /// The name of the span
    #[serde(default)]
    pub name: String,
}

impl TempoTraceScope {
    pub fn builder() -> TempoTraceScopeBuilder {
        <TempoTraceScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceScopeBuilder {
    name: Option<String>,
}

impl TempoTraceScopeBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TempoTraceScopeBuilder::name)
    pub fn build(self) -> Result<TempoTraceScope, BuildError> {
        Ok(TempoTraceScope {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
