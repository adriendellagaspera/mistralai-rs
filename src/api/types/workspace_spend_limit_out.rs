pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceSpendLimitOut {
    /// Currency used for the Workspace spending limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Whether the Workspace has no monthly spending limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_monthly_limit: Option<bool>,
    /// Monthly spending limit for the Workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
}

impl WorkspaceSpendLimitOut {
    pub fn builder() -> WorkspaceSpendLimitOutBuilder {
        <WorkspaceSpendLimitOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceSpendLimitOutBuilder {
    currency: Option<String>,
    no_monthly_limit: Option<bool>,
    value: Option<f64>,
}

impl WorkspaceSpendLimitOutBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn no_monthly_limit(mut self, value: bool) -> Self {
        self.no_monthly_limit = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceSpendLimitOut`].
    pub fn build(self) -> Result<WorkspaceSpendLimitOut, BuildError> {
        Ok(WorkspaceSpendLimitOut {
            currency: self.currency,
            no_monthly_limit: self.no_monthly_limit,
            value: self.value,
        })
    }
}
