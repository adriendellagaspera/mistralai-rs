use mistralai_candidate_raw_check::generated::{
    client::HttpClient,
    types::{ListSortDirection, ListSortField},
};
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

fn capture_headers(event_stream: bool) -> (String, thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test socket");
    let url = format!("http://{}", listener.local_addr().expect("local test address"));
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().expect("generated raw HTTP request");
        socket
            .set_read_timeout(Some(Duration::from_secs(15)))
            .expect("read timeout");
        let mut bytes = Vec::new();
        let mut chunk = [0_u8; 4096];
        while !bytes.windows(4).any(|window| window == b"\r\n\r\n") {
            let count = socket.read(&mut chunk).expect("HTTP request headers");
            assert!(count > 0, "socket closed before request headers");
            bytes.extend_from_slice(&chunk[..count]);
        }
        if event_stream {
            socket
                .write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .expect("mock SSE response");
        } else {
            socket
                .write_all(
                    b"HTTP/1.1 422 Unprocessable Entity\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}",
                )
                .expect("mock JSON error");
        }
        String::from_utf8(bytes).expect("generated request headers are UTF-8")
    });
    (url, server)
}

fn request_target(captured: &str, expected_path: &str) -> BTreeMap<String, String> {
    let request_line = captured.lines().next().expect("HTTP request line");
    let parts: Vec<_> = request_line.split_whitespace().collect();
    assert_eq!(parts.len(), 3, "{request_line}");
    assert_eq!(parts[0], "GET");
    let target = url::Url::parse(&format!("http://localhost{}", parts[1]))
        .expect("parse generated HTTP target");
    assert_eq!(target.path(), expected_path);
    let pairs = target.query_pairs().collect::<Vec<_>>();
    let query: BTreeMap<String, String> = pairs
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect();
    assert_eq!(query.len(), pairs.len(), "duplicate generated query parameters");
    query
}

fn header<'a>(captured: &'a str, name: &str) -> Vec<&'a str> {
    captured
        .lines()
        .skip(1)
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (actual, value) = line.split_once(':')?;
            actual.eq_ignore_ascii_case(name).then_some(value.trim())
        })
        .collect()
}

#[tokio::test]
async fn prompts_and_skills_preserve_distinct_dotted_and_plain_query_parameters() {
    for (path, is_prompt) in [("/v2/prompts", true), ("/v2/skills", false)] {
        let (url, server) = capture_headers(false);
        let client = HttpClient::new().with_base_url(url);
        if is_prompt {
            let result = client
                .prompts_list(
                    Some(7),
                    Some("page-cursor"),
                    None::<&str>,
                    None,
                    Some(ListSortField::ListSortFieldName),
                    Some(ListSortDirection::ListSortDirectionDesc),
                    Some("created_at"),
                    Some("asc"),
                )
                .await;
            assert!(result.is_err(), "mock deliberately returns 422");
        } else {
            let result = client
                .skills_list(
                    Some(7),
                    Some("page-cursor"),
                    None::<&str>,
                    None,
                    Some(ListSortField::ListSortFieldName),
                    Some(ListSortDirection::ListSortDirectionDesc),
                    Some("created_at"),
                    Some("asc"),
                )
                .await;
            assert!(result.is_err(), "mock deliberately returns 422");
        }
        let captured = server.join().expect("captured GET");
        let query = request_target(&captured, path);
        assert_eq!(query.len(), 6, "all six distinct wire parameters must remain");
        assert_eq!(query.get("pageSize").map(String::as_str), Some("7"));
        assert_eq!(query.get("pageToken").map(String::as_str), Some("page-cursor"));
        assert_eq!(
            query.get("sort.field").map(String::as_str),
            Some("list_sort_field_name")
        );
        assert_eq!(
            query.get("sort.direction").map(String::as_str),
            Some("list_sort_direction_desc")
        );
        assert_eq!(query.get("sort_by").map(String::as_str), Some("created_at"));
        assert_eq!(query.get("sort_direction").map(String::as_str), Some("asc"));
    }
}

#[tokio::test]
async fn stream_routes_preserve_distinct_query_and_header_cursors() {
    for (path, deployment) in [
        ("/v1/workflows/deployments/deployment-a/logs/stream", true),
        (
            "/v1/workflows/executions/execution-a/logs/stream",
            false,
        ),
    ] {
        let (url, server) = capture_headers(true);
        let client = HttpClient::new().with_base_url(url);
        if deployment {
            let result = client
                .stream_deployment_logs(
                    "deployment-a",
                    None::<&str>,
                    None::<&str>,
                    None::<&str>,
                    Some("query-cursor"),
                    Some("header-cursor"),
                )
                .await;
            assert!(result.is_ok(), "mock returns successful SSE handshake");
        } else {
            let result = client
                .stream_workflow_execution_logs(
                    "execution-a",
                    None::<&str>,
                    None::<&str>,
                    None::<&str>,
                    Some("query-cursor"),
                    Some("header-cursor"),
                )
                .await;
            assert!(result.is_ok(), "mock returns successful SSE handshake");
        }
        let captured = server.join().expect("captured SSE GET");
        let query = request_target(&captured, path);
        assert_eq!(query.len(), 1, "only query cursor was provided");
        assert_eq!(
            query.get("last_event_id").map(String::as_str),
            Some("query-cursor")
        );
        assert_eq!(header(&captured, "Last-Event-ID"), vec!["header-cursor"]);
    }
}
