pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSkillVersionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SkillVersion>>,
}

impl ListSkillVersionsResponse {
    pub fn builder() -> ListSkillVersionsResponseBuilder {
        <ListSkillVersionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSkillVersionsResponseBuilder {
    data: Option<Vec<SkillVersion>>,
}

impl ListSkillVersionsResponseBuilder {
    pub fn data(mut self, value: Vec<SkillVersion>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSkillVersionsResponse`].
    pub fn build(self) -> Result<ListSkillVersionsResponse, BuildError> {
        Ok(ListSkillVersionsResponse { data: self.data })
    }
}
