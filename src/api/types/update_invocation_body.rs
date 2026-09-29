pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateInvocationBody {
    /// The name of the update to request
    #[serde(default)]
    pub name: String,
    /// Input data for the update, matching its schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<UpdateInvocationBodyInput>,
}

impl UpdateInvocationBody {
    pub fn builder() -> UpdateInvocationBodyBuilder {
        <UpdateInvocationBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateInvocationBodyBuilder {
    name: Option<String>,
    input: Option<UpdateInvocationBodyInput>,
}

impl UpdateInvocationBodyBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn input(mut self, value: UpdateInvocationBodyInput) -> Self {
        self.input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateInvocationBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateInvocationBodyBuilder::name)
    pub fn build(self) -> Result<UpdateInvocationBody, BuildError> {
        Ok(UpdateInvocationBody {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            input: self.input,
        })
    }
}
