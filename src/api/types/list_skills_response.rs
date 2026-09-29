pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSkillsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Skill>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListSkillsResponse {
    pub fn builder() -> ListSkillsResponseBuilder {
        <ListSkillsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSkillsResponseBuilder {
    data: Option<Vec<Skill>>,
    next_page_token: Option<String>,
}

impl ListSkillsResponseBuilder {
    pub fn data(mut self, value: Vec<Skill>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListSkillsResponse`].
    pub fn build(self) -> Result<ListSkillsResponse, BuildError> {
        Ok(ListSkillsResponse {
            data: self.data,
            next_page_token: self.next_page_token,
        })
    }
}
