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
        #[serde(default)]
        id: String,
        #[serde(default)]
        next_agent_id: String,
        #[serde(default)]
        next_agent_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
    },

    #[serde(rename = "agent.handoff.started")]
    #[non_exhaustive]
    AgentHandoffStarted {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
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
        #[serde(default)]
        code: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        message: String,
    },

    #[serde(rename = "conversation.response.started")]
    #[non_exhaustive]
    ConversationResponseStarted {
        #[serde(default)]
        conversation_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
    },

    #[serde(rename = "function.call.delta")]
    #[non_exhaustive]
    FunctionCallDelta {
        #[serde(skip_serializing_if = "Option::is_none")]
        agent_id: Option<String>,
        #[serde(default)]
        arguments: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
        #[serde(default)]
        tool_call_id: String,
    },

    #[serde(rename = "message.output.delta")]
    #[non_exhaustive]
    MessageOutputDelta {
        #[serde(skip_serializing_if = "Option::is_none")]
        agent_id: Option<String>,
        content: MessageOutputEventContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        content_index: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        role: Option<MessageOutputEventRole>,
    },

    #[serde(rename = "tool.execution.delta")]
    #[non_exhaustive]
    ToolExecutionDelta {
        #[serde(default)]
        arguments: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        id: String,
        name: ToolExecutionDeltaEventName,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
    },

    #[serde(rename = "tool.execution.done")]
    #[non_exhaustive]
    ToolExecutionDone {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        info: Option<ToolExecutionInfo>,
        name: ToolExecutionDoneEventName,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
    },

    #[serde(rename = "tool.execution.started")]
    #[non_exhaustive]
    ToolExecutionStarted {
        #[serde(skip_serializing_if = "Option::is_none")]
        agent_id: Option<String>,
        #[serde(default)]
        arguments: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        created_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        name: ToolExecutionStartedEventName,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_index: Option<i64>,
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
            id,
            next_agent_id,
            next_agent_name,
            output_index: None,
        }
    }

    pub fn agent_handoff_started(
        id: String,
        previous_agent_id: String,
        previous_agent_name: String,
    ) -> Self {
        Self::AgentHandoffStarted {
            created_at: None,
            id,
            output_index: None,
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

    pub fn conversation_response_error(code: i64, message: String) -> Self {
        Self::ConversationResponseError {
            code,
            created_at: None,
            message,
        }
    }

    pub fn conversation_response_started(conversation_id: String) -> Self {
        Self::ConversationResponseStarted {
            conversation_id,
            created_at: None,
        }
    }

    pub fn function_call_delta(
        arguments: String,
        id: String,
        name: String,
        tool_call_id: String,
    ) -> Self {
        Self::FunctionCallDelta {
            agent_id: None,
            arguments,
            confirmation_status: None,
            created_at: None,
            id,
            model: None,
            name,
            output_index: None,
            tool_call_id,
        }
    }

    pub fn message_output_delta(content: MessageOutputEventContent, id: String) -> Self {
        Self::MessageOutputDelta {
            agent_id: None,
            content,
            content_index: None,
            created_at: None,
            id,
            model: None,
            output_index: None,
            role: None,
        }
    }

    pub fn tool_execution_delta(
        arguments: String,
        id: String,
        name: ToolExecutionDeltaEventName,
    ) -> Self {
        Self::ToolExecutionDelta {
            arguments,
            created_at: None,
            id,
            name,
            output_index: None,
        }
    }

    pub fn tool_execution_done(id: String, name: ToolExecutionDoneEventName) -> Self {
        Self::ToolExecutionDone {
            created_at: None,
            id,
            info: None,
            name,
            output_index: None,
        }
    }

    pub fn tool_execution_started(
        arguments: String,
        id: String,
        name: ToolExecutionStartedEventName,
    ) -> Self {
        Self::ToolExecutionStarted {
            agent_id: None,
            arguments,
            created_at: None,
            id,
            model: None,
            name,
            output_index: None,
        }
    }

    pub fn agent_handoff_done_with_created_at(
        created_at: DateTime<FixedOffset>,
        id: String,
        next_agent_id: String,
        next_agent_name: String,
        output_index: Option<i64>,
    ) -> Self {
        Self::AgentHandoffDone {
            created_at: Some(created_at),
            id,
            next_agent_id,
            next_agent_name,
            output_index,
        }
    }

    pub fn agent_handoff_done_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        next_agent_id: String,
        next_agent_name: String,
        output_index: i64,
    ) -> Self {
        Self::AgentHandoffDone {
            created_at,
            id,
            next_agent_id,
            next_agent_name,
            output_index: Some(output_index),
        }
    }

    pub fn agent_handoff_started_with_created_at(
        created_at: DateTime<FixedOffset>,
        id: String,
        output_index: Option<i64>,
        previous_agent_id: String,
        previous_agent_name: String,
    ) -> Self {
        Self::AgentHandoffStarted {
            created_at: Some(created_at),
            id,
            output_index,
            previous_agent_id,
            previous_agent_name,
        }
    }

    pub fn agent_handoff_started_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        output_index: i64,
        previous_agent_id: String,
        previous_agent_name: String,
    ) -> Self {
        Self::AgentHandoffStarted {
            created_at,
            id,
            output_index: Some(output_index),
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
        code: i64,
        created_at: DateTime<FixedOffset>,
        message: String,
    ) -> Self {
        Self::ConversationResponseError {
            code,
            created_at: Some(created_at),
            message,
        }
    }

    pub fn conversation_response_started_with_created_at(
        conversation_id: String,
        created_at: DateTime<FixedOffset>,
    ) -> Self {
        Self::ConversationResponseStarted {
            conversation_id,
            created_at: Some(created_at),
        }
    }

    pub fn function_call_delta_with_agent_id(
        agent_id: String,
        arguments: String,
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: Option<String>,
        name: String,
        output_index: Option<i64>,
        tool_call_id: String,
    ) -> Self {
        Self::FunctionCallDelta {
            agent_id: Some(agent_id),
            arguments,
            confirmation_status,
            created_at,
            id,
            model,
            name,
            output_index,
            tool_call_id,
        }
    }

    pub fn function_call_delta_with_confirmation_status(
        agent_id: Option<String>,
        arguments: String,
        confirmation_status: FunctionCallEventConfirmationStatus,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: Option<String>,
        name: String,
        output_index: Option<i64>,
        tool_call_id: String,
    ) -> Self {
        Self::FunctionCallDelta {
            agent_id,
            arguments,
            confirmation_status: Some(confirmation_status),
            created_at,
            id,
            model,
            name,
            output_index,
            tool_call_id,
        }
    }

    pub fn function_call_delta_with_created_at(
        agent_id: Option<String>,
        arguments: String,
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
        created_at: DateTime<FixedOffset>,
        id: String,
        model: Option<String>,
        name: String,
        output_index: Option<i64>,
        tool_call_id: String,
    ) -> Self {
        Self::FunctionCallDelta {
            agent_id,
            arguments,
            confirmation_status,
            created_at: Some(created_at),
            id,
            model,
            name,
            output_index,
            tool_call_id,
        }
    }

    pub fn function_call_delta_with_model(
        agent_id: Option<String>,
        arguments: String,
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: String,
        name: String,
        output_index: Option<i64>,
        tool_call_id: String,
    ) -> Self {
        Self::FunctionCallDelta {
            agent_id,
            arguments,
            confirmation_status,
            created_at,
            id,
            model: Some(model),
            name,
            output_index,
            tool_call_id,
        }
    }

    pub fn function_call_delta_with_output_index(
        agent_id: Option<String>,
        arguments: String,
        confirmation_status: Option<FunctionCallEventConfirmationStatus>,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: Option<String>,
        name: String,
        output_index: i64,
        tool_call_id: String,
    ) -> Self {
        Self::FunctionCallDelta {
            agent_id,
            arguments,
            confirmation_status,
            created_at,
            id,
            model,
            name,
            output_index: Some(output_index),
            tool_call_id,
        }
    }

    pub fn message_output_delta_with_agent_id(
        agent_id: String,
        content: MessageOutputEventContent,
        content_index: Option<i64>,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: Option<String>,
        output_index: Option<i64>,
        role: Option<MessageOutputEventRole>,
    ) -> Self {
        Self::MessageOutputDelta {
            agent_id: Some(agent_id),
            content,
            content_index,
            created_at,
            id,
            model,
            output_index,
            role,
        }
    }

    pub fn message_output_delta_with_content_index(
        agent_id: Option<String>,
        content: MessageOutputEventContent,
        content_index: i64,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: Option<String>,
        output_index: Option<i64>,
        role: Option<MessageOutputEventRole>,
    ) -> Self {
        Self::MessageOutputDelta {
            agent_id,
            content,
            content_index: Some(content_index),
            created_at,
            id,
            model,
            output_index,
            role,
        }
    }

    pub fn message_output_delta_with_created_at(
        agent_id: Option<String>,
        content: MessageOutputEventContent,
        content_index: Option<i64>,
        created_at: DateTime<FixedOffset>,
        id: String,
        model: Option<String>,
        output_index: Option<i64>,
        role: Option<MessageOutputEventRole>,
    ) -> Self {
        Self::MessageOutputDelta {
            agent_id,
            content,
            content_index,
            created_at: Some(created_at),
            id,
            model,
            output_index,
            role,
        }
    }

    pub fn message_output_delta_with_model(
        agent_id: Option<String>,
        content: MessageOutputEventContent,
        content_index: Option<i64>,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: String,
        output_index: Option<i64>,
        role: Option<MessageOutputEventRole>,
    ) -> Self {
        Self::MessageOutputDelta {
            agent_id,
            content,
            content_index,
            created_at,
            id,
            model: Some(model),
            output_index,
            role,
        }
    }

    pub fn message_output_delta_with_output_index(
        agent_id: Option<String>,
        content: MessageOutputEventContent,
        content_index: Option<i64>,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: Option<String>,
        output_index: i64,
        role: Option<MessageOutputEventRole>,
    ) -> Self {
        Self::MessageOutputDelta {
            agent_id,
            content,
            content_index,
            created_at,
            id,
            model,
            output_index: Some(output_index),
            role,
        }
    }

    pub fn message_output_delta_with_role(
        agent_id: Option<String>,
        content: MessageOutputEventContent,
        content_index: Option<i64>,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: Option<String>,
        output_index: Option<i64>,
        role: MessageOutputEventRole,
    ) -> Self {
        Self::MessageOutputDelta {
            agent_id,
            content,
            content_index,
            created_at,
            id,
            model,
            output_index,
            role: Some(role),
        }
    }

    pub fn tool_execution_delta_with_created_at(
        arguments: String,
        created_at: DateTime<FixedOffset>,
        id: String,
        name: ToolExecutionDeltaEventName,
        output_index: Option<i64>,
    ) -> Self {
        Self::ToolExecutionDelta {
            arguments,
            created_at: Some(created_at),
            id,
            name,
            output_index,
        }
    }

    pub fn tool_execution_delta_with_output_index(
        arguments: String,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        name: ToolExecutionDeltaEventName,
        output_index: i64,
    ) -> Self {
        Self::ToolExecutionDelta {
            arguments,
            created_at,
            id,
            name,
            output_index: Some(output_index),
        }
    }

    pub fn tool_execution_done_with_created_at(
        created_at: DateTime<FixedOffset>,
        id: String,
        info: Option<ToolExecutionInfo>,
        name: ToolExecutionDoneEventName,
        output_index: Option<i64>,
    ) -> Self {
        Self::ToolExecutionDone {
            created_at: Some(created_at),
            id,
            info,
            name,
            output_index,
        }
    }

    pub fn tool_execution_done_with_info(
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        info: ToolExecutionInfo,
        name: ToolExecutionDoneEventName,
        output_index: Option<i64>,
    ) -> Self {
        Self::ToolExecutionDone {
            created_at,
            id,
            info: Some(info),
            name,
            output_index,
        }
    }

    pub fn tool_execution_done_with_output_index(
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        info: Option<ToolExecutionInfo>,
        name: ToolExecutionDoneEventName,
        output_index: i64,
    ) -> Self {
        Self::ToolExecutionDone {
            created_at,
            id,
            info,
            name,
            output_index: Some(output_index),
        }
    }

    pub fn tool_execution_started_with_agent_id(
        agent_id: String,
        arguments: String,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: Option<String>,
        name: ToolExecutionStartedEventName,
        output_index: Option<i64>,
    ) -> Self {
        Self::ToolExecutionStarted {
            agent_id: Some(agent_id),
            arguments,
            created_at,
            id,
            model,
            name,
            output_index,
        }
    }

    pub fn tool_execution_started_with_created_at(
        agent_id: Option<String>,
        arguments: String,
        created_at: DateTime<FixedOffset>,
        id: String,
        model: Option<String>,
        name: ToolExecutionStartedEventName,
        output_index: Option<i64>,
    ) -> Self {
        Self::ToolExecutionStarted {
            agent_id,
            arguments,
            created_at: Some(created_at),
            id,
            model,
            name,
            output_index,
        }
    }

    pub fn tool_execution_started_with_model(
        agent_id: Option<String>,
        arguments: String,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: String,
        name: ToolExecutionStartedEventName,
        output_index: Option<i64>,
    ) -> Self {
        Self::ToolExecutionStarted {
            agent_id,
            arguments,
            created_at,
            id,
            model: Some(model),
            name,
            output_index,
        }
    }

    pub fn tool_execution_started_with_output_index(
        agent_id: Option<String>,
        arguments: String,
        created_at: Option<DateTime<FixedOffset>>,
        id: String,
        model: Option<String>,
        name: ToolExecutionStartedEventName,
        output_index: i64,
    ) -> Self {
        Self::ToolExecutionStarted {
            agent_id,
            arguments,
            created_at,
            id,
            model,
            name,
            output_index: Some(output_index),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
