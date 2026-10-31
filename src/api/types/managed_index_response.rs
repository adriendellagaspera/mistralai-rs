pub use crate::prelude::*;

/// Wire representation of a managed index (decoupled from the domain model).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManagedIndexResponse {
    pub config: ManagedIndexConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub creator_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub schema: ManagedIndexFields,
    pub status: ManagedIndexStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

impl ManagedIndexResponse {
    pub fn builder() -> ManagedIndexResponseBuilder {
        <ManagedIndexResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManagedIndexResponseBuilder {
    config: Option<ManagedIndexConfig>,
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    id: Option<String>,
    modified_at: Option<DateTime<FixedOffset>>,
    name: Option<String>,
    schema: Option<ManagedIndexFields>,
    status: Option<ManagedIndexStatus>,
    status_message: Option<String>,
}

impl ManagedIndexResponseBuilder {
    pub fn config(mut self, value: ManagedIndexConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
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

    pub fn schema(mut self, value: ManagedIndexFields) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn status(mut self, value: ManagedIndexStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_message(mut self, value: impl Into<String>) -> Self {
        self.status_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ManagedIndexResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](ManagedIndexResponseBuilder::config)
    /// - [`creator_id`](ManagedIndexResponseBuilder::creator_id)
    /// - [`id`](ManagedIndexResponseBuilder::id)
    /// - [`name`](ManagedIndexResponseBuilder::name)
    /// - [`schema`](ManagedIndexResponseBuilder::schema)
    /// - [`status`](ManagedIndexResponseBuilder::status)
    pub fn build(self) -> Result<ManagedIndexResponse, BuildError> {
        Ok(ManagedIndexResponse {
            config: self
                .config
                .ok_or_else(|| BuildError::missing_field("config"))?,
            created_at: self.created_at,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            modified_at: self.modified_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            schema: self
                .schema
                .ok_or_else(|| BuildError::missing_field("schema"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            status_message: self.status_message,
        })
    }
}
