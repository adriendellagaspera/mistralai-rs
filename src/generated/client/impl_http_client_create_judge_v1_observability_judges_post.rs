impl HttpClient {
    /// Create a new judge
    ///
    /// `POST /v1/observability/judges`
    pub async fn create_judge_v1_observability_judges_post(
        &self,
        request: PostJudgeInSchema,
    ) -> Result<JudgePreview, ApiOpError<CreateJudgeV1ObservabilityJudgesPostApiError>> {
        let request_url = format!("{}{}", self.base_url, "/v1/observability/judges");
        let mut req = self.http_client.post(request_url);
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
        req = req.header(reqwest::header::ACCEPT, "application/json");
        let response = req.send().await?;
        let status = response.status();
        let status_code = status.as_u16();
        let headers = response.headers().clone();
        let body_bytes =
            __read_bounded_response_body(response, self.max_response_body_bytes).await?;
        let raw_body = body_bytes;
        let body_text = String::from_utf8_lossy(&raw_body).into_owned();
        if false || status_code == 201u16 {
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
                    status_code, "201",
                )),
            }))
        } else {
            let typed: Option<CreateJudgeV1ObservabilityJudgesPostApiError>;
            let parse_error: Option<String>;
            match status_code {
                400u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(CreateJudgeV1ObservabilityJudgesPostApiError::Status400(v));
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                404u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(CreateJudgeV1ObservabilityJudgesPostApiError::Status404(v));
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                408u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(CreateJudgeV1ObservabilityJudgesPostApiError::Status408(v));
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                409u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(CreateJudgeV1ObservabilityJudgesPostApiError::Status409(v));
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                422u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed = Some(CreateJudgeV1ObservabilityJudgesPostApiError::Status422(v));
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
