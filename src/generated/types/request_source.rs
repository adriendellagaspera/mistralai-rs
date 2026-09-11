#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RequestSource {
    #[default]
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "playground")]
    Playground,
    #[serde(rename = "agent_builder_v1")]
    AgentBuilderV1,
}
