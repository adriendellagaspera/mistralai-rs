pub use crate::prelude::*;

/// A single scalar computed run-level statistic: its canonical id and numeric value.
///
/// ``value`` is ``None`` when the statistic is not evaluable (e.g. a ``sum`` or percentile over an
/// empty numeric sample). A declared ``count`` is always evaluable and is ``0`` on an empty sample.
///
/// The ``type`` discriminator lets future result shapes (e.g. a categorical ``categories`` or a
/// ``distribution``) join a union without overloading this model; consumers dispatch on ``type``.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScalarStatisticValue {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ScalarStatisticValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl ScalarStatisticValue {
    pub fn builder() -> ScalarStatisticValueBuilder {
        <ScalarStatisticValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScalarStatisticValueBuilder {
    id: Option<String>,
    r#type: Option<ScalarStatisticValueType>,
    value: Option<f64>,
}

impl ScalarStatisticValueBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ScalarStatisticValueType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScalarStatisticValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ScalarStatisticValueBuilder::id)
    pub fn build(self) -> Result<ScalarStatisticValue, BuildError> {
        Ok(ScalarStatisticValue {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self.r#type,
            value: self.value,
        })
    }
}
