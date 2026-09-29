pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassificationRequest {
    /// ID of the model to use.
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Text to classify.
    pub input: ClassificationRequestInput,
}

impl ClassificationRequest {
    pub fn builder() -> ClassificationRequestBuilder {
        <ClassificationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClassificationRequestBuilder {
    model: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    input: Option<ClassificationRequestInput>,
}

impl ClassificationRequestBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn input(mut self, value: ClassificationRequestInput) -> Self {
        self.input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClassificationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](ClassificationRequestBuilder::model)
    /// - [`input`](ClassificationRequestBuilder::input)
    pub fn build(self) -> Result<ClassificationRequest, BuildError> {
        Ok(ClassificationRequest {
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            metadata: self.metadata,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
        })
    }
}
