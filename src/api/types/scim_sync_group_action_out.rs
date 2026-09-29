pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScimSyncGroupActionOut {
    #[serde(default)]
    pub group: ScimSyncGroupRefOut,
}

impl ScimSyncGroupActionOut {
    pub fn builder() -> ScimSyncGroupActionOutBuilder {
        <ScimSyncGroupActionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScimSyncGroupActionOutBuilder {
    group: Option<ScimSyncGroupRefOut>,
}

impl ScimSyncGroupActionOutBuilder {
    pub fn group(mut self, value: ScimSyncGroupRefOut) -> Self {
        self.group = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScimSyncGroupActionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group`](ScimSyncGroupActionOutBuilder::group)
    pub fn build(self) -> Result<ScimSyncGroupActionOut, BuildError> {
        Ok(ScimSyncGroupActionOut {
            group: self
                .group
                .ok_or_else(|| BuildError::missing_field("group"))?,
        })
    }
}
