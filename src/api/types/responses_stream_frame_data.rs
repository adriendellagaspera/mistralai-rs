pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ResponsesStreamFrameData {
    #[serde(rename = "error")]
    #[non_exhaustive]
    Error {
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<String>,
        #[serde(default)]
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        param: Option<String>,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.completed")]
    #[non_exhaustive]
    ResponseCompleted {
        response: CreateResponseResponse,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.content_part.added")]
    #[non_exhaustive]
    ResponseContentPartAdded {
        #[serde(default)]
        content_index: i64,
        #[serde(default)]
        item_id: String,
        #[serde(default)]
        output_index: i64,
        part: ResponsesContentPartAddedEventPart,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.content_part.done")]
    #[non_exhaustive]
    ResponseContentPartDone {
        #[serde(default)]
        content_index: i64,
        #[serde(default)]
        item_id: String,
        #[serde(default)]
        output_index: i64,
        part: ResponsesContentPartDoneEventPart,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.created")]
    #[non_exhaustive]
    ResponseCreated {
        response: CreateResponseResponse,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.failed")]
    #[non_exhaustive]
    ResponseFailed {
        response: CreateResponseResponse,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.function_call_arguments.delta")]
    #[non_exhaustive]
    ResponseFunctionCallArgumentsDelta {
        #[serde(default)]
        delta: String,
        #[serde(default)]
        item_id: String,
        #[serde(default)]
        output_index: i64,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.function_call_arguments.done")]
    #[non_exhaustive]
    ResponseFunctionCallArgumentsDone {
        #[serde(default)]
        arguments: String,
        #[serde(default)]
        item_id: String,
        #[serde(default)]
        name: String,
        #[serde(default)]
        output_index: i64,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.in_progress")]
    #[non_exhaustive]
    ResponseInProgress {
        response: CreateResponseResponse,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.incomplete")]
    #[non_exhaustive]
    ResponseIncomplete {
        response: CreateResponseResponse,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.output_item.added")]
    #[non_exhaustive]
    ResponseOutputItemAdded {
        item: ResponsesOutputItemAddedEventItem,
        #[serde(default)]
        output_index: i64,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.output_item.done")]
    #[non_exhaustive]
    ResponseOutputItemDone {
        item: ResponsesOutputItemDoneEventItem,
        #[serde(default)]
        output_index: i64,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.output_text.delta")]
    #[non_exhaustive]
    ResponseOutputTextDelta {
        #[serde(default)]
        content_index: i64,
        #[serde(default)]
        delta: String,
        #[serde(default)]
        item_id: String,
        #[serde(default)]
        logprobs: Vec<ResponsesTextDeltaEventLogprob>,
        #[serde(default)]
        output_index: i64,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.output_text.done")]
    #[non_exhaustive]
    ResponseOutputTextDone {
        #[serde(default)]
        content_index: i64,
        #[serde(default)]
        item_id: String,
        #[serde(default)]
        logprobs: Vec<ResponsesTextDoneEventLogprob>,
        #[serde(default)]
        output_index: i64,
        #[serde(default)]
        sequence_number: i64,
        #[serde(default)]
        text: String,
    },

    #[serde(rename = "response.reasoning_text.delta")]
    #[non_exhaustive]
    ResponseReasoningTextDelta {
        #[serde(default)]
        content_index: i64,
        #[serde(default)]
        delta: String,
        #[serde(default)]
        item_id: String,
        #[serde(default)]
        output_index: i64,
        #[serde(default)]
        sequence_number: i64,
    },

    #[serde(rename = "response.reasoning_text.done")]
    #[non_exhaustive]
    ResponseReasoningTextDone {
        #[serde(default)]
        content_index: i64,
        #[serde(default)]
        item_id: String,
        #[serde(default)]
        output_index: i64,
        #[serde(default)]
        sequence_number: i64,
        #[serde(default)]
        text: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ResponsesStreamFrameData {
    pub fn error(message: String, sequence_number: i64) -> Self {
        Self::Error {
            code: None,
            message,
            param: None,
            sequence_number,
        }
    }

    pub fn response_completed(response: CreateResponseResponse, sequence_number: i64) -> Self {
        Self::ResponseCompleted {
            response,
            sequence_number,
        }
    }

    pub fn response_content_part_added(
        content_index: i64,
        item_id: String,
        output_index: i64,
        part: ResponsesContentPartAddedEventPart,
        sequence_number: i64,
    ) -> Self {
        Self::ResponseContentPartAdded {
            content_index,
            item_id,
            output_index,
            part,
            sequence_number,
        }
    }

    pub fn response_content_part_done(
        content_index: i64,
        item_id: String,
        output_index: i64,
        part: ResponsesContentPartDoneEventPart,
        sequence_number: i64,
    ) -> Self {
        Self::ResponseContentPartDone {
            content_index,
            item_id,
            output_index,
            part,
            sequence_number,
        }
    }

    pub fn response_created(response: CreateResponseResponse, sequence_number: i64) -> Self {
        Self::ResponseCreated {
            response,
            sequence_number,
        }
    }

    pub fn response_failed(response: CreateResponseResponse, sequence_number: i64) -> Self {
        Self::ResponseFailed {
            response,
            sequence_number,
        }
    }

    pub fn response_function_call_arguments_delta(
        delta: String,
        item_id: String,
        output_index: i64,
        sequence_number: i64,
    ) -> Self {
        Self::ResponseFunctionCallArgumentsDelta {
            delta,
            item_id,
            output_index,
            sequence_number,
        }
    }

    pub fn response_function_call_arguments_done(
        arguments: String,
        item_id: String,
        name: String,
        output_index: i64,
        sequence_number: i64,
    ) -> Self {
        Self::ResponseFunctionCallArgumentsDone {
            arguments,
            item_id,
            name,
            output_index,
            sequence_number,
        }
    }

    pub fn response_in_progress(response: CreateResponseResponse, sequence_number: i64) -> Self {
        Self::ResponseInProgress {
            response,
            sequence_number,
        }
    }

    pub fn response_incomplete(response: CreateResponseResponse, sequence_number: i64) -> Self {
        Self::ResponseIncomplete {
            response,
            sequence_number,
        }
    }

    pub fn response_output_item_added(
        item: ResponsesOutputItemAddedEventItem,
        output_index: i64,
        sequence_number: i64,
    ) -> Self {
        Self::ResponseOutputItemAdded {
            item,
            output_index,
            sequence_number,
        }
    }

    pub fn response_output_item_done(
        item: ResponsesOutputItemDoneEventItem,
        output_index: i64,
        sequence_number: i64,
    ) -> Self {
        Self::ResponseOutputItemDone {
            item,
            output_index,
            sequence_number,
        }
    }

    pub fn response_output_text_delta(
        content_index: i64,
        delta: String,
        item_id: String,
        logprobs: Vec<ResponsesTextDeltaEventLogprob>,
        output_index: i64,
        sequence_number: i64,
    ) -> Self {
        Self::ResponseOutputTextDelta {
            content_index,
            delta,
            item_id,
            logprobs,
            output_index,
            sequence_number,
        }
    }

    pub fn response_output_text_done(
        content_index: i64,
        item_id: String,
        logprobs: Vec<ResponsesTextDoneEventLogprob>,
        output_index: i64,
        sequence_number: i64,
        text: String,
    ) -> Self {
        Self::ResponseOutputTextDone {
            content_index,
            item_id,
            logprobs,
            output_index,
            sequence_number,
            text,
        }
    }

    pub fn response_reasoning_text_delta(
        content_index: i64,
        delta: String,
        item_id: String,
        output_index: i64,
        sequence_number: i64,
    ) -> Self {
        Self::ResponseReasoningTextDelta {
            content_index,
            delta,
            item_id,
            output_index,
            sequence_number,
        }
    }

    pub fn response_reasoning_text_done(
        content_index: i64,
        item_id: String,
        output_index: i64,
        sequence_number: i64,
        text: String,
    ) -> Self {
        Self::ResponseReasoningTextDone {
            content_index,
            item_id,
            output_index,
            sequence_number,
            text,
        }
    }

    pub fn error_with_code(
        code: String,
        message: String,
        param: Option<String>,
        sequence_number: i64,
    ) -> Self {
        Self::Error {
            code: Some(code),
            message,
            param,
            sequence_number,
        }
    }

    pub fn error_with_param(
        code: Option<String>,
        message: String,
        param: String,
        sequence_number: i64,
    ) -> Self {
        Self::Error {
            code,
            message,
            param: Some(param),
            sequence_number,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
