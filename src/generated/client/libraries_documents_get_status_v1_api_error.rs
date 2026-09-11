///Typed error responses for `libraries_documents_get_status_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesDocumentsGetStatusV1ApiError {
    Status422(HTTPValidationError),
}
