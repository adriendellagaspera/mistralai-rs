pub use crate::prelude::*;

/// Query parameters for skills_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SkillsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    pub fields: Vec<Option<String>>,
}

impl SkillsGetQueryRequest {
    pub fn builder() -> SkillsGetQueryRequestBuilder {
        <SkillsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillsGetQueryRequestBuilder {
    version: Option<i64>,
    alias: Option<String>,
    fields: Option<Vec<Option<String>>>,
}

impl SkillsGetQueryRequestBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SkillsGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](SkillsGetQueryRequestBuilder::fields)
    pub fn build(self) -> Result<SkillsGetQueryRequest, BuildError> {
        Ok(SkillsGetQueryRequest {
            version: self.version,
            alias: self.alias,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
