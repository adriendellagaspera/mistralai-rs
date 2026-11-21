pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMetadataSkillsRequest {
    /// Registry sharing scope.
    #[serde(rename = "sharingScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_scope: Option<RegistrySharingScope>,
    /// Access every workspace member gets when sharing_scope is `workspace`. Defaults to `writer`, which is what a workspace publish granted before this field existed, so omitting it preserves today's behaviour exactly. Rejected unless sharing_scope is `workspace`.
    #[serde(rename = "workspaceRelation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_relation: Option<ShareRelation>,
}

impl UpdateMetadataSkillsRequest {
    pub fn builder() -> UpdateMetadataSkillsRequestBuilder {
        <UpdateMetadataSkillsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMetadataSkillsRequestBuilder {
    sharing_scope: Option<RegistrySharingScope>,
    workspace_relation: Option<ShareRelation>,
}

impl UpdateMetadataSkillsRequestBuilder {
    pub fn sharing_scope(mut self, value: RegistrySharingScope) -> Self {
        self.sharing_scope = Some(value);
        self
    }

    pub fn workspace_relation(mut self, value: ShareRelation) -> Self {
        self.workspace_relation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMetadataSkillsRequest`].
    pub fn build(self) -> Result<UpdateMetadataSkillsRequest, BuildError> {
        Ok(UpdateMetadataSkillsRequest {
            sharing_scope: self.sharing_scope,
            workspace_relation: self.workspace_relation,
        })
    }
}
