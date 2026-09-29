pub use crate::prelude::*;

/// Enable users to specify an expected completion, optimizing response times by leveraging known or predictable content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Prediction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PredictionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl Prediction {
    pub fn builder() -> PredictionBuilder {
        <PredictionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PredictionBuilder {
    r#type: Option<PredictionType>,
    content: Option<String>,
}

impl PredictionBuilder {
    pub fn r#type(mut self, value: PredictionType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Prediction`].
    pub fn build(self) -> Result<Prediction, BuildError> {
        Ok(Prediction {
            r#type: self.r#type,
            content: self.content,
        })
    }
}
