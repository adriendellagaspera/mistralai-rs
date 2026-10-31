pub use crate::prelude::*;

/// Field equals a scalar value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Equal {
    #[serde(default)]
    pub field: String,
    pub value: EqualValue,
}

impl Equal {
    pub fn builder() -> EqualBuilder {
        <EqualBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EqualBuilder {
    field: Option<String>,
    value: Option<EqualValue>,
}

impl EqualBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn value(mut self, value: EqualValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Equal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](EqualBuilder::field)
    /// - [`value`](EqualBuilder::value)
    pub fn build(self) -> Result<Equal, BuildError> {
        Ok(Equal {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
