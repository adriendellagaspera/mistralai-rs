///Specific pages to process. Accepts a list of integers or a string of comma-separated numbers and ranges (e.g. '0,1,2' or '0-5' or '0,2-4'). Page numbers start from 0.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OCRRequestPages {
    String(String),
    OCRRequestPagesArray(OCRRequestPagesArray),
}
