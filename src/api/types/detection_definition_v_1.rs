pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DetectionDefinitionV1 {
    #[serde(default)]
    pub patterns: Vec<DetectionPattern>,
    #[serde(default)]
    pub target_attributes: Vec<String>,
}

impl DetectionDefinitionV1 {
    pub fn builder() -> DetectionDefinitionV1Builder {
        <DetectionDefinitionV1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectionDefinitionV1Builder {
    patterns: Option<Vec<DetectionPattern>>,
    target_attributes: Option<Vec<String>>,
}

impl DetectionDefinitionV1Builder {
    pub fn patterns(mut self, value: Vec<DetectionPattern>) -> Self {
        self.patterns = Some(value);
        self
    }

    pub fn target_attributes(mut self, value: Vec<String>) -> Self {
        self.target_attributes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DetectionDefinitionV1`].
    /// This method will fail if any of the following fields are not set:
    /// - [`patterns`](DetectionDefinitionV1Builder::patterns)
    /// - [`target_attributes`](DetectionDefinitionV1Builder::target_attributes)
    pub fn build(self) -> Result<DetectionDefinitionV1, BuildError> {
        Ok(DetectionDefinitionV1 {
            patterns: self
                .patterns
                .ok_or_else(|| BuildError::missing_field("patterns"))?,
            target_attributes: self
                .target_attributes
                .ok_or_else(|| BuildError::missing_field("target_attributes"))?,
        })
    }
}
