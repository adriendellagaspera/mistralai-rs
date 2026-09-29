pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCampaignRequest {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub judge_id: String,
    #[serde(default)]
    pub max_nb_events: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub search_params: FilterPayload,
}

impl CreateCampaignRequest {
    pub fn builder() -> CreateCampaignRequestBuilder {
        <CreateCampaignRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCampaignRequestBuilder {
    description: Option<String>,
    judge_id: Option<String>,
    max_nb_events: Option<i64>,
    name: Option<String>,
    search_params: Option<FilterPayload>,
}

impl CreateCampaignRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn judge_id(mut self, value: impl Into<String>) -> Self {
        self.judge_id = Some(value.into());
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

    pub fn search_params(mut self, value: FilterPayload) -> Self {
        self.search_params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCampaignRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](CreateCampaignRequestBuilder::description)
    /// - [`judge_id`](CreateCampaignRequestBuilder::judge_id)
    /// - [`max_nb_events`](CreateCampaignRequestBuilder::max_nb_events)
    /// - [`name`](CreateCampaignRequestBuilder::name)
    /// - [`search_params`](CreateCampaignRequestBuilder::search_params)
    pub fn build(self) -> Result<CreateCampaignRequest, BuildError> {
        Ok(CreateCampaignRequest {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            judge_id: self
                .judge_id
                .ok_or_else(|| BuildError::missing_field("judge_id"))?,
            max_nb_events: self
                .max_nb_events
                .ok_or_else(|| BuildError::missing_field("max_nb_events"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            search_params: self
                .search_params
                .ok_or_else(|| BuildError::missing_field("search_params"))?,
        })
    }
}
