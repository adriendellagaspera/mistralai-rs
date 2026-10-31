pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateServiceAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateServiceAccountRequest {
    pub fn builder() -> UpdateServiceAccountRequestBuilder {
        <UpdateServiceAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateServiceAccountRequestBuilder {
    description: Option<String>,
}

impl UpdateServiceAccountRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateServiceAccountRequest`].
    pub fn build(self) -> Result<UpdateServiceAccountRequest, BuildError> {
        Ok(UpdateServiceAccountRequest {
            description: self.description,
        })
    }
}
