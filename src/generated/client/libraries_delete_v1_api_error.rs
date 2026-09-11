///Typed error responses for `libraries_delete_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesDeleteV1ApiError {
    Status422(HTTPValidationError),
}
