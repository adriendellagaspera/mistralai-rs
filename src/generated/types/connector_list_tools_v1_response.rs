#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConnectorListToolsV1Response {
    IntegrationsSchemasApiToolToolArray(IntegrationsSchemasApiToolToolArray),
    MCPToolArray(MCPToolArray),
    ConnectorListToolsV1ResponseItemArray(ConnectorListToolsV1ResponseItemArray),
}
