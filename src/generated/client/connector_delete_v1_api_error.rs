///Typed error responses for `connector_delete_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ConnectorDeleteV1ApiError {
    Status422(HTTPValidationError),
}
