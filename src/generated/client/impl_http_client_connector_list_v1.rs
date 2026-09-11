impl HttpClient {
    /// List all connectors.
    ///
    /// List all your custom connectors with keyset pagination and filters.
    ///
    /// `GET /v1/connectors`
    pub async fn connector_list_v1(
        &self,
        query_filters: Option<ConnectorsQueryFilters>,
        cursor: Option<impl AsRef<str>>,
        page_size: Option<i64>,
    ) -> Result<PaginatedConnectors, ApiOpError<ConnectorListV1ApiError>> {
        let request_url = format!("{}{}", self.base_url, "/v1/connectors");
        let mut req = self.http_client.get(request_url);
        {
            let mut query_params: Vec<(String, String)> = Vec::new();
            if let Some(v) = cursor {
                query_params.push(("cursor".to_string(), v.as_ref().to_string()));
            }
            if let Some(v) = page_size {
                query_params.push(("page_size".to_string(), v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(v) = query_filters {
            let __empty = match serde_json::to_value(&v).map_err(HttpError::serialization_error)? {
                serde_json::Value::Object(map) => map.is_empty(),
                _ => false,
            };
            if __empty {
                req = req.query(&[(format!("{}[]", "query_filters"), String::new())]);
            } else {
                req = req.query(&v);
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
            let typed: Option<ConnectorListV1ApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed = Some(ConnectorListV1ApiError::Status422(v));
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
