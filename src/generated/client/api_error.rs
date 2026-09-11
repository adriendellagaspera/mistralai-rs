/// Envelope returned for any HTTP response that we received but
/// couldn't (or didn't) treat as a successful typed result.
///
/// Includes both non-2xx responses and 2xx responses whose body
/// failed to deserialize into the expected success type. `status`,
/// `headers`, and `raw_body` preserve what the server actually sent,
/// while `body` is a convenient lossy UTF-8 rendering. `typed`
/// carries the parsed per-operation error variant
/// when the body matched a declared schema. Formatting the error
/// limits only the displayed body preview; the public fields
/// retain the complete response and parsing details.
#[derive(Debug, Clone)]
pub struct ApiError<E> {
    pub status: u16,
    pub headers: reqwest::header::HeaderMap,
    pub body: String,
    /// Exact response bytes before lossy UTF-8 conversion.
    pub raw_body: Vec<u8>,
    pub typed: Option<E>,
    pub parse_error: Option<String>,
}
