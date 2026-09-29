pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterCondition {
    #[serde(default)]
    pub field: String,
    pub op: FilterConditionOp,
    pub value: serde_json::Value,
}

impl FilterCondition {
    pub fn builder() -> FilterConditionBuilder {
        <FilterConditionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilterConditionBuilder {
    field: Option<String>,
    op: Option<FilterConditionOp>,
    value: Option<serde_json::Value>,
}

impl FilterConditionBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: FilterConditionOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FilterCondition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](FilterConditionBuilder::field)
    /// - [`op`](FilterConditionBuilder::op)
    /// - [`value`](FilterConditionBuilder::value)
    pub fn build(self) -> Result<FilterCondition, BuildError> {
        Ok(FilterCondition {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
