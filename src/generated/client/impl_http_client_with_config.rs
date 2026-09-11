impl HttpClient {
    /// Create a new HTTP client with custom configuration
    pub fn with_config(retry_config: Option<RetryConfig>, enable_tracing: bool) -> Self {
        let reqwest_client = reqwest::Client::new();
        let mut client_builder = ClientBuilder::new(reqwest_client);
        if enable_tracing {
            use reqwest_tracing::TracingMiddleware;
            client_builder = client_builder.with(TracingMiddleware::default());
        }
        if let Some(config) = retry_config {
            use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
            let retry_policy = ExponentialBackoff::builder()
                .retry_bounds(
                    std::time::Duration::from_millis(config.initial_delay_ms),
                    std::time::Duration::from_millis(config.max_delay_ms),
                )
                .build_with_max_retries(config.max_retries);
            let retry_middleware = RetryTransientMiddleware::new_with_policy(retry_policy);
            client_builder = client_builder.with(retry_middleware);
        }
        let http_client = client_builder.build();
        Self {
            base_url: "https://api.mistral.ai".to_string(),
            api_key: None,
            http_client,
            custom_headers: BTreeMap::new(),
            upload_filename: "upload".to_string(),
            max_response_body_bytes: 8388608usize,
        }
    }
}
