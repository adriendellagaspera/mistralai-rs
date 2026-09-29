pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateInvocationBody {
    /// Input data for the update, matching its schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<UpdateInvocationBodyInput>,
    /// The name of the update to request
    #[serde(default)]
    pub name: String,
}

impl UpdateInvocationBody {
    pub fn builder() -> UpdateInvocationBodyBuilder {
        <UpdateInvocationBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateInvocationBodyBuilder {
    input: Option<UpdateInvocationBodyInput>,
    name: Option<String>,
}

impl UpdateInvocationBodyBuilder {
    pub fn input(mut self, value: UpdateInvocationBodyInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateInvocationBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateInvocationBodyBuilder::name)
    pub fn build(self) -> Result<UpdateInvocationBody, BuildError> {
        Ok(UpdateInvocationBody {
            input: self.input,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
