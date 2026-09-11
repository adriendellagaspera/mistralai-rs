use super::SdkError;
use crate::generated::client::HttpClient;
use crate::generated::types::{OCRRequest, OCRResponse as GeneratedOCRResponse};

/// OCR response with the generated model deliberately kept behind an explicit
/// escape hatch. Higher-level block accessors can evolve without exposing
/// generator-invented nested type names as the primary SDK surface.
#[derive(Debug)]
pub struct OcrResponse {
    raw: GeneratedOCRResponse,
}

impl OcrResponse {
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

/// Ergonomic request for OCR over a remotely accessible document.
#[derive(Debug, Clone)]
pub struct OcrRequest {
    model: String,
    document_url: String,
}

impl OcrRequest {
    pub fn document_url(model: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            document_url: url.into(),
        }
    }

    fn into_raw(self) -> Result<OCRRequest, SdkError> {
        Ok(serde_json::from_value(serde_json::json!({
            "model": self.model,
            "document": {
                "type": "document_url",
                "document_url": self.document_url,
            }
        }))?)
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
            .ocr_v1_ocr_post(request.into_raw()?)
            .await
            .map(OcrResponse::from)
            .map_err(SdkError::api)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_url_request_maps_to_generated_ocr_contract() {
        let raw =
            OcrRequest::document_url("mistral-ocr-latest", "https://example.com/document.pdf")
                .into_raw()
                .unwrap();

        let value = serde_json::to_value(raw).unwrap();
        assert_eq!(value["model"], "mistral-ocr-latest");
        assert_eq!(value["document"]["type"], "document_url");
        assert_eq!(
            value["document"]["document_url"],
            "https://example.com/document.pdf"
        );
    }
}
