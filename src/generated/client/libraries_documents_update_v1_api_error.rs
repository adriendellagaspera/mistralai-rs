///Typed error responses for `libraries_documents_update_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesDocumentsUpdateV1ApiError {
    Status422(HTTPValidationError),
}
