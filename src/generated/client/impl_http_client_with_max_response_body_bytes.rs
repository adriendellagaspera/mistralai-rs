impl HttpClient {
    /// Set the maximum number of response-body bytes buffered in memory.
    pub fn with_max_response_body_bytes(mut self, limit: usize) -> Self {
        self.max_response_body_bytes = limit;
        self
    }
}
