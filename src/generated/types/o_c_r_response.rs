#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRResponse {
    ///Formatted response in the request_format if provided in json str
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub document_annotation: Option<Option<String>>,
    ///The model used to generate the OCR.
    pub model: String,
    ///List of OCR info for pages.
    pub pages: Vec<OCRPageObject>,
    pub usage_info: OCRUsageInfo,
}
