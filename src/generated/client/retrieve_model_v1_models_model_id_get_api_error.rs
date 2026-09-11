///Typed error responses for `retrieve_model_v1_models__model_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum RetrieveModelV1ModelsModelIdGetApiError {
    Status422(HTTPValidationError),
}
