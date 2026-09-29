pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTraces {
    #[serde(default)]
    pub traces: FeedResultGetTrace,
}

impl GetTraces {
    pub fn builder() -> GetTracesBuilder {
        <GetTracesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTracesBuilder {
    traces: Option<FeedResultGetTrace>,
}

impl GetTracesBuilder {
    pub fn traces(mut self, value: FeedResultGetTrace) -> Self {
        self.traces = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTraces`].
    /// This method will fail if any of the following fields are not set:
    /// - [`traces`](GetTracesBuilder::traces)
    pub fn build(self) -> Result<GetTraces, BuildError> {
        Ok(GetTraces {
            traces: self
                .traces
                .ok_or_else(|| BuildError::missing_field("traces"))?,
        })
    }
}
