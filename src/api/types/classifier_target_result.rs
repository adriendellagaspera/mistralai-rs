pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassifierTargetResult {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub weight: f64,
    pub loss_function: FtClassifierLossFunction,
}

impl ClassifierTargetResult {
    pub fn builder() -> ClassifierTargetResultBuilder {
        <ClassifierTargetResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClassifierTargetResultBuilder {
    name: Option<String>,
    labels: Option<Vec<String>>,
    weight: Option<f64>,
    loss_function: Option<FtClassifierLossFunction>,
}

impl ClassifierTargetResultBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn weight(mut self, value: f64) -> Self {
        self.weight = Some(value);
        self
    }

    pub fn loss_function(mut self, value: FtClassifierLossFunction) -> Self {
        self.loss_function = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClassifierTargetResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ClassifierTargetResultBuilder::name)
    /// - [`labels`](ClassifierTargetResultBuilder::labels)
    /// - [`weight`](ClassifierTargetResultBuilder::weight)
    /// - [`loss_function`](ClassifierTargetResultBuilder::loss_function)
    pub fn build(self) -> Result<ClassifierTargetResult, BuildError> {
        Ok(ClassifierTargetResult {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            labels: self
                .labels
                .ok_or_else(|| BuildError::missing_field("labels"))?,
            weight: self
                .weight
                .ok_or_else(|| BuildError::missing_field("weight"))?,
            loss_function: self
                .loss_function
                .ok_or_else(|| BuildError::missing_field("loss_function"))?,
        })
    }
}
