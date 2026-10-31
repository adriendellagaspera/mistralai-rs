pub use crate::prelude::*;

/// A confusion matrix over ``(expected, predicted)`` label pairs.
///
/// ``matrix[i][j]`` is the number of samples whose expected label is ``labels[i]`` and predicted
/// label is ``labels[j]``. It is the complete, aggregatable representation: precision/recall/f1 for
/// any class derive from it, and matrices from several runs merge by element-wise sum.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfusionMatrix {
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub matrix: Vec<Vec<i64>>,
}

impl ConfusionMatrix {
    pub fn builder() -> ConfusionMatrixBuilder {
        <ConfusionMatrixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfusionMatrixBuilder {
    labels: Option<Vec<String>>,
    matrix: Option<Vec<Vec<i64>>>,
}

impl ConfusionMatrixBuilder {
    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn matrix(mut self, value: Vec<Vec<i64>>) -> Self {
        self.matrix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfusionMatrix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`labels`](ConfusionMatrixBuilder::labels)
    /// - [`matrix`](ConfusionMatrixBuilder::matrix)
    pub fn build(self) -> Result<ConfusionMatrix, BuildError> {
        Ok(ConfusionMatrix {
            labels: self
                .labels
                .ok_or_else(|| BuildError::missing_field("labels"))?,
            matrix: self
                .matrix
                .ok_or_else(|| BuildError::missing_field("matrix"))?,
        })
    }
}
