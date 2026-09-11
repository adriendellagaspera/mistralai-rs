impl<E: std::fmt::Debug> std::fmt::Display for ApiError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "API error {}: {}",
            self.status,
            display_api_error_body(&self.body)
        )?;
        if let Some(typed) = &self.typed {
            write!(f, "; typed: {typed:?}")?;
        }
        if let Some(parse_error) = &self.parse_error {
            write!(f, "; parse error: {parse_error}")?;
        }
        Ok(())
    }
}
