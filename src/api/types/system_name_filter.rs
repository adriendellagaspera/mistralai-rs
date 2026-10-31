pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SystemNameFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<ContentSearchOperator>,
    #[serde(default)]
    pub value: String,
}

impl SystemNameFilter {
    pub fn builder() -> SystemNameFilterBuilder {
        <SystemNameFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemNameFilterBuilder {
    operator: Option<ContentSearchOperator>,
    value: Option<String>,
}

impl SystemNameFilterBuilder {
    pub fn operator(mut self, value: ContentSearchOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SystemNameFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](SystemNameFilterBuilder::value)
    pub fn build(self) -> Result<SystemNameFilter, BuildError> {
        Ok(SystemNameFilter {
            operator: self.operator,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
