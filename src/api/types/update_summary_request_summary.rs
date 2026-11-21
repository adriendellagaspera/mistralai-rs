pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateSummaryRequestSummary {
    #[serde(default)]
    pub content: String,
    pub status: UpdateSummaryRequestSummaryStatus,
    #[serde(default)]
    pub translated: bool,
}

impl UpdateSummaryRequestSummary {
    pub fn builder() -> UpdateSummaryRequestSummaryBuilder {
        <UpdateSummaryRequestSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSummaryRequestSummaryBuilder {
    content: Option<String>,
    status: Option<UpdateSummaryRequestSummaryStatus>,
    translated: Option<bool>,
}

impl UpdateSummaryRequestSummaryBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn status(mut self, value: UpdateSummaryRequestSummaryStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn translated(mut self, value: bool) -> Self {
        self.translated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSummaryRequestSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](UpdateSummaryRequestSummaryBuilder::content)
    /// - [`status`](UpdateSummaryRequestSummaryBuilder::status)
    /// - [`translated`](UpdateSummaryRequestSummaryBuilder::translated)
    pub fn build(self) -> Result<UpdateSummaryRequestSummary, BuildError> {
        Ok(UpdateSummaryRequestSummary {
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
