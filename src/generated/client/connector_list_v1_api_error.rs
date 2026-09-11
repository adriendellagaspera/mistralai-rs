///Typed error responses for `connector_list_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ConnectorListV1ApiError {
    Status422(HTTPValidationError),
}
