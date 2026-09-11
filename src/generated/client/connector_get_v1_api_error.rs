///Typed error responses for `connector_get_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ConnectorGetV1ApiError {
    Status422(HTTPValidationError),
}
