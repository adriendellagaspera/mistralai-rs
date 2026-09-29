pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteApiKeyOut {
    /// API key deletion result message.
    #[serde(default)]
    pub detail: String,
}

impl DeleteApiKeyOut {
    pub fn builder() -> DeleteApiKeyOutBuilder {
        <DeleteApiKeyOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteApiKeyOutBuilder {
    detail: Option<String>,
}

impl DeleteApiKeyOutBuilder {
    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteApiKeyOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`detail`](DeleteApiKeyOutBuilder::detail)
    pub fn build(self) -> Result<DeleteApiKeyOut, BuildError> {
        Ok(DeleteApiKeyOut {
            detail: self
                .detail
                .ok_or_else(|| BuildError::missing_field("detail"))?,
        })
    }
}
