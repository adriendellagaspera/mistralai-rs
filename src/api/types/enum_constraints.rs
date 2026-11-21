pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnumConstraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl EnumConstraints {
    pub fn builder() -> EnumConstraintsBuilder {
        <EnumConstraintsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnumConstraintsBuilder {
    options: Option<Vec<String>>,
}

impl EnumConstraintsBuilder {
    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnumConstraints`].
    pub fn build(self) -> Result<EnumConstraints, BuildError> {
        Ok(EnumConstraints {
            options: self.options,
        })
    }
}
