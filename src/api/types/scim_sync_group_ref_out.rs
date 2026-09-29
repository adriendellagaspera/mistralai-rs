pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncGroupRefOut {
    #[serde(default)]
    pub uuid: String,
    #[serde(rename = "groupName")]
    #[serde(default)]
    pub group_name: String,
}

impl ScimSyncGroupRefOut {
    pub fn builder() -> ScimSyncGroupRefOutBuilder {
        <ScimSyncGroupRefOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncGroupRefOutBuilder {
    uuid: Option<String>,
    group_name: Option<String>,
}

impl ScimSyncGroupRefOutBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncGroupRefOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](ScimSyncGroupRefOutBuilder::uuid)
    /// - [`group_name`](ScimSyncGroupRefOutBuilder::group_name)
    pub fn build(self) -> Result<ScimSyncGroupRefOut, BuildError> {
        Ok(ScimSyncGroupRefOut {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            group_name: self
                .group_name
                .ok_or_else(|| BuildError::missing_field("group_name"))?,
        })
    }
}
