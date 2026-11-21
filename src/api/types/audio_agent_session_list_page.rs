pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioAgentSessionListPage {
    #[serde(default)]
    pub data: Vec<AudioAgentSessionSummary>,
    /// Cursor for the next page, null on the last page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl AudioAgentSessionListPage {
    pub fn builder() -> AudioAgentSessionListPageBuilder {
        <AudioAgentSessionListPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAgentSessionListPageBuilder {
    data: Option<Vec<AudioAgentSessionSummary>>,
    next_page_token: Option<String>,
}

impl AudioAgentSessionListPageBuilder {
    pub fn data(mut self, value: Vec<AudioAgentSessionSummary>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioAgentSessionListPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](AudioAgentSessionListPageBuilder::data)
    pub fn build(self) -> Result<AudioAgentSessionListPage, BuildError> {
        Ok(AudioAgentSessionListPage {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_page_token: self.next_page_token,
        })
    }
}
