pub use crate::prelude::*;

/// Query parameters for audio_v2_voices_list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioV2VoicesListQueryRequest {
    /// Maximum number of voices to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Cursor returned as next_page_token by the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// Filter the voices between customs and presets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AudioV2VoicesListVoicesRequestType>,
    /// Keep voices matching any of these genders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Vec<VoiceGender>>,
    /// Keep voices supporting any of these languages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,
    /// Case-insensitive match on voice name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl AudioV2VoicesListQueryRequest {
    pub fn builder() -> AudioV2VoicesListQueryRequestBuilder {
        <AudioV2VoicesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioV2VoicesListQueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
    r#type: Option<AudioV2VoicesListVoicesRequestType>,
    gender: Option<Vec<VoiceGender>>,
    language: Option<Vec<String>>,
    query: Option<String>,
}

impl AudioV2VoicesListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AudioV2VoicesListVoicesRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn gender(mut self, value: Vec<VoiceGender>) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn language(mut self, value: Vec<String>) -> Self {
        self.language = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioV2VoicesListQueryRequest`].
    pub fn build(self) -> Result<AudioV2VoicesListQueryRequest, BuildError> {
        Ok(AudioV2VoicesListQueryRequest {
            page_size: self.page_size,
            page_token: self.page_token,
            r#type: self.r#type,
            gender: self.gender,
            language: self.language,
            query: self.query,
        })
    }
}
