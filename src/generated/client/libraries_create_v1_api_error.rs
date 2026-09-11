///Typed error responses for `libraries_create_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum LibrariesCreateV1ApiError {
    Status422(HTTPValidationError),
}
