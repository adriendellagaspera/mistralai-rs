use crate::{ApiError, ClientConfig, HttpClient};

pub mod prompts;
pub use prompts::PromptsClient;
pub mod skills;
pub use skills::SkillsClient;
pub mod conversations;
pub use conversations::ConversationsClient;
pub mod agents;
pub use agents::AgentsClient2;
pub mod libraries;
pub use libraries::LibrariesClient;
pub mod connectors;
pub use connectors::ConnectorsClient;
pub mod users;
pub use users::UsersClient;
pub mod admin;
pub use admin::AdminClient;
pub mod observability;
pub use observability::ObservabilityClient;
pub mod rag;
pub use rag::RagClient;
pub struct BetaClient {
    pub http_client: HttpClient,
    pub prompts: PromptsClient,
    pub skills: SkillsClient,
    pub conversations: ConversationsClient,
    pub agents: AgentsClient2,
    pub libraries: LibrariesClient,
    pub connectors: ConnectorsClient,
    pub users: UsersClient,
    pub admin: AdminClient,
    pub observability: ObservabilityClient,
    pub rag: RagClient,
}

impl BetaClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            prompts: PromptsClient::new(config.clone())?,
            skills: SkillsClient::new(config.clone())?,
            conversations: ConversationsClient::new(config.clone())?,
            agents: AgentsClient2::new(config.clone())?,
            libraries: LibrariesClient::new(config.clone())?,
            connectors: ConnectorsClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            admin: AdminClient::new(config.clone())?,
            observability: ObservabilityClient::new(config.clone())?,
            rag: RagClient::new(config.clone())?,
        })
    }
}
