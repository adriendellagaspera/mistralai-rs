impl HttpClient {
    /// Stream
    ///
    /// `GET /v1/workflows/executions/{execution_id}/stream`
    pub async fn stream_v1_workflows_executions_execution_id_stream_get(
        &self,
        execution_id: impl AsRef<str>,
        event_source: Option<impl AsRef<str>>,
        last_event_id: Option<impl AsRef<str>>,
    ) -> Result<
        impl futures_util::Stream<Item = Result<bytes::Bytes, reqwest::Error>>,
        ApiOpError<StreamV1WorkflowsExecutionsExecutionIdStreamGetApiError>,
    > {
        let request_url = format!(
            "{}{}",
            self.base_url,
            format!(
                "/v1/workflows/executions/{}/stream",
                __pct_encode_path_segment(execution_id.as_ref())
            )
        );
        let mut req = self.http_client.get(request_url);
        {
            let mut query_params: Vec<(String, String)> = Vec::new();
            if let Some(v) = event_source {
                query_params.push(("event_source".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = last_event_id {
                query_params.push(("last_event_id".to_string(), v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
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
            let typed: Option<StreamV1WorkflowsExecutionsExecutionIdStreamGetApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                            StreamV1WorkflowsExecutionsExecutionIdStreamGetApiError::Status422(v),
                        );
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
