impl HttpClient {
    /// Get Fine Tuning Jobs
    ///
    /// Get a list of fine-tuning jobs for your organization and user.
    ///
    /// `GET /v1/fine_tuning/jobs`
    pub async fn jobs_api_routes_fine_tuning_get_fine_tuning_jobs(
        &self,
        page: Option<i64>,
        page_size: Option<i64>,
        model: Option<impl AsRef<str>>,
        created_after: Option<impl AsRef<str>>,
        created_before: Option<impl AsRef<str>>,
        created_by_me: Option<bool>,
        status: Option<impl AsRef<str>>,
        wandb_project: Option<impl AsRef<str>>,
        wandb_name: Option<impl AsRef<str>>,
        suffix: Option<impl AsRef<str>>,
    ) -> Result<JobsOut, ApiOpError<serde_json::Value>> {
        let request_url = format!("{}{}", self.base_url, "/v1/fine_tuning/jobs");
        let mut req = self.http_client.get(request_url);
        {
            let mut query_params: Vec<(String, String)> = Vec::new();
            if let Some(v) = page {
                query_params.push(("page".to_string(), v.to_string()));
            }
            if let Some(v) = page_size {
                query_params.push(("page_size".to_string(), v.to_string()));
            }
            if let Some(v) = model {
                query_params.push(("model".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = created_after {
                query_params.push(("created_after".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = created_before {
                query_params.push(("created_before".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = created_by_me {
                query_params.push(("created_by_me".to_string(), v.to_string()));
            }
            if let Some(v) = status {
                query_params.push(("status".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = wandb_project {
                query_params.push(("wandb_project".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = wandb_name {
                query_params.push(("wandb_name".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = suffix {
                query_params.push(("suffix".to_string(), v.as_ref().to_string()));
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
            let typed: Option<serde_json::Value>;
            let parse_error: Option<String>;
            match status_code {
                _ => match serde_json::from_str::<serde_json::Value>(&body_text) {
                    Ok(v) => {
                        typed = Some(v);
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
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
