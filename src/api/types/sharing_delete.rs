pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SharingDelete {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    /// The id of the entity (user, workspace or organization) to share with
    #[serde(default)]
    pub share_with_uuid: String,
    pub share_with_type: EntityType,
}

impl SharingDelete {
    pub fn builder() -> SharingDeleteBuilder {
        <SharingDeleteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SharingDeleteBuilder {
    org_id: Option<String>,
    share_with_uuid: Option<String>,
    share_with_type: Option<EntityType>,
}

impl SharingDeleteBuilder {
    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    pub fn share_with_uuid(mut self, value: impl Into<String>) -> Self {
        self.share_with_uuid = Some(value.into());
        self
    }

    pub fn share_with_type(mut self, value: EntityType) -> Self {
        self.share_with_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SharingDelete`].
    /// This method will fail if any of the following fields are not set:
    /// - [`share_with_uuid`](SharingDeleteBuilder::share_with_uuid)
    /// - [`share_with_type`](SharingDeleteBuilder::share_with_type)
    pub fn build(self) -> Result<SharingDelete, BuildError> {
        Ok(SharingDelete {
            org_id: self.org_id,
            share_with_uuid: self
                .share_with_uuid
                .ok_or_else(|| BuildError::missing_field("share_with_uuid"))?,
            share_with_type: self
                .share_with_type
                .ok_or_else(|| BuildError::missing_field("share_with_type"))?,
        })
    }
}
