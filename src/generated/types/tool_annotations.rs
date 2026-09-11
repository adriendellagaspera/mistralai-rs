/**Additional properties describing a Tool to clients.

NOTE: all properties in ToolAnnotations are **hints**.
They are not guaranteed to provide a faithful description of
tool behavior (including descriptive properties like `title`).

Clients should never make tool use decisions based on ToolAnnotations
received from untrusted servers.*/
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ToolAnnotations {
    #[serde(
        rename = "destructiveHint",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub destructive_hint: Option<Option<bool>>,
    #[serde(
        rename = "idempotentHint",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub idempotent_hint: Option<Option<bool>>,
    #[serde(
        rename = "openWorldHint",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub open_world_hint: Option<Option<bool>>,
    #[serde(
        rename = "readOnlyHint",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub read_only_hint: Option<Option<bool>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub title: Option<Option<String>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
