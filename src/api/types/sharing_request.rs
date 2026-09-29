pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SharingRequest {
    pub level: ShareEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    pub share_with_type: EntityType,
    /// The id of the entity (user, workspace or organization) to share with
    #[serde(default)]
    pub share_with_uuid: String,
}

impl SharingRequest {
    pub fn builder() -> SharingRequestBuilder {
        <SharingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SharingRequestBuilder {
    level: Option<ShareEnum>,
    org_id: Option<String>,
    share_with_type: Option<EntityType>,
    share_with_uuid: Option<String>,
}

impl SharingRequestBuilder {
    pub fn level(mut self, value: ShareEnum) -> Self {
        self.level = Some(value);
        self
    }

    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    pub fn share_with_type(mut self, value: EntityType) -> Self {
        self.share_with_type = Some(value);
        self
    }

    pub fn share_with_uuid(mut self, value: impl Into<String>) -> Self {
        self.share_with_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SharingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`level`](SharingRequestBuilder::level)
    /// - [`share_with_type`](SharingRequestBuilder::share_with_type)
    /// - [`share_with_uuid`](SharingRequestBuilder::share_with_uuid)
    pub fn build(self) -> Result<SharingRequest, BuildError> {
        Ok(SharingRequest {
            level: self
                .level
                .ok_or_else(|| BuildError::missing_field("level"))?,
            org_id: self.org_id,
            share_with_type: self
                .share_with_type
                .ok_or_else(|| BuildError::missing_field("share_with_type"))?,
            share_with_uuid: self
                .share_with_uuid
                .ok_or_else(|| BuildError::missing_field("share_with_uuid"))?,
        })
    }
}
