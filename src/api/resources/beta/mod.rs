use crate::{ApiError, ClientConfig, HttpClient};

pub mod agents;
pub use agents::AgentsClient2;
pub mod connectors;
pub use connectors::ConnectorsClient;
pub mod conversations;
pub use conversations::ConversationsClient;
pub mod libraries;
pub use libraries::LibrariesClient;
pub mod users;
pub use users::UsersClient;
pub mod prompts;
pub use prompts::PromptsClient;
pub mod skills;
pub use skills::SkillsClient;
pub mod admin;
pub use admin::AdminClient;
pub mod observability;
pub use observability::ObservabilityClient;
pub mod rag;
pub use rag::RagClient;
pub struct BetaClient {
    pub http_client: HttpClient,
    pub agents: AgentsClient2,
    pub connectors: ConnectorsClient,
    pub conversations: ConversationsClient,
    pub libraries: LibrariesClient,
    pub users: UsersClient,
    pub prompts: PromptsClient,
    pub skills: SkillsClient,
    pub admin: AdminClient,
    pub observability: ObservabilityClient,
    pub rag: RagClient,
}

impl BetaClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            agents: AgentsClient2::new(config.clone())?,
            connectors: ConnectorsClient::new(config.clone())?,
            conversations: ConversationsClient::new(config.clone())?,
            libraries: LibrariesClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            prompts: PromptsClient::new(config.clone())?,
            skills: SkillsClient::new(config.clone())?,
            admin: AdminClient::new(config.clone())?,
            observability: ObservabilityClient::new(config.clone())?,
            rag: RagClient::new(config.clone())?,
        })
    }
}
