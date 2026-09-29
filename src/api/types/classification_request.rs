pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassificationRequest {
    /// Text to classify.
    pub input: ClassificationRequestInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// ID of the model to use.
    #[serde(default)]
    pub model: String,
}

impl ClassificationRequest {
    pub fn builder() -> ClassificationRequestBuilder {
        <ClassificationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClassificationRequestBuilder {
    input: Option<ClassificationRequestInput>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    model: Option<String>,
}

impl ClassificationRequestBuilder {
    pub fn input(mut self, value: ClassificationRequestInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClassificationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](ClassificationRequestBuilder::input)
    /// - [`model`](ClassificationRequestBuilder::model)
    pub fn build(self) -> Result<ClassificationRequest, BuildError> {
        Ok(ClassificationRequest {
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            metadata: self.metadata,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
        })
    }
}
