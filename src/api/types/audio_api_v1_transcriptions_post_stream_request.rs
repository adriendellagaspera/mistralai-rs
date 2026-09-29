pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioApiV1TranscriptionsPostStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_bias: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarize: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
}
impl AudioApiV1TranscriptionsPostStreamRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref value) = self.context_bias {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("context_bias", json_str);
            }
        }

        if let Some(ref value) = self.diarize {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("diarize", json_str);
            }
        }

        if let Some(ref value) = self.file {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("file", json_str);
            }
        }

        if let Some(ref value) = self.file_id {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("file_id", json_str);
            }
        }

        if let Some(ref value) = self.file_url {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("file_url", json_str);
            }
        }

        if let Some(ref value) = self.language {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("language", json_str);
            }
        }

        form = form.text("model", self.model.clone());

        if let Some(ref value) = self.stream {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("stream", json_str);
            }
        }

        if let Some(ref value) = self.temperature {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("temperature", json_str);
            }
        }

        if let Some(ref value) = self.timestamp_granularities {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("timestamp_granularities", json_str);
            }
        }

        form
    }
}

impl AudioApiV1TranscriptionsPostStreamRequest {
    pub fn builder() -> AudioApiV1TranscriptionsPostStreamRequestBuilder {
        <AudioApiV1TranscriptionsPostStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioApiV1TranscriptionsPostStreamRequestBuilder {
    context_bias: Option<Vec<String>>,
    diarize: Option<bool>,
    file: Option<File>,
    file_id: Option<String>,
    file_url: Option<String>,
    language: Option<String>,
    model: Option<String>,
    stream: Option<bool>,
    temperature: Option<f64>,
    timestamp_granularities: Option<Vec<TimestampGranularity>>,
}

impl AudioApiV1TranscriptionsPostStreamRequestBuilder {
    pub fn context_bias(mut self, value: Vec<String>) -> Self {
        self.context_bias = Some(value);
        self
    }

    pub fn diarize(mut self, value: bool) -> Self {
        self.diarize = Some(value);
        self
    }

    pub fn file(mut self, value: File) -> Self {
        self.file = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn timestamp_granularities(mut self, value: Vec<TimestampGranularity>) -> Self {
        self.timestamp_granularities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioApiV1TranscriptionsPostStreamRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](AudioApiV1TranscriptionsPostStreamRequestBuilder::model)
    pub fn build(self) -> Result<AudioApiV1TranscriptionsPostStreamRequest, BuildError> {
        Ok(AudioApiV1TranscriptionsPostStreamRequest {
            context_bias: self.context_bias,
            diarize: self.diarize,
            file: self.file,
            file_id: self.file_id,
            file_url: self.file_url,
            language: self.language,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            stream: self.stream,
            temperature: self.temperature,
            timestamp_granularities: self.timestamp_granularities,
        })
    }
}
