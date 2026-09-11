///Typed error responses for `list_models_v1_models_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ListModelsV1ModelsGetApiError {
    Status422(HTTPValidationError),
}
