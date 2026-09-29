pub use crate::prelude::*;

/// Query parameters for skills_get_version
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SkillsGetVersionQueryRequest {
    #[serde(default)]
    pub fields: Vec<Option<String>>,
}

impl SkillsGetVersionQueryRequest {
    pub fn builder() -> SkillsGetVersionQueryRequestBuilder {
        <SkillsGetVersionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillsGetVersionQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
}

impl SkillsGetVersionQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SkillsGetVersionQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](SkillsGetVersionQueryRequestBuilder::fields)
    pub fn build(self) -> Result<SkillsGetVersionQueryRequest, BuildError> {
        Ok(SkillsGetVersionQueryRequest {
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
