impl HttpClient {
    /// Unschedule Workflow
    ///
    /// `DELETE /v1/workflows/schedules/{schedule_id}`
    pub async fn unschedule_workflow_v1_workflows_schedules_schedule_id_delete(
        &self,
        schedule_id: impl AsRef<str>,
    ) -> Result<(), ApiOpError<UnscheduleWorkflowV1WorkflowsSchedulesScheduleIdDeleteApiError>>
    {
        let request_url = format!(
            "{}{}",
            self.base_url,
            format!(
                "/v1/workflows/schedules/{}",
                __pct_encode_path_segment(schedule_id.as_ref())
            )
        );
        let mut req = self.http_client.delete(request_url);
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
            let typed: Option<UnscheduleWorkflowV1WorkflowsSchedulesScheduleIdDeleteApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                                UnscheduleWorkflowV1WorkflowsSchedulesScheduleIdDeleteApiError::Status422(
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
