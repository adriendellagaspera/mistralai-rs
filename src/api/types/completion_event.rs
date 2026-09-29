pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompletionEvent {
    #[serde(default)]
    pub data: CompletionChunk,
}

impl CompletionEvent {
    pub fn builder() -> CompletionEventBuilder {
        <CompletionEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionEventBuilder {
    data: Option<CompletionChunk>,
}

impl CompletionEventBuilder {
    pub fn data(mut self, value: CompletionChunk) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompletionEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](CompletionEventBuilder::data)
    pub fn build(self) -> Result<CompletionEvent, BuildError> {
        Ok(CompletionEvent {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
