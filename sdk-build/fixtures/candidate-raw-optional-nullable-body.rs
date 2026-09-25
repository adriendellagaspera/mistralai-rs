use mistralai_candidate_raw_check::generated::{
    client::HttpClient,
    types::{WorkflowSchedulePauseRequest, WorkflowScheduleTriggerRequest},
};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

fn capture_one() -> (String, thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test socket");
    let url = format!("http://{}", listener.local_addr().expect("local address"));
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().expect("generated raw HTTP request");
        socket
            .set_read_timeout(Some(Duration::from_secs(15)))
            .expect("read timeout");
        let mut bytes = Vec::new();
        let mut chunk = [0_u8; 8192];
        let header_end = loop {
            let count = socket.read(&mut chunk).expect("read request");
            assert!(count > 0, "request ended before headers");
            bytes.extend_from_slice(&chunk[..count]);
            if let Some(offset) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
                break offset + 4;
            }
        };
        let headers = std::str::from_utf8(&bytes[..header_end]).expect("ASCII headers");
        let length = headers
            .lines()
            .filter_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().expect("content length"))
            })
            .next()
            .unwrap_or(0);
        while bytes.len() < header_end + length {
            let count = socket.read(&mut chunk).expect("read request body");
            assert!(count > 0, "request ended before body");
            bytes.extend_from_slice(&chunk[..count]);
        }
        socket
            .write_all(
                b"HTTP/1.1 204 No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )
            .expect("mock response");
        bytes
    });
    (url, server)
}

fn request_parts(bytes: &[u8]) -> (&str, &[u8]) {
    let split = bytes
        .windows(4)
        .position(|part| part == b"\r\n\r\n")
        .expect("header boundary");
    (
        std::str::from_utf8(&bytes[..split]).expect("request headers"),
        &bytes[split + 4..],
    )
}

#[tokio::test]
async fn optional_nullable_schedule_none_is_absent_not_json_null() {
    for (suffix, kind) in [("pause", 0_u8), ("resume", 1), ("trigger", 2)] {
        let (url, server) = capture_one();
        let client = HttpClient::new().with_base_url(url);
        match kind {
            0 => client
                .pause_schedule_v1_workflows_schedules_schedule_id_pause_post(
                    "schedule-a",
                    None::<Option<WorkflowSchedulePauseRequest>>,
                )
                .await
                .expect("mock 204 pause"),
            1 => client
                .resume_schedule_v1_workflows_schedules_schedule_id_resume_post(
                    "schedule-a",
                    None::<Option<WorkflowSchedulePauseRequest>>,
                )
                .await
                .expect("mock 204 resume"),
            _ => client
                .trigger_schedule_v1_workflows_schedules_schedule_id_trigger_post(
                    "schedule-a",
                    None::<Option<WorkflowScheduleTriggerRequest>>,
                )
                .await
                .expect("mock 204 trigger"),
        }
        let captured = server.join().expect("captured request");
        let (headers, body) = request_parts(&captured);
        assert!(
            headers.starts_with(&format!(
                "POST /v1/workflows/schedules/schedule-a/{suffix} HTTP/1.1"
            )),
            "{headers}"
        );
        assert!(body.is_empty(), "None emitted a body: {:?}", body);
        assert_ne!(body, b"null", "None must not be mistaken for explicit JSON null");
    }
}

#[tokio::test]
async fn optional_nullable_schedule_explicit_null_emits_json_null() {
    for (suffix, kind) in [("pause", 0_u8), ("resume", 1), ("trigger", 2)] {
        let (url, server) = capture_one();
        let client = HttpClient::new().with_base_url(url);
        match kind {
            0 => client
                .pause_schedule_v1_workflows_schedules_schedule_id_pause_post(
                    "schedule-a",
                    Some(None::<WorkflowSchedulePauseRequest>),
                )
                .await
                .expect("mock 204 pause"),
            1 => client
                .resume_schedule_v1_workflows_schedules_schedule_id_resume_post(
                    "schedule-a",
                    Some(None::<WorkflowSchedulePauseRequest>),
                )
                .await
                .expect("mock 204 resume"),
            _ => client
                .trigger_schedule_v1_workflows_schedules_schedule_id_trigger_post(
                    "schedule-a",
                    Some(None::<WorkflowScheduleTriggerRequest>),
                )
                .await
                .expect("mock 204 trigger"),
        }
        let captured = server.join().expect("captured request");
        let (headers, body) = request_parts(&captured);
        assert!(
            headers.starts_with(&format!(
                "POST /v1/workflows/schedules/schedule-a/{suffix} HTTP/1.1"
            )),
            "{headers}"
        );
        assert!(
            headers.lines().any(|line| {
                line.eq_ignore_ascii_case("content-type: application/json")
            }),
            "explicit null must carry JSON content type: {headers}"
        );
        assert_eq!(body, b"null");
    }
}

#[tokio::test]
async fn optional_nullable_schedule_some_emits_json_object() {
    let (url, server) = capture_one();
    HttpClient::new()
        .with_base_url(url)
        .pause_schedule_v1_workflows_schedules_schedule_id_pause_post(
            "schedule-a",
            Some(Some(WorkflowSchedulePauseRequest {
                note: Some(Some("maintenance".to_owned())),
            })),
        )
        .await
        .expect("mock 204");
    let captured = server.join().expect("captured request");
    let (_, body) = request_parts(&captured);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(body).expect("JSON body"),
        serde_json::json!({"note": "maintenance"})
    );

    let (url, server) = capture_one();
    HttpClient::new()
        .with_base_url(url)
        .trigger_schedule_v1_workflows_schedules_schedule_id_trigger_post(
            "schedule-a",
            Some(Some(WorkflowScheduleTriggerRequest { overlap: None })),
        )
        .await
        .expect("mock 204");
    let captured = server.join().expect("captured request");
    let (_, body) = request_parts(&captured);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(body).expect("JSON body"),
        serde_json::json!({})
    );
}
