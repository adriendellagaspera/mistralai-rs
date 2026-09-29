pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Campaign {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub id: String,
    pub judge: Judge,
    #[serde(default)]
    pub max_nb_events: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub owner_id: String,
    #[serde(default)]
    pub search_params: FilterPayload,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub workspace_id: String,
}

impl Campaign {
    pub fn builder() -> CampaignBuilder {
        <CampaignBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    judge: Option<Judge>,
    max_nb_events: Option<i64>,
    name: Option<String>,
    owner_id: Option<String>,
    search_params: Option<FilterPayload>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl CampaignBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn judge(mut self, value: Judge) -> Self {
        self.judge = Some(value);
        self
    }

    pub fn max_nb_events(mut self, value: i64) -> Self {
        self.max_nb_events = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn search_params(mut self, value: FilterPayload) -> Self {
        self.search_params = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Campaign`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CampaignBuilder::created_at)
    /// - [`description`](CampaignBuilder::description)
    /// - [`id`](CampaignBuilder::id)
    /// - [`judge`](CampaignBuilder::judge)
    /// - [`max_nb_events`](CampaignBuilder::max_nb_events)
    /// - [`name`](CampaignBuilder::name)
    /// - [`owner_id`](CampaignBuilder::owner_id)
    /// - [`search_params`](CampaignBuilder::search_params)
    /// - [`updated_at`](CampaignBuilder::updated_at)
    /// - [`workspace_id`](CampaignBuilder::workspace_id)
    pub fn build(self) -> Result<Campaign, BuildError> {
        Ok(Campaign {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            judge: self
                .judge
                .ok_or_else(|| BuildError::missing_field("judge"))?,
            max_nb_events: self
                .max_nb_events
                .ok_or_else(|| BuildError::missing_field("max_nb_events"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            owner_id: self
                .owner_id
                .ok_or_else(|| BuildError::missing_field("owner_id"))?,
            search_params: self
                .search_params
                .ok_or_else(|| BuildError::missing_field("search_params"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
