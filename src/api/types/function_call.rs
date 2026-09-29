pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionCall {
    pub arguments: FunctionCallArguments,
    #[serde(default)]
    pub name: String,
}

impl FunctionCall {
    pub fn builder() -> FunctionCallBuilder {
        <FunctionCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunctionCallBuilder {
    arguments: Option<FunctionCallArguments>,
    name: Option<String>,
}

impl FunctionCallBuilder {
    pub fn arguments(mut self, value: FunctionCallArguments) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FunctionCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`arguments`](FunctionCallBuilder::arguments)
    /// - [`name`](FunctionCallBuilder::name)
    pub fn build(self) -> Result<FunctionCall, BuildError> {
        Ok(FunctionCall {
            arguments: self
                .arguments
                .ok_or_else(|| BuildError::missing_field("arguments"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
