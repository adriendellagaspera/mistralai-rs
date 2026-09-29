pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSkillVersionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicated: Option<bool>,
}

impl CreateSkillVersionResponse {
    pub fn builder() -> CreateSkillVersionResponseBuilder {
        <CreateSkillVersionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSkillVersionResponseBuilder {
    version: Option<i64>,
    deduplicated: Option<bool>,
}

impl CreateSkillVersionResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn deduplicated(mut self, value: bool) -> Self {
        self.deduplicated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSkillVersionResponse`].
    pub fn build(self) -> Result<CreateSkillVersionResponse, BuildError> {
        Ok(CreateSkillVersionResponse {
            version: self.version,
            deduplicated: self.deduplicated,
        })
    }
}
