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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
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
    ///                 model: None,
    ///                 pages: None,
    ///                 include_image_base64: None,
    ///                 image_limit: None,
    ///                 image_min_size: None,
    ///                 bbox_annotation_format: None,
    ///                 document_annotation_format: None,
    ///                 document_annotation_prompt: None,
    ///                 table_format: None,
    ///                 extract_header: None,
    ///                 extract_footer: None,
    ///                 include_blocks: None,
    ///                 confidence_scores_granularity: None,
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
