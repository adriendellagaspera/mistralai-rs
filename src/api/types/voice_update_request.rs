pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoiceUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl VoiceUpdateRequest {
    pub fn builder() -> VoiceUpdateRequestBuilder {
        <VoiceUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceUpdateRequestBuilder {
    name: Option<String>,
    languages: Option<Vec<String>>,
    gender: Option<String>,
    age: Option<i64>,
    tags: Option<Vec<String>>,
    description: Option<String>,
}

impl VoiceUpdateRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoiceUpdateRequest`].
    pub fn build(self) -> Result<VoiceUpdateRequest, BuildError> {
        Ok(VoiceUpdateRequest {
            name: self.name,
            languages: self.languages,
            gender: self.gender,
            age: self.age,
            tags: self.tags,
            description: self.description,
        })
    }
}
