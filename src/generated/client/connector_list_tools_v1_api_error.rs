///Typed error responses for `connector_list_tools_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ConnectorListToolsV1ApiError {
    Status422(HTTPValidationError),
}
