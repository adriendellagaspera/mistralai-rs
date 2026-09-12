use super::SdkError;
use crate::generated::client::HttpClient;
use crate::generated::types::{
    DocumentURLChunk, FileChunk, ImageURLChunk, ImageURLChunkImageUrl, OCRPageObject, OCRRequest,
    OCRRequestDocument, OCRResponse as GeneratedOCRResponse,
};

/// OCR response with useful stable accessors and an explicit raw escape hatch.
#[derive(Debug)]
pub struct OcrResponse {
    raw: GeneratedOCRResponse,
}

impl OcrResponse {
    pub fn model(&self) -> &str {
        &self.raw.model
    }

    pub fn pages(&self) -> impl ExactSizeIterator<Item = OcrPage<'_>> {
        self.raw.pages.iter().map(OcrPage::new)
    }

    pub fn raw(&self) -> &GeneratedOCRResponse {
        &self.raw
    }

    pub fn into_raw(self) -> GeneratedOCRResponse {
        self.raw
    }
}

impl From<GeneratedOCRResponse> for OcrResponse {
    fn from(raw: GeneratedOCRResponse) -> Self {
        Self { raw }
    }
}

/// Stable read-only view of one OCR page.
#[derive(Debug, Clone, Copy)]
pub struct OcrPage<'a> {
    raw: &'a OCRPageObject,
}

impl<'a> OcrPage<'a> {
    fn new(raw: &'a OCRPageObject) -> Self {
        Self { raw }
    }

    pub fn index(&self) -> i64 {
        self.raw.index
    }

    pub fn markdown(&self) -> &str {
        &self.raw.markdown
    }

    pub fn raw(&self) -> &OCRPageObject {
        self.raw
    }
}

/// An OCR request.
///
/// Constructors cover every input kind accepted by Mistral. OCRRequest::from_raw
/// remains available for advanced generated options.
#[derive(Debug, Clone)]
pub struct OcrRequest {
    raw: OCRRequest,
}

impl OcrRequest {
    pub fn document_url(model: impl Into<String>, url: impl Into<String>) -> Self {
        let document = OCRRequestDocument::DocumentURLChunk(DocumentURLChunk {
            document_name: None,
            document_url: url.into(),
            r#type: None,
        });
        Self::new(model, document)
    }

    pub fn image_url(model: impl Into<String>, url: impl Into<String>) -> Self {
        let document = OCRRequestDocument::ImageURLChunk(ImageURLChunk {
            image_url: ImageURLChunkImageUrl::String(url.into()),
            r#type: None,
        });
        Self::new(model, document)
    }

    pub fn file_id(model: impl Into<String>, file_id: uuid::Uuid) -> Self {
        let document = OCRRequestDocument::FileChunk(FileChunk {
            file_id,
            r#type: None,
        });
        Self::new(model, document)
    }

    fn new(model: impl Into<String>, document: OCRRequestDocument) -> Self {
        Self {
            raw: OCRRequest::new(document, Some(model.into())),
        }
    }

    pub fn from_raw(raw: OCRRequest) -> Self {
        Self { raw }
    }

    pub fn include_image_base64(mut self, include: bool) -> Self {
        self.raw.include_image_base64 = Some(Some(include));
        self
    }

    pub fn include_blocks(mut self, include: bool) -> Self {
        self.raw.include_blocks = Some(include);
        self
    }

    pub fn extract_header(mut self, extract: bool) -> Self {
        self.raw.extract_header = Some(extract);
        self
    }

    pub fn extract_footer(mut self, extract: bool) -> Self {
        self.raw.extract_footer = Some(extract);
        self
    }

    pub fn as_raw(&self) -> &OCRRequest {
        &self.raw
    }

    pub fn into_raw(self) -> OCRRequest {
        self.raw
    }
}

/// OCR resource, matching the taxonomy of Mistral's official SDKs.
#[derive(Clone, Copy)]
pub struct Ocr<'a> {
    raw: &'a HttpClient,
}

impl<'a> Ocr<'a> {
    pub(crate) fn new(raw: &'a HttpClient) -> Self {
        Self { raw }
    }

    /// Process a document with OCR.
    pub async fn process(&self, request: OcrRequest) -> Result<OcrResponse, SdkError> {
        self.raw
            .ocr_v1_ocr_post(request.into_raw())
            .await
            .map(OcrResponse::from)
            .map_err(SdkError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_url_request_maps_directly_to_generated_ocr_contract() {
        let raw =
            OcrRequest::document_url("mistral-ocr-latest", "https://example.com/document.pdf")
                .include_blocks(true)
                .into_raw();

        let value = serde_json::to_value(raw).unwrap();
        assert_eq!(value["model"], "mistral-ocr-latest");
        assert_eq!(value["document"]["type"], "document_url");
        assert_eq!(
            value["document"]["document_url"],
            "https://example.com/document.pdf"
        );
        assert_eq!(value["include_blocks"], true);
    }

    #[test]
    fn all_ocr_input_kinds_have_typed_constructors() {
        let image = OcrRequest::image_url("ocr", "https://example.com/image.png").into_raw();
        assert!(matches!(
            image.document,
            OCRRequestDocument::ImageURLChunk(_)
        ));

        let file = OcrRequest::file_id("ocr", uuid::Uuid::nil()).into_raw();
        assert!(matches!(file.document, OCRRequestDocument::FileChunk(_)));
    }
}
