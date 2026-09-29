pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModerationResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub results: Vec<ModerationObject>,
}

impl ModerationResponse {
    pub fn builder() -> ModerationResponseBuilder {
        <ModerationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerationResponseBuilder {
    id: Option<String>,
    model: Option<String>,
    results: Option<Vec<ModerationObject>>,
}

impl ModerationResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<ModerationObject>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ModerationResponseBuilder::id)
    /// - [`model`](ModerationResponseBuilder::model)
    /// - [`results`](ModerationResponseBuilder::results)
    pub fn build(self) -> Result<ModerationResponse, BuildError> {
        Ok(ModerationResponse {
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
