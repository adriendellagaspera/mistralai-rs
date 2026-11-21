pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioAgentListPage {
    #[serde(default)]
    pub data: Vec<AudioAgentResponse>,
    /// Cursor for the next page, null on the last page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl AudioAgentListPage {
    pub fn builder() -> AudioAgentListPageBuilder {
        <AudioAgentListPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAgentListPageBuilder {
    data: Option<Vec<AudioAgentResponse>>,
    next_page_token: Option<String>,
}

impl AudioAgentListPageBuilder {
    pub fn data(mut self, value: Vec<AudioAgentResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioAgentListPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](AudioAgentListPageBuilder::data)
    pub fn build(self) -> Result<AudioAgentListPage, BuildError> {
        Ok(AudioAgentListPage {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_page_token: self.next_page_token,
        })
    }
}
