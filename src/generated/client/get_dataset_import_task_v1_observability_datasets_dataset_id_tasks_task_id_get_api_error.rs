///Typed error responses for `get_dataset_import_task_v1_observability_datasets__dataset_id__tasks__task_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetDatasetImportTaskV1ObservabilityDatasetsDatasetIdTasksTaskIdGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
