#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FunctionCallEventType {
    #[default]
    #[serde(rename = "function.call.delta")]
    FunctionCallDelta,
}
