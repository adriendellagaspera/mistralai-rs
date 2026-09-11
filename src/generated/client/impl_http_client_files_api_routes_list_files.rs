impl HttpClient {
    /// List Files
    ///
    /// Returns a list of files that belong to the user's organization.
    ///
    /// `GET /v1/files`
    pub async fn files_api_routes_list_files(
        &self,
        page: Option<i64>,
        page_size: Option<i64>,
        include_total: Option<bool>,
        sample_type: Option<impl AsRef<str>>,
        source: Option<impl AsRef<str>>,
        search: Option<impl AsRef<str>>,
        purpose: Option<impl AsRef<str>>,
        mimetypes: Option<impl AsRef<str>>,
    ) -> Result<ListFilesOut, ApiOpError<serde_json::Value>> {
        let request_url = format!("{}{}", self.base_url, "/v1/files");
        let mut req = self.http_client.get(request_url);
        {
            let mut query_params: Vec<(String, String)> = Vec::new();
            if let Some(v) = page {
                query_params.push(("page".to_string(), v.to_string()));
            }
            if let Some(v) = page_size {
                query_params.push(("page_size".to_string(), v.to_string()));
            }
            if let Some(v) = include_total {
                query_params.push(("include_total".to_string(), v.to_string()));
            }
            if let Some(v) = sample_type {
                query_params.push(("sample_type".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = source {
                query_params.push(("source".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = search {
                query_params.push(("search".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = purpose {
                query_params.push(("purpose".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = mimetypes {
                query_params.push(("mimetypes".to_string(), v.as_ref().to_string()));
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
