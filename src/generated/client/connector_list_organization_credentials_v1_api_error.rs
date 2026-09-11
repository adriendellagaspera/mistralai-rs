///Typed error responses for `connector_list_organization_credentials_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ConnectorListOrganizationCredentialsV1ApiError {
    Status422(HTTPValidationError),
}
