pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeactivateUserOut {
    /// Deactivation result message.
    #[serde(default)]
    pub message: String,
}

impl DeactivateUserOut {
    pub fn builder() -> DeactivateUserOutBuilder {
        <DeactivateUserOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeactivateUserOutBuilder {
    message: Option<String>,
}

impl DeactivateUserOutBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeactivateUserOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](DeactivateUserOutBuilder::message)
    pub fn build(self) -> Result<DeactivateUserOut, BuildError> {
        Ok(DeactivateUserOut {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
