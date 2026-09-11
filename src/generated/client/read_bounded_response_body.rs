async fn __read_bounded_response_body(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<Vec<u8>, HttpError> {
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(HttpError::Network)? {
        let next_len = body.len().checked_add(chunk.len());
        if next_len.is_none_or(|next_len| next_len > limit) {
            return Err(HttpError::ResponseTooLarge { limit });
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}
