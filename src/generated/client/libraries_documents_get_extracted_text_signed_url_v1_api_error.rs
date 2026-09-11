///Typed error responses for `libraries_documents_get_extracted_text_signed_url_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesDocumentsGetExtractedTextSignedUrlV1ApiError {
    Status422(HTTPValidationError),
}
