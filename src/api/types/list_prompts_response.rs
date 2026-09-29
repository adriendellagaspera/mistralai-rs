pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPromptsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Prompt>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListPromptsResponse {
    pub fn builder() -> ListPromptsResponseBuilder {
        <ListPromptsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPromptsResponseBuilder {
    data: Option<Vec<Prompt>>,
    next_page_token: Option<String>,
}

impl ListPromptsResponseBuilder {
    pub fn data(mut self, value: Vec<Prompt>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPromptsResponse`].
    pub fn build(self) -> Result<ListPromptsResponse, BuildError> {
        Ok(ListPromptsResponse {
            data: self.data,
            next_page_token: self.next_page_token,
        })
    }
}
