pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OcrResponse {
    /// Formatted response in the request_format if provided in json str
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_annotation: Option<String>,
    /// The model used to generate the OCR.
    #[serde(default)]
    pub model: String,
    /// List of OCR info for pages.
    #[serde(default)]
    pub pages: Vec<OcrPageObject>,
    /// Usage info for the OCR request.
    #[serde(default)]
    pub usage_info: OcrUsageInfo,
}

impl OcrResponse {
    pub fn builder() -> OcrResponseBuilder {
        <OcrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrResponseBuilder {
    document_annotation: Option<String>,
    model: Option<String>,
    pages: Option<Vec<OcrPageObject>>,
    usage_info: Option<OcrUsageInfo>,
}

impl OcrResponseBuilder {
    pub fn document_annotation(mut self, value: impl Into<String>) -> Self {
        self.document_annotation = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn pages(mut self, value: Vec<OcrPageObject>) -> Self {
        self.pages = Some(value);
        self
    }

    pub fn usage_info(mut self, value: OcrUsageInfo) -> Self {
        self.usage_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](OcrResponseBuilder::model)
    /// - [`pages`](OcrResponseBuilder::pages)
    /// - [`usage_info`](OcrResponseBuilder::usage_info)
    pub fn build(self) -> Result<OcrResponse, BuildError> {
        Ok(OcrResponse {
            document_annotation: self.document_annotation,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            pages: self
                .pages
                .ok_or_else(|| BuildError::missing_field("pages"))?,
            usage_info: self
                .usage_info
                .ok_or_else(|| BuildError::missing_field("usage_info"))?,
        })
    }
}
