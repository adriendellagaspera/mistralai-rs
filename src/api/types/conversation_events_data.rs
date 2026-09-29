pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConversationEventsData {
    #[serde(rename = "agent.handoff.done")]
    #[non_exhaustive]
    AgentHandoffDone {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
        #[serde(default)]
        id: String,
        #[serde(default)]
        next_agent_id: String,
        #[serde(default)]
        next_agent_name: String,
    },

    #[serde(rename = "agent.handoff.started")]
    #[non_exhaustive]
    AgentHandoffStarted {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
        #[serde(default)]
        id: String,
        #[serde(default)]
        previous_agent_id: String,
        #[serde(default)]
        previous_agent_name: String,
    },

    #[serde(rename = "conversation.response.done")]
    #[non_exhaustive]
    ConversationResponseDone {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        usage: ConversationUsageInfo,
    },

    #[serde(rename = "conversation.response.error")]
    #[non_exhaustive]
    ConversationResponseError {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        message: String,
        #[serde(default)]
        code: i64,
    },

    #[serde(rename = "conversation.response.started")]
    #[non_exhaustive]
    ConversationResponseStarted {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        conversation_id: String,
    },

    #[serde(rename = "function.call.delta")]
    #[non_exhaustive]
    FunctionCallDelta {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        agent_id: Option<String>,
        #[serde(default)]
        name: String,
        #[serde(default)]
        tool_call_id: String,
        #[serde(default)]
        arguments: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
    },

    #[serde(rename = "message.output.delta")]
    #[non_exhaustive]
    MessageOutputDelta {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        content_index: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        agent_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        role: Option<MessageOutputEventRole>,
        content: MessageOutputEventContent,
    },

    #[serde(rename = "tool.execution.delta")]
    #[non_exhaustive]
    ToolExecutionDelta {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
        #[serde(default)]
        id: String,
        name: ToolExecutionDeltaEventName,
        #[serde(default)]
        arguments: String,
    },

    #[serde(rename = "tool.execution.done")]
    #[non_exhaustive]
    ToolExecutionDone {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
        #[serde(default)]
        id: String,
        name: ToolExecutionDoneEventName,
        #[serde(skip_serializing_if = "Option::is_none")]
        info: Option<ToolExecutionInfo>,
    },

    #[serde(rename = "tool.execution.started")]
    #[non_exhaustive]
    ToolExecutionStarted {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        agent_id: Option<String>,
        name: ToolExecutionStartedEventName,
        #[serde(default)]
        arguments: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ConversationEventsData {
    pub fn agent_handoff_done(id: String, next_agent_id: String, next_agent_name: String) -> Self {
        Self::AgentHandoffDone {
            created_at: None,
            output_index: None,
            id,
            next_agent_id,
            next_agent_name,
        }
    }

    pub fn agent_handoff_started(
        id: String,
        previous_agent_id: String,
        previous_agent_name: String,
    ) -> Self {
        Self::AgentHandoffStarted {
            created_at: None,
            output_index: None,
            id,
            previous_agent_id,
            previous_agent_name,
        }
    }

    pub fn conversation_response_done(usage: ConversationUsageInfo) -> Self {
        Self::ConversationResponseDone {
            created_at: None,
            usage,
        }
    }

    pub fn conversation_response_error(message: String, code: i64) -> Self {
        Self::ConversationResponseError {
            created_at: None,
            message,
            code,
        }
    }

    pub fn conversation_response_started(conversation_id: String) -> Self {
        Self::ConversationResponseStarted {
            created_at: None,
            conversation_id,
        }
    }

    pub fn function_call_delta(
        id: String,
        name: String,
        tool_call_id: String,
        arguments: String,
    ) -> Self {
        Self::FunctionCallDelta {
            created_at: None,
            output_index: None,
            id,
            model: None,
            agent_id: None,
            name,
            tool_call_id,
            arguments,
            confirmation_status: None,
        }
    }

    pub fn message_output_delta(id: String, content: MessageOutputEventContent) -> Self {
        Self::MessageOutputDelta {
            created_at: None,
            output_index: None,
            id,
            content_index: None,
            model: None,
            agent_id: None,
            role: None,
            content,
        }
    }

    pub fn tool_execution_delta(
        id: String,
        name: ToolExecutionDeltaEventName,
        arguments: String,
    ) -> Self {
        Self::ToolExecutionDelta {
            created_at: None,
            output_index: None,
            id,
            name,
            arguments,
        }
    }

    pub fn tool_execution_done(id: String, name: ToolExecutionDoneEventName) -> Self {
        Self::ToolExecutionDone {
            created_at: None,
            output_index: None,
            id,
            name,
            info: None,
        }
    }

    pub fn tool_execution_started(
        id: String,
        name: ToolExecutionStartedEventName,
        arguments: String,
    ) -> Self {
        Self::ToolExecutionStarted {
            created_at: None,
            output_index: None,
            id,
            model: None,
            agent_id: None,
            name,
            arguments,
        }
    }

    pub fn agent_handoff_done_with_created_at(
        created_at: DateTime<FixedOffset>,
        output_index: Option<i64>,
        id: String,
        next_agent_id: String,
        next_agent_name: String,
    ) -> Self {
        Self::AgentHandoffDone {
            created_at: Some(created_at),
            output_index,
            id,
            next_agent_id,
            next_agent_name,
        }
    }

    pub fn agent_handoff_done_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: i64,
        id: String,
        next_agent_id: String,
        next_agent_name: String,
    ) -> Self {
        Self::AgentHandoffDone {
            created_at,
            output_index: Some(output_index),
            id,
            next_agent_id,
            next_agent_name,
        }
    }

    pub fn agent_handoff_started_with_created_at(
        created_at: DateTime<FixedOffset>,
        output_index: Option<i64>,
        id: String,
        previous_agent_id: String,
        previous_agent_name: String,
    ) -> Self {
        Self::AgentHandoffStarted {
            created_at: Some(created_at),
            output_index,
            id,
            previous_agent_id,
            previous_agent_name,
        }
    }

    pub fn agent_handoff_started_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: i64,
        id: String,
        previous_agent_id: String,
        previous_agent_name: String,
    ) -> Self {
        Self::AgentHandoffStarted {
            created_at,
            output_index: Some(output_index),
            id,
            previous_agent_id,
            previous_agent_name,
        }
    }

    pub fn conversation_response_done_with_created_at(
        created_at: DateTime<FixedOffset>,
        usage: ConversationUsageInfo,
    ) -> Self {
        Self::ConversationResponseDone {
            created_at: Some(created_at),
            usage,
        }
    }

    pub fn conversation_response_error_with_created_at(
        created_at: DateTime<FixedOffset>,
        message: String,
        code: i64,
    ) -> Self {
        Self::ConversationResponseError {
            created_at: Some(created_at),
            message,
            code,
        }
    }

    pub fn conversation_response_started_with_created_at(
        created_at: DateTime<FixedOffset>,
        conversation_id: String,
    ) -> Self {
        Self::ConversationResponseStarted {
            created_at: Some(created_at),
            conversation_id,
        }
    }

    pub fn function_call_delta_with_created_at(
        created_at: DateTime<FixedOffset>,
        output_index: Option<i64>,
        id: String,
        model: Option<String>,
        agent_id: Option<String>,
        name: String,
        tool_call_id: String,
        arguments: String,
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
    ) -> Self {
        Self::FunctionCallDelta {
            created_at: Some(created_at),
            output_index,
            id,
            model,
            agent_id,
            name,
            tool_call_id,
            arguments,
            confirmation_status,
        }
    }

    pub fn function_call_delta_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: i64,
        id: String,
        model: Option<String>,
        agent_id: Option<String>,
        name: String,
        tool_call_id: String,
        arguments: String,
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
    ) -> Self {
        Self::FunctionCallDelta {
            created_at,
            output_index: Some(output_index),
            id,
            model,
            agent_id,
            name,
            tool_call_id,
            arguments,
            confirmation_status,
        }
    }

    pub fn function_call_delta_with_model(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        model: String,
        agent_id: Option<String>,
        name: String,
        tool_call_id: String,
        arguments: String,
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
    ) -> Self {
        Self::FunctionCallDelta {
            created_at,
            output_index,
            id,
            model: Some(model),
            agent_id,
            name,
            tool_call_id,
            arguments,
            confirmation_status,
        }
    }

    pub fn function_call_delta_with_agent_id(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        model: Option<String>,
        agent_id: String,
        name: String,
        tool_call_id: String,
        arguments: String,
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
    ) -> Self {
        Self::FunctionCallDelta {
            created_at,
            output_index,
            id,
            model,
            agent_id: Some(agent_id),
            name,
            tool_call_id,
            arguments,
            confirmation_status,
        }
    }

    pub fn function_call_delta_with_confirmation_status(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        model: Option<String>,
        agent_id: Option<String>,
        name: String,
        tool_call_id: String,
        arguments: String,
        confirmation_status: FunctionCallEventConfirmationStatus,
    ) -> Self {
        Self::FunctionCallDelta {
            created_at,
            output_index,
            id,
            model,
            agent_id,
            name,
            tool_call_id,
            arguments,
            confirmation_status: Some(confirmation_status),
        }
    }

    pub fn message_output_delta_with_created_at(
        created_at: DateTime<FixedOffset>,
        output_index: Option<i64>,
        id: String,
        content_index: Option<i64>,
        model: Option<String>,
        agent_id: Option<String>,
        role: Option<MessageOutputEventRole>,
        content: MessageOutputEventContent,
    ) -> Self {
        Self::MessageOutputDelta {
            created_at: Some(created_at),
            output_index,
            id,
            content_index,
            model,
            agent_id,
            role,
            content,
        }
    }

    pub fn message_output_delta_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: i64,
        id: String,
        content_index: Option<i64>,
        model: Option<String>,
        agent_id: Option<String>,
        role: Option<MessageOutputEventRole>,
        content: MessageOutputEventContent,
    ) -> Self {
        Self::MessageOutputDelta {
            created_at,
            output_index: Some(output_index),
            id,
            content_index,
            model,
            agent_id,
            role,
            content,
        }
    }

    pub fn message_output_delta_with_content_index(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        content_index: i64,
        model: Option<String>,
        agent_id: Option<String>,
        role: Option<MessageOutputEventRole>,
        content: MessageOutputEventContent,
    ) -> Self {
        Self::MessageOutputDelta {
            created_at,
            output_index,
            id,
            content_index: Some(content_index),
            model,
            agent_id,
            role,
            content,
        }
    }

    pub fn message_output_delta_with_model(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        content_index: Option<i64>,
        model: String,
        agent_id: Option<String>,
        role: Option<MessageOutputEventRole>,
        content: MessageOutputEventContent,
    ) -> Self {
        Self::MessageOutputDelta {
            created_at,
            output_index,
            id,
            content_index,
            model: Some(model),
            agent_id,
            role,
            content,
        }
    }

    pub fn message_output_delta_with_agent_id(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        content_index: Option<i64>,
        model: Option<String>,
        agent_id: String,
        role: Option<MessageOutputEventRole>,
        content: MessageOutputEventContent,
    ) -> Self {
        Self::MessageOutputDelta {
            created_at,
            output_index,
            id,
            content_index,
            model,
            agent_id: Some(agent_id),
            role,
            content,
        }
    }

    pub fn message_output_delta_with_role(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        content_index: Option<i64>,
        model: Option<String>,
        agent_id: Option<String>,
        role: MessageOutputEventRole,
        content: MessageOutputEventContent,
    ) -> Self {
        Self::MessageOutputDelta {
            created_at,
            output_index,
            id,
            content_index,
            model,
            agent_id,
            role: Some(role),
            content,
        }
    }

    pub fn tool_execution_delta_with_created_at(
        created_at: DateTime<FixedOffset>,
        output_index: Option<i64>,
        id: String,
        name: ToolExecutionDeltaEventName,
        arguments: String,
    ) -> Self {
        Self::ToolExecutionDelta {
            created_at: Some(created_at),
            output_index,
            id,
            name,
            arguments,
        }
    }

    pub fn tool_execution_delta_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: i64,
        id: String,
        name: ToolExecutionDeltaEventName,
        arguments: String,
    ) -> Self {
        Self::ToolExecutionDelta {
            created_at,
            output_index: Some(output_index),
            id,
            name,
            arguments,
        }
    }

    pub fn tool_execution_done_with_created_at(
        created_at: DateTime<FixedOffset>,
        output_index: Option<i64>,
        id: String,
        name: ToolExecutionDoneEventName,
        info: Option<ToolExecutionInfo>,
    ) -> Self {
        Self::ToolExecutionDone {
            created_at: Some(created_at),
            output_index,
            id,
            name,
            info,
        }
    }

    pub fn tool_execution_done_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: i64,
        id: String,
        name: ToolExecutionDoneEventName,
        info: Option<ToolExecutionInfo>,
    ) -> Self {
        Self::ToolExecutionDone {
            created_at,
            output_index: Some(output_index),
            id,
            name,
            info,
        }
    }

    pub fn tool_execution_done_with_info(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        name: ToolExecutionDoneEventName,
        info: ToolExecutionInfo,
    ) -> Self {
        Self::ToolExecutionDone {
            created_at,
            output_index,
            id,
            name,
            info: Some(info),
        }
    }

    pub fn tool_execution_started_with_created_at(
        created_at: DateTime<FixedOffset>,
        output_index: Option<i64>,
        id: String,
        model: Option<String>,
        agent_id: Option<String>,
        name: ToolExecutionStartedEventName,
        arguments: String,
    ) -> Self {
        Self::ToolExecutionStarted {
            created_at: Some(created_at),
            output_index,
            id,
            model,
            agent_id,
            name,
            arguments,
        }
    }

    pub fn tool_execution_started_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: i64,
        id: String,
        model: Option<String>,
        agent_id: Option<String>,
        name: ToolExecutionStartedEventName,
        arguments: String,
    ) -> Self {
        Self::ToolExecutionStarted {
            created_at,
            output_index: Some(output_index),
            id,
            model,
            agent_id,
            name,
            arguments,
        }
    }

    pub fn tool_execution_started_with_model(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        model: String,
        agent_id: Option<String>,
        name: ToolExecutionStartedEventName,
        arguments: String,
    ) -> Self {
        Self::ToolExecutionStarted {
            created_at,
            output_index,
            id,
            model: Some(model),
            agent_id,
            name,
            arguments,
        }
    }

    pub fn tool_execution_started_with_agent_id(
        created_at: Option<DateTime<FixedOffset>>,
        output_index: Option<i64>,
        id: String,
        model: Option<String>,
        agent_id: String,
        name: ToolExecutionStartedEventName,
        arguments: String,
    ) -> Self {
        Self::ToolExecutionStarted {
            created_at,
            output_index,
            id,
            model,
            agent_id: Some(agent_id),
            name,
            arguments,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
