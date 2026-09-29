pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OcrUsageInfo {
    /// Number of pages processed
    #[serde(default)]
    pub pages_processed: i64,
    /// Document size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_size_bytes: Option<i64>,
}

impl OcrUsageInfo {
    pub fn builder() -> OcrUsageInfoBuilder {
        <OcrUsageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrUsageInfoBuilder {
    pages_processed: Option<i64>,
    doc_size_bytes: Option<i64>,
}

impl OcrUsageInfoBuilder {
    pub fn pages_processed(mut self, value: i64) -> Self {
        self.pages_processed = Some(value);
        self
    }

    pub fn doc_size_bytes(mut self, value: i64) -> Self {
        self.doc_size_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrUsageInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pages_processed`](OcrUsageInfoBuilder::pages_processed)
    pub fn build(self) -> Result<OcrUsageInfo, BuildError> {
        Ok(OcrUsageInfo {
            pages_processed: self
                .pages_processed
                .ok_or_else(|| BuildError::missing_field("pages_processed"))?,
            doc_size_bytes: self.doc_size_bytes,
        })
    }
}
