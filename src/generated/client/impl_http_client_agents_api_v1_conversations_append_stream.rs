impl HttpClient {
    /// Append new entries to an existing conversation.
    ///
    /// Run completion on the history of the conversation and the user entries. Return the new created entries.
    ///
    /// `POST /v1/conversations/{conversation_id}`
    pub async fn agents_api_v1_conversations_append_stream(
        &self,
        conversation_id: impl AsRef<str>,
        request: ConversationAppendStreamRequest,
    ) -> Result<
        impl futures_util::Stream<Item = Result<bytes::Bytes, reqwest::Error>>,
        ApiOpError<AgentsApiV1ConversationsAppendStreamApiError>,
    > {
        let request_url = format!(
            "{}{}",
            self.base_url,
            format!(
                "/v1/conversations/{}",
                __pct_encode_path_segment(conversation_id.as_ref())
            )
        );
        let mut req = self.http_client.post(request_url);
        let mut request = request;
        request.stream = Some(true);
        req = req
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
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
            let typed: Option<AgentsApiV1ConversationsAppendStreamApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed = Some(AgentsApiV1ConversationsAppendStreamApiError::Status422(v));
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
