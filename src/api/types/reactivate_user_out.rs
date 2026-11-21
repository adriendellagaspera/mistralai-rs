pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReactivateUserOut {
    /// Reactivation result message.
    #[serde(default)]
    pub message: String,
}

impl ReactivateUserOut {
    pub fn builder() -> ReactivateUserOutBuilder {
        <ReactivateUserOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReactivateUserOutBuilder {
    message: Option<String>,
}

impl ReactivateUserOutBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReactivateUserOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ReactivateUserOutBuilder::message)
    pub fn build(self) -> Result<ReactivateUserOut, BuildError> {
        Ok(ReactivateUserOut {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
