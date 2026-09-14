use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use mistralai::raw::{Client, types::ChatCompletionRequest};
use serde_json::{Value, json};

fn server(status: &str, body: Value) -> (String, JoinHandle<(String, Value)>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let body = body.to_string();
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let worker = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut received = Vec::new();
        let mut buffer = [0; 4096];
        let (header, payload) = loop {
            let count = socket.read(&mut buffer).unwrap();
            assert_ne!(count, 0, "client closed before sending a complete request");
            received.extend_from_slice(&buffer[..count]);
            if let Some(end) = received.windows(4).position(|v| v == b"\r\n\r\n") {
                let header = String::from_utf8(received[..end].to_vec()).unwrap();
                let length: usize = header
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse().unwrap())
                    })
                    .unwrap_or(0);
                if received.len() >= end + 4 + length {
                    let payload = if length == 0 {
                        Value::Null
                    } else {
                        serde_json::from_slice(&received[end + 4..end + 4 + length]).unwrap()
                    };
                    break (header, payload);
                }
            }
        };
        socket.write_all(response.as_bytes()).unwrap();
        (header, payload)
    });
    (address, worker)
}

fn request() -> ChatCompletionRequest {
    serde_json::from_value(json!({
        "model": "test-model", "messages": [{"role": "user", "content": "Hello"}],
        "stream": false
    }))
    .unwrap()
}

#[tokio::test]
async fn chat_sends_auth_and_typed_request_and_decodes_usage() {
    let (url, worker) = server(
        "200 OK",
        json!({
            "id": "test", "object": "chat.completion", "created": 1700000000,
            "model": "test-model", "choices": [],
            "usage": {"prompt_tokens": 5, "completion_tokens": 3, "total_tokens": 8}
        }),
    );
    let client = Client::new().with_base_url(&url).with_api_key("test-key");
    let response = client
        .chat_completion_v1_chat_completions_post(request())
        .await
        .unwrap();
    assert_eq!(response.usage.prompt_tokens, 5);
    assert_eq!(response.usage.completion_tokens, 3);
    let (header, payload) = worker.join().unwrap();
    assert!(header.starts_with("POST /v1/chat/completions HTTP/1.1\r\n"));
    assert!(
        header
            .to_ascii_lowercase()
            .contains("authorization: bearer test-key")
    );
    assert_eq!(
        payload["messages"],
        json!([{"role": "user", "content": "Hello"}])
    );
    assert_eq!(payload["stream"], false);
}

#[tokio::test]
async fn api_error_is_returned_to_caller() {
    let (url, worker) = server("401 Unauthorized", json!({"message": "Invalid API key"}));
    let client = Client::new().with_base_url(&url).with_api_key("test-key");
    let result = client
        .chat_completion_v1_chat_completions_post(request())
        .await;
    assert!(result.is_err());
    worker.join().unwrap();
}

#[tokio::test]
async fn generated_query_request_owns_strings_and_preserves_wire_encoding() {
    use mistralai::{Mistral, models::ListModelsRequest};

    let (url, worker) = server("200 OK", json!({"data": [], "object": "list"}));
    let request = {
        let temporary = String::from("a provider&co");
        ListModelsRequest::default()
            .provider(temporary)
            .model("small/latest")
    };
    let client = Mistral::new("test-key").with_base_url(url);
    let response = client.models().list_with(request).await.unwrap();
    assert!(response.raw().data.as_ref().unwrap().is_empty());
    let (header, payload) = worker.join().unwrap();
    assert!(
        header.starts_with(
            "GET /v1/models?provider=a+provider%26co&model=small%2Flatest HTTP/1.1\r\n"
        )
    );
    assert_eq!(payload, Value::Null);
}

#[tokio::test]
async fn generated_parameterless_convenience_omits_optional_query() {
    let (url, worker) = server("200 OK", json!({"data": []}));
    let client = mistralai::Mistral::new("test-key").with_base_url(url);
    client.models().list().await.unwrap();
    let (header, _) = worker.join().unwrap();
    assert!(header.starts_with("GET /v1/models HTTP/1.1\r\n"));
}
