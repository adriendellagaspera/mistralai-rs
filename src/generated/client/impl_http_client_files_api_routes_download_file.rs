impl HttpClient {
    /// Download File
    ///
    /// Download a file
    ///
    /// `GET /v1/files/{file_id}/content`
    pub async fn files_api_routes_download_file(
        &self,
        file_id: impl AsRef<str>,
    ) -> Result<bytes::Bytes, ApiOpError<serde_json::Value>> {
        let request_url = format!(
            "{}{}",
            self.base_url,
            format!(
                "/v1/files/{}/content",
                __pct_encode_path_segment(file_id.as_ref())
            )
        );
        let mut req = self.http_client.get(request_url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            if !name.eq_ignore_ascii_case("accept") {
                req = req.header(name, value);
            }
        }
        req = req.header(reqwest::header::ACCEPT, "application/octet-stream");
        let response = req.send().await?;
        let status = response.status();
        let status_code = status.as_u16();
        let headers = response.headers().clone();
        let body_bytes =
            __read_bounded_response_body(response, self.max_response_body_bytes).await?;
        if false || status_code == 200u16 {
            Ok(bytes::Bytes::from(body_bytes))
        } else {
            let raw_body = body_bytes;
            let body_text = String::from_utf8_lossy(&raw_body).into_owned();
            if status.is_success() {
                return Err(ApiOpError::Api(ApiError {
                    status: status_code,
                    headers,
                    body: body_text,
                    raw_body,
                    typed: None,
                    parse_error: Some(format!(
                        "unexpected successful status {}; generated return type selects `{}`",
                        status_code, "200",
                    )),
                }));
            }
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
