impl HttpClient {
    /// Set the API key for authentication
    pub fn with_upload_filename(mut self, filename: impl Into<String>) -> Self {
        self.upload_filename = filename.into();
        self
    }
}
