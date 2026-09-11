impl HttpClient {
    /// Create or update organization credentials for a connector.
    ///
    /// Create or update credentials at the organization level for a given connector.
    ///
    /// `POST /v1/connectors/{connector_id_or_name}/organization/credentials`
    pub async fn connector_create_or_update_organization_credentials_v1(
        &self,
        connector_id_or_name: impl AsRef<str>,
        request: CredentialsCreateOrUpdate,
    ) -> Result<MessageResponse, ApiOpError<ConnectorCreateOrUpdateOrganizationCredentialsV1ApiError>>
    {
        let request_url = format!(
            "{}{}",
            self.base_url,
            format!(
                "/v1/connectors/{}/organization/credentials",
                __pct_encode_path_segment(connector_id_or_name.as_ref())
            )
        );
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
            let typed: Option<ConnectorCreateOrUpdateOrganizationCredentialsV1ApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                            ConnectorCreateOrUpdateOrganizationCredentialsV1ApiError::Status422(v),
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
