pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassifierTargetResult {
    #[serde(default)]
    pub labels: Vec<String>,
    pub loss_function: FtClassifierLossFunction,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub weight: f64,
}

impl ClassifierTargetResult {
    pub fn builder() -> ClassifierTargetResultBuilder {
        <ClassifierTargetResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClassifierTargetResultBuilder {
    labels: Option<Vec<String>>,
    loss_function: Option<FtClassifierLossFunction>,
    name: Option<String>,
    weight: Option<f64>,
}

impl ClassifierTargetResultBuilder {
    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn loss_function(mut self, value: FtClassifierLossFunction) -> Self {
        self.loss_function = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn weight(mut self, value: f64) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClassifierTargetResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`labels`](ClassifierTargetResultBuilder::labels)
    /// - [`loss_function`](ClassifierTargetResultBuilder::loss_function)
    /// - [`name`](ClassifierTargetResultBuilder::name)
    /// - [`weight`](ClassifierTargetResultBuilder::weight)
    pub fn build(self) -> Result<ClassifierTargetResult, BuildError> {
        Ok(ClassifierTargetResult {
            labels: self
                .labels
                .ok_or_else(|| BuildError::missing_field("labels"))?,
            loss_function: self
                .loss_function
                .ok_or_else(|| BuildError::missing_field("loss_function"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            weight: self
                .weight
                .ok_or_else(|| BuildError::missing_field("weight"))?,
        })
    }
}
