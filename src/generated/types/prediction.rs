///Enable users to specify an expected completion, optimizing response times by leveraging known or predictable content.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Prediction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PredictionType>,
}
