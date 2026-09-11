impl HttpClient {
    /// List agent entities, cursor-paginated.
    ///
    /// Retrieve a page of agent entities. Unlike the deprecated `GET /v1/agents`, this endpoint paginates by opaque cursor and honors per-agent sharing, returning only agents the caller is authorized to see.
    ///
    /// `GET /v1/agents/pages`
    pub async fn agents_api_v1_agents_list_pages(
        &self,
        page_size: Option<i64>,
        deployment_chat: Option<impl AsRef<str>>,
        sources: Option<impl AsRef<str>>,
        name: Option<impl AsRef<str>>,
        search: Option<impl AsRef<str>>,
        id: Option<impl AsRef<str>>,
        metadata: Option<impl AsRef<str>>,
        page_token: Option<impl AsRef<str>>,
    ) -> Result<AgentListPage, ApiOpError<AgentsApiV1AgentsListPagesApiError>> {
        let request_url = format!("{}{}", self.base_url, "/v1/agents/pages");
        let mut req = self.http_client.get(request_url);
        {
            let mut query_params: Vec<(String, String)> = Vec::new();
            if let Some(v) = page_size {
                query_params.push(("page_size".to_string(), v.to_string()));
            }
            if let Some(v) = deployment_chat {
                query_params.push(("deployment_chat".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = sources {
                query_params.push(("sources".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = search {
                query_params.push(("search".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = id {
                query_params.push(("id".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = metadata {
                query_params.push(("metadata".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = page_token {
                query_params.push(("page_token".to_string(), v.as_ref().to_string()));
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
            let typed: Option<AgentsApiV1AgentsListPagesApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed = Some(AgentsApiV1AgentsListPagesApiError::Status422(v));
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
