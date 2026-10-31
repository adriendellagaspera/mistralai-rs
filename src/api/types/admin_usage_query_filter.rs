pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdminUsageQueryFilter {
    /// Usage dimension to filter.
    pub field: AdminUsageQueryFilterField,
    /// Whether rows must match or must not match the supplied values.
    pub operator: AdminUsageQueryFilterOperator,
    /// Accepted or excluded values. Use null to filter missing dimension values.
    #[serde(default)]
    pub values: Vec<Option<String>>,
}

impl AdminUsageQueryFilter {
    pub fn builder() -> AdminUsageQueryFilterBuilder {
        <AdminUsageQueryFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUsageQueryFilterBuilder {
    field: Option<AdminUsageQueryFilterField>,
    operator: Option<AdminUsageQueryFilterOperator>,
    values: Option<Vec<Option<String>>>,
}

impl AdminUsageQueryFilterBuilder {
    pub fn field(mut self, value: AdminUsageQueryFilterField) -> Self {
        self.field = Some(value);
        self
    }

    pub fn operator(mut self, value: AdminUsageQueryFilterOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<Option<String>>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminUsageQueryFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AdminUsageQueryFilterBuilder::field)
    /// - [`operator`](AdminUsageQueryFilterBuilder::operator)
    /// - [`values`](AdminUsageQueryFilterBuilder::values)
    pub fn build(self) -> Result<AdminUsageQueryFilter, BuildError> {
        Ok(AdminUsageQueryFilter {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
            values: self
                .values
                .ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
