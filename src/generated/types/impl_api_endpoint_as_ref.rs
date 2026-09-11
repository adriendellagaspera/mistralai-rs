impl AsRef<str> for ApiEndpoint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
