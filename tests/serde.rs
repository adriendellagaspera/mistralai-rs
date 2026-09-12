use mistralai::raw::types::{ChatCompletionRequest, ChatCompletionResponse};
use serde_json::json;

#[test]
fn chat_response_exposes_integer_token_usage() {
    let response: ChatCompletionResponse = serde_json::from_value(json!({
        "id": "mock-completion", "object": "chat.completion", "created": 1700000000,
        "model": "test-model",
        "choices": [{"index": 0, "finish_reason": "stop",
                     "message": {"role": "assistant", "content": "Bonjour"}}],
        "usage": {"prompt_tokens": 13, "completion_tokens": 7, "total_tokens": 20}
    }))
    .unwrap();
    assert_eq!(response.usage.prompt_tokens, 13);
    assert_eq!(response.usage.completion_tokens, 7);
    assert_eq!(response.usage.total_tokens, 20);
}

#[test]
fn request_preserves_roles_tools_and_json_schema() {
    let value = json!({
        "model": "test-model",
        "messages": [
            {"role": "system", "content": "Use tools."},
            {"role": "user", "content": "Weather in Paris?"},
            {"role": "assistant", "content": "", "tool_calls": [
                {"id": "call_1", "type": "function", "function": {
                    "name": "weather", "arguments": "{\"city\":\"Paris\"}"}}
            ]},
            {"role": "tool", "tool_call_id": "call_1", "content": "Sunny"}
        ],
        "tools": [{"type": "function", "function": {
            "name": "weather", "parameters": {"type": "object",
                "properties": {"city": {"type": "string"}}, "required": ["city"]}
        }}],
        "tool_choice": "auto", "stream": false, "max_tokens": 32,
        "response_format": {"type": "json_schema", "json_schema": {
            "name": "answer", "schema": {"type": "object", "properties": {
                "answer": {"type": "string"}}}, "strict": true}}
    });
    let request: ChatCompletionRequest = serde_json::from_value(value.clone()).unwrap();
    let output = serde_json::to_value(request).unwrap();
    for key in [
        "model",
        "messages",
        "tools",
        "tool_choice",
        "stream",
        "max_tokens",
        "response_format",
    ] {
        assert_eq!(output[key], value[key], "field {key} changed on the wire");
    }
}
