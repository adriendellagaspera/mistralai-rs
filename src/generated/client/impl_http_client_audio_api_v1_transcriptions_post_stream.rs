impl HttpClient {
    /// Create Streaming Transcription (SSE)
    ///
    /// `POST /v1/audio/transcriptions`
    pub async fn audio_api_v1_transcriptions_post_stream(
        &self,
        request: AudioTranscriptionRequestStream,
    ) -> Result<
        impl futures_util::Stream<Item = Result<bytes::Bytes, reqwest::Error>>,
        ApiOpError<serde_json::Value>,
    > {
        let request_url = format!("{}{}", self.base_url, "/v1/audio/transcriptions");
        let mut req = self.http_client.post(request_url);
        let mut request = request;
        request.stream = Some(true);
        let mut form = reqwest::multipart::Form::new();
        if let Some(value) = &request.context_bias {
            for item in value {
                form = form.text("context_bias", item.to_string());
            }
        }
        if let Some(value) = &request.diarize {
            form = form.text("diarize", value.to_string());
        }
        if let Some(Some(value)) = &request.file {
            form = form.part(
                "file",
                reqwest::multipart::Part::bytes(value.to_vec())
                    .file_name(self.upload_filename.clone()),
            );
        }
        if let Some(Some(value)) = &request.file_id {
            form = form.text("file_id", value.to_string());
        }
        if let Some(Some(value)) = &request.file_url {
            form = form.text("file_url", value.to_string());
        }
        if let Some(Some(value)) = &request.language {
            form = form.text("language", value.to_string());
        }
        let value = &request.model;
        form = form.text("model", value.to_string());
        if let Some(value) = &request.stream {
            form = form.text("stream", value.to_string());
        }
        if let Some(Some(value)) = &request.temperature {
            form = form.text("temperature", value.to_string());
        }
        if let Some(value) = &request.timestamp_granularities {
            for item in value {
                form = form.text("timestamp_granularities", item.to_string());
            }
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
