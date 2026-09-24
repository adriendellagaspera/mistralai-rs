use mistralai_candidate_raw_check::generated::{
    client::HttpClient,
    types::PromptsUpdateRequest,
};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

#[test]
fn candidate_nullable_request_preserves_omission_null_and_value() {
    let omitted = PromptsUpdateRequest {
        description: None,
        sharing_scope: None,
        title: None,
    };
    assert_eq!(serde_json::to_value(omitted).expect("serialize omission"), serde_json::json!({}));

    let explicit_null = PromptsUpdateRequest {
        description: Some(None),
        sharing_scope: None,
        title: None,
    };
    assert_eq!(
        serde_json::to_value(explicit_null).expect("serialize explicit null"),
        serde_json::json!({"description": null})
    );

    let value = PromptsUpdateRequest {
        description: Some(Some("updated".to_owned())),
        sharing_scope: None,
        title: None,
    };
    assert_eq!(
        serde_json::to_value(value).expect("serialize present value"),
        serde_json::json!({"description": "updated"})
    );
}

fn capture_one_request() -> (String, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test socket");
    let url = format!("http://{}", listener.local_addr().expect("test address"));
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().expect("generated raw HTTP request");
        socket
            .set_read_timeout(Some(Duration::from_secs(15)))
            .expect("read timeout");
        let mut bytes = Vec::new();
        let mut chunk = [0_u8; 8192];
        let header_end = loop {
            let read = socket.read(&mut chunk).expect("read HTTP headers");
            assert!(read > 0, "socket closed before complete headers");
            bytes.extend_from_slice(&chunk[..read]);
            if let Some(start) = bytes.windows(4).position(|window| window == b"\\r\\n\\r\\n") {
                break start + 4;
            }
        };
        let headers = std::str::from_utf8(&bytes[..header_end]).expect("ASCII HTTP headers");
        let content_length = headers
            .lines()
            .find_map(|line| {
                let (key, value) = line.split_once(':')?;
                key.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().expect("numeric content length"))
            })
            .expect("generated JSON request sets Content-Length");
        while bytes.len() < header_end + content_length {
            let read = socket.read(&mut chunk).expect("read HTTP body");
            assert!(read > 0, "socket closed before complete JSON body");
            bytes.extend_from_slice(&chunk[..read]);
        }
        socket
            .write_all(
                b"HTTP/1.1 422 Unprocessable Entity\\r\\nContent-Type: application/json\\r\\nContent-Length: 2\\r\\nConnection: close\\r\\n\\r\\n{}",
            )
            .expect("send synthetic validation error");
        String::from_utf8(bytes).expect("UTF-8 JSON HTTP request")
    });
    (url, server)
}

#[tokio::test]
async fn candidate_raw_http_preserves_optional_nullable_request_wire_values() {
    let cases = [
        (
            PromptsUpdateRequest {
                description: None,
                sharing_scope: None,
                title: None,
            },
            serde_json::json!({}),
        ),
        (
            PromptsUpdateRequest {
                description: Some(None),
                sharing_scope: None,
                title: None,
            },
            serde_json::json!({"description": null}),
        ),
        (
            PromptsUpdateRequest {
                description: Some(Some("updated".to_owned())),
                sharing_scope: None,
                title: None,
            },
            serde_json::json!({"description": "updated"}),
        ),
    ];
    for (request, expected) in cases {
        let (url, server) = capture_one_request();
        let response = HttpClient::new()
            .with_base_url(url)
            .prompts_update("p-test", request)
            .await;
        assert!(response.is_err(), "mock deliberately returns HTTP 422");
        let captured = server.join().expect("captured HTTP request");
        assert!(
            captured.starts_with("PATCH /v2/prompts/p-test HTTP/1.1\\r\\n"),
            "unexpected generated request: {captured}"
        );
        let (_, body) = captured.split_once("\\r\\n\\r\\n").expect("HTTP body boundary");
        let actual: serde_json::Value = serde_json::from_str(body).expect("valid JSON wire body");
        assert_eq!(actual, expected);
    }
}
