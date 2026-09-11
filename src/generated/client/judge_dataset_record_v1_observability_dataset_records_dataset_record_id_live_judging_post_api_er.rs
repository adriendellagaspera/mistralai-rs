///Typed error responses for `judge_dataset_record_v1_observability_dataset_records__dataset_record_id__live_judging_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum JudgeDatasetRecordV1ObservabilityDatasetRecordsDatasetRecordIdLiveJudgingPostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
