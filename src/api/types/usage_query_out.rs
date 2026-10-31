pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsageQueryOut {
    /// Aggregates over the complete filtered result set when requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregates: Option<UsageQueryAggregatesOut>,
    /// Grouped usage rows.
    #[serde(default)]
    pub groups: Vec<UsageQueryRowOut>,
    /// Query parameters and pagination metadata.
    #[serde(default)]
    pub metadata: UsageQueryMetadataOut,
}

impl UsageQueryOut {
    pub fn builder() -> UsageQueryOutBuilder {
        <UsageQueryOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageQueryOutBuilder {
    aggregates: Option<UsageQueryAggregatesOut>,
    groups: Option<Vec<UsageQueryRowOut>>,
    metadata: Option<UsageQueryMetadataOut>,
}

impl UsageQueryOutBuilder {
    pub fn aggregates(mut self, value: UsageQueryAggregatesOut) -> Self {
        self.aggregates = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<UsageQueryRowOut>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn metadata(mut self, value: UsageQueryMetadataOut) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageQueryOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`groups`](UsageQueryOutBuilder::groups)
    /// - [`metadata`](UsageQueryOutBuilder::metadata)
    pub fn build(self) -> Result<UsageQueryOut, BuildError> {
        Ok(UsageQueryOut {
            aggregates: self.aggregates,
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
        })
    }
}
