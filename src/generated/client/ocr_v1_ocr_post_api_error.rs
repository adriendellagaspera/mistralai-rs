///Typed error responses for `ocr_v1_ocr_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum OcrV1OcrPostApiError {
    Status422(HTTPValidationError),
}
