use mistralai_candidate_raw_check::generated::{
    client::HttpClient,
    types::{
        OrganizationMemberCreate,
        UpdateIndexMetricsV1RagDeploymentsDeploymentIdMetricsPutRequest,
        UpdateMetricsRequestDeploymentMetricsOffline,
        UpdateMetricsRequestDeploymentMetricsOfflineStatus,
        UpdateMetricsRequestDeploymentMetricsOnline,
        UpdateMetricsRequestDeploymentMetricsOnlineStatus,
    },
};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

fn capture_one() -> (String, thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test socket");
    let url = format!("http://{}", listener.local_addr().expect("local address"));
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().expect("generated HTTP request");
        socket
            .set_read_timeout(Some(Duration::from_secs(15)))
            .expect("read timeout");
        let mut bytes = Vec::new();
        let mut chunk = [0_u8; 8192];
        let header_end = loop {
            let count = socket.read(&mut chunk).expect("request bytes");
            assert!(count > 0, "request ended before headers");
            bytes.extend_from_slice(&chunk[..count]);
            if let Some(offset) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
                break offset + 4;
            }
        };
        let headers = std::str::from_utf8(&bytes[..header_end]).expect("ASCII headers");
        let length = headers
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().expect("content length"))
            })
            .unwrap_or(0);
        while bytes.len() < header_end + length {
            let count = socket.read(&mut chunk).expect("request body");
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

fn body(bytes: &[u8]) -> serde_json::Value {
    let split = bytes
        .windows(4)
        .position(|part| part == b"\r\n\r\n")
        .expect("header boundary");
    serde_json::from_slice(&bytes[split + 4..]).expect("JSON request body")
}

fn member(email: &str) -> OrganizationMemberCreate {
    OrganizationMemberCreate {
        email: email.to_owned(),
        first_name: "Ada".to_owned(),
        last_name: "Lovelace".to_owned(),
        role: None,
        role_name: None,
        role_names: None,
        roles: None,
        subscription_types: None,
    }
}

#[tokio::test]
async fn required_top_level_array_preserves_empty_and_multiple_items() {
    for (request, expected) in [
        (Vec::<OrganizationMemberCreate>::new(), serde_json::json!([])),
        (
            vec![member("ada@example.test"), member("grace@example.test")],
            serde_json::json!([
                {"email":"ada@example.test","first_name":"Ada","last_name":"Lovelace"},
                {"email":"grace@example.test","first_name":"Ada","last_name":"Lovelace"}
            ]),
        ),
    ] {
        let (url, server) = capture_one();
        HttpClient::new()
            .with_base_url(url)
            .users_api_admin_users_create_users(request)
            .await
            .expect("mock 204");
        let captured = server.join().expect("captured request");
        assert_eq!(body(&captured), expected);
    }
}

#[tokio::test]
async fn required_metrics_union_preserves_both_root_variants() {
    let cases = [
        (
            UpdateIndexMetricsV1RagDeploymentsDeploymentIdMetricsPutRequest::
                UpdateMetricsRequestDeploymentMetricsOnline(
                    UpdateMetricsRequestDeploymentMetricsOnline {
                        document_count: 3,
                        index_metrics: vec![],
                        status: UpdateMetricsRequestDeploymentMetricsOnlineStatus::Online,
                    },
                ),
            serde_json::json!({
                "document_count": 3,
                "index_metrics": [],
                "status": "online"
            }),
        ),
        (
            UpdateIndexMetricsV1RagDeploymentsDeploymentIdMetricsPutRequest::
                UpdateMetricsRequestDeploymentMetricsOffline(
                    UpdateMetricsRequestDeploymentMetricsOffline {
                        clear_metrics: Some(true),
                        status: UpdateMetricsRequestDeploymentMetricsOfflineStatus::Offline,
                    },
                ),
            serde_json::json!({
                "clear_metrics": true,
                "status": "offline"
            }),
        ),
    ];
    for (request, expected) in cases {
        let (url, server) = capture_one();
        HttpClient::new()
            .with_base_url(url)
            .update_index_metrics_v1_rag_deployments_deployment_id_metrics_put(
                "deployment-a",
                request,
            )
            .await
            .expect("mock 204");
        let captured = server.join().expect("captured request");
        assert_eq!(body(&captured), expected);
    }
}
