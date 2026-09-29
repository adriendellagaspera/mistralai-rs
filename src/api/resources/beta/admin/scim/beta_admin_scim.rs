use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ScimClient {
    pub http_client: HttpClient,
}

impl ScimClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Trigger an on-demand SCIM synchronization for the Organization.
    ///
    /// Requires SAML authentication to be enabled and the Organization to be in SCIM
    /// user provisioning mode. A dry run previews every change without applying it;
    /// a real run applies the categories selected in `sync_config`. Only one run may
    /// be active at a time — a conflicting request returns the already-active run.
    ///
    /// # Arguments
    ///
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .scim
    ///         .users_api_admin_scim_sync_trigger_scim_sync(
    ///             &AdminSCIMSyncTriggerIn {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_scim_sync_trigger_scim_sync(
        &self,
        request: &AdminScimSyncTriggerIn,
        options: Option<RequestOptions>,
    ) -> Result<AdminScimSyncTriggerOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/scim/sync",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve an on-demand SCIM synchronization run for the Organization.
    ///
    /// Returns the run's lifecycle status along with the preview or result summary
    /// once the synchronization plan has been built.
    ///
    /// # Arguments
    ///
    /// * `run_id` - Identifier of the SCIM synchronization run.
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .scim
    ///         .users_api_admin_scim_sync_get_scim_sync_run(
    ///             &"6f9619ff-8b86-d011-b42d-00cf4fc964ff".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_scim_sync_get_scim_sync_run(
        &self,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdminScimSyncRunOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/admin/scim/sync/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }
}
