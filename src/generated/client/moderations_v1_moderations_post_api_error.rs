///Typed error responses for `moderations_v1_moderations_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ModerationsV1ModerationsPostApiError {
    Status422(HTTPValidationError),
}
