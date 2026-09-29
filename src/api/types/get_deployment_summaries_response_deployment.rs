pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDeploymentSummariesResponseDeployment {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub creator_id: String,
    #[serde(default)]
    pub document_count: i64,
    pub status: GetDeploymentSummariesResponseDeploymentStatus,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub modified_at: DateTime<FixedOffset>,
    pub deployment: GetDeploymentSummariesResponseDeploymentDeployment,
}

impl GetDeploymentSummariesResponseDeployment {
    pub fn builder() -> GetDeploymentSummariesResponseDeploymentBuilder {
        <GetDeploymentSummariesResponseDeploymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDeploymentSummariesResponseDeploymentBuilder {
    id: Option<String>,
    name: Option<String>,
    creator_id: Option<String>,
    document_count: Option<i64>,
    status: Option<GetDeploymentSummariesResponseDeploymentStatus>,
    created_at: Option<DateTime<FixedOffset>>,
    modified_at: Option<DateTime<FixedOffset>>,
    deployment: Option<GetDeploymentSummariesResponseDeploymentDeployment>,
}

impl GetDeploymentSummariesResponseDeploymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn document_count(mut self, value: i64) -> Self {
        self.document_count = Some(value);
        self
    }

    pub fn status(mut self, value: GetDeploymentSummariesResponseDeploymentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn deployment(mut self, value: GetDeploymentSummariesResponseDeploymentDeployment) -> Self {
        self.deployment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDeploymentSummariesResponseDeployment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetDeploymentSummariesResponseDeploymentBuilder::id)
    /// - [`name`](GetDeploymentSummariesResponseDeploymentBuilder::name)
    /// - [`creator_id`](GetDeploymentSummariesResponseDeploymentBuilder::creator_id)
    /// - [`document_count`](GetDeploymentSummariesResponseDeploymentBuilder::document_count)
    /// - [`status`](GetDeploymentSummariesResponseDeploymentBuilder::status)
    /// - [`created_at`](GetDeploymentSummariesResponseDeploymentBuilder::created_at)
    /// - [`modified_at`](GetDeploymentSummariesResponseDeploymentBuilder::modified_at)
    /// - [`deployment`](GetDeploymentSummariesResponseDeploymentBuilder::deployment)
    pub fn build(self) -> Result<GetDeploymentSummariesResponseDeployment, BuildError> {
        Ok(GetDeploymentSummariesResponseDeployment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            document_count: self
                .document_count
                .ok_or_else(|| BuildError::missing_field("document_count"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            deployment: self
                .deployment
                .ok_or_else(|| BuildError::missing_field("deployment"))?,
        })
    }
}
