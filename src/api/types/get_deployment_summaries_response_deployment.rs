pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDeploymentSummariesResponseDeployment {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub creator_id: String,
    pub deployment: GetDeploymentSummariesResponseDeploymentDeployment,
    #[serde(default)]
    pub document_count: i64,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub modified_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub name: String,
    pub status: GetDeploymentSummariesResponseDeploymentStatus,
}

impl GetDeploymentSummariesResponseDeployment {
    pub fn builder() -> GetDeploymentSummariesResponseDeploymentBuilder {
        <GetDeploymentSummariesResponseDeploymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDeploymentSummariesResponseDeploymentBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    deployment: Option<GetDeploymentSummariesResponseDeploymentDeployment>,
    document_count: Option<i64>,
    id: Option<String>,
    modified_at: Option<DateTime<FixedOffset>>,
    name: Option<String>,
    status: Option<GetDeploymentSummariesResponseDeploymentStatus>,
}

impl GetDeploymentSummariesResponseDeploymentBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn deployment(mut self, value: GetDeploymentSummariesResponseDeploymentDeployment) -> Self {
        self.deployment = Some(value);
        self
    }

    pub fn document_count(mut self, value: i64) -> Self {
        self.document_count = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn modified_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetDeploymentSummariesResponseDeploymentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDeploymentSummariesResponseDeployment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](GetDeploymentSummariesResponseDeploymentBuilder::created_at)
    /// - [`creator_id`](GetDeploymentSummariesResponseDeploymentBuilder::creator_id)
    /// - [`deployment`](GetDeploymentSummariesResponseDeploymentBuilder::deployment)
    /// - [`document_count`](GetDeploymentSummariesResponseDeploymentBuilder::document_count)
    /// - [`id`](GetDeploymentSummariesResponseDeploymentBuilder::id)
    /// - [`modified_at`](GetDeploymentSummariesResponseDeploymentBuilder::modified_at)
    /// - [`name`](GetDeploymentSummariesResponseDeploymentBuilder::name)
    /// - [`status`](GetDeploymentSummariesResponseDeploymentBuilder::status)
    pub fn build(self) -> Result<GetDeploymentSummariesResponseDeployment, BuildError> {
        Ok(GetDeploymentSummariesResponseDeployment {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            deployment: self
                .deployment
                .ok_or_else(|| BuildError::missing_field("deployment"))?,
            document_count: self
                .document_count
                .ok_or_else(|| BuildError::missing_field("document_count"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
