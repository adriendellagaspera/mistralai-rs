#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum SampleType {
    #[default]
    #[serde(rename = "pretrain")]
    Pretrain,
    #[serde(rename = "instruct")]
    Instruct,
    #[serde(rename = "batch_request")]
    BatchRequest,
    #[serde(rename = "batch_result")]
    BatchResult,
    #[serde(rename = "batch_error")]
    BatchError,
}
