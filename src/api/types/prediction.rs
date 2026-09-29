pub use crate::prelude::*;

/// Enable users to specify an expected completion, optimizing response times by leveraging known or predictable content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Prediction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PredictionType>,
}

impl Prediction {
    pub fn builder() -> PredictionBuilder {
        <PredictionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PredictionBuilder {
    content: Option<String>,
    r#type: Option<PredictionType>,
}

impl PredictionBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PredictionType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Prediction`].
    pub fn build(self) -> Result<Prediction, BuildError> {
        Ok(Prediction {
            content: self.content,
            r#type: self.r#type,
        })
    }
}
