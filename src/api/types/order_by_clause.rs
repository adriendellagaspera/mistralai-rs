pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrderByClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<OrderByClauseDirection>,
    #[serde(default)]
    pub field: String,
}

impl OrderByClause {
    pub fn builder() -> OrderByClauseBuilder {
        <OrderByClauseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderByClauseBuilder {
    direction: Option<OrderByClauseDirection>,
    field: Option<String>,
}

impl OrderByClauseBuilder {
    pub fn direction(mut self, value: OrderByClauseDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrderByClause`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrderByClauseBuilder::field)
    pub fn build(self) -> Result<OrderByClause, BuildError> {
        Ok(OrderByClause {
            direction: self.direction,
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
        })
    }
}
