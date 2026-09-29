pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentListPage {
    #[serde(default)]
    pub data: Vec<Agent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<AgentListPageObject>,
}

impl AgentListPage {
    pub fn builder() -> AgentListPageBuilder {
        <AgentListPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentListPageBuilder {
    data: Option<Vec<Agent>>,
    next_page_token: Option<String>,
    object: Option<AgentListPageObject>,
}

impl AgentListPageBuilder {
    pub fn data(mut self, value: Vec<Agent>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    pub fn object(mut self, value: AgentListPageObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentListPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](AgentListPageBuilder::data)
    pub fn build(self) -> Result<AgentListPage, BuildError> {
        Ok(AgentListPage {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_page_token: self.next_page_token,
            object: self.object,
        })
    }
}
