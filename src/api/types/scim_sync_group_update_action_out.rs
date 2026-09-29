pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncGroupUpdateActionOut {
    #[serde(default)]
    pub group: ScimSyncGroupRefOut,
    #[serde(rename = "previous_groupName")]
    #[serde(default)]
    pub previous_group_name: String,
}

impl ScimSyncGroupUpdateActionOut {
    pub fn builder() -> ScimSyncGroupUpdateActionOutBuilder {
        <ScimSyncGroupUpdateActionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncGroupUpdateActionOutBuilder {
    group: Option<ScimSyncGroupRefOut>,
    previous_group_name: Option<String>,
}

impl ScimSyncGroupUpdateActionOutBuilder {
    pub fn group(mut self, value: ScimSyncGroupRefOut) -> Self {
        self.group = Some(value);
        self
    }

    pub fn previous_group_name(mut self, value: impl Into<String>) -> Self {
        self.previous_group_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncGroupUpdateActionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group`](ScimSyncGroupUpdateActionOutBuilder::group)
    /// - [`previous_group_name`](ScimSyncGroupUpdateActionOutBuilder::previous_group_name)
    pub fn build(self) -> Result<ScimSyncGroupUpdateActionOut, BuildError> {
        Ok(ScimSyncGroupUpdateActionOut {
            group: self
                .group
                .ok_or_else(|| BuildError::missing_field("group"))?,
            previous_group_name: self
                .previous_group_name
                .ok_or_else(|| BuildError::missing_field("previous_group_name"))?,
        })
    }
}
