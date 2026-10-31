pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DetectionPattern {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub confidence: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub regex: String,
}

impl DetectionPattern {
    pub fn builder() -> DetectionPatternBuilder {
        <DetectionPatternBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectionPatternBuilder {
    category: Option<String>,
    confidence: Option<String>,
    name: Option<String>,
    regex: Option<String>,
}

impl DetectionPatternBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: impl Into<String>) -> Self {
        self.confidence = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn regex(mut self, value: impl Into<String>) -> Self {
        self.regex = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DetectionPattern`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](DetectionPatternBuilder::category)
    /// - [`confidence`](DetectionPatternBuilder::confidence)
    /// - [`name`](DetectionPatternBuilder::name)
    /// - [`regex`](DetectionPatternBuilder::regex)
    pub fn build(self) -> Result<DetectionPattern, BuildError> {
        Ok(DetectionPattern {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            confidence: self
                .confidence
                .ok_or_else(|| BuildError::missing_field("confidence"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            regex: self
                .regex
                .ok_or_else(|| BuildError::missing_field("regex"))?,
        })
    }
}
