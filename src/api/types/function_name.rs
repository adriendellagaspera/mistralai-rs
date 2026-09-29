pub use crate::prelude::*;

/// this restriction of `Function` is used to select a specific function to call
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FunctionName {
    #[serde(default)]
    pub name: String,
}

impl FunctionName {
    pub fn builder() -> FunctionNameBuilder {
        <FunctionNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunctionNameBuilder {
    name: Option<String>,
}

impl FunctionNameBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FunctionName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](FunctionNameBuilder::name)
    pub fn build(self) -> Result<FunctionName, BuildError> {
        Ok(FunctionName {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
