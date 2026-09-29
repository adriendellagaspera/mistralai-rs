pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClassificationTargetResult {
    #[serde(default)]
    pub scores: HashMap<String, f64>,
}

impl ClassificationTargetResult {
    pub fn builder() -> ClassificationTargetResultBuilder {
        <ClassificationTargetResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClassificationTargetResultBuilder {
    scores: Option<HashMap<String, f64>>,
}

impl ClassificationTargetResultBuilder {
    pub fn scores(mut self, value: HashMap<String, f64>) -> Self {
        self.scores = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClassificationTargetResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scores`](ClassificationTargetResultBuilder::scores)
    pub fn build(self) -> Result<ClassificationTargetResult, BuildError> {
        Ok(ClassificationTargetResult {
            scores: self
                .scores
                .ok_or_else(|| BuildError::missing_field("scores"))?,
        })
    }
}
