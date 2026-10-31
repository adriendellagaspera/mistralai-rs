pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceListPage {
    #[serde(default)]
    pub data: Vec<VoiceResponse>,
    /// Cursor for the next page, null on the last page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl VoiceListPage {
    pub fn builder() -> VoiceListPageBuilder {
        <VoiceListPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceListPageBuilder {
    data: Option<Vec<VoiceResponse>>,
    next_page_token: Option<String>,
}

impl VoiceListPageBuilder {
    pub fn data(mut self, value: Vec<VoiceResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoiceListPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](VoiceListPageBuilder::data)
    pub fn build(self) -> Result<VoiceListPage, BuildError> {
        Ok(VoiceListPage {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_page_token: self.next_page_token,
        })
    }
}
