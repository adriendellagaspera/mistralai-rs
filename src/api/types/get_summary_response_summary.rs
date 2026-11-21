pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetSummaryResponseSummary {
    #[serde(default)]
    pub content: String,
    pub status: SummaryStatus,
    #[serde(default)]
    pub translated: bool,
}

impl GetSummaryResponseSummary {
    pub fn builder() -> GetSummaryResponseSummaryBuilder {
        <GetSummaryResponseSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSummaryResponseSummaryBuilder {
    content: Option<String>,
    status: Option<SummaryStatus>,
    translated: Option<bool>,
}

impl GetSummaryResponseSummaryBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn status(mut self, value: SummaryStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn translated(mut self, value: bool) -> Self {
        self.translated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSummaryResponseSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](GetSummaryResponseSummaryBuilder::content)
    /// - [`status`](GetSummaryResponseSummaryBuilder::status)
    /// - [`translated`](GetSummaryResponseSummaryBuilder::translated)
    pub fn build(self) -> Result<GetSummaryResponseSummary, BuildError> {
        Ok(GetSummaryResponseSummary {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            translated: self
                .translated
                .ok_or_else(|| BuildError::missing_field("translated"))?,
        })
    }
}
