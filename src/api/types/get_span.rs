pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetSpan {
    #[serde(default)]
    pub customer_id: String,
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub span_id: String,
    #[serde(default)]
    pub parent_span_id: String,
    #[serde(default)]
    pub trace_state: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_time: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end_time: DateTime<FixedOffset>,
    #[serde(default)]
    pub duration_ns: i64,
    #[serde(default)]
    pub span_name: String,
    #[serde(default)]
    pub span_kind: String,
    #[serde(default)]
    pub service_name: String,
    pub status_code: GetSpanStatusCode,
    #[serde(default)]
    pub status_message: String,
    #[serde(default)]
    pub error_type: String,
    #[serde(default)]
    pub operation_name: String,
    #[serde(default)]
    pub provider_name: String,
    #[serde(default)]
    pub request_model: String,
    #[serde(default)]
    pub response_model: String,
    #[serde(default)]
    pub response_id: String,
    #[serde(default)]
    pub output_type: String,
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub data_source_id: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub agent_name: String,
    #[serde(default)]
    pub agent_version: String,
    #[serde(default)]
    pub agent_description: String,
    #[serde(default)]
    pub workflow_name: String,
    #[serde(default)]
    pub prompt_name: String,
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub tool_type: String,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub input_messages: String,
    #[serde(default)]
    pub output_messages: String,
    #[serde(default)]
    pub system_instructions: String,
    #[serde(default)]
    pub tool_definitions: String,
    #[serde(default)]
    pub tool_call_arguments: String,
    #[serde(default)]
    pub tool_call_result: String,
    #[serde(default)]
    pub request_choice_count: i64,
    #[serde(default)]
    pub request_max_tokens: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub request_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub request_top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub request_top_k: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub request_presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub request_frequency_penalty: Option<f64>,
    #[serde(default)]
    pub request_seed: i64,
    #[serde(default)]
    pub request_stop_sequences: Vec<String>,
    #[serde(default)]
    pub request_encoding_formats: Vec<String>,
    #[serde(default)]
    pub response_finish_reasons: Vec<String>,
    #[serde(default)]
    pub usage_input_tokens: i64,
    #[serde(default)]
    pub usage_output_tokens: i64,
    #[serde(default)]
    pub usage_cache_read_input_tokens: i64,
    #[serde(default)]
    pub usage_cache_creation_input_tokens: i64,
    #[serde(default)]
    pub resource_attributes: HashMap<String, String>,
    #[serde(default)]
    pub span_attributes: HashMap<String, String>,
    #[serde(default)]
    pub scope_name: String,
    #[serde(default)]
    pub scope_version: String,
}

impl GetSpan {
    pub fn builder() -> GetSpanBuilder {
        <GetSpanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanBuilder {
    customer_id: Option<String>,
    organization_id: Option<String>,
    workspace_id: Option<String>,
    user_id: Option<String>,
    trace_id: Option<String>,
    span_id: Option<String>,
    parent_span_id: Option<String>,
    trace_state: Option<String>,
    start_time: Option<DateTime<FixedOffset>>,
    end_time: Option<DateTime<FixedOffset>>,
    duration_ns: Option<i64>,
    span_name: Option<String>,
    span_kind: Option<String>,
    service_name: Option<String>,
    status_code: Option<GetSpanStatusCode>,
    status_message: Option<String>,
    error_type: Option<String>,
    operation_name: Option<String>,
    provider_name: Option<String>,
    request_model: Option<String>,
    response_model: Option<String>,
    response_id: Option<String>,
    output_type: Option<String>,
    conversation_id: Option<String>,
    data_source_id: Option<String>,
    agent_id: Option<String>,
    agent_name: Option<String>,
    agent_version: Option<String>,
    agent_description: Option<String>,
    workflow_name: Option<String>,
    prompt_name: Option<String>,
    tool_name: Option<String>,
    tool_type: Option<String>,
    tool_call_id: Option<String>,
    input_messages: Option<String>,
    output_messages: Option<String>,
    system_instructions: Option<String>,
    tool_definitions: Option<String>,
    tool_call_arguments: Option<String>,
    tool_call_result: Option<String>,
    request_choice_count: Option<i64>,
    request_max_tokens: Option<i64>,
    request_temperature: Option<f64>,
    request_top_p: Option<f64>,
    request_top_k: Option<f64>,
    request_presence_penalty: Option<f64>,
    request_frequency_penalty: Option<f64>,
    request_seed: Option<i64>,
    request_stop_sequences: Option<Vec<String>>,
    request_encoding_formats: Option<Vec<String>>,
    response_finish_reasons: Option<Vec<String>>,
    usage_input_tokens: Option<i64>,
    usage_output_tokens: Option<i64>,
    usage_cache_read_input_tokens: Option<i64>,
    usage_cache_creation_input_tokens: Option<i64>,
    resource_attributes: Option<HashMap<String, String>>,
    span_attributes: Option<HashMap<String, String>>,
    scope_name: Option<String>,
    scope_version: Option<String>,
}

impl GetSpanBuilder {
    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn parent_span_id(mut self, value: impl Into<String>) -> Self {
        self.parent_span_id = Some(value.into());
        self
    }

    pub fn trace_state(mut self, value: impl Into<String>) -> Self {
        self.trace_state = Some(value.into());
        self
    }

    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn duration_ns(mut self, value: i64) -> Self {
        self.duration_ns = Some(value);
        self
    }

    pub fn span_name(mut self, value: impl Into<String>) -> Self {
        self.span_name = Some(value.into());
        self
    }

    pub fn span_kind(mut self, value: impl Into<String>) -> Self {
        self.span_kind = Some(value.into());
        self
    }

    pub fn service_name(mut self, value: impl Into<String>) -> Self {
        self.service_name = Some(value.into());
        self
    }

    pub fn status_code(mut self, value: GetSpanStatusCode) -> Self {
        self.status_code = Some(value);
        self
    }

    pub fn status_message(mut self, value: impl Into<String>) -> Self {
        self.status_message = Some(value.into());
        self
    }

    pub fn error_type(mut self, value: impl Into<String>) -> Self {
        self.error_type = Some(value.into());
        self
    }

    pub fn operation_name(mut self, value: impl Into<String>) -> Self {
        self.operation_name = Some(value.into());
        self
    }

    pub fn provider_name(mut self, value: impl Into<String>) -> Self {
        self.provider_name = Some(value.into());
        self
    }

    pub fn request_model(mut self, value: impl Into<String>) -> Self {
        self.request_model = Some(value.into());
        self
    }

    pub fn response_model(mut self, value: impl Into<String>) -> Self {
        self.response_model = Some(value.into());
        self
    }

    pub fn response_id(mut self, value: impl Into<String>) -> Self {
        self.response_id = Some(value.into());
        self
    }

    pub fn output_type(mut self, value: impl Into<String>) -> Self {
        self.output_type = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn data_source_id(mut self, value: impl Into<String>) -> Self {
        self.data_source_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn agent_version(mut self, value: impl Into<String>) -> Self {
        self.agent_version = Some(value.into());
        self
    }

    pub fn agent_description(mut self, value: impl Into<String>) -> Self {
        self.agent_description = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn prompt_name(mut self, value: impl Into<String>) -> Self {
        self.prompt_name = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn tool_type(mut self, value: impl Into<String>) -> Self {
        self.tool_type = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn input_messages(mut self, value: impl Into<String>) -> Self {
        self.input_messages = Some(value.into());
        self
    }

    pub fn output_messages(mut self, value: impl Into<String>) -> Self {
        self.output_messages = Some(value.into());
        self
    }

    pub fn system_instructions(mut self, value: impl Into<String>) -> Self {
        self.system_instructions = Some(value.into());
        self
    }

    pub fn tool_definitions(mut self, value: impl Into<String>) -> Self {
        self.tool_definitions = Some(value.into());
        self
    }

    pub fn tool_call_arguments(mut self, value: impl Into<String>) -> Self {
        self.tool_call_arguments = Some(value.into());
        self
    }

    pub fn tool_call_result(mut self, value: impl Into<String>) -> Self {
        self.tool_call_result = Some(value.into());
        self
    }

    pub fn request_choice_count(mut self, value: i64) -> Self {
        self.request_choice_count = Some(value);
        self
    }

    pub fn request_max_tokens(mut self, value: i64) -> Self {
        self.request_max_tokens = Some(value);
        self
    }

    pub fn request_temperature(mut self, value: f64) -> Self {
        self.request_temperature = Some(value);
        self
    }

    pub fn request_top_p(mut self, value: f64) -> Self {
        self.request_top_p = Some(value);
        self
    }

    pub fn request_top_k(mut self, value: f64) -> Self {
        self.request_top_k = Some(value);
        self
    }

    pub fn request_presence_penalty(mut self, value: f64) -> Self {
        self.request_presence_penalty = Some(value);
        self
    }

    pub fn request_frequency_penalty(mut self, value: f64) -> Self {
        self.request_frequency_penalty = Some(value);
        self
    }

    pub fn request_seed(mut self, value: i64) -> Self {
        self.request_seed = Some(value);
        self
    }

    pub fn request_stop_sequences(mut self, value: Vec<String>) -> Self {
        self.request_stop_sequences = Some(value);
        self
    }

    pub fn request_encoding_formats(mut self, value: Vec<String>) -> Self {
        self.request_encoding_formats = Some(value);
        self
    }

    pub fn response_finish_reasons(mut self, value: Vec<String>) -> Self {
        self.response_finish_reasons = Some(value);
        self
    }

    pub fn usage_input_tokens(mut self, value: i64) -> Self {
        self.usage_input_tokens = Some(value);
        self
    }

    pub fn usage_output_tokens(mut self, value: i64) -> Self {
        self.usage_output_tokens = Some(value);
        self
    }

    pub fn usage_cache_read_input_tokens(mut self, value: i64) -> Self {
        self.usage_cache_read_input_tokens = Some(value);
        self
    }

    pub fn usage_cache_creation_input_tokens(mut self, value: i64) -> Self {
        self.usage_cache_creation_input_tokens = Some(value);
        self
    }

    pub fn resource_attributes(mut self, value: HashMap<String, String>) -> Self {
        self.resource_attributes = Some(value);
        self
    }

    pub fn span_attributes(mut self, value: HashMap<String, String>) -> Self {
        self.span_attributes = Some(value);
        self
    }

    pub fn scope_name(mut self, value: impl Into<String>) -> Self {
        self.scope_name = Some(value.into());
        self
    }

    pub fn scope_version(mut self, value: impl Into<String>) -> Self {
        self.scope_version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSpan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_id`](GetSpanBuilder::customer_id)
    /// - [`organization_id`](GetSpanBuilder::organization_id)
    /// - [`workspace_id`](GetSpanBuilder::workspace_id)
    /// - [`user_id`](GetSpanBuilder::user_id)
    /// - [`trace_id`](GetSpanBuilder::trace_id)
    /// - [`span_id`](GetSpanBuilder::span_id)
    /// - [`parent_span_id`](GetSpanBuilder::parent_span_id)
    /// - [`trace_state`](GetSpanBuilder::trace_state)
    /// - [`start_time`](GetSpanBuilder::start_time)
    /// - [`end_time`](GetSpanBuilder::end_time)
    /// - [`duration_ns`](GetSpanBuilder::duration_ns)
    /// - [`span_name`](GetSpanBuilder::span_name)
    /// - [`span_kind`](GetSpanBuilder::span_kind)
    /// - [`service_name`](GetSpanBuilder::service_name)
    /// - [`status_code`](GetSpanBuilder::status_code)
    /// - [`status_message`](GetSpanBuilder::status_message)
    /// - [`error_type`](GetSpanBuilder::error_type)
    /// - [`operation_name`](GetSpanBuilder::operation_name)
    /// - [`provider_name`](GetSpanBuilder::provider_name)
    /// - [`request_model`](GetSpanBuilder::request_model)
    /// - [`response_model`](GetSpanBuilder::response_model)
    /// - [`response_id`](GetSpanBuilder::response_id)
    /// - [`output_type`](GetSpanBuilder::output_type)
    /// - [`conversation_id`](GetSpanBuilder::conversation_id)
    /// - [`data_source_id`](GetSpanBuilder::data_source_id)
    /// - [`agent_id`](GetSpanBuilder::agent_id)
    /// - [`agent_name`](GetSpanBuilder::agent_name)
    /// - [`agent_version`](GetSpanBuilder::agent_version)
    /// - [`agent_description`](GetSpanBuilder::agent_description)
    /// - [`workflow_name`](GetSpanBuilder::workflow_name)
    /// - [`prompt_name`](GetSpanBuilder::prompt_name)
    /// - [`tool_name`](GetSpanBuilder::tool_name)
    /// - [`tool_type`](GetSpanBuilder::tool_type)
    /// - [`tool_call_id`](GetSpanBuilder::tool_call_id)
    /// - [`input_messages`](GetSpanBuilder::input_messages)
    /// - [`output_messages`](GetSpanBuilder::output_messages)
    /// - [`system_instructions`](GetSpanBuilder::system_instructions)
    /// - [`tool_definitions`](GetSpanBuilder::tool_definitions)
    /// - [`tool_call_arguments`](GetSpanBuilder::tool_call_arguments)
    /// - [`tool_call_result`](GetSpanBuilder::tool_call_result)
    /// - [`request_choice_count`](GetSpanBuilder::request_choice_count)
    /// - [`request_max_tokens`](GetSpanBuilder::request_max_tokens)
    /// - [`request_seed`](GetSpanBuilder::request_seed)
    /// - [`request_stop_sequences`](GetSpanBuilder::request_stop_sequences)
    /// - [`request_encoding_formats`](GetSpanBuilder::request_encoding_formats)
    /// - [`response_finish_reasons`](GetSpanBuilder::response_finish_reasons)
    /// - [`usage_input_tokens`](GetSpanBuilder::usage_input_tokens)
    /// - [`usage_output_tokens`](GetSpanBuilder::usage_output_tokens)
    /// - [`usage_cache_read_input_tokens`](GetSpanBuilder::usage_cache_read_input_tokens)
    /// - [`usage_cache_creation_input_tokens`](GetSpanBuilder::usage_cache_creation_input_tokens)
    /// - [`resource_attributes`](GetSpanBuilder::resource_attributes)
    /// - [`span_attributes`](GetSpanBuilder::span_attributes)
    /// - [`scope_name`](GetSpanBuilder::scope_name)
    /// - [`scope_version`](GetSpanBuilder::scope_version)
    pub fn build(self) -> Result<GetSpan, BuildError> {
        Ok(GetSpan {
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            parent_span_id: self
                .parent_span_id
                .ok_or_else(|| BuildError::missing_field("parent_span_id"))?,
            trace_state: self
                .trace_state
                .ok_or_else(|| BuildError::missing_field("trace_state"))?,
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self
                .end_time
                .ok_or_else(|| BuildError::missing_field("end_time"))?,
            duration_ns: self
                .duration_ns
                .ok_or_else(|| BuildError::missing_field("duration_ns"))?,
            span_name: self
                .span_name
                .ok_or_else(|| BuildError::missing_field("span_name"))?,
            span_kind: self
                .span_kind
                .ok_or_else(|| BuildError::missing_field("span_kind"))?,
            service_name: self
                .service_name
                .ok_or_else(|| BuildError::missing_field("service_name"))?,
            status_code: self
                .status_code
                .ok_or_else(|| BuildError::missing_field("status_code"))?,
            status_message: self
                .status_message
                .ok_or_else(|| BuildError::missing_field("status_message"))?,
            error_type: self
                .error_type
                .ok_or_else(|| BuildError::missing_field("error_type"))?,
            operation_name: self
                .operation_name
                .ok_or_else(|| BuildError::missing_field("operation_name"))?,
            provider_name: self
                .provider_name
                .ok_or_else(|| BuildError::missing_field("provider_name"))?,
            request_model: self
                .request_model
                .ok_or_else(|| BuildError::missing_field("request_model"))?,
            response_model: self
                .response_model
                .ok_or_else(|| BuildError::missing_field("response_model"))?,
            response_id: self
                .response_id
                .ok_or_else(|| BuildError::missing_field("response_id"))?,
            output_type: self
                .output_type
                .ok_or_else(|| BuildError::missing_field("output_type"))?,
            conversation_id: self
                .conversation_id
                .ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            data_source_id: self
                .data_source_id
                .ok_or_else(|| BuildError::missing_field("data_source_id"))?,
            agent_id: self
                .agent_id
                .ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_name: self
                .agent_name
                .ok_or_else(|| BuildError::missing_field("agent_name"))?,
            agent_version: self
                .agent_version
                .ok_or_else(|| BuildError::missing_field("agent_version"))?,
            agent_description: self
                .agent_description
                .ok_or_else(|| BuildError::missing_field("agent_description"))?,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
            prompt_name: self
                .prompt_name
                .ok_or_else(|| BuildError::missing_field("prompt_name"))?,
            tool_name: self
                .tool_name
                .ok_or_else(|| BuildError::missing_field("tool_name"))?,
            tool_type: self
                .tool_type
                .ok_or_else(|| BuildError::missing_field("tool_type"))?,
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            input_messages: self
                .input_messages
                .ok_or_else(|| BuildError::missing_field("input_messages"))?,
            output_messages: self
                .output_messages
                .ok_or_else(|| BuildError::missing_field("output_messages"))?,
            system_instructions: self
                .system_instructions
                .ok_or_else(|| BuildError::missing_field("system_instructions"))?,
            tool_definitions: self
                .tool_definitions
                .ok_or_else(|| BuildError::missing_field("tool_definitions"))?,
            tool_call_arguments: self
                .tool_call_arguments
                .ok_or_else(|| BuildError::missing_field("tool_call_arguments"))?,
            tool_call_result: self
                .tool_call_result
                .ok_or_else(|| BuildError::missing_field("tool_call_result"))?,
            request_choice_count: self
                .request_choice_count
                .ok_or_else(|| BuildError::missing_field("request_choice_count"))?,
            request_max_tokens: self
                .request_max_tokens
                .ok_or_else(|| BuildError::missing_field("request_max_tokens"))?,
            request_temperature: self.request_temperature,
            request_top_p: self.request_top_p,
            request_top_k: self.request_top_k,
            request_presence_penalty: self.request_presence_penalty,
            request_frequency_penalty: self.request_frequency_penalty,
            request_seed: self
                .request_seed
                .ok_or_else(|| BuildError::missing_field("request_seed"))?,
            request_stop_sequences: self
                .request_stop_sequences
                .ok_or_else(|| BuildError::missing_field("request_stop_sequences"))?,
            request_encoding_formats: self
                .request_encoding_formats
                .ok_or_else(|| BuildError::missing_field("request_encoding_formats"))?,
            response_finish_reasons: self
                .response_finish_reasons
                .ok_or_else(|| BuildError::missing_field("response_finish_reasons"))?,
            usage_input_tokens: self
                .usage_input_tokens
                .ok_or_else(|| BuildError::missing_field("usage_input_tokens"))?,
            usage_output_tokens: self
                .usage_output_tokens
                .ok_or_else(|| BuildError::missing_field("usage_output_tokens"))?,
            usage_cache_read_input_tokens: self
                .usage_cache_read_input_tokens
                .ok_or_else(|| BuildError::missing_field("usage_cache_read_input_tokens"))?,
            usage_cache_creation_input_tokens: self
                .usage_cache_creation_input_tokens
                .ok_or_else(|| BuildError::missing_field("usage_cache_creation_input_tokens"))?,
            resource_attributes: self
                .resource_attributes
                .ok_or_else(|| BuildError::missing_field("resource_attributes"))?,
            span_attributes: self
                .span_attributes
                .ok_or_else(|| BuildError::missing_field("span_attributes"))?,
            scope_name: self
                .scope_name
                .ok_or_else(|| BuildError::missing_field("scope_name"))?,
            scope_version: self
                .scope_version
                .ok_or_else(|| BuildError::missing_field("scope_version"))?,
        })
    }
}
