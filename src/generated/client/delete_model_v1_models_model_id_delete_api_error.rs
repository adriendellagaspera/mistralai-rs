///Typed error responses for `delete_model_v1_models__model_id__delete`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum DeleteModelV1ModelsModelIdDeleteApiError {
    Status422(HTTPValidationError),
}
