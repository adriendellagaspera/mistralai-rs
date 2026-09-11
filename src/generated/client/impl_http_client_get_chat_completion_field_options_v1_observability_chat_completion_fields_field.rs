impl HttpClient {
    /// Get Chat Completion Field Options
    ///
    /// `GET /v1/observability/chat-completion-fields/{field_name}/options`
    pub async fn get_chat_completion_field_options_v1_observability_chat_completion_fields_field_name_options_get(
        &self,
        field_name: impl AsRef<str>,
        operator: GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetOperator,
    ) -> Result<
        ChatCompletionFieldOptions,
        ApiOpError<
            GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetApiError,
        >,
    >{
        let request_url = format!(
            "{}{}",
            self.base_url,
            format!(
                "/v1/observability/chat-completion-fields/{}/options",
                __pct_encode_path_segment(field_name.as_ref())
            )
        );
        let mut req = self.http_client.get(request_url);
        {
            let mut query_params: Vec<(String, String)> = Vec::new();
            query_params.push(("operator".to_string(), operator.to_string()));
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
            let typed: Option<
                GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetApiError,
            >;
            let parse_error: Option<String>;
            match status_code {
                400u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                                GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetApiError::Status400(
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
                404u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                                GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetApiError::Status404(
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
                408u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                                GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetApiError::Status408(
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
                409u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                                GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetApiError::Status409(
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
                422u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                                GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetApiError::Status422(
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
