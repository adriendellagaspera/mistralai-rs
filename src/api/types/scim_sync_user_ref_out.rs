pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncUserRefOut {
    #[serde(default)]
    pub uuid: String,
    #[serde(rename = "userName")]
    #[serde(default)]
    pub user_name: String,
}

impl ScimSyncUserRefOut {
    pub fn builder() -> ScimSyncUserRefOutBuilder {
        <ScimSyncUserRefOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncUserRefOutBuilder {
    uuid: Option<String>,
    user_name: Option<String>,
}

impl ScimSyncUserRefOutBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.user_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncUserRefOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](ScimSyncUserRefOutBuilder::uuid)
    /// - [`user_name`](ScimSyncUserRefOutBuilder::user_name)
    pub fn build(self) -> Result<ScimSyncUserRefOut, BuildError> {
        Ok(ScimSyncUserRefOut {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            user_name: self
                .user_name
                .ok_or_else(|| BuildError::missing_field("user_name"))?,
        })
    }
}
