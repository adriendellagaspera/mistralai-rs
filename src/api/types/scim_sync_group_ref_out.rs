pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncGroupRefOut {
    #[serde(rename = "groupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(default)]
    pub uuid: String,
}

impl ScimSyncGroupRefOut {
    pub fn builder() -> ScimSyncGroupRefOutBuilder {
        <ScimSyncGroupRefOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncGroupRefOutBuilder {
    group_name: Option<String>,
    uuid: Option<String>,
}

impl ScimSyncGroupRefOutBuilder {
    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncGroupRefOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_name`](ScimSyncGroupRefOutBuilder::group_name)
    /// - [`uuid`](ScimSyncGroupRefOutBuilder::uuid)
    pub fn build(self) -> Result<ScimSyncGroupRefOut, BuildError> {
        Ok(ScimSyncGroupRefOut {
            group_name: self
                .group_name
                .ok_or_else(|| BuildError::missing_field("group_name"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
