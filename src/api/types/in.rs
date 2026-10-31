pub use crate::prelude::*;

/// Field is one of a set of values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct In {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub values: Vec<InValuesItem>,
}

impl In {
    pub fn builder() -> InBuilder {
        <InBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InBuilder {
    field: Option<String>,
    values: Option<Vec<InValuesItem>>,
}

impl InBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn values(mut self, value: Vec<InValuesItem>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`In`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](InBuilder::field)
    /// - [`values`](InBuilder::values)
    pub fn build(self) -> Result<In, BuildError> {
        Ok(In {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            values: self
                .values
                .ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
