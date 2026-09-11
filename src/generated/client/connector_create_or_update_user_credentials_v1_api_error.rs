///Typed error responses for `connector_create_or_update_user_credentials_v1`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ConnectorCreateOrUpdateUserCredentialsV1ApiError {
    Status422(HTTPValidationError),
}
