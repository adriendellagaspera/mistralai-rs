pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoiceCreateRequest {
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
    /// Base64-encoded audio file
    #[serde(default)]
    pub sample_audio: String,
    /// Original filename for extension detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_filename: Option<String>,
}

impl VoiceCreateRequest {
    pub fn builder() -> VoiceCreateRequestBuilder {
        <VoiceCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceCreateRequestBuilder {
    name: Option<String>,
    slug: Option<String>,
    languages: Option<Vec<String>>,
    gender: Option<String>,
    age: Option<i64>,
    tags: Option<Vec<String>>,
    color: Option<String>,
    description: Option<String>,
    retention_notice: Option<i64>,
    sample_audio: Option<String>,
    sample_filename: Option<String>,
}

impl VoiceCreateRequestBuilder {
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

    pub fn sample_audio(mut self, value: impl Into<String>) -> Self {
        self.sample_audio = Some(value.into());
        self
    }

    pub fn sample_filename(mut self, value: impl Into<String>) -> Self {
        self.sample_filename = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoiceCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](VoiceCreateRequestBuilder::name)
    /// - [`sample_audio`](VoiceCreateRequestBuilder::sample_audio)
    pub fn build(self) -> Result<VoiceCreateRequest, BuildError> {
        Ok(VoiceCreateRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug,
            languages: self.languages,
            gender: self.gender,
            age: self.age,
            tags: self.tags,
            color: self.color,
            description: self.description,
            retention_notice: self.retention_notice,
            sample_audio: self
                .sample_audio
                .ok_or_else(|| BuildError::missing_field("sample_audio"))?,
            sample_filename: self.sample_filename,
        })
    }
}
