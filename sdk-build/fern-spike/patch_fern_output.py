#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path

root = Path(sys.argv[1])

http_client = root / "src/core/http_client.rs"
source = http_client.read_text()

marker = "    /// Execute a multipart/form-data request and return a streaming response (ByteStream).\n"
if marker not in source:
    raise SystemExit("Fern multipart stream insertion marker not found")

method = r'''    /// Experimental spike shim for multipart/form-data requests returning SSE.
    ///
    /// Fern 0.48.0 has separate multipart JSON/binary executors and a JSON-body
    /// SSE executor, but no multipart SSE executor.
    #[cfg(all(feature = "multipart", feature = "sse"))]
    pub async fn execute_multipart_sse_request<T>(
        &self,
        method: Method,
        path: &str,
        form: reqwest::multipart::Form,
        query_params: Option<Vec<(String, String)>>,
        options: Option<RequestOptions>,
        terminator: Option<String>,
    ) -> Result<crate::SseStream<T>, ApiError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let url = join_url(&self.config.base_url, path);
        let mut request = self.client.request(method, &url);

        if let Some(params) = query_params {
            request = request.query(&params);
        }
        if let Some(opts) = &options {
            if !opts.additional_query_params.is_empty() {
                request = request.query(&opts.additional_query_params);
            }
        }

        request = request.multipart(form);
        let mut req = request.build().map_err(ApiError::Network)?;

        let timeout = options
            .as_ref()
            .and_then(|opts| opts.timeout_seconds)
            .map(std::time::Duration::from_secs)
            .unwrap_or(self.config.timeout);

        let response = if let Some(executor) = &self.executor {
            self.apply_custom_headers(&mut req, &options)?;
            req.headers_mut().insert(
                "Accept",
                "text/event-stream"
                    .parse()
                    .map_err(|_| ApiError::InvalidHeader)?,
            );
            req.headers_mut().insert(
                "Cache-Control",
                "no-store".parse().map_err(|_| ApiError::InvalidHeader)?,
            );
            executor.execute(req).await.map_err(ApiError::Executor)?
        } else {
            self.apply_auth_headers(&mut req, &options).await?;
            self.apply_custom_headers(&mut req, &options)?;
            req.headers_mut().insert(
                "Accept",
                "text/event-stream"
                    .parse()
                    .map_err(|_| ApiError::InvalidHeader)?,
            );
            req.headers_mut().insert(
                "Cache-Control",
                "no-store".parse().map_err(|_| ApiError::InvalidHeader)?,
            );
            self.client.execute(req).await.map_err(ApiError::Network)?
        };

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let body = response.text().await.ok();
            return Err(ApiError::from_response(status_code, body.as_deref()));
        }

        crate::SseStream::new(response, terminator, timeout).await
    }

'''

source = source.replace(marker, method + marker, 1)
http_client.write_text(source)

client = root / "src/api/resources/audio_transcriptions/audio_transcriptions.rs"
source = client.read_text()

old = '''            .execute_multipart_request(
                Method::POST,
                "v1/audio/transcriptions#stream",
                request.clone().to_multipart(),
                None,
                options,
            )'''
new = '''            .execute_multipart_sse_request::<TranscriptionStreamEvents>(
                Method::POST,
                "v1/audio/transcriptions#stream",
                request.clone().to_multipart(),
                None,
                options,
                None,
            )'''

if old not in source:
    raise SystemExit("Fern multipart SSE call site not found")
client.write_text(source.replace(old, new, 1))
