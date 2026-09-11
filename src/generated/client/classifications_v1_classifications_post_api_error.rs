///Typed error responses for `classifications_v1_classifications_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ClassificationsV1ClassificationsPostApiError {
    Status422(HTTPValidationError),
}
