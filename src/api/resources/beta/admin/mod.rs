use crate::{ApiError, ClientConfig, HttpClient};

pub mod vibe_code_analytics;
pub use vibe_code_analytics::VibeCodeAnalyticsClient;
pub mod vibe_work_analytics;
pub use vibe_work_analytics::VibeWorkAnalyticsClient;
pub mod api_keys;
pub use api_keys::ApiKeysClient;
pub mod audit_logs;
pub use audit_logs::AuditLogsClient;
pub mod billing;
pub use billing::BillingClient;
pub mod users;
pub use users::UsersClient2;
pub mod scim;
pub use scim::ScimClient;
pub mod user_groups;
pub use user_groups::UserGroupsClient;
pub mod workspaces;
pub use workspaces::WorkspacesClient;
pub struct AdminClient {
    pub http_client: HttpClient,
    pub vibe_code_analytics: VibeCodeAnalyticsClient,
    pub vibe_work_analytics: VibeWorkAnalyticsClient,
    pub api_keys: ApiKeysClient,
    pub audit_logs: AuditLogsClient,
    pub billing: BillingClient,
    pub users: UsersClient2,
    pub scim: ScimClient,
    pub user_groups: UserGroupsClient,
    pub workspaces: WorkspacesClient,
}

impl AdminClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            vibe_code_analytics: VibeCodeAnalyticsClient::new(config.clone())?,
            vibe_work_analytics: VibeWorkAnalyticsClient::new(config.clone())?,
            api_keys: ApiKeysClient::new(config.clone())?,
            audit_logs: AuditLogsClient::new(config.clone())?,
            billing: BillingClient::new(config.clone())?,
            users: UsersClient2::new(config.clone())?,
            scim: ScimClient::new(config.clone())?,
            user_groups: UserGroupsClient::new(config.clone())?,
            workspaces: WorkspacesClient::new(config.clone())?,
        })
    }
}
