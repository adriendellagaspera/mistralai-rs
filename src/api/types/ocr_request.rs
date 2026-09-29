pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Document to run OCR on
    pub document: OcrRequestDocument,
    /// Specific pages to process. Accepts a list of integers or a string of comma-separated numbers and ranges (e.g. '0,1,2' or '0-5' or '0,2-4'). Page numbers start from 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<OcrRequestPages>,
    /// Include image URLs in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_image_base64: Option<bool>,
    /// Max images to extract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_limit: Option<i64>,
    /// Minimum height and width of image to extract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_min_size: Option<i64>,
    /// Structured output class for extracting useful information from each extracted bounding box / image from document. Only json_schema is valid for this field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox_annotation_format: Option<ResponseFormat>,
    /// Structured output class for extracting useful information from the entire document. Only json_schema is valid for this field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_annotation_format: Option<ResponseFormat>,
    /// Optional prompt to guide the model in extracting structured output from the entire document. A document_annotation_format must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_annotation_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_format: Option<OcrRequestTableFormat>,
    /// Extract the page header into the response's `header` field and remove it from the markdown content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_header: Option<bool>,
    /// Extract the page footer into the response's `footer` field and remove it from the markdown content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_footer: Option<bool>,
    /// Return paragraph-level bounding boxes for all content blocks in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blocks: Option<bool>,
    /// Granularity for confidence scores: 'page' (aggregate only), 'word' (per-word scores), or 'block' (per-block scores). Defaults to None (no confidence scores) to keep response payload small.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_scores_granularity: Option<OcrRequestConfidenceScoresGranularity>,
}

impl OcrRequest {
    pub fn builder() -> OcrRequestBuilder {
        <OcrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrRequestBuilder {
    model: Option<String>,
    document: Option<OcrRequestDocument>,
    pages: Option<OcrRequestPages>,
    include_image_base64: Option<bool>,
    image_limit: Option<i64>,
    image_min_size: Option<i64>,
    bbox_annotation_format: Option<ResponseFormat>,
    document_annotation_format: Option<ResponseFormat>,
    document_annotation_prompt: Option<String>,
    table_format: Option<OcrRequestTableFormat>,
    extract_header: Option<bool>,
    extract_footer: Option<bool>,
    include_blocks: Option<bool>,
    confidence_scores_granularity: Option<OcrRequestConfidenceScoresGranularity>,
}

impl OcrRequestBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn document(mut self, value: OcrRequestDocument) -> Self {
        self.document = Some(value);
        self
    }

    pub fn pages(mut self, value: OcrRequestPages) -> Self {
        self.pages = Some(value);
        self
    }

    pub fn include_image_base64(mut self, value: bool) -> Self {
        self.include_image_base64 = Some(value);
        self
    }

    pub fn image_limit(mut self, value: i64) -> Self {
        self.image_limit = Some(value);
        self
    }

    pub fn image_min_size(mut self, value: i64) -> Self {
        self.image_min_size = Some(value);
        self
    }

    pub fn bbox_annotation_format(mut self, value: ResponseFormat) -> Self {
        self.bbox_annotation_format = Some(value);
        self
    }

    pub fn document_annotation_format(mut self, value: ResponseFormat) -> Self {
        self.document_annotation_format = Some(value);
        self
    }

    pub fn document_annotation_prompt(mut self, value: impl Into<String>) -> Self {
        self.document_annotation_prompt = Some(value.into());
        self
    }

    pub fn table_format(mut self, value: OcrRequestTableFormat) -> Self {
        self.table_format = Some(value);
        self
    }

    pub fn extract_header(mut self, value: bool) -> Self {
        self.extract_header = Some(value);
        self
    }

    pub fn extract_footer(mut self, value: bool) -> Self {
        self.extract_footer = Some(value);
        self
    }

    pub fn include_blocks(mut self, value: bool) -> Self {
        self.include_blocks = Some(value);
        self
    }

    pub fn confidence_scores_granularity(
        mut self,
        value: OcrRequestConfidenceScoresGranularity,
    ) -> Self {
        self.confidence_scores_granularity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document`](OcrRequestBuilder::document)
    pub fn build(self) -> Result<OcrRequest, BuildError> {
        Ok(OcrRequest {
            model: self.model,
            document: self
                .document
                .ok_or_else(|| BuildError::missing_field("document"))?,
            pages: self.pages,
            include_image_base64: self.include_image_base64,
            image_limit: self.image_limit,
            image_min_size: self.image_min_size,
            bbox_annotation_format: self.bbox_annotation_format,
            document_annotation_format: self.document_annotation_format,
            document_annotation_prompt: self.document_annotation_prompt,
            table_format: self.table_format,
            extract_header: self.extract_header,
            extract_footer: self.extract_footer,
            include_blocks: self.include_blocks,
            confidence_scores_granularity: self.confidence_scores_granularity,
        })
    }
}
