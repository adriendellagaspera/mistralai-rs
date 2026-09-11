#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowCodeDefinition {
    ///Whether the workflow enforces deterministic execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_determinism: Option<bool>,
    ///Maximum total execution time including retries and continue-as-new
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<f64>,
    ///Input schema of the workflow's run method
    pub input_schema: WorkflowCodeDefinitionInputSchema,
    ///Output schema of the workflow's run method
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub output_schema: Option<Option<WorkflowCodeDefinitionOutputSchema>>,
    ///Query handlers defined by the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<QueryDefinition>>,
    ///Signal handlers defined by the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals: Option<Vec<SignalDefinition>>,
    ///Update handlers defined by the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<UpdateDefinition>>,
}
