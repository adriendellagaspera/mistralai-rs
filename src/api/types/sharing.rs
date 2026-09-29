pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Sharing {
    #[serde(default)]
    pub library_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default)]
    pub org_id: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub share_with_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_with_uuid: Option<String>,
}

impl Sharing {
    pub fn builder() -> SharingBuilder {
        <SharingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SharingBuilder {
    library_id: Option<String>,
    user_id: Option<String>,
    org_id: Option<String>,
    role: Option<String>,
    share_with_type: Option<String>,
    share_with_uuid: Option<String>,
}

impl SharingBuilder {
    pub fn library_id(mut self, value: impl Into<String>) -> Self {
        self.library_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn share_with_type(mut self, value: impl Into<String>) -> Self {
        self.share_with_type = Some(value.into());
        self
    }

    pub fn share_with_uuid(mut self, value: impl Into<String>) -> Self {
        self.share_with_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Sharing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`library_id`](SharingBuilder::library_id)
    /// - [`org_id`](SharingBuilder::org_id)
    /// - [`role`](SharingBuilder::role)
    /// - [`share_with_type`](SharingBuilder::share_with_type)
    pub fn build(self) -> Result<Sharing, BuildError> {
        Ok(Sharing {
            library_id: self
                .library_id
                .ok_or_else(|| BuildError::missing_field("library_id"))?,
            user_id: self.user_id,
            org_id: self
                .org_id
                .ok_or_else(|| BuildError::missing_field("org_id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            share_with_type: self
                .share_with_type
                .ok_or_else(|| BuildError::missing_field("share_with_type"))?,
            share_with_uuid: self.share_with_uuid,
        })
    }
}
