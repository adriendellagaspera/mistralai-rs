pub use crate::prelude::*;

/// Query parameters for get_version
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaSkillsGetVersionQueryRequest {
    #[serde(default)]
    pub fields: Vec<Option<String>>,
}

impl BetaSkillsGetVersionQueryRequest {
    pub fn builder() -> BetaSkillsGetVersionQueryRequestBuilder {
        <BetaSkillsGetVersionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaSkillsGetVersionQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
}

impl BetaSkillsGetVersionQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaSkillsGetVersionQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](BetaSkillsGetVersionQueryRequestBuilder::fields)
    pub fn build(self) -> Result<BetaSkillsGetVersionQueryRequest, BuildError> {
        Ok(BetaSkillsGetVersionQueryRequest {
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
