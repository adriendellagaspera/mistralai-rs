///Typed error responses for `export_dataset_to_jsonl_v1_observability_datasets__dataset_id__exports_to_jsonl_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ExportDatasetToJsonlV1ObservabilityDatasetsDatasetIdExportsToJsonlGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
