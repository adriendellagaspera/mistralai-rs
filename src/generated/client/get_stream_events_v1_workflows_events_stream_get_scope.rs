///Allowed values for the `scope` query parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum GetStreamEventsV1WorkflowsEventsStreamGetScope {
    #[serde(rename = "activity")]
    Activity,
    #[serde(rename = "workflow")]
    Workflow,
    #[serde(rename = "*")]
    Value,
}
