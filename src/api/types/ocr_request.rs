pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OcrRequest {
    /// Structured output class for extracting useful information from each extracted bounding box / image from document. Only json_schema is valid for this field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox_annotation_format: Option<ResponseFormat>,
    /// Granularity for confidence scores: 'page' (aggregate only), 'word' (per-word scores), or 'block' (per-block scores). Defaults to None (no confidence scores) to keep response payload small.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_scores_granularity: Option<OcrRequestConfidenceScoresGranularity>,
    /// Document to run OCR on
    pub document: OcrRequestDocument,
    /// Structured output class for extracting useful information from the entire document. Only json_schema is valid for this field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_annotation_format: Option<ResponseFormat>,
    /// Optional prompt to guide the model in extracting structured output from the entire document. A document_annotation_format must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_annotation_prompt: Option<String>,
    /// Extract the page footer into the response's `footer` field and remove it from the markdown content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_footer: Option<bool>,
    /// Extract the page header into the response's `header` field and remove it from the markdown content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_header: Option<bool>,
    /// Max images to extract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_limit: Option<i64>,
    /// Minimum height and width of image to extract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_min_size: Option<i64>,
    /// Return paragraph-level bounding boxes for all content blocks in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blocks: Option<bool>,
    /// Include image URLs in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_image_base64: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Specific pages to process. Accepts a list of integers or a string of comma-separated numbers and ranges (e.g. '0,1,2' or '0-5' or '0,2-4'). Page numbers start from 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<OcrRequestPages>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_format: Option<OcrRequestTableFormat>,
}

impl OcrRequest {
    pub fn builder() -> OcrRequestBuilder {
        <OcrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrRequestBuilder {
    bbox_annotation_format: Option<ResponseFormat>,
    confidence_scores_granularity: Option<OcrRequestConfidenceScoresGranularity>,
    document: Option<OcrRequestDocument>,
    document_annotation_format: Option<ResponseFormat>,
    document_annotation_prompt: Option<String>,
    extract_footer: Option<bool>,
    extract_header: Option<bool>,
    image_limit: Option<i64>,
    image_min_size: Option<i64>,
    include_blocks: Option<bool>,
    include_image_base64: Option<bool>,
    model: Option<String>,
    pages: Option<OcrRequestPages>,
    table_format: Option<OcrRequestTableFormat>,
}

impl OcrRequestBuilder {
    pub fn bbox_annotation_format(mut self, value: ResponseFormat) -> Self {
        self.bbox_annotation_format = Some(value);
        self
    }

    pub fn confidence_scores_granularity(
        mut self,
        value: OcrRequestConfidenceScoresGranularity,
    ) -> Self {
        self.confidence_scores_granularity = Some(value);
        self
    }

    pub fn document(mut self, value: OcrRequestDocument) -> Self {
        self.document = Some(value);
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

    pub fn extract_footer(mut self, value: bool) -> Self {
        self.extract_footer = Some(value);
        self
    }

    pub fn extract_header(mut self, value: bool) -> Self {
        self.extract_header = Some(value);
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

    pub fn include_blocks(mut self, value: bool) -> Self {
        self.include_blocks = Some(value);
        self
    }

    pub fn include_image_base64(mut self, value: bool) -> Self {
        self.include_image_base64 = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn pages(mut self, value: OcrRequestPages) -> Self {
        self.pages = Some(value);
        self
    }

    pub fn table_format(mut self, value: OcrRequestTableFormat) -> Self {
        self.table_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document`](OcrRequestBuilder::document)
    pub fn build(self) -> Result<OcrRequest, BuildError> {
        Ok(OcrRequest {
            bbox_annotation_format: self.bbox_annotation_format,
            confidence_scores_granularity: self.confidence_scores_granularity,
            document: self
                .document
                .ok_or_else(|| BuildError::missing_field("document"))?,
            document_annotation_format: self.document_annotation_format,
            document_annotation_prompt: self.document_annotation_prompt,
            extract_footer: self.extract_footer,
            extract_header: self.extract_header,
            image_limit: self.image_limit,
            image_min_size: self.image_min_size,
            include_blocks: self.include_blocks,
            include_image_base64: self.include_image_base64,
            model: self.model,
            pages: self.pages,
            table_format: self.table_format,
        })
    }
}
