pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetTrace {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub agent_name: String,
    #[serde(default)]
    pub cache_creation_input_tokens: i64,
    #[serde(default)]
    pub cache_read_input_tokens: i64,
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub customer_id: String,
    #[serde(default)]
    pub duration_ns: i64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end_time: DateTime<FixedOffset>,
    #[serde(default)]
    pub environment: String,
    #[serde(default)]
    pub error_count: i64,
    #[serde(default)]
    pub evaluation_count: i64,
    #[serde(default)]
    pub first_turn_last_input_message: String,
    #[serde(default)]
    pub first_turn_last_output_message: String,
    #[serde(default)]
    pub gen_ai_span_count: i64,
    #[serde(default)]
    pub input_tokens: i64,
    #[serde(default)]
    pub last_turn_last_input_message: String,
    #[serde(default)]
    pub last_turn_last_output_message: String,
    #[serde(default)]
    pub llm_call_count: i64,
    #[serde(default)]
    pub models_used: Vec<String>,
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub output_tokens: i64,
    #[serde(default)]
    pub retrieval_count: i64,
    #[serde(default)]
    pub root_span_id: String,
    #[serde(default)]
    pub root_span_name: String,
    #[serde(default)]
    pub service_name: String,
    #[serde(default)]
    pub span_count: i64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_time: DateTime<FixedOffset>,
    pub status_code: GetTraceStatusCode,
    #[serde(default)]
    pub tool_call_count: i64,
    #[serde(default)]
    pub tools_used: Vec<String>,
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub workflow_name: String,
    #[serde(default)]
    pub workspace_id: String,
}

impl GetTrace {
    pub fn builder() -> GetTraceBuilder {
        <GetTraceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTraceBuilder {
    agent_id: Option<String>,
    agent_name: Option<String>,
    cache_creation_input_tokens: Option<i64>,
    cache_read_input_tokens: Option<i64>,
    conversation_id: Option<String>,
    customer_id: Option<String>,
    duration_ns: Option<i64>,
    end_time: Option<DateTime<FixedOffset>>,
    environment: Option<String>,
    error_count: Option<i64>,
    evaluation_count: Option<i64>,
    first_turn_last_input_message: Option<String>,
    first_turn_last_output_message: Option<String>,
    gen_ai_span_count: Option<i64>,
    input_tokens: Option<i64>,
    last_turn_last_input_message: Option<String>,
    last_turn_last_output_message: Option<String>,
    llm_call_count: Option<i64>,
    models_used: Option<Vec<String>>,
    organization_id: Option<String>,
    output_tokens: Option<i64>,
    retrieval_count: Option<i64>,
    root_span_id: Option<String>,
    root_span_name: Option<String>,
    service_name: Option<String>,
    span_count: Option<i64>,
    start_time: Option<DateTime<FixedOffset>>,
    status_code: Option<GetTraceStatusCode>,
    tool_call_count: Option<i64>,
    tools_used: Option<Vec<String>>,
    trace_id: Option<String>,
    user_id: Option<String>,
    workflow_name: Option<String>,
    workspace_id: Option<String>,
}

impl GetTraceBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn cache_creation_input_tokens(mut self, value: i64) -> Self {
        self.cache_creation_input_tokens = Some(value);
        self
    }

    pub fn cache_read_input_tokens(mut self, value: i64) -> Self {
        self.cache_read_input_tokens = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn duration_ns(mut self, value: i64) -> Self {
        self.duration_ns = Some(value);
        self
    }

    pub fn end_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn error_count(mut self, value: i64) -> Self {
        self.error_count = Some(value);
        self
    }

    pub fn evaluation_count(mut self, value: i64) -> Self {
        self.evaluation_count = Some(value);
        self
    }

    pub fn first_turn_last_input_message(mut self, value: impl Into<String>) -> Self {
        self.first_turn_last_input_message = Some(value.into());
        self
    }

    pub fn first_turn_last_output_message(mut self, value: impl Into<String>) -> Self {
        self.first_turn_last_output_message = Some(value.into());
        self
    }

    pub fn gen_ai_span_count(mut self, value: i64) -> Self {
        self.gen_ai_span_count = Some(value);
        self
    }

    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn last_turn_last_input_message(mut self, value: impl Into<String>) -> Self {
        self.last_turn_last_input_message = Some(value.into());
        self
    }

    pub fn last_turn_last_output_message(mut self, value: impl Into<String>) -> Self {
        self.last_turn_last_output_message = Some(value.into());
        self
    }

    pub fn llm_call_count(mut self, value: i64) -> Self {
        self.llm_call_count = Some(value);
        self
    }

    pub fn models_used(mut self, value: Vec<String>) -> Self {
        self.models_used = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    pub fn retrieval_count(mut self, value: i64) -> Self {
        self.retrieval_count = Some(value);
        self
    }

    pub fn root_span_id(mut self, value: impl Into<String>) -> Self {
        self.root_span_id = Some(value.into());
        self
    }

    pub fn root_span_name(mut self, value: impl Into<String>) -> Self {
        self.root_span_name = Some(value.into());
        self
    }

    pub fn service_name(mut self, value: impl Into<String>) -> Self {
        self.service_name = Some(value.into());
        self
    }

    pub fn span_count(mut self, value: i64) -> Self {
        self.span_count = Some(value);
        self
    }

    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn status_code(mut self, value: GetTraceStatusCode) -> Self {
        self.status_code = Some(value);
        self
    }

    pub fn tool_call_count(mut self, value: i64) -> Self {
        self.tool_call_count = Some(value);
        self
    }

    pub fn tools_used(mut self, value: Vec<String>) -> Self {
        self.tools_used = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTrace`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](GetTraceBuilder::agent_id)
    /// - [`agent_name`](GetTraceBuilder::agent_name)
    /// - [`cache_creation_input_tokens`](GetTraceBuilder::cache_creation_input_tokens)
    /// - [`cache_read_input_tokens`](GetTraceBuilder::cache_read_input_tokens)
    /// - [`conversation_id`](GetTraceBuilder::conversation_id)
    /// - [`customer_id`](GetTraceBuilder::customer_id)
    /// - [`duration_ns`](GetTraceBuilder::duration_ns)
    /// - [`end_time`](GetTraceBuilder::end_time)
    /// - [`environment`](GetTraceBuilder::environment)
    /// - [`error_count`](GetTraceBuilder::error_count)
    /// - [`evaluation_count`](GetTraceBuilder::evaluation_count)
    /// - [`first_turn_last_input_message`](GetTraceBuilder::first_turn_last_input_message)
    /// - [`first_turn_last_output_message`](GetTraceBuilder::first_turn_last_output_message)
    /// - [`gen_ai_span_count`](GetTraceBuilder::gen_ai_span_count)
    /// - [`input_tokens`](GetTraceBuilder::input_tokens)
    /// - [`last_turn_last_input_message`](GetTraceBuilder::last_turn_last_input_message)
    /// - [`last_turn_last_output_message`](GetTraceBuilder::last_turn_last_output_message)
    /// - [`llm_call_count`](GetTraceBuilder::llm_call_count)
    /// - [`models_used`](GetTraceBuilder::models_used)
    /// - [`organization_id`](GetTraceBuilder::organization_id)
    /// - [`output_tokens`](GetTraceBuilder::output_tokens)
    /// - [`retrieval_count`](GetTraceBuilder::retrieval_count)
    /// - [`root_span_id`](GetTraceBuilder::root_span_id)
    /// - [`root_span_name`](GetTraceBuilder::root_span_name)
    /// - [`service_name`](GetTraceBuilder::service_name)
    /// - [`span_count`](GetTraceBuilder::span_count)
    /// - [`start_time`](GetTraceBuilder::start_time)
    /// - [`status_code`](GetTraceBuilder::status_code)
    /// - [`tool_call_count`](GetTraceBuilder::tool_call_count)
    /// - [`tools_used`](GetTraceBuilder::tools_used)
    /// - [`trace_id`](GetTraceBuilder::trace_id)
    /// - [`user_id`](GetTraceBuilder::user_id)
    /// - [`workflow_name`](GetTraceBuilder::workflow_name)
    /// - [`workspace_id`](GetTraceBuilder::workspace_id)
    pub fn build(self) -> Result<GetTrace, BuildError> {
        Ok(GetTrace {
            agent_id: self
                .agent_id
                .ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_name: self
                .agent_name
                .ok_or_else(|| BuildError::missing_field("agent_name"))?,
            cache_creation_input_tokens: self
                .cache_creation_input_tokens
                .ok_or_else(|| BuildError::missing_field("cache_creation_input_tokens"))?,
            cache_read_input_tokens: self
                .cache_read_input_tokens
                .ok_or_else(|| BuildError::missing_field("cache_read_input_tokens"))?,
            conversation_id: self
                .conversation_id
                .ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            duration_ns: self
                .duration_ns
                .ok_or_else(|| BuildError::missing_field("duration_ns"))?,
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
            environment: self
                .environment
                .ok_or_else(|| BuildError::missing_field("environment"))?,
            error_count: self
                .error_count
                .ok_or_else(|| BuildError::missing_field("error_count"))?,
            evaluation_count: self
                .evaluation_count
                .ok_or_else(|| BuildError::missing_field("evaluation_count"))?,
            first_turn_last_input_message: self
                .first_turn_last_input_message
                .ok_or_else(|| BuildError::missing_field("first_turn_last_input_message"))?,
            first_turn_last_output_message: self
                .first_turn_last_output_message
                .ok_or_else(|| BuildError::missing_field("first_turn_last_output_message"))?,
            gen_ai_span_count: self
                .gen_ai_span_count
                .ok_or_else(|| BuildError::missing_field("gen_ai_span_count"))?,
            input_tokens: self
                .input_tokens
                .ok_or_else(|| BuildError::missing_field("input_tokens"))?,
            last_turn_last_input_message: self
                .last_turn_last_input_message
                .ok_or_else(|| BuildError::missing_field("last_turn_last_input_message"))?,
            last_turn_last_output_message: self
                .last_turn_last_output_message
                .ok_or_else(|| BuildError::missing_field("last_turn_last_output_message"))?,
            llm_call_count: self
                .llm_call_count
                .ok_or_else(|| BuildError::missing_field("llm_call_count"))?,
            models_used: self
                .models_used
                .ok_or_else(|| BuildError::missing_field("models_used"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            output_tokens: self
                .output_tokens
                .ok_or_else(|| BuildError::missing_field("output_tokens"))?,
            retrieval_count: self
                .retrieval_count
                .ok_or_else(|| BuildError::missing_field("retrieval_count"))?,
            root_span_id: self
                .root_span_id
                .ok_or_else(|| BuildError::missing_field("root_span_id"))?,
            root_span_name: self
                .root_span_name
                .ok_or_else(|| BuildError::missing_field("root_span_name"))?,
            service_name: self
                .service_name
                .ok_or_else(|| BuildError::missing_field("service_name"))?,
            span_count: self
                .span_count
                .ok_or_else(|| BuildError::missing_field("span_count"))?,
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            status_code: self
                .status_code
                .ok_or_else(|| BuildError::missing_field("status_code"))?,
            tool_call_count: self
                .tool_call_count
                .ok_or_else(|| BuildError::missing_field("tool_call_count"))?,
            tools_used: self
                .tools_used
                .ok_or_else(|| BuildError::missing_field("tools_used"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
