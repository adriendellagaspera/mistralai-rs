pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClassificationResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub results: Vec<HashMap<String, ClassificationTargetResult>>,
}

impl ClassificationResponse {
    pub fn builder() -> ClassificationResponseBuilder {
        <ClassificationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClassificationResponseBuilder {
    id: Option<String>,
    model: Option<String>,
    results: Option<Vec<HashMap<String, ClassificationTargetResult>>>,
}

impl ClassificationResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<HashMap<String, ClassificationTargetResult>>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClassificationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ClassificationResponseBuilder::id)
    /// - [`model`](ClassificationResponseBuilder::model)
    /// - [`results`](ClassificationResponseBuilder::results)
    pub fn build(self) -> Result<ClassificationResponse, BuildError> {
        Ok(ClassificationResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            results: self
                .results
                .ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
