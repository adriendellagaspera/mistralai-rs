use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use bytes::Bytes;
use futures_util::StreamExt;
use mistralai::{
    raw::{
        Client,
        types::{
            AudioTranscriptionRequest, ChatCompletionRequest, CompletionChunk, EmbeddingRequest,
            FilesApiRoutesUploadFileRequest,
        },
    },
    streaming,
};
use serde_json::json;

// Serve one bounded offline request, retaining raw bytes (including multipart).
fn server(status: &str, media: &str, body: &[u8]) -> (String, JoinHandle<(String, Vec<u8>)>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let mut response = format!("HTTP/1.1 {status}\r\nContent-Type: {media}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len()).into_bytes();
    response.extend_from_slice(body);
    let worker = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut socket = loop {
            match listener.accept() {
                Ok((socket, _)) => break socket,
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < deadline =>
                {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(e) => panic!("mock server accept: {e}"),
            }
        };
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut received = Vec::new();
        let mut buffer = [0; 4096];
        let (headers, body) = loop {
            let count = socket.read(&mut buffer).unwrap();
            assert_ne!(count, 0);
            received.extend_from_slice(&buffer[..count]);
            assert!(received.len() < 1024 * 1024);
            if let Some(end) = received.windows(4).position(|v| v == b"\r\n\r\n") {
                let headers = String::from_utf8(received[..end].to_vec()).unwrap();
                assert!(
                    !headers
                        .to_lowercase()
                        .contains("transfer-encoding: chunked"),
                    "these byte-backed test bodies must have a known length"
                );
                let length = headers
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                if received.len() >= end + 4 + length {
                    break (headers, received[end + 4..end + 4 + length].to_vec());
                }
            }
        };
        socket.write_all(&response).unwrap();
        (headers, body)
    });
    (url, worker)
}

#[tokio::test]
async fn transcription_multipart_preserves_file_arrays_and_null_omission() {
    let body = json!({"model":"voxtral-test", "text":"hello", "language":"en",
        "usage":{"prompt_tokens":2,"completion_tokens":1,"total_tokens":3}})
    .to_string();
    let (url, worker) = server("200 OK", "application/json", body.as_bytes());
    let mut request = AudioTranscriptionRequest::new("voxtral-test".into());
    request.file = Some(Some(Bytes::from_static(b"RIFF\x00\xffWAVE")));
    request.context_bias = Some(vec!["Mistral".into(), "Voxtral".into()]);
    request.timestamp_granularities =
        Some(serde_json::from_value(json!(["word", "segment"])).unwrap());
    request.language = Some(None);
    let response = Client::new()
        .with_base_url(&url)
        .with_api_key("test-key")
        .audio_api_v1_transcriptions_post_with_multipart_filenames(
            request,
            &[("file", "sample.wav")],
        )
        .await
        .unwrap();
    assert_eq!(response.usage.completion_tokens, 1);
    let (headers, payload) = worker.join().unwrap();
    assert!(headers.starts_with("POST /v1/audio/transcriptions HTTP/1.1"));
    assert!(
        headers
            .to_lowercase()
            .contains("content-type: multipart/form-data; boundary=")
    );
    assert!(
        headers
            .to_lowercase()
            .contains("authorization: bearer test-key")
    );
    let text = String::from_utf8_lossy(&payload);
    assert_eq!(text.matches("name=\"context_bias\"").count(), 2);
    assert_eq!(text.matches("name=\"timestamp_granularities\"").count(), 2);
    assert!(text.contains("filename=\"sample.wav\""));
    assert!(!text.contains("name=\"language\""));
    assert!(payload.windows(10).any(|w| w == b"RIFF\x00\xffWAVE"));
}

#[tokio::test]
async fn file_upload_decodes_typed_response_and_sends_filename() {
    let body = json!({"id":"00000000-0000-0000-0000-000000000001", "object":"file",
        "bytes":3,"created_at":1700000000,"filename":"batch.jsonl",
        "purpose":"batch","sample_type":"batch_request","source":"upload"})
    .to_string();
    let (url, worker) = server("200 OK", "application/json", body.as_bytes());
    let request = FilesApiRoutesUploadFileRequest::new(Bytes::from_static(b"{}\n"));
    let response = Client::new()
        .with_base_url(&url)
        .files_api_routes_upload_file_with_multipart_filenames(request, &[("file", "batch.jsonl")])
        .await
        .unwrap();
    assert_eq!(response.filename, "batch.jsonl");
    let (_, payload) = worker.join().unwrap();
    assert!(String::from_utf8_lossy(&payload).contains("filename=\"batch.jsonl\""));
}

#[tokio::test]
async fn binary_downloads_and_wav_use_correct_media_and_escaped_paths() {
    let bytes = b"RIFF\x00\xffWAVE";
    let (url, worker) = server("200 OK", "application/octet-stream", bytes);
    let result = Client::new()
        .with_base_url(&url)
        .files_api_routes_download_file("a/b ?")
        .await
        .unwrap();
    assert_eq!(result.as_ref(), bytes);
    let (headers, _) = worker.join().unwrap();
    assert!(headers.starts_with("GET /v1/files/a%2Fb%20%3F/content HTTP/1.1"));

    let (url, worker) = server("200 OK", "audio/wav", bytes);
    let result = Client::new()
        .with_base_url(&url)
        .get_voice_sample_audio_v1_audio_voices_voice_id_sample_get_wav("test")
        .await
        .unwrap();
    assert_eq!(result.as_ref(), bytes);
    let (headers, _) = worker.join().unwrap();
    assert!(headers.starts_with("GET /v1/audio/voices/test/sample HTTP/1.1"));
    assert!(headers.to_lowercase().contains("accept: audio/wav"));

    let (url, worker) = server("200 OK", "application/octet-stream", bytes);
    let streamed = Client::new()
        .with_base_url(&url)
        .files_api_routes_download_file_stream("streamed")
        .await
        .unwrap()
        .fold(Vec::new(), |mut body, chunk| async move {
            body.extend_from_slice(&chunk.unwrap());
            body
        })
        .await;
    assert_eq!(streamed, bytes);
    let (headers, _) = worker.join().unwrap();
    assert!(headers.starts_with("GET /v1/files/streamed/content HTTP/1.1"));
}

#[tokio::test]
async fn embeddings_send_typed_json_and_expose_usage() {
    let body = json!({"id":"e1","object":"list","model":"test","data":[
        {"object":"embedding","index":0,"embedding":[0.25,0.75]}],
        "usage":{"prompt_tokens":4,"completion_tokens":0,"total_tokens":4}})
    .to_string();
    let (url, worker) = server("200 OK", "application/json", body.as_bytes());
    let request: EmbeddingRequest =
        serde_json::from_value(json!({"model":"test","input":["hello"]})).unwrap();
    let result = Client::new()
        .with_base_url(&url)
        .embeddings_v1_embeddings_post(request)
        .await
        .unwrap();
    assert_eq!(result.usage.prompt_tokens, 4);
    assert_eq!(result.data.len(), 1);
    let (headers, payload) = worker.join().unwrap();
    assert!(headers.starts_with("POST /v1/embeddings HTTP/1.1"));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&payload).unwrap()["input"],
        json!(["hello"])
    );
}

#[tokio::test]
async fn generated_chat_stream_sends_stream_flag_and_decodes_sse() {
    let body = b"data: {\"id\":\"c1\",\"model\":\"test\",\"choices\":[]}\n\ndata: [DONE]\n\n";
    let (url, worker) = server("200 OK", "text/event-stream", body);
    let request: ChatCompletionRequest = serde_json::from_value(json!({"model":"test",
        "messages":[{"role":"user","content":"hello"}]}))
    .unwrap();
    let client = Client::new().with_base_url(&url).with_api_key("test-key");
    let bytes = client
        .chat_completion_v1_chat_completions_post_stream(request)
        .await
        .unwrap();
    let events: Vec<_> = streaming::json_events::<_, _, CompletionChunk>(bytes)
        .collect()
        .await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].as_ref().unwrap().data.id, "c1");
    let (headers, payload) = worker.join().unwrap();
    assert!(headers.starts_with("POST /v1/chat/completions HTTP/1.1"));
    assert!(headers.to_lowercase().contains("accept: text/event-stream"));
    assert!(
        headers
            .to_lowercase()
            .contains("authorization: bearer test-key")
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&payload).unwrap()["stream"],
        true
    );
}

#[test]
fn repaired_workflow_list_and_sharing_delete_match_wire_fields() {
    let workflows: mistralai::raw::types::WorkflowListResponse =
        serde_json::from_value(json!({"workflows":[],"next_cursor":null})).unwrap();
    assert!(workflows.workflows.is_empty());
    assert!(
        serde_json::from_value::<mistralai::raw::types::WorkflowListResponse>(
            json!({"next_cursor":null}),
        )
        .is_err()
    );
    let sharing: mistralai::raw::types::SharingDelete = serde_json::from_value(json!({
        "share_with_uuid":"00000000-0000-0000-0000-000000000001", "share_with_type":"User"}))
    .unwrap();
    assert!(
        serde_json::to_value(sharing)
            .unwrap()
            .get("level")
            .is_none()
    );
}

#[tokio::test]
async fn native_audio_stream_sets_mode_and_decodes_typed_envelope() {
    let body = b"event: transcription.text.delta\ndata: {\"type\":\"transcription.text.delta\",\"text\":\"hello\"}\n\n";
    let (url, worker) = server("200 OK", "text/event-stream", body);
    let mut request =
        mistralai::raw::types::AudioTranscriptionRequestStream::new("voxtral-test".into());
    request.file_id = Some(Some("file-id".into()));
    let client = Client::new().with_base_url(&url);
    let bytes = client
        .audio_api_v1_transcriptions_post_stream(request)
        .await
        .unwrap();
    let events: Vec<_> = streaming::events(bytes).collect().await;
    let event: mistralai::raw::types::TranscriptionStreamEvents =
        events[0].as_ref().unwrap().envelope().unwrap();
    assert_eq!(event.event.to_string(), "transcription.text.delta");
    let (headers, payload) = worker.join().unwrap();
    assert!(headers.starts_with("POST /v1/audio/transcriptions HTTP/1.1"));
    assert!(String::from_utf8_lossy(&payload).contains("name=\"stream\"\r\n\r\ntrue\r\n"));
}

#[tokio::test]
async fn workflow_feed_encodes_query_parameters_without_buffering_json() {
    let (url, worker) = server("200 OK", "text/event-stream", b"data: {\"test\":true}\n\n");
    let client = Client::new().with_base_url(&url);
    let bytes = client
        .stream_v1_workflows_executions_execution_id_stream_get(
            "execution/id",
            Some("worker a"),
            Some("cursor-7"),
        )
        .await
        .unwrap();
    let events: Vec<_> = streaming::events(bytes).collect().await;
    assert_eq!(events.len(), 1);
    let (headers, _) = worker.join().unwrap();
    assert!(headers.starts_with(
        "GET /v1/workflows/executions/execution%2Fid/stream?event_source=worker+a&last_event_id=cursor-7 HTTP/1.1"
    ));
}
