pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataFilter {
    #[serde(default)]
    pub key: String,
    pub operator: SystemParamOperator,
    pub value: MetadataFilterValue,
}

impl MetadataFilter {
    pub fn builder() -> MetadataFilterBuilder {
        <MetadataFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetadataFilterBuilder {
    key: Option<String>,
    operator: Option<SystemParamOperator>,
    value: Option<MetadataFilterValue>,
}

impl MetadataFilterBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn operator(mut self, value: SystemParamOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn value(mut self, value: MetadataFilterValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetadataFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](MetadataFilterBuilder::key)
    /// - [`operator`](MetadataFilterBuilder::operator)
    /// - [`value`](MetadataFilterBuilder::value)
    pub fn build(self) -> Result<MetadataFilter, BuildError> {
        Ok(MetadataFilter {
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
