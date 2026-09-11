impl HttpClient {
    /// Get Stream Events
    ///
    /// `GET /v1/workflows/events/stream`
    pub async fn get_stream_events_v1_workflows_events_stream_get(
        &self,
        scope: Option<GetStreamEventsV1WorkflowsEventsStreamGetScope>,
        activity_name: Option<impl AsRef<str>>,
        activity_id: Option<impl AsRef<str>>,
        workflow_name: Option<impl AsRef<str>>,
        workflow_exec_id: Option<impl AsRef<str>>,
        root_workflow_exec_id: Option<impl AsRef<str>>,
        parent_workflow_exec_id: Option<impl AsRef<str>>,
        stream: Option<impl AsRef<str>>,
        start_seq: Option<i64>,
        metadata_filters: Option<impl AsRef<str>>,
        workflow_event_types: Option<impl AsRef<str>>,
        last_event_id: Option<impl AsRef<str>>,
    ) -> Result<
        impl futures_util::Stream<Item = Result<bytes::Bytes, reqwest::Error>>,
        ApiOpError<GetStreamEventsV1WorkflowsEventsStreamGetApiError>,
    > {
        let request_url = format!("{}{}", self.base_url, "/v1/workflows/events/stream");
        let mut req = self.http_client.get(request_url);
        {
            let mut query_params: Vec<(String, String)> = Vec::new();
            if let Some(v) = scope {
                query_params.push(("scope".to_string(), v.to_string()));
            }
            if let Some(v) = activity_name {
                query_params.push(("activity_name".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = activity_id {
                query_params.push(("activity_id".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = workflow_name {
                query_params.push(("workflow_name".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = workflow_exec_id {
                query_params.push(("workflow_exec_id".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = root_workflow_exec_id {
                query_params.push(("root_workflow_exec_id".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = parent_workflow_exec_id {
                query_params.push((
                    "parent_workflow_exec_id".to_string(),
                    v.as_ref().to_string(),
                ));
            }
            if let Some(v) = stream {
                query_params.push(("stream".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = start_seq {
                query_params.push(("start_seq".to_string(), v.to_string()));
            }
            if let Some(v) = metadata_filters {
                query_params.push(("metadata_filters".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = workflow_event_types {
                query_params.push(("workflow_event_types".to_string(), v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(v) = last_event_id {
            req = req.header("last-event-id", v.as_ref());
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            if !name.eq_ignore_ascii_case("accept") {
                req = req.header(name, value);
            }
        }
        req = req.header(reqwest::header::ACCEPT, "text/event-stream");
        let response = req.send().await?;
        let status = response.status();
        let status_code = status.as_u16();
        let headers = response.headers().clone();
        if false || status_code == 200u16 {
            Ok(response.bytes_stream())
        } else {
            if status.is_success() {
                return Err(ApiOpError::Api(ApiError {
                    status: status_code,
                    headers,
                    body: String::new(),
                    raw_body: Vec::new(),
                    typed: None,
                    parse_error: Some(format!(
                        "unexpected successful status {}; generated return type selects `{}`; live response body was not buffered",
                        status_code, "200",
                    )),
                }));
            }
            let body_bytes =
                __read_bounded_response_body(response, self.max_response_body_bytes).await?;
            let raw_body = body_bytes;
            let body_text = String::from_utf8_lossy(&raw_body).into_owned();
            let typed: Option<GetStreamEventsV1WorkflowsEventsStreamGetApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed =
                            Some(GetStreamEventsV1WorkflowsEventsStreamGetApiError::Status422(v));
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                _ => {
                    typed = None;
                    parse_error = None;
                }
            }
            Err(ApiOpError::Api(ApiError {
                status: status_code,
                headers,
                body: body_text,
                raw_body,
                typed,
                parse_error,
            }))
        }
    }
}
