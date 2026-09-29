pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TokenLimitsByModel {
    /// Maximum tokens allowed per minute.
    #[serde(default)]
    pub tokens_per_minute: i64,
    /// Maximum tokens allowed per month.
    #[serde(default)]
    pub tokens_per_month: i64,
}

impl TokenLimitsByModel {
    pub fn builder() -> TokenLimitsByModelBuilder {
        <TokenLimitsByModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TokenLimitsByModelBuilder {
    tokens_per_minute: Option<i64>,
    tokens_per_month: Option<i64>,
}

impl TokenLimitsByModelBuilder {
    pub fn tokens_per_minute(mut self, value: i64) -> Self {
        self.tokens_per_minute = Some(value);
        self
    }

    pub fn tokens_per_month(mut self, value: i64) -> Self {
        self.tokens_per_month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TokenLimitsByModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tokens_per_minute`](TokenLimitsByModelBuilder::tokens_per_minute)
    /// - [`tokens_per_month`](TokenLimitsByModelBuilder::tokens_per_month)
    pub fn build(self) -> Result<TokenLimitsByModel, BuildError> {
        Ok(TokenLimitsByModel {
            tokens_per_minute: self
                .tokens_per_minute
                .ok_or_else(|| BuildError::missing_field("tokens_per_minute"))?,
            tokens_per_month: self
                .tokens_per_month
                .ok_or_else(|| BuildError::missing_field("tokens_per_month"))?,
        })
    }
}
