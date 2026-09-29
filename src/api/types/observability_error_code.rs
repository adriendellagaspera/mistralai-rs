pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObservabilityErrorCode {
    UnknownError,
    ValidationError,
    AuthForbidden,
    AuthForbiddenNotWorkspaceAdmin,
    AuthForbiddenWorkspaceNotFound,
    AuthForbiddenRoleNotFound,
    AuthUnauthorized,
    FeatureNotSupported,
    FieldsBadRequest,
    FieldsNotFound,
    SearchNotFound,
    SearchBadRequest,
    SearchServiceUnavailable,
    DatabaseError,
    DatabaseTimeout,
    DatabaseUnavailable,
    DatabaseQueryError,
    SearchFilterToSqlConversionError,
    JudgeConversationFormatError,
    JudgeMistralApiError,
    JudgeMistralApiTimeout,
    JudgeNameAlreadyExists,
    JudgeNotFound,
    JudgeAlreadyHasNewVersion,
    JudgeUsedInCampaignCannotBeUpdated,
    JudgeDidNotChange,
    CampaignNotFound,
    CampaignNoMatchingEvents,
    DatasetNotFound,
    DatasetTaskNotFound,
    DatasetRecordNotFound,
    DatasetRecordFormatError,
    AgentNotFound,
    AgentMistralApiError,
    EvaluationNotFound,
    EvaluationCurrentlyRunning,
    EvaluationRecordNotFound,
    EvaluationRunNotFound,
    EvaluationRunTransitionIsInvalid,
    EvaluationRunTransitionIsRunningAlready,
    EvaluationRunTransitionError,
    TemplateError,
    TemplateSyntaxError,
    ProjectNameAlreadyExists,
    EvaluationNameAlreadyExists,
    OptimizationTrialKeyAlreadyExists,
    TracesFilterQueryParseError,
    TraceNotFound,
    SpanNotFound,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ObservabilityErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UnknownError => serializer.serialize_str("UNKNOWN_ERROR"),
            Self::ValidationError => serializer.serialize_str("VALIDATION_ERROR"),
            Self::AuthForbidden => serializer.serialize_str("AUTH_FORBIDDEN"),
            Self::AuthForbiddenNotWorkspaceAdmin => {
                serializer.serialize_str("AUTH_FORBIDDEN_NOT_WORKSPACE_ADMIN")
            }
            Self::AuthForbiddenWorkspaceNotFound => {
                serializer.serialize_str("AUTH_FORBIDDEN_WORKSPACE_NOT_FOUND")
            }
            Self::AuthForbiddenRoleNotFound => {
                serializer.serialize_str("AUTH_FORBIDDEN_ROLE_NOT_FOUND")
            }
            Self::AuthUnauthorized => serializer.serialize_str("AUTH_UNAUTHORIZED"),
            Self::FeatureNotSupported => serializer.serialize_str("FEATURE_NOT_SUPPORTED"),
            Self::FieldsBadRequest => serializer.serialize_str("FIELDS_BAD_REQUEST"),
            Self::FieldsNotFound => serializer.serialize_str("FIELDS_NOT_FOUND"),
            Self::SearchNotFound => serializer.serialize_str("SEARCH_NOT_FOUND"),
            Self::SearchBadRequest => serializer.serialize_str("SEARCH_BAD_REQUEST"),
            Self::SearchServiceUnavailable => {
                serializer.serialize_str("SEARCH_SERVICE_UNAVAILABLE")
            }
            Self::DatabaseError => serializer.serialize_str("DATABASE_ERROR"),
            Self::DatabaseTimeout => serializer.serialize_str("DATABASE_TIMEOUT"),
            Self::DatabaseUnavailable => serializer.serialize_str("DATABASE_UNAVAILABLE"),
            Self::DatabaseQueryError => serializer.serialize_str("DATABASE_QUERY_ERROR"),
            Self::SearchFilterToSqlConversionError => {
                serializer.serialize_str("SEARCH_FILTER_TO_SQL_CONVERSION_ERROR")
            }
            Self::JudgeConversationFormatError => {
                serializer.serialize_str("JUDGE_CONVERSATION_FORMAT_ERROR")
            }
            Self::JudgeMistralApiError => serializer.serialize_str("JUDGE_MISTRAL_API_ERROR"),
            Self::JudgeMistralApiTimeout => serializer.serialize_str("JUDGE_MISTRAL_API_TIMEOUT"),
            Self::JudgeNameAlreadyExists => serializer.serialize_str("JUDGE_NAME_ALREADY_EXISTS"),
            Self::JudgeNotFound => serializer.serialize_str("JUDGE_NOT_FOUND"),
            Self::JudgeAlreadyHasNewVersion => {
                serializer.serialize_str("JUDGE_ALREADY_HAS_NEW_VERSION")
            }
            Self::JudgeUsedInCampaignCannotBeUpdated => {
                serializer.serialize_str("JUDGE_USED_IN_CAMPAIGN_CANNOT_BE_UPDATED")
            }
            Self::JudgeDidNotChange => serializer.serialize_str("JUDGE_DID_NOT_CHANGE"),
            Self::CampaignNotFound => serializer.serialize_str("CAMPAIGN_NOT_FOUND"),
            Self::CampaignNoMatchingEvents => {
                serializer.serialize_str("CAMPAIGN_NO_MATCHING_EVENTS")
            }
            Self::DatasetNotFound => serializer.serialize_str("DATASET_NOT_FOUND"),
            Self::DatasetTaskNotFound => serializer.serialize_str("DATASET_TASK_NOT_FOUND"),
            Self::DatasetRecordNotFound => serializer.serialize_str("DATASET_RECORD_NOT_FOUND"),
            Self::DatasetRecordFormatError => {
                serializer.serialize_str("DATASET_RECORD_FORMAT_ERROR")
            }
            Self::AgentNotFound => serializer.serialize_str("AGENT_NOT_FOUND"),
            Self::AgentMistralApiError => serializer.serialize_str("AGENT_MISTRAL_API_ERROR"),
            Self::EvaluationNotFound => serializer.serialize_str("EVALUATION_NOT_FOUND"),
            Self::EvaluationCurrentlyRunning => {
                serializer.serialize_str("EVALUATION_CURRENTLY_RUNNING")
            }
            Self::EvaluationRecordNotFound => {
                serializer.serialize_str("EVALUATION_RECORD_NOT_FOUND")
            }
            Self::EvaluationRunNotFound => serializer.serialize_str("EVALUATION_RUN_NOT_FOUND"),
            Self::EvaluationRunTransitionIsInvalid => {
                serializer.serialize_str("EVALUATION_RUN_TRANSITION_IS_INVALID")
            }
            Self::EvaluationRunTransitionIsRunningAlready => {
                serializer.serialize_str("EVALUATION_RUN_TRANSITION_IS_RUNNING_ALREADY")
            }
            Self::EvaluationRunTransitionError => {
                serializer.serialize_str("EVALUATION_RUN_TRANSITION_ERROR")
            }
            Self::TemplateError => serializer.serialize_str("TEMPLATE_ERROR"),
            Self::TemplateSyntaxError => serializer.serialize_str("TEMPLATE_SYNTAX_ERROR"),
            Self::ProjectNameAlreadyExists => {
                serializer.serialize_str("PROJECT_NAME_ALREADY_EXISTS")
            }
            Self::EvaluationNameAlreadyExists => {
                serializer.serialize_str("EVALUATION_NAME_ALREADY_EXISTS")
            }
            Self::OptimizationTrialKeyAlreadyExists => {
                serializer.serialize_str("OPTIMIZATION_TRIAL_KEY_ALREADY_EXISTS")
            }
            Self::TracesFilterQueryParseError => {
                serializer.serialize_str("TRACES_FILTER_QUERY_PARSE_ERROR")
            }
            Self::TraceNotFound => serializer.serialize_str("TRACE_NOT_FOUND"),
            Self::SpanNotFound => serializer.serialize_str("SPAN_NOT_FOUND"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ObservabilityErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "UNKNOWN_ERROR" => Ok(Self::UnknownError),
            "VALIDATION_ERROR" => Ok(Self::ValidationError),
            "AUTH_FORBIDDEN" => Ok(Self::AuthForbidden),
            "AUTH_FORBIDDEN_NOT_WORKSPACE_ADMIN" => Ok(Self::AuthForbiddenNotWorkspaceAdmin),
            "AUTH_FORBIDDEN_WORKSPACE_NOT_FOUND" => Ok(Self::AuthForbiddenWorkspaceNotFound),
            "AUTH_FORBIDDEN_ROLE_NOT_FOUND" => Ok(Self::AuthForbiddenRoleNotFound),
            "AUTH_UNAUTHORIZED" => Ok(Self::AuthUnauthorized),
            "FEATURE_NOT_SUPPORTED" => Ok(Self::FeatureNotSupported),
            "FIELDS_BAD_REQUEST" => Ok(Self::FieldsBadRequest),
            "FIELDS_NOT_FOUND" => Ok(Self::FieldsNotFound),
            "SEARCH_NOT_FOUND" => Ok(Self::SearchNotFound),
            "SEARCH_BAD_REQUEST" => Ok(Self::SearchBadRequest),
            "SEARCH_SERVICE_UNAVAILABLE" => Ok(Self::SearchServiceUnavailable),
            "DATABASE_ERROR" => Ok(Self::DatabaseError),
            "DATABASE_TIMEOUT" => Ok(Self::DatabaseTimeout),
            "DATABASE_UNAVAILABLE" => Ok(Self::DatabaseUnavailable),
            "DATABASE_QUERY_ERROR" => Ok(Self::DatabaseQueryError),
            "SEARCH_FILTER_TO_SQL_CONVERSION_ERROR" => Ok(Self::SearchFilterToSqlConversionError),
            "JUDGE_CONVERSATION_FORMAT_ERROR" => Ok(Self::JudgeConversationFormatError),
            "JUDGE_MISTRAL_API_ERROR" => Ok(Self::JudgeMistralApiError),
            "JUDGE_MISTRAL_API_TIMEOUT" => Ok(Self::JudgeMistralApiTimeout),
            "JUDGE_NAME_ALREADY_EXISTS" => Ok(Self::JudgeNameAlreadyExists),
            "JUDGE_NOT_FOUND" => Ok(Self::JudgeNotFound),
            "JUDGE_ALREADY_HAS_NEW_VERSION" => Ok(Self::JudgeAlreadyHasNewVersion),
            "JUDGE_USED_IN_CAMPAIGN_CANNOT_BE_UPDATED" => {
                Ok(Self::JudgeUsedInCampaignCannotBeUpdated)
            }
            "JUDGE_DID_NOT_CHANGE" => Ok(Self::JudgeDidNotChange),
            "CAMPAIGN_NOT_FOUND" => Ok(Self::CampaignNotFound),
            "CAMPAIGN_NO_MATCHING_EVENTS" => Ok(Self::CampaignNoMatchingEvents),
            "DATASET_NOT_FOUND" => Ok(Self::DatasetNotFound),
            "DATASET_TASK_NOT_FOUND" => Ok(Self::DatasetTaskNotFound),
            "DATASET_RECORD_NOT_FOUND" => Ok(Self::DatasetRecordNotFound),
            "DATASET_RECORD_FORMAT_ERROR" => Ok(Self::DatasetRecordFormatError),
            "AGENT_NOT_FOUND" => Ok(Self::AgentNotFound),
            "AGENT_MISTRAL_API_ERROR" => Ok(Self::AgentMistralApiError),
            "EVALUATION_NOT_FOUND" => Ok(Self::EvaluationNotFound),
            "EVALUATION_CURRENTLY_RUNNING" => Ok(Self::EvaluationCurrentlyRunning),
            "EVALUATION_RECORD_NOT_FOUND" => Ok(Self::EvaluationRecordNotFound),
            "EVALUATION_RUN_NOT_FOUND" => Ok(Self::EvaluationRunNotFound),
            "EVALUATION_RUN_TRANSITION_IS_INVALID" => Ok(Self::EvaluationRunTransitionIsInvalid),
            "EVALUATION_RUN_TRANSITION_IS_RUNNING_ALREADY" => {
                Ok(Self::EvaluationRunTransitionIsRunningAlready)
            }
            "EVALUATION_RUN_TRANSITION_ERROR" => Ok(Self::EvaluationRunTransitionError),
            "TEMPLATE_ERROR" => Ok(Self::TemplateError),
            "TEMPLATE_SYNTAX_ERROR" => Ok(Self::TemplateSyntaxError),
            "PROJECT_NAME_ALREADY_EXISTS" => Ok(Self::ProjectNameAlreadyExists),
            "EVALUATION_NAME_ALREADY_EXISTS" => Ok(Self::EvaluationNameAlreadyExists),
            "OPTIMIZATION_TRIAL_KEY_ALREADY_EXISTS" => Ok(Self::OptimizationTrialKeyAlreadyExists),
            "TRACES_FILTER_QUERY_PARSE_ERROR" => Ok(Self::TracesFilterQueryParseError),
            "TRACE_NOT_FOUND" => Ok(Self::TraceNotFound),
            "SPAN_NOT_FOUND" => Ok(Self::SpanNotFound),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ObservabilityErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownError => write!(f, "UNKNOWN_ERROR"),
            Self::ValidationError => write!(f, "VALIDATION_ERROR"),
            Self::AuthForbidden => write!(f, "AUTH_FORBIDDEN"),
            Self::AuthForbiddenNotWorkspaceAdmin => write!(f, "AUTH_FORBIDDEN_NOT_WORKSPACE_ADMIN"),
            Self::AuthForbiddenWorkspaceNotFound => write!(f, "AUTH_FORBIDDEN_WORKSPACE_NOT_FOUND"),
            Self::AuthForbiddenRoleNotFound => write!(f, "AUTH_FORBIDDEN_ROLE_NOT_FOUND"),
            Self::AuthUnauthorized => write!(f, "AUTH_UNAUTHORIZED"),
            Self::FeatureNotSupported => write!(f, "FEATURE_NOT_SUPPORTED"),
            Self::FieldsBadRequest => write!(f, "FIELDS_BAD_REQUEST"),
            Self::FieldsNotFound => write!(f, "FIELDS_NOT_FOUND"),
            Self::SearchNotFound => write!(f, "SEARCH_NOT_FOUND"),
            Self::SearchBadRequest => write!(f, "SEARCH_BAD_REQUEST"),
            Self::SearchServiceUnavailable => write!(f, "SEARCH_SERVICE_UNAVAILABLE"),
            Self::DatabaseError => write!(f, "DATABASE_ERROR"),
            Self::DatabaseTimeout => write!(f, "DATABASE_TIMEOUT"),
            Self::DatabaseUnavailable => write!(f, "DATABASE_UNAVAILABLE"),
            Self::DatabaseQueryError => write!(f, "DATABASE_QUERY_ERROR"),
            Self::SearchFilterToSqlConversionError => {
                write!(f, "SEARCH_FILTER_TO_SQL_CONVERSION_ERROR")
            }
            Self::JudgeConversationFormatError => write!(f, "JUDGE_CONVERSATION_FORMAT_ERROR"),
            Self::JudgeMistralApiError => write!(f, "JUDGE_MISTRAL_API_ERROR"),
            Self::JudgeMistralApiTimeout => write!(f, "JUDGE_MISTRAL_API_TIMEOUT"),
            Self::JudgeNameAlreadyExists => write!(f, "JUDGE_NAME_ALREADY_EXISTS"),
            Self::JudgeNotFound => write!(f, "JUDGE_NOT_FOUND"),
            Self::JudgeAlreadyHasNewVersion => write!(f, "JUDGE_ALREADY_HAS_NEW_VERSION"),
            Self::JudgeUsedInCampaignCannotBeUpdated => {
                write!(f, "JUDGE_USED_IN_CAMPAIGN_CANNOT_BE_UPDATED")
            }
            Self::JudgeDidNotChange => write!(f, "JUDGE_DID_NOT_CHANGE"),
            Self::CampaignNotFound => write!(f, "CAMPAIGN_NOT_FOUND"),
            Self::CampaignNoMatchingEvents => write!(f, "CAMPAIGN_NO_MATCHING_EVENTS"),
            Self::DatasetNotFound => write!(f, "DATASET_NOT_FOUND"),
            Self::DatasetTaskNotFound => write!(f, "DATASET_TASK_NOT_FOUND"),
            Self::DatasetRecordNotFound => write!(f, "DATASET_RECORD_NOT_FOUND"),
            Self::DatasetRecordFormatError => write!(f, "DATASET_RECORD_FORMAT_ERROR"),
            Self::AgentNotFound => write!(f, "AGENT_NOT_FOUND"),
            Self::AgentMistralApiError => write!(f, "AGENT_MISTRAL_API_ERROR"),
            Self::EvaluationNotFound => write!(f, "EVALUATION_NOT_FOUND"),
            Self::EvaluationCurrentlyRunning => write!(f, "EVALUATION_CURRENTLY_RUNNING"),
            Self::EvaluationRecordNotFound => write!(f, "EVALUATION_RECORD_NOT_FOUND"),
            Self::EvaluationRunNotFound => write!(f, "EVALUATION_RUN_NOT_FOUND"),
            Self::EvaluationRunTransitionIsInvalid => {
                write!(f, "EVALUATION_RUN_TRANSITION_IS_INVALID")
            }
            Self::EvaluationRunTransitionIsRunningAlready => {
                write!(f, "EVALUATION_RUN_TRANSITION_IS_RUNNING_ALREADY")
            }
            Self::EvaluationRunTransitionError => write!(f, "EVALUATION_RUN_TRANSITION_ERROR"),
            Self::TemplateError => write!(f, "TEMPLATE_ERROR"),
            Self::TemplateSyntaxError => write!(f, "TEMPLATE_SYNTAX_ERROR"),
            Self::ProjectNameAlreadyExists => write!(f, "PROJECT_NAME_ALREADY_EXISTS"),
            Self::EvaluationNameAlreadyExists => write!(f, "EVALUATION_NAME_ALREADY_EXISTS"),
            Self::OptimizationTrialKeyAlreadyExists => {
                write!(f, "OPTIMIZATION_TRIAL_KEY_ALREADY_EXISTS")
            }
            Self::TracesFilterQueryParseError => write!(f, "TRACES_FILTER_QUERY_PARSE_ERROR"),
            Self::TraceNotFound => write!(f, "TRACE_NOT_FOUND"),
            Self::SpanNotFound => write!(f, "SPAN_NOT_FOUND"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
