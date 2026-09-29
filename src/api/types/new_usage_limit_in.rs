pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NewUsageLimitIn {
    /// New monthly usage limit amount.
    #[serde(default)]
    pub amount: i64,
    /// Whether to remove the monthly usage limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_monthly_limit: Option<bool>,
}

impl NewUsageLimitIn {
    pub fn builder() -> NewUsageLimitInBuilder {
        <NewUsageLimitInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NewUsageLimitInBuilder {
    amount: Option<i64>,
    no_monthly_limit: Option<bool>,
}

impl NewUsageLimitInBuilder {
    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn no_monthly_limit(mut self, value: bool) -> Self {
        self.no_monthly_limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NewUsageLimitIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](NewUsageLimitInBuilder::amount)
    pub fn build(self) -> Result<NewUsageLimitIn, BuildError> {
        Ok(NewUsageLimitIn {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            no_monthly_limit: self.no_monthly_limit,
        })
    }
}
