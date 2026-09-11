impl HttpClient {
    /// Update an agent version.
    ///
    /// Switch the version of an agent.
    ///
    /// `PATCH /v1/agents/{agent_id}/version`
    pub async fn agents_api_v1_agents_update_version(
        &self,
        agent_id: impl AsRef<str>,
        version: i64,
    ) -> Result<Agent, ApiOpError<AgentsApiV1AgentsUpdateVersionApiError>> {
        let request_url = format!(
            "{}{}",
            self.base_url,
            format!(
                "/v1/agents/{}/version",
                __pct_encode_path_segment(agent_id.as_ref())
            )
        );
        let mut req = self.http_client.patch(request_url);
        req = req.header(reqwest::header::CONTENT_LENGTH, "0");
        {
            let mut query_params: Vec<(String, String)> = Vec::new();
            query_params.push(("version".to_string(), version.to_string()));
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
        req = req.header(reqwest::header::ACCEPT, "application/json");
        let response = req.send().await?;
        let status = response.status();
        let status_code = status.as_u16();
        let headers = response.headers().clone();
        let body_bytes =
            __read_bounded_response_body(response, self.max_response_body_bytes).await?;
        let raw_body = body_bytes;
        let body_text = String::from_utf8_lossy(&raw_body).into_owned();
        if false || status_code == 200u16 {
            match serde_json::from_str(&body_text) {
                Ok(body) => Ok(body),
                Err(e) => Err(ApiOpError::Api(ApiError {
                    status: status_code,
                    headers: headers,
                    body: body_text,
                    raw_body,
                    typed: None,
                    parse_error: Some(format!("failed to deserialize 2xx response body: {}", e)),
                })),
            }
        } else if status.is_success() {
            Err(ApiOpError::Api(ApiError {
                status: status_code,
                headers,
                body: body_text,
                raw_body,
                typed: None,
                parse_error: Some(format!(
                    "unexpected successful status {}; generated return type selects `{}`",
                    status_code, "200",
                )),
            }))
        } else {
            let typed: Option<AgentsApiV1AgentsUpdateVersionApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed = Some(AgentsApiV1AgentsUpdateVersionApiError::Status422(v));
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
