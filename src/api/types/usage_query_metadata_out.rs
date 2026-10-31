pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsageQueryMetadataOut {
    /// Whether another page of grouped rows is available.
    #[serde(default)]
    pub has_more: bool,
    /// Normalized parameters used by the usage service.
    #[serde(default)]
    pub query_params: UsageQueryParamsOut,
}

impl UsageQueryMetadataOut {
    pub fn builder() -> UsageQueryMetadataOutBuilder {
        <UsageQueryMetadataOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageQueryMetadataOutBuilder {
    has_more: Option<bool>,
    query_params: Option<UsageQueryParamsOut>,
}

impl UsageQueryMetadataOutBuilder {
    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn query_params(mut self, value: UsageQueryParamsOut) -> Self {
        self.query_params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageQueryMetadataOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_more`](UsageQueryMetadataOutBuilder::has_more)
    /// - [`query_params`](UsageQueryMetadataOutBuilder::query_params)
    pub fn build(self) -> Result<UsageQueryMetadataOut, BuildError> {
        Ok(UsageQueryMetadataOut {
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
            query_params: self
                .query_params
                .ok_or_else(|| BuildError::missing_field("query_params"))?,
        })
    }
}
