impl HttpClient {
    /// Get voice sample audio
    ///
    /// Get the audio sample for a voice
    ///
    /// `GET /v1/audio/voices/{voice_id}/sample`
    pub async fn get_voice_sample_audio_v1_audio_voices_voice_id_sample_get_wav(
        &self,
        voice_id: impl AsRef<str>,
    ) -> Result<bytes::Bytes, ApiOpError<GetVoiceSampleAudioV1AudioVoicesVoiceIdSampleGetWavApiError>>
    {
        let request_url = format!(
            "{}{}",
            self.base_url,
            format!(
                "/v1/audio/voices/{}/sample",
                __pct_encode_path_segment(voice_id.as_ref())
            )
        );
        let mut req = self.http_client.get(request_url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            if !name.eq_ignore_ascii_case("accept") {
                req = req.header(name, value);
            }
        }
        req = req.header(reqwest::header::ACCEPT, "audio/wav");
        let response = req.send().await?;
        let status = response.status();
        let status_code = status.as_u16();
        let headers = response.headers().clone();
        let body_bytes =
            __read_bounded_response_body(response, self.max_response_body_bytes).await?;
        if false || status_code == 200u16 {
            Ok(bytes::Bytes::from(body_bytes))
        } else {
            let raw_body = body_bytes;
            let body_text = String::from_utf8_lossy(&raw_body).into_owned();
            if status.is_success() {
                return Err(ApiOpError::Api(ApiError {
                    status: status_code,
                    headers,
                    body: body_text,
                    raw_body,
                    typed: None,
                    parse_error: Some(format!(
                        "unexpected successful status {}; generated return type selects `{}`",
                        status_code, "200",
                    )),
                }));
            }
            let typed: Option<GetVoiceSampleAudioV1AudioVoicesVoiceIdSampleGetWavApiError>;
            let parse_error: Option<String>;
            match status_code {
                422u16 => match serde_json::from_str::<HTTPValidationError>(&body_text) {
                    Ok(v) => {
                        typed = Some(
                            GetVoiceSampleAudioV1AudioVoicesVoiceIdSampleGetWavApiError::Status422(
                                v,
                            ),
                        );
                        parse_error = None;
                    }
                    Err(e) => {
                        typed = None;
                        parse_error = Some(e.to_string());
                    }
                },
                _ => {
                    typed = None;
                    parse_error = None;
                }
            }
            Err(ApiOpError::Api(ApiError {
                status: status_code,
                headers,
                body: body_text,
                raw_body,
                typed,
                parse_error,
            }))
        }
    }
}
