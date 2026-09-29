pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSkillVersionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl CreateSkillVersionResponse {
    pub fn builder() -> CreateSkillVersionResponseBuilder {
        <CreateSkillVersionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSkillVersionResponseBuilder {
    deduplicated: Option<bool>,
    version: Option<i64>,
}

impl CreateSkillVersionResponseBuilder {
    pub fn deduplicated(mut self, value: bool) -> Self {
        self.deduplicated = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSkillVersionResponse`].
    pub fn build(self) -> Result<CreateSkillVersionResponse, BuildError> {
        Ok(CreateSkillVersionResponse {
            deduplicated: self.deduplicated,
            version: self.version,
        })
    }
}
