pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncUserRefOut {
    #[serde(rename = "userName")]
    #[serde(default)]
    pub user_name: String,
    #[serde(default)]
    pub uuid: String,
}

impl ScimSyncUserRefOut {
    pub fn builder() -> ScimSyncUserRefOutBuilder {
        <ScimSyncUserRefOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncUserRefOutBuilder {
    user_name: Option<String>,
    uuid: Option<String>,
}

impl ScimSyncUserRefOutBuilder {
    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.user_name = Some(value.into());
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncUserRefOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_name`](ScimSyncUserRefOutBuilder::user_name)
    /// - [`uuid`](ScimSyncUserRefOutBuilder::uuid)
    pub fn build(self) -> Result<ScimSyncUserRefOut, BuildError> {
        Ok(ScimSyncUserRefOut {
            user_name: self
                .user_name
                .ok_or_else(|| BuildError::missing_field("user_name"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
