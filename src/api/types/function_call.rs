pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionCall {
    #[serde(default)]
    pub name: String,
    pub arguments: FunctionCallArguments,
}

impl FunctionCall {
    pub fn builder() -> FunctionCallBuilder {
        <FunctionCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunctionCallBuilder {
    name: Option<String>,
    arguments: Option<FunctionCallArguments>,
}

impl FunctionCallBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn arguments(mut self, value: FunctionCallArguments) -> Self {
        self.arguments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FunctionCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](FunctionCallBuilder::name)
    /// - [`arguments`](FunctionCallBuilder::arguments)
    pub fn build(self) -> Result<FunctionCall, BuildError> {
        Ok(FunctionCall {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            arguments: self
                .arguments
                .ok_or_else(|| BuildError::missing_field("arguments"))?,
        })
    }
}
