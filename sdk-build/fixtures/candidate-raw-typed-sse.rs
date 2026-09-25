use futures_util::StreamExt;
use mistralai_candidate_raw_check::generated::{
    client::HttpClient,
    types::{DeploymentLogRecord, ExecutionLogRecord, StreamError},
};
use serde::Deserialize;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DeploymentEvent {
    Record(DeploymentLogRecord),
    Error(StreamError),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ExecutionEvent {
    Record(ExecutionLogRecord),
    Error(StreamError),
}

fn serve_sse(body: String) -> (String, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local SSE fixture");
    let url = format!("http://{}", listener.local_addr().expect("fixture address"));
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().expect("generated SSE request");
        socket
            .set_read_timeout(Some(Duration::from_secs(15)))
            .expect("read timeout");
        let mut request = Vec::new();
        let mut chunk = [0_u8; 4096];
        while !request.windows(4).any(|window| window == b"\r\n\r\n") {
            let count = socket.read(&mut chunk).expect("HTTP request headers");
            assert!(count > 0, "socket closed before request headers");
            request.extend_from_slice(&chunk[..count]);
        }
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        socket
            .write_all(response.as_bytes())
            .expect("write SSE response");
        String::from_utf8(request).expect("request is UTF-8")
    });
    (url, server)
}

async fn collect_stream<S, E>(mut stream: S) -> String
where
    S: futures_util::Stream<Item = Result<bytes::Bytes, E>> + Unpin,
    E: std::fmt::Debug,
{
    let mut bytes = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.expect("SSE byte chunk");
        bytes.extend_from_slice(&chunk);
    }
    String::from_utf8(bytes).expect("SSE body UTF-8")
}

fn data_payloads(body: &str) -> Vec<&str> {
    body.lines()
        .filter_map(|line| line.strip_prefix("data: "))
        .collect()
}

#[tokio::test]
async fn deployment_log_stream_decodes_record_and_error_payloads() {
    let record = r#"{"timestamp":"2026-09-25T12:00:00Z","trace_id":"trace","span_id":"span","severity_text":"INFO","body":"started","log_attributes":{"worker":"alpha"}}"#;
    let error = r#"{"error":"stream_failed","reason":"worker unavailable"}"#;
    let sse = format!("event: message\ndata: {record}\n\nevent: error\ndata: {error}\n\n");
    let (url, server) = serve_sse(sse);
    let client = HttpClient::new().with_base_url(url);

    let stream = client
        .stream_deployment_logs(
            "deployment-a",
            None::<&str>,
            None::<&str>,
            None::<&str>,
            None::<&str>,
            None::<&str>,
        )
        .await
        .expect("generated deployment log SSE request");
    let body = collect_stream(stream).await;
    let payloads = data_payloads(&body);
    assert_eq!(payloads.len(), 2);

    match serde_json::from_str::<DeploymentEvent>(payloads[0]).expect("record branch") {
        DeploymentEvent::Record(record) => assert_eq!(record.body, "started"),
        DeploymentEvent::Error(_) => panic!("record payload decoded as error branch"),
    }
    match serde_json::from_str::<DeploymentEvent>(payloads[1]).expect("error branch") {
        DeploymentEvent::Error(error) => {
            assert_eq!(error.error, "stream_failed");
            assert_eq!(error.reason, "worker unavailable");
        }
        DeploymentEvent::Record(_) => panic!("error payload decoded as record branch"),
    }

    let request = server.join().expect("deployment request captured");
    assert!(
        request.starts_with("GET /v1/workflows/deployments/deployment-a/logs/stream "),
        "{request}"
    );
}

#[tokio::test]
async fn execution_log_stream_decodes_record_and_error_payloads() {
    let record = r#"{"timestamp":"2026-09-25T12:00:00Z","trace_id":"trace","span_id":"span","severity_text":"WARN","body":"retrying","log_attributes":{"activity":"beta"}}"#;
    let error = r#"{"error":"stream_failed","reason":"execution unavailable"}"#;
    let sse = format!("data: {record}\n\ndata: {error}\n\n");
    let (url, server) = serve_sse(sse);
    let client = HttpClient::new().with_base_url(url);

    let stream = client
        .stream_workflow_execution_logs(
            "execution-a",
            None::<&str>,
            None::<&str>,
            None::<&str>,
            None::<&str>,
            None::<&str>,
        )
        .await
        .expect("generated execution log SSE request");
    let body = collect_stream(stream).await;
    let payloads = data_payloads(&body);
    assert_eq!(payloads.len(), 2);

    match serde_json::from_str::<ExecutionEvent>(payloads[0]).expect("record branch") {
        ExecutionEvent::Record(record) => assert_eq!(record.body, "retrying"),
        ExecutionEvent::Error(_) => panic!("record payload decoded as error branch"),
    }
    match serde_json::from_str::<ExecutionEvent>(payloads[1]).expect("error branch") {
        ExecutionEvent::Error(error) => assert_eq!(error.reason, "execution unavailable"),
        ExecutionEvent::Record(_) => panic!("error payload decoded as record branch"),
    }

    let request = server.join().expect("execution request captured");
    assert!(
        request.starts_with("GET /v1/workflows/executions/execution-a/logs/stream "),
        "{request}"
    );
}
