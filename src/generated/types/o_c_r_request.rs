#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRRequest {
    ///Structured output class for extracting useful information from each extracted bounding box / image from document. Only json_schema is valid for this field
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub bbox_annotation_format: Option<Option<ResponseFormat>>,
    ///Granularity for confidence scores: 'word' (per-word scores) or 'page' (aggregate only). Defaults to None (no confidence scores) to keep response payload small.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub confidence_scores_granularity: Option<Option<OCRRequestConfidenceScoresGranularity>>,
    ///Document to run OCR on
    pub document: OCRRequestDocument,
    ///Structured output class for extracting useful information from the entire document. Only json_schema is valid for this field
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub document_annotation_format: Option<Option<ResponseFormat>>,
    ///Optional prompt to guide the model in extracting structured output from the entire document. A document_annotation_format must be provided.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub document_annotation_prompt: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_footer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Max images to extract
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub image_limit: Option<Option<i64>>,
    ///Minimum height and width of image to extract
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub image_min_size: Option<Option<i64>>,
    ///Return paragraph-level bounding boxes for all content blocks in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blocks: Option<bool>,
    ///Include image URLs in response
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub include_image_base64: Option<Option<bool>>,
    pub model: Option<String>,
    ///Specific pages to process. Accepts a list of integers or a string of comma-separated numbers and ranges (e.g. '0,1,2' or '0-5' or '0,2-4'). Page numbers start from 0.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub pages: Option<Option<OCRRequestPages>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub table_format: Option<Option<OCRRequestTableFormat>>,
}
