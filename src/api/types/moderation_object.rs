pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModerationObject {
    /// Moderation result thresholds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<HashMap<String, bool>>,
    /// Moderation result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_scores: Option<HashMap<String, f64>>,
}

impl ModerationObject {
    pub fn builder() -> ModerationObjectBuilder {
        <ModerationObjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerationObjectBuilder {
    categories: Option<HashMap<String, bool>>,
    category_scores: Option<HashMap<String, f64>>,
}

impl ModerationObjectBuilder {
    pub fn categories(mut self, value: HashMap<String, bool>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn category_scores(mut self, value: HashMap<String, f64>) -> Self {
        self.category_scores = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerationObject`].
    pub fn build(self) -> Result<ModerationObject, BuildError> {
        Ok(ModerationObject {
            categories: self.categories,
            category_scores: self.category_scores,
        })
    }
}
