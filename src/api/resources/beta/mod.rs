use crate::{ApiError, ClientConfig, HttpClient};

pub mod agents;
pub use agents::AgentsClient3;
pub mod billing;
pub use billing::BillingClient;
pub mod connectors;
pub use connectors::ConnectorsClient;
pub mod conversations;
pub use conversations::ConversationsClient;
pub mod libraries;
pub use libraries::LibrariesClient;
pub mod service_accounts;
pub use service_accounts::ServiceAccountsClient;
pub mod triggers;
pub use triggers::TriggersClient;
pub mod users;
pub use users::UsersClient;
pub mod webhooks;
pub use webhooks::WebhooksClient;
pub mod prompts;
pub use prompts::PromptsClient;
pub mod skills;
pub use skills::SkillsClient;
pub mod admin;
pub use admin::AdminClient;
pub mod context;
pub use context::ContextClient;
pub mod observability;
pub use observability::ObservabilityClient;
pub mod rag;
pub use rag::RagClient;
pub struct BetaClient {
    pub http_client: HttpClient,
    pub agents: AgentsClient3,
    pub billing: BillingClient,
    pub connectors: ConnectorsClient,
    pub conversations: ConversationsClient,
    pub libraries: LibrariesClient,
    pub service_accounts: ServiceAccountsClient,
    pub triggers: TriggersClient,
    pub users: UsersClient,
    pub webhooks: WebhooksClient,
    pub prompts: PromptsClient,
    pub skills: SkillsClient,
    pub admin: AdminClient,
    pub context: ContextClient,
    pub observability: ObservabilityClient,
    pub rag: RagClient,
}

impl BetaClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            agents: AgentsClient3::new(config.clone())?,
            billing: BillingClient::new(config.clone())?,
            connectors: ConnectorsClient::new(config.clone())?,
            conversations: ConversationsClient::new(config.clone())?,
            libraries: LibrariesClient::new(config.clone())?,
            service_accounts: ServiceAccountsClient::new(config.clone())?,
            triggers: TriggersClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            prompts: PromptsClient::new(config.clone())?,
            skills: SkillsClient::new(config.clone())?,
            admin: AdminClient::new(config.clone())?,
            context: ContextClient::new(config.clone())?,
            observability: ObservabilityClient::new(config.clone())?,
            rag: RagClient::new(config.clone())?,
        })
    }
}
