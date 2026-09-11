fn display_api_error_body(body: &str) -> std::borrow::Cow<'_, str> {
    let Some((end, _)) = body.char_indices().nth(API_ERROR_BODY_DISPLAY_LIMIT) else {
        return std::borrow::Cow::Borrowed(body);
    };
    let mut displayed = String::with_capacity(end + API_ERROR_BODY_TRUNCATION_MARKER.len());
    displayed.push_str(&body[..end]);
    displayed.push_str(API_ERROR_BODY_TRUNCATION_MARKER);
    std::borrow::Cow::Owned(displayed)
}
