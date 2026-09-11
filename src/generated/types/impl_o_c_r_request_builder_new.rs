impl OCRRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(document: OCRRequestDocument, model: Option<String>) -> Self {
        Self {
            value: OCRRequest::new(document, model),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "bbox_annotation_format",
        "` request field to a value."
    )]
    #[must_use]
    pub fn bbox_annotation_format(mut self, bbox_annotation_format: ResponseFormat) -> Self {
        self.value.bbox_annotation_format = Some(Some(bbox_annotation_format));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "bbox_annotation_format",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn bbox_annotation_format_null(mut self) -> Self {
        self.value.bbox_annotation_format = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "bbox_annotation_format", "` request field."
    )]
    #[must_use]
    pub fn bbox_annotation_format_absent(mut self) -> Self {
        self.value.bbox_annotation_format = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "confidence_scores_granularity",
        "` request field to a value."
    )]
    #[must_use]
    pub fn confidence_scores_granularity(
        mut self,
        confidence_scores_granularity: OCRRequestConfidenceScoresGranularity,
    ) -> Self {
        self.value.confidence_scores_granularity = Some(Some(confidence_scores_granularity));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "confidence_scores_granularity",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn confidence_scores_granularity_null(mut self) -> Self {
        self.value.confidence_scores_granularity = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "confidence_scores_granularity",
        "` request field."
    )]
    #[must_use]
    pub fn confidence_scores_granularity_absent(mut self) -> Self {
        self.value.confidence_scores_granularity = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "document_annotation_format",
        "` request field to a value."
    )]
    #[must_use]
    pub fn document_annotation_format(
        mut self,
        document_annotation_format: ResponseFormat,
    ) -> Self {
        self.value.document_annotation_format = Some(Some(document_annotation_format));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "document_annotation_format",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn document_annotation_format_null(mut self) -> Self {
        self.value.document_annotation_format = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "document_annotation_format", "` request field."
    )]
    #[must_use]
    pub fn document_annotation_format_absent(mut self) -> Self {
        self.value.document_annotation_format = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "document_annotation_prompt",
        "` request field to a value."
    )]
    #[must_use]
    pub fn document_annotation_prompt(mut self, document_annotation_prompt: String) -> Self {
        self.value.document_annotation_prompt = Some(Some(document_annotation_prompt));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "document_annotation_prompt",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn document_annotation_prompt_null(mut self) -> Self {
        self.value.document_annotation_prompt = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "document_annotation_prompt", "` request field."
    )]
    #[must_use]
    pub fn document_annotation_prompt_absent(mut self) -> Self {
        self.value.document_annotation_prompt = None;
        self
    }
    #[doc = concat!("Set the optional `", "extract_footer", "` request field.")]
    #[must_use]
    pub fn extract_footer(mut self, extract_footer: bool) -> Self {
        self.value.extract_footer = Some(extract_footer);
        self
    }
    #[doc = concat!("Set the optional `", "extract_header", "` request field.")]
    #[must_use]
    pub fn extract_header(mut self, extract_header: bool) -> Self {
        self.value.extract_header = Some(extract_header);
        self
    }
    #[doc = concat!("Set the optional `", "id", "` request field.")]
    #[must_use]
    pub fn id(mut self, id: String) -> Self {
        self.value.id = Some(id);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "image_limit", "` request field to a value."
    )]
    #[must_use]
    pub fn image_limit(mut self, image_limit: i64) -> Self {
        self.value.image_limit = Some(Some(image_limit));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "image_limit", "` request field to JSON null."
    )]
    #[must_use]
    pub fn image_limit_null(mut self) -> Self {
        self.value.image_limit = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "image_limit", "` request field.")]
    #[must_use]
    pub fn image_limit_absent(mut self) -> Self {
        self.value.image_limit = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "image_min_size", "` request field to a value."
    )]
    #[must_use]
    pub fn image_min_size(mut self, image_min_size: i64) -> Self {
        self.value.image_min_size = Some(Some(image_min_size));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "image_min_size", "` request field to JSON null."
    )]
    #[must_use]
    pub fn image_min_size_null(mut self) -> Self {
        self.value.image_min_size = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "image_min_size", "` request field."
    )]
    #[must_use]
    pub fn image_min_size_absent(mut self) -> Self {
        self.value.image_min_size = None;
        self
    }
    #[doc = concat!("Set the optional `", "include_blocks", "` request field.")]
    #[must_use]
    pub fn include_blocks(mut self, include_blocks: bool) -> Self {
        self.value.include_blocks = Some(include_blocks);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "include_image_base64",
        "` request field to a value."
    )]
    #[must_use]
    pub fn include_image_base64(mut self, include_image_base64: bool) -> Self {
        self.value.include_image_base64 = Some(Some(include_image_base64));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "include_image_base64",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn include_image_base64_null(mut self) -> Self {
        self.value.include_image_base64 = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "include_image_base64", "` request field."
    )]
    #[must_use]
    pub fn include_image_base64_absent(mut self) -> Self {
        self.value.include_image_base64 = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "pages", "` request field to a value."
    )]
    #[must_use]
    pub fn pages(mut self, pages: OCRRequestPages) -> Self {
        self.value.pages = Some(Some(pages));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "pages", "` request field to JSON null."
    )]
    #[must_use]
    pub fn pages_null(mut self) -> Self {
        self.value.pages = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "pages", "` request field.")]
    #[must_use]
    pub fn pages_absent(mut self) -> Self {
        self.value.pages = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "table_format", "` request field to a value."
    )]
    #[must_use]
    pub fn table_format(mut self, table_format: OCRRequestTableFormat) -> Self {
        self.value.table_format = Some(Some(table_format));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "table_format", "` request field to JSON null."
    )]
    #[must_use]
    pub fn table_format_null(mut self) -> Self {
        self.value.table_format = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "table_format", "` request field.")]
    #[must_use]
    pub fn table_format_absent(mut self) -> Self {
        self.value.table_format = None;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> OCRRequest {
        self.value
    }
}
