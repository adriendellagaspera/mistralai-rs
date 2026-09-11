///Typed error responses for `connector_get_auth_url_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ConnectorGetAuthUrlV1ApiError {
    Status422(HTTPValidationError),
}
