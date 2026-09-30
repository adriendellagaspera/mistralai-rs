use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OcrClient {
    pub http_client: HttpClient,
}

impl OcrClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// OCR
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mistralai_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .ocr
    ///         .process(
    ///             &OcrRequest {
    ///                 document: OcrRequestDocument::File {
    ///                     data: FileChunk {
    ///                         file_id: "file_id".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 bbox_annotation_format: None,
    ///                 confidence_scores_granularity: None,
    ///                 document_annotation_format: None,
    ///                 document_annotation_prompt: None,
    ///                 extract_footer: None,
    ///                 extract_header: None,
    ///                 image_limit: None,
    ///                 image_min_size: None,
    ///                 include_blocks: None,
    ///                 include_image_base64: None,
    ///                 model: None,
    ///                 pages: None,
    ///                 table_format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn process(
        &self,
        request: &OcrRequest,
        options: Option<RequestOptions>,
    ) -> Result<OcrResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ocr",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
