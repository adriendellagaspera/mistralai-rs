///Typed error responses for `libraries_get_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesGetV1ApiError {
    Status422(HTTPValidationError),
}
