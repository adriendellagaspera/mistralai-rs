///Typed error responses for `get_dataset_import_tasks_v1_observability_datasets__dataset_id__tasks_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
