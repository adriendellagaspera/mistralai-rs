/// Result alias for transport-only error paths (e.g. helpers that
/// don't have a per-operation error type). Generated operation
/// methods use [`ApiOpError`] directly.
pub type HttpResult<T> = Result<T, HttpError>;
