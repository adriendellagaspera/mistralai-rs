use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AuditLogsClient {
    pub http_client: HttpClient,
}

impl AuditLogsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List audit log entries for the Organization.
    ///
    /// # Arguments
    ///
    /// * `actor_type` - Actor types to include in the audit log results.
    /// * `event_type` - Event types to include in the audit log results.
    /// * `target_type` - Target resource types to include in the audit log results.
    /// * `actor_user_uuid` - Filter logs by the UUID of the user who performed the action.
    /// * `sort` - Sort order for audit log entries.
    /// * `after` - Return audit log entries after this time.
    /// * `before` - Return audit log entries before this time.
    /// * `limit` - Maximum number of results to return.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .audit_logs
    ///         .users_api_admin_audit_logs_get_audit_logs(
    ///             &UsersAPIAdminAuditLogsGetAuditLogsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_audit_logs_get_audit_logs(
        &self,
        request: &UsersApiAdminAuditLogsGetAuditLogsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<AuditLogOut>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/audit-logs",
                None,
                QueryBuilder::new()
                    .serialize("actor_type", request.actor_type.clone())
                    .serialize("event_type", request.event_type.clone())
                    .serialize("target_type", request.target_type.clone())
                    .serialize("actor_user_uuid", request.actor_user_uuid.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("after", request.after.clone())
                    .serialize("before", request.before.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }
}
