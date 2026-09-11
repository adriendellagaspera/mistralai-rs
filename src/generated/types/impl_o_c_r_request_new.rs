impl OCRRequest {
    /// Construct this request with every required wire field.
    pub fn new(document: OCRRequestDocument, model: Option<String>) -> Self {
        Self {
            document,
            model,
            bbox_annotation_format: None,
            confidence_scores_granularity: None,
            document_annotation_format: None,
            document_annotation_prompt: None,
            extract_footer: None,
            extract_header: None,
            id: None,
            image_limit: None,
            image_min_size: None,
            include_blocks: None,
            include_image_base64: None,
            pages: None,
            table_format: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(document: OCRRequestDocument, model: Option<String>) -> OCRRequestBuilder {
        OCRRequestBuilder::new(document, model)
    }
}
