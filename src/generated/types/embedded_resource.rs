/**The contents of a resource, embedded into a prompt or tool call result.

It is up to the client how best to render embedded resources for the benefit
of the LLM and/or the user.*/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddedResource {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub meta: Option<Option<EmbeddedResourceMeta>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub annotations: Option<Option<Annotations>>,
    pub resource: EmbeddedResourceResource,
    pub r#type: EmbeddedResourceType,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
