pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteOut {
    /// Deletion result message.
    #[serde(default)]
    pub message: String,
}

impl DeleteOut {
    pub fn builder() -> DeleteOutBuilder {
        <DeleteOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteOutBuilder {
    message: Option<String>,
}

impl DeleteOutBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](DeleteOutBuilder::message)
    pub fn build(self) -> Result<DeleteOut, BuildError> {
        Ok(DeleteOut {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
