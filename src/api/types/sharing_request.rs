pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SharingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    pub level: ShareEnum,
    /// The id of the entity (user, workspace or organization) to share with
    #[serde(default)]
    pub share_with_uuid: String,
    pub share_with_type: EntityType,
}

impl SharingRequest {
    pub fn builder() -> SharingRequestBuilder {
        <SharingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SharingRequestBuilder {
    org_id: Option<String>,
    level: Option<ShareEnum>,
    share_with_uuid: Option<String>,
    share_with_type: Option<EntityType>,
}

impl SharingRequestBuilder {
    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    pub fn level(mut self, value: ShareEnum) -> Self {
        self.level = Some(value);
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

    /// Consumes the builder and constructs a [`SharingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`level`](SharingRequestBuilder::level)
    /// - [`share_with_uuid`](SharingRequestBuilder::share_with_uuid)
    /// - [`share_with_type`](SharingRequestBuilder::share_with_type)
    pub fn build(self) -> Result<SharingRequest, BuildError> {
        Ok(SharingRequest {
            org_id: self.org_id,
            level: self
                .level
                .ok_or_else(|| BuildError::missing_field("level"))?,
            share_with_uuid: self
                .share_with_uuid
                .ok_or_else(|| BuildError::missing_field("share_with_uuid"))?,
            share_with_type: self
                .share_with_type
                .ok_or_else(|| BuildError::missing_field("share_with_type"))?,
        })
    }
}
