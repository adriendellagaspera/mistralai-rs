///Typed error responses for `libraries_documents_list_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesDocumentsListV1ApiError {
    Status422(HTTPValidationError),
}
