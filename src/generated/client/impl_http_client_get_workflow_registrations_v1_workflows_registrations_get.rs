impl HttpClient {
    /// Get Workflow Registrations
    ///
    /// `GET /v1/workflows/registrations`
    pub async fn get_workflow_registrations_v1_workflows_registrations_get(
        &self,
        workflow_id: Option<impl AsRef<str>>,
        task_queue: Option<impl AsRef<str>>,
        active_only: Option<bool>,
        include_shared: Option<bool>,
        workflow_search: Option<impl AsRef<str>>,
        archived: Option<impl AsRef<str>>,
        with_workflow: Option<bool>,
        available_in_chat_assistant: Option<impl AsRef<str>>,
        limit: Option<i64>,
        cursor: Option<impl AsRef<str>>,
    ) -> Result<
        WorkflowRegistrationListResponse,
        ApiOpError<GetWorkflowRegistrationsV1WorkflowsRegistrationsGetApiError>,
    > {
        let request_url = format!("{}{}", self.base_url, "/v1/workflows/registrations");
        let mut req = self.http_client.get(request_url);
        {
            let mut query_params: Vec<(String, String)> = Vec::new();
            if let Some(v) = workflow_id {
                query_params.push(("workflow_id".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = task_queue {
                query_params.push(("task_queue".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = active_only {
                query_params.push(("active_only".to_string(), v.to_string()));
            }
            if let Some(v) = include_shared {
                query_params.push(("include_shared".to_string(), v.to_string()));
            }
            if let Some(v) = workflow_search {
                query_params.push(("workflow_search".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = archived {
                query_params.push(("archived".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = with_workflow {
                query_params.push(("with_workflow".to_string(), v.to_string()));
            }
            if let Some(v) = available_in_chat_assistant {
                query_params.push((
                    "available_in_chat_assistant".to_string(),
                    v.as_ref().to_string(),
                ));
            }
            if let Some(v) = limit {
                query_params.push(("limit".to_string(), v.to_string()));
            }
            if let Some(v) = cursor {
                query_params.push(("cursor".to_string(), v.as_ref().to_string()));
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
            let typed: Option<GetWorkflowRegistrationsV1WorkflowsRegistrationsGetApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                            GetWorkflowRegistrationsV1WorkflowsRegistrationsGetApiError::Status422(
                                v,
                            ),
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
