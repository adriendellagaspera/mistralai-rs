use futures::StreamExt;
use mistralai_sdk::{ClientConfig, HttpClient};
use reqwest::Method;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

async fn serve_once(response: &'static [u8]) -> (String, oneshot::Receiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut bytes = vec![0_u8; 32 * 1024];
        let mut used = 0;
        loop {
            let n = socket.read(&mut bytes[used..]).await.unwrap();
            if n == 0 {
                break;
            }
            used += n;
            if bytes[..used].windows(4).any(|window| window == b"\r\n\r\n") || used == bytes.len() {
                break;
            }
        }
        let request = String::from_utf8_lossy(&bytes[..used]).into_owned();
        let _ = tx.send(request);
        socket.write_all(response).await.unwrap();
        socket.shutdown().await.unwrap();
    });
    (format!("http://{addr}/"), rx)
}

fn client(base_url: String) -> HttpClient {
    HttpClient::new(ClientConfig {
        base_url,
        ..Default::default()
    })
    .unwrap()
}

#[tokio::test]
async fn json_transport_round_trip() {
    let (base_url, request) = serve_once(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 11\r\nConnection: close\r\n\r\n{\"ok\":true}",
    )
    .await;
    let value: serde_json::Value = client(base_url)
        .execute_request(Method::POST, "json", Some(json!({"hello": "world"})), None, None)
        .await
        .unwrap();
    assert_eq!(value, json!({"ok": true}));
    assert!(request.await.unwrap().starts_with("POST /json HTTP/1.1"));
}

#[tokio::test]
async fn sse_transport_round_trip() {
    let body = "data: {\"value\":1}\n\n";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(), body
    );
    let leaked: &'static [u8] = Box::leak(response.into_bytes().into_boxed_slice());
    let (base_url, request) = serve_once(leaked).await;
    let stream = client(base_url)
        .execute_sse_request::<serde_json::Value>(Method::GET, "events", None, None, None, None)
        .await
        .unwrap();
    let mut stream = Box::pin(stream);
    assert_eq!(stream.next().await.unwrap().unwrap(), json!({"value": 1}));
    let request = request.await.unwrap().to_ascii_lowercase();
    assert!(request.starts_with("get /events http/1.1"));
    assert!(request.contains("accept: text/event-stream"));
}

#[tokio::test]
async fn multipart_sse_workaround_round_trip() {
    let body = "data: {\"value\":2}\n\n";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(), body
    );
    let leaked: &'static [u8] = Box::leak(response.into_bytes().into_boxed_slice());
    let (base_url, request) = serve_once(leaked).await;
    let form = reqwest::multipart::Form::new().text("model", "voxtral-mini-latest");
    let stream = client(base_url)
        .execute_multipart_sse_request::<serde_json::Value>(
            Method::POST,
            "multipart-events",
            form,
            None,
            None,
            None,
        )
        .await
        .unwrap();
    let mut stream = Box::pin(stream);
    assert_eq!(stream.next().await.unwrap().unwrap(), json!({"value": 2}));
    let request = request.await.unwrap().to_ascii_lowercase();
    assert!(request.starts_with("post /multipart-events http/1.1"));
    assert!(request.contains("content-type: multipart/form-data; boundary="));
    assert!(request.contains("accept: text/event-stream"));
}

#[tokio::test]
async fn binary_stream_round_trip() {
    let (base_url, request) = serve_once(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: 4\r\nConnection: close\r\n\r\n\x00\x01\x02\x03",
    )
    .await;
    let bytes = client(base_url)
        .execute_stream_request(Method::GET, "binary", None, None, None)
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();
    assert_eq!(bytes, vec![0, 1, 2, 3]);
    assert!(request.await.unwrap().starts_with("GET /binary HTTP/1.1"));
}
