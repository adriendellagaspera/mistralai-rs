pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSpans {
    #[serde(default)]
    pub spans: FeedResultGetSpan,
}

impl GetSpans {
    pub fn builder() -> GetSpansBuilder {
        <GetSpansBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpansBuilder {
    spans: Option<FeedResultGetSpan>,
}

impl GetSpansBuilder {
    pub fn spans(mut self, value: FeedResultGetSpan) -> Self {
        self.spans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpans`].
    /// This method will fail if any of the following fields are not set:
    /// - [`spans`](GetSpansBuilder::spans)
    pub fn build(self) -> Result<GetSpans, BuildError> {
        Ok(GetSpans {
            spans: self
                .spans
                .ok_or_else(|| BuildError::missing_field("spans"))?,
        })
    }
}
