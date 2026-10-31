pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GoalSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<JsonValue>>,
}

impl GoalSpec {
    pub fn builder() -> GoalSpecBuilder {
        <GoalSpecBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GoalSpecBuilder {
    max: Option<f64>,
    metric: Option<String>,
    min: Option<f64>,
    operator: Option<String>,
    value: Option<JsonValue>,
    values: Option<Vec<JsonValue>>,
}

impl GoalSpecBuilder {
    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn metric(mut self, value: impl Into<String>) -> Self {
        self.metric = Some(value.into());
        self
    }

    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    pub fn operator(mut self, value: impl Into<String>) -> Self {
        self.operator = Some(value.into());
        self
    }

    pub fn value(mut self, value: JsonValue) -> Self {
        self.value = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<JsonValue>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GoalSpec`].
    pub fn build(self) -> Result<GoalSpec, BuildError> {
        Ok(GoalSpec {
            max: self.max,
            metric: self.metric,
            min: self.min,
            operator: self.operator,
            value: self.value,
            values: self.values,
        })
    }
}
