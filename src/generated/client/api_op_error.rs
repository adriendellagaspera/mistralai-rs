/// Result error type returned by every generated operation method.
///
/// `Transport` covers failures where we never got an inspectable
/// response (network, timeout, middleware, request-side
/// serialization). `Api` covers any case where the server *did*
/// respond — the envelope always carries status + headers + raw
/// body even when the typed deserialize fails.
#[derive(Debug, Error)]
pub enum ApiOpError<E: std::fmt::Debug> {
    #[error(transparent)]
    Transport(#[from] HttpError),
    #[error(transparent)]
    Api(ApiError<E>),
}
