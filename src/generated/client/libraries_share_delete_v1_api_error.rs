///Typed error responses for `libraries_share_delete_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesShareDeleteV1ApiError {
    Status422(HTTPValidationError),
}
