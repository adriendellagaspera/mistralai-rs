impl HttpClient {
    /// Update a judge
    ///
    /// `PUT /v1/observability/judges/{judge_id}`
    pub async fn update_judge_v1_observability_judges_judge_id_put(
        &self,
        judge_id: impl AsRef<str>,
        request: PutJudgeInSchema,
    ) -> Result<(), ApiOpError<UpdateJudgeV1ObservabilityJudgesJudgeIdPutApiError>> {
        let request_url = format!(
            "{}{}",
            self.base_url,
            format!(
                "/v1/observability/judges/{}",
                __pct_encode_path_segment(judge_id.as_ref())
            )
        );
        let mut req = self.http_client.put(request_url);
        req = req
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        let status_code = status.as_u16();
        let headers = response.headers().clone();
        let body_bytes =
            __read_bounded_response_body(response, self.max_response_body_bytes).await?;
        let raw_body = body_bytes;
        let body_text = String::from_utf8_lossy(&raw_body).into_owned();
        if false || status_code == 204u16 {
            let _ = body_text;
            let _ = raw_body;
            let _ = headers;
            Ok(())
        } else if status.is_success() {
            Err(ApiOpError::Api(ApiError {
                status: status_code,
                headers,
                body: body_text,
                raw_body,
                typed: None,
                parse_error: Some(format!(
                    "unexpected successful status {}; generated return type selects `{}`",
                    status_code, "204",
                )),
            }))
        } else {
            let typed: Option<UpdateJudgeV1ObservabilityJudgesJudgeIdPutApiError>;
            let parse_error: Option<String>;
            match status_code {
                400u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed =
                            Some(UpdateJudgeV1ObservabilityJudgesJudgeIdPutApiError::Status400(v));
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                404u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed =
                            Some(UpdateJudgeV1ObservabilityJudgesJudgeIdPutApiError::Status404(v));
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                408u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed =
                            Some(UpdateJudgeV1ObservabilityJudgesJudgeIdPutApiError::Status408(v));
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                409u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed =
                            Some(UpdateJudgeV1ObservabilityJudgesJudgeIdPutApiError::Status409(v));
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                422u16 => match serde_json::from_str::<ObservabilityError>(&body_text) {
                    Ok(v) => {
                        typed =
                            Some(UpdateJudgeV1ObservabilityJudgesJudgeIdPutApiError::Status422(v));
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
