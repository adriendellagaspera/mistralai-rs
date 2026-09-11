#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ObservabilityErrorCode {
    #[default]
    #[serde(rename = "UNKNOWN_ERROR")]
    UnknownError,
    #[serde(rename = "VALIDATION_ERROR")]
    ValidationError,
    #[serde(rename = "AUTH_FORBIDDEN")]
    AuthForbidden,
    #[serde(rename = "AUTH_FORBIDDEN_NOT_WORKSPACE_ADMIN")]
    AuthForbiddenNotWorkspaceAdmin,
    #[serde(rename = "AUTH_FORBIDDEN_WORKSPACE_NOT_FOUND")]
    AuthForbiddenWorkspaceNotFound,
    #[serde(rename = "AUTH_FORBIDDEN_ROLE_NOT_FOUND")]
    AuthForbiddenRoleNotFound,
    #[serde(rename = "AUTH_FORBIDDEN_ORG_NOT_WHITELISTED")]
    AuthForbiddenOrgNotWhitelisted,
    #[serde(rename = "AUTH_UNAUTHORIZED")]
    AuthUnauthorized,
    #[serde(rename = "FEATURE_NOT_SUPPORTED")]
    FeatureNotSupported,
    #[serde(rename = "FIELDS_BAD_REQUEST")]
    FieldsBadRequest,
    #[serde(rename = "FIELDS_NOT_FOUND")]
    FieldsNotFound,
    #[serde(rename = "SEARCH_NOT_FOUND")]
    SearchNotFound,
    #[serde(rename = "SEARCH_BAD_REQUEST")]
    SearchBadRequest,
    #[serde(rename = "SEARCH_SERVICE_UNAVAILABLE")]
    SearchServiceUnavailable,
    #[serde(rename = "DATABASE_ERROR")]
    DatabaseError,
    #[serde(rename = "DATABASE_TIMEOUT")]
    DatabaseTimeout,
    #[serde(rename = "DATABASE_UNAVAILABLE")]
    DatabaseUnavailable,
    #[serde(rename = "DATABASE_QUERY_ERROR")]
    DatabaseQueryError,
    #[serde(rename = "SEARCH_FILTER_TO_SQL_CONVERSION_ERROR")]
    SearchFilterToSqlConversionError,
    #[serde(rename = "JUDGE_CONVERSATION_FORMAT_ERROR")]
    JudgeConversationFormatError,
    #[serde(rename = "JUDGE_MISTRAL_API_ERROR")]
    JudgeMistralApiError,
    #[serde(rename = "JUDGE_MISTRAL_API_TIMEOUT")]
    JudgeMistralApiTimeout,
    #[serde(rename = "JUDGE_NAME_ALREADY_EXISTS")]
    JudgeNameAlreadyExists,
    #[serde(rename = "JUDGE_NOT_FOUND")]
    JudgeNotFound,
    #[serde(rename = "JUDGE_ALREADY_HAS_NEW_VERSION")]
    JudgeAlreadyHasNewVersion,
    #[serde(rename = "JUDGE_USED_IN_CAMPAIGN_CANNOT_BE_UPDATED")]
    JudgeUsedInCampaignCannotBeUpdated,
    #[serde(rename = "JUDGE_DID_NOT_CHANGE")]
    JudgeDidNotChange,
    #[serde(rename = "CAMPAIGN_NOT_FOUND")]
    CampaignNotFound,
    #[serde(rename = "CAMPAIGN_NO_MATCHING_EVENTS")]
    CampaignNoMatchingEvents,
    #[serde(rename = "DATASET_NOT_FOUND")]
    DatasetNotFound,
    #[serde(rename = "DATASET_TASK_NOT_FOUND")]
    DatasetTaskNotFound,
    #[serde(rename = "DATASET_RECORD_NOT_FOUND")]
    DatasetRecordNotFound,
    #[serde(rename = "DATASET_RECORD_FORMAT_ERROR")]
    DatasetRecordFormatError,
    #[serde(rename = "AGENT_NOT_FOUND")]
    AgentNotFound,
    #[serde(rename = "AGENT_MISTRAL_API_ERROR")]
    AgentMistralApiError,
    #[serde(rename = "EVALUATION_NOT_FOUND")]
    EvaluationNotFound,
    #[serde(rename = "EVALUATION_CURRENTLY_RUNNING")]
    EvaluationCurrentlyRunning,
    #[serde(rename = "EVALUATION_RECORD_NOT_FOUND")]
    EvaluationRecordNotFound,
    #[serde(rename = "EVALUATION_RUN_NOT_FOUND")]
    EvaluationRunNotFound,
    #[serde(rename = "EVALUATION_RUN_TRANSITION_IS_INVALID")]
    EvaluationRunTransitionIsInvalid,
    #[serde(rename = "EVALUATION_RUN_TRANSITION_IS_RUNNING_ALREADY")]
    EvaluationRunTransitionIsRunningAlready,
    #[serde(rename = "EVALUATION_RUN_TRANSITION_ERROR")]
    EvaluationRunTransitionError,
    #[serde(rename = "TEMPLATE_SYNTAX_ERROR")]
    TemplateSyntaxError,
}
