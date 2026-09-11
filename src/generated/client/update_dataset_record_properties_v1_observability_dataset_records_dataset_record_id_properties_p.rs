///Typed error responses for `update_dataset_record_properties_v1_observability_dataset_records__dataset_record_id__properties_put`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum UpdateDatasetRecordPropertiesV1ObservabilityDatasetRecordsDatasetRecordIdPropertiesPutApiError
{
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
