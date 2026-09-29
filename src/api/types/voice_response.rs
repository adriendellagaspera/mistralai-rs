pub use crate::prelude::*;

/// Schema for voice response
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceResponse {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_notice: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trimmed_seconds: Option<f64>,
}

impl VoiceResponse {
    pub fn builder() -> VoiceResponseBuilder {
        <VoiceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceResponseBuilder {
    name: Option<String>,
    slug: Option<String>,
    languages: Option<Vec<String>>,
    gender: Option<String>,
    age: Option<i64>,
    tags: Option<Vec<String>>,
    color: Option<String>,
    description: Option<String>,
    retention_notice: Option<i64>,
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    user_id: Option<String>,
    trimmed_seconds: Option<f64>,
}

impl VoiceResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(value.into());
        self
    }

    pub fn age(mut self, value: i64) -> Self {
        self.age = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn retention_notice(mut self, value: i64) -> Self {
        self.retention_notice = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn trimmed_seconds(mut self, value: f64) -> Self {
        self.trimmed_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](VoiceResponseBuilder::name)
    /// - [`id`](VoiceResponseBuilder::id)
    /// - [`created_at`](VoiceResponseBuilder::created_at)
    pub fn build(self) -> Result<VoiceResponse, BuildError> {
        Ok(VoiceResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug,
            languages: self.languages,
            gender: self.gender,
            age: self.age,
            tags: self.tags,
            color: self.color,
            description: self.description,
            retention_notice: self.retention_notice,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            user_id: self.user_id,
            trimmed_seconds: self.trimmed_seconds,
        })
    }
}
