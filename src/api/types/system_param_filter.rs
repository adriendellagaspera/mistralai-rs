pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemParamFilter {
    #[serde(default)]
    pub key: String,
    pub operator: SystemParamOperator,
    pub value: SystemParamFilterValue,
}

impl SystemParamFilter {
    pub fn builder() -> SystemParamFilterBuilder {
        <SystemParamFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemParamFilterBuilder {
    key: Option<String>,
    operator: Option<SystemParamOperator>,
    value: Option<SystemParamFilterValue>,
}

impl SystemParamFilterBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn operator(mut self, value: SystemParamOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn value(mut self, value: SystemParamFilterValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemParamFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](SystemParamFilterBuilder::key)
    /// - [`operator`](SystemParamFilterBuilder::operator)
    /// - [`value`](SystemParamFilterBuilder::value)
    pub fn build(self) -> Result<SystemParamFilter, BuildError> {
        Ok(SystemParamFilter {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
