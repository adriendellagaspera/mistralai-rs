pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoiceUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl VoiceUpdateRequest {
    pub fn builder() -> VoiceUpdateRequestBuilder {
        <VoiceUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceUpdateRequestBuilder {
    age: Option<i64>,
    description: Option<String>,
    gender: Option<String>,
    languages: Option<Vec<String>>,
    name: Option<String>,
    tags: Option<Vec<String>>,
}

impl VoiceUpdateRequestBuilder {
    pub fn age(mut self, value: i64) -> Self {
        self.age = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(value.into());
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceUpdateRequest`].
    pub fn build(self) -> Result<VoiceUpdateRequest, BuildError> {
        Ok(VoiceUpdateRequest {
            age: self.age,
            description: self.description,
            gender: self.gender,
            languages: self.languages,
            name: self.name,
            tags: self.tags,
        })
    }
}
