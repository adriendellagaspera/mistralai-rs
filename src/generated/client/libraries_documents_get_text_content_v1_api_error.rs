///Typed error responses for `libraries_documents_get_text_content_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesDocumentsGetTextContentV1ApiError {
    Status422(HTTPValidationError),
}
