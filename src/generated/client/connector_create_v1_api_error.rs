///Typed error responses for `connector_create_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ConnectorCreateV1ApiError {
    Status422(HTTPValidationError),
}
