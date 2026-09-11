///Typed error responses for `libraries_documents_upload_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesDocumentsUploadV1ApiError {
    Status422(HTTPValidationError),
}
