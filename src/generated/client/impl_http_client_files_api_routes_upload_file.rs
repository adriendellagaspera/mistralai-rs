impl HttpClient {
    /// Upload File
    ///
    /// Upload a file that can be used across various endpoints.
    ///
    /// The size of individual files can be a maximum of 512 MB. The Fine-tuning API only supports .jsonl files.
    ///
    /// Please contact us if you need to increase these storage limits.
    ///
    /// `POST /v1/files`
    pub async fn files_api_routes_upload_file(
        &self,
        request: FilesApiRoutesUploadFileRequest,
    ) -> Result<UploadFileOut, ApiOpError<serde_json::Value>> {
        let request_url = format!("{}{}", self.base_url, "/v1/files");
        let mut req = self.http_client.post(request_url);
        let mut form = reqwest::multipart::Form::new();
        if let Some(Some(value)) = &request.expiry {
            form = form.text("expiry", value.to_string());
        }
        let value = &request.file;
        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(value.to_vec()).file_name(self.upload_filename.clone()),
        );
        if let Some(value) = &request.purpose {
            form = form.text("purpose", value.to_string());
        }
        if let Some(value) = &request.visibility {
            form = form.text("visibility", value.to_string());
        }
        req = req.multipart(form);
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
