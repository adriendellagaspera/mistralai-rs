///Typed error responses for `connector_call_tool_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ConnectorCallToolV1ApiError {
    Status422(HTTPValidationError),
}
