///Typed error responses for `libraries_documents_get_signed_url_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesDocumentsGetSignedUrlV1ApiError {
    Status422(HTTPValidationError),
}
