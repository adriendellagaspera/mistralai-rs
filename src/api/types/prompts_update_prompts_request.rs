pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptsUpdatePromptsRequest {
    /// Display description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Registry sharing scope.
    #[serde(rename = "sharingScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_scope: Option<RegistrySharingScope>,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Access every workspace member gets when sharing_scope is `workspace`. Defaults to `writer`, which is what a workspace publish granted before this field existed, so omitting it preserves today's behaviour exactly. Rejected unless sharing_scope is `workspace`.
    #[serde(rename = "workspaceRelation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_relation: Option<ShareRelation>,
}

impl PromptsUpdatePromptsRequest {
    pub fn builder() -> PromptsUpdatePromptsRequestBuilder {
        <PromptsUpdatePromptsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptsUpdatePromptsRequestBuilder {
    description: Option<String>,
    sharing_scope: Option<RegistrySharingScope>,
    title: Option<String>,
    workspace_relation: Option<ShareRelation>,
}

impl PromptsUpdatePromptsRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn sharing_scope(mut self, value: RegistrySharingScope) -> Self {
        self.sharing_scope = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn workspace_relation(mut self, value: ShareRelation) -> Self {
        self.workspace_relation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptsUpdatePromptsRequest`].
    pub fn build(self) -> Result<PromptsUpdatePromptsRequest, BuildError> {
        Ok(PromptsUpdatePromptsRequest {
            description: self.description,
            sharing_scope: self.sharing_scope,
            title: self.title,
            workspace_relation: self.workspace_relation,
        })
    }
}
