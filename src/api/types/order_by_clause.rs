pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrderByClause {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<OrderByClauseDirection>,
}

impl OrderByClause {
    pub fn builder() -> OrderByClauseBuilder {
        <OrderByClauseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderByClauseBuilder {
    field: Option<String>,
    direction: Option<OrderByClauseDirection>,
}

impl OrderByClauseBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn direction(mut self, value: OrderByClauseDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrderByClause`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](OrderByClauseBuilder::field)
    pub fn build(self) -> Result<OrderByClause, BuildError> {
        Ok(OrderByClause {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            direction: self.direction,
        })
    }
}
