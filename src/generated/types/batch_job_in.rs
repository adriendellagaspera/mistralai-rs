#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchJobIn {
    ///In case you want to use a specific agent from the **deprecated** agents api for batch inference, you can specify the agent ID here.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub agent_id: Option<Option<String>>,
    pub endpoint: ApiEndpoint,
    ///The list of input files to be used for batch inference, these files should be `jsonl` files, containing the input data corresponding to the bory request for the batch inference in a "body" field. An example of such file is the following: ```json {"custom_id": "0", "body": {"max_tokens": 100, "messages": [{"role": "user", "content": "What is the best French cheese?"}]}} {"custom_id": "1", "body": {"max_tokens": 100, "messages": [{"role": "user", "content": "What is the best French wine?"}]}} ```
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub input_files: Option<Option<Vec<String>>>,
    ///The metadata of your choice to be associated with the batch inference job.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub metadata: Option<Option<BatchJobInMetadata>>,
    ///The model to be used for batch inference.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub model: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub requests: Option<Option<Vec<BatchRequest>>>,
    ///The timeout in hours for the batch inference job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_hours: Option<i64>,
}
