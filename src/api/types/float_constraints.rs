pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FloatConstraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

impl FloatConstraints {
    pub fn builder() -> FloatConstraintsBuilder {
        <FloatConstraintsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FloatConstraintsBuilder {
    max: Option<f64>,
    min: Option<f64>,
}

impl FloatConstraintsBuilder {
    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FloatConstraints`].
    pub fn build(self) -> Result<FloatConstraints, BuildError> {
        Ok(FloatConstraints {
            max: self.max,
            min: self.min,
        })
    }
}
