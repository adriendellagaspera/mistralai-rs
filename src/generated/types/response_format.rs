///Specify the format that the model must output. By default it will use `{ "type": "text" }`. Setting to `{ "type": "json_object" }` enables JSON mode, which guarantees the message the model generates is in JSON. When using JSON mode you MUST also instruct the model to produce JSON yourself with a system or a user message. Setting to `{ "type": "json_schema" }` enables JSON schema mode, which guarantees the message the model generates is in JSON and follows the schema you provide.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ResponseFormat {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub json_schema: Option<Option<JsonSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ResponseFormats>,
}
