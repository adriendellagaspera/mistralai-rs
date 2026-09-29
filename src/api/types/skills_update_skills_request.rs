pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SkillsUpdateSkillsRequest {
    /// Registry sharing scope.
    #[serde(rename = "sharingScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_scope: Option<RegistrySharingScope>,
}

impl SkillsUpdateSkillsRequest {
    pub fn builder() -> SkillsUpdateSkillsRequestBuilder {
        <SkillsUpdateSkillsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillsUpdateSkillsRequestBuilder {
    sharing_scope: Option<RegistrySharingScope>,
}

impl SkillsUpdateSkillsRequestBuilder {
    pub fn sharing_scope(mut self, value: RegistrySharingScope) -> Self {
        self.sharing_scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SkillsUpdateSkillsRequest`].
    pub fn build(self) -> Result<SkillsUpdateSkillsRequest, BuildError> {
        Ok(SkillsUpdateSkillsRequest {
            sharing_scope: self.sharing_scope,
        })
    }
}
