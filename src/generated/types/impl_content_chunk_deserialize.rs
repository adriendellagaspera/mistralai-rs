impl<'de> serde::Deserialize<'de> for ContentChunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let discriminator = match value.get("type") {
            Some(serde_json::Value::String(discriminator)) => Some(discriminator.as_str()),
            Some(_) => {
                return Err(serde::de::Error::custom(concat!(
                    "non-string discriminator `",
                    "type",
                    "`",
                )));
            }
            None => None,
        };
        match discriminator {
            Some(discriminator) => match discriminator {
                "text" => {
                    let primary_error = match serde_json::from_value::<TextChunk>(value.clone()) {
                        Ok(payload) => return Ok(Self::TextChunk(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(ImageURLChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ImageURLChunk(payload), stringify!(ImageURLChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(DocumentURLChunk),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentURLChunk(payload),
                            stringify!(DocumentURLChunk),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(ReferenceChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ReferenceChunk(payload), stringify!(ReferenceChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(FileChunk),
                            )));
                        }
                        structural_match = Some((Self::FileChunk(payload), stringify!(FileChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ThinkChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(ThinkChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ThinkChunk(payload), stringify!(ThinkChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(AudioChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::AudioChunk(payload), stringify!(AudioChunk)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "image_url" => {
                    let primary_error = match serde_json::from_value::<ImageURLChunk>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::ImageURLChunk(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_url",
                                first_name,
                                stringify!(TextChunk),
                            )));
                        }
                        structural_match = Some((Self::TextChunk(payload), stringify!(TextChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_url",
                                first_name,
                                stringify!(DocumentURLChunk),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentURLChunk(payload),
                            stringify!(DocumentURLChunk),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_url",
                                first_name,
                                stringify!(ReferenceChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ReferenceChunk(payload), stringify!(ReferenceChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_url",
                                first_name,
                                stringify!(FileChunk),
                            )));
                        }
                        structural_match = Some((Self::FileChunk(payload), stringify!(FileChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ThinkChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_url",
                                first_name,
                                stringify!(ThinkChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ThinkChunk(payload), stringify!(ThinkChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_url",
                                first_name,
                                stringify!(AudioChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::AudioChunk(payload), stringify!(AudioChunk)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "document_url" => {
                    let primary_error =
                        match serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                            Ok(payload) => return Ok(Self::DocumentURLChunk(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_url",
                                first_name,
                                stringify!(TextChunk),
                            )));
                        }
                        structural_match = Some((Self::TextChunk(payload), stringify!(TextChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_url",
                                first_name,
                                stringify!(ImageURLChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ImageURLChunk(payload), stringify!(ImageURLChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_url",
                                first_name,
                                stringify!(ReferenceChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ReferenceChunk(payload), stringify!(ReferenceChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_url",
                                first_name,
                                stringify!(FileChunk),
                            )));
                        }
                        structural_match = Some((Self::FileChunk(payload), stringify!(FileChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ThinkChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_url",
                                first_name,
                                stringify!(ThinkChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ThinkChunk(payload), stringify!(ThinkChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_url",
                                first_name,
                                stringify!(AudioChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::AudioChunk(payload), stringify!(AudioChunk)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "reference" => {
                    let primary_error =
                        match serde_json::from_value::<ReferenceChunk>(value.clone()) {
                            Ok(payload) => return Ok(Self::ReferenceChunk(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "reference",
                                first_name,
                                stringify!(TextChunk),
                            )));
                        }
                        structural_match = Some((Self::TextChunk(payload), stringify!(TextChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "reference",
                                first_name,
                                stringify!(ImageURLChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ImageURLChunk(payload), stringify!(ImageURLChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "reference",
                                first_name,
                                stringify!(DocumentURLChunk),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentURLChunk(payload),
                            stringify!(DocumentURLChunk),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "reference",
                                first_name,
                                stringify!(FileChunk),
                            )));
                        }
                        structural_match = Some((Self::FileChunk(payload), stringify!(FileChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ThinkChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "reference",
                                first_name,
                                stringify!(ThinkChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ThinkChunk(payload), stringify!(ThinkChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "reference",
                                first_name,
                                stringify!(AudioChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::AudioChunk(payload), stringify!(AudioChunk)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "file" => {
                    let primary_error = match serde_json::from_value::<FileChunk>(value.clone()) {
                        Ok(payload) => return Ok(Self::FileChunk(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "file",
                                first_name,
                                stringify!(TextChunk),
                            )));
                        }
                        structural_match = Some((Self::TextChunk(payload), stringify!(TextChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "file",
                                first_name,
                                stringify!(ImageURLChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ImageURLChunk(payload), stringify!(ImageURLChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "file",
                                first_name,
                                stringify!(DocumentURLChunk),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentURLChunk(payload),
                            stringify!(DocumentURLChunk),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "file",
                                first_name,
                                stringify!(ReferenceChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ReferenceChunk(payload), stringify!(ReferenceChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ThinkChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "file",
                                first_name,
                                stringify!(ThinkChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ThinkChunk(payload), stringify!(ThinkChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "file",
                                first_name,
                                stringify!(AudioChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::AudioChunk(payload), stringify!(AudioChunk)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "thinking" => {
                    let primary_error = match serde_json::from_value::<ThinkChunk>(value.clone()) {
                        Ok(payload) => return Ok(Self::ThinkChunk(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "thinking",
                                first_name,
                                stringify!(TextChunk),
                            )));
                        }
                        structural_match = Some((Self::TextChunk(payload), stringify!(TextChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "thinking",
                                first_name,
                                stringify!(ImageURLChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ImageURLChunk(payload), stringify!(ImageURLChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "thinking",
                                first_name,
                                stringify!(DocumentURLChunk),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentURLChunk(payload),
                            stringify!(DocumentURLChunk),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "thinking",
                                first_name,
                                stringify!(ReferenceChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ReferenceChunk(payload), stringify!(ReferenceChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "thinking",
                                first_name,
                                stringify!(FileChunk),
                            )));
                        }
                        structural_match = Some((Self::FileChunk(payload), stringify!(FileChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "thinking",
                                first_name,
                                stringify!(AudioChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::AudioChunk(payload), stringify!(AudioChunk)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "input_audio" => {
                    let primary_error = match serde_json::from_value::<AudioChunk>(value.clone()) {
                        Ok(payload) => return Ok(Self::AudioChunk(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio",
                                first_name,
                                stringify!(TextChunk),
                            )));
                        }
                        structural_match = Some((Self::TextChunk(payload), stringify!(TextChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio",
                                first_name,
                                stringify!(ImageURLChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ImageURLChunk(payload), stringify!(ImageURLChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio",
                                first_name,
                                stringify!(DocumentURLChunk),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentURLChunk(payload),
                            stringify!(DocumentURLChunk),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio",
                                first_name,
                                stringify!(ReferenceChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ReferenceChunk(payload), stringify!(ReferenceChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio",
                                first_name,
                                stringify!(FileChunk),
                            )));
                        }
                        structural_match = Some((Self::FileChunk(payload), stringify!(FileChunk)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ThinkChunk>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio",
                                first_name,
                                stringify!(ThinkChunk),
                            )));
                        }
                        structural_match =
                            Some((Self::ThinkChunk(payload), stringify!(ThinkChunk)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown discriminator value `{other}` for `{}`",
                    "type",
                ))),
            },
            None => {
                let mut structural_match: Option<(Self, &'static str)> = None;
                if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(TextChunk),
                        )));
                    }
                    structural_match = Some((Self::TextChunk(payload), stringify!(TextChunk)));
                }
                if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(ImageURLChunk),
                        )));
                    }
                    structural_match =
                        Some((Self::ImageURLChunk(payload), stringify!(ImageURLChunk)));
                }
                if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(DocumentURLChunk),
                        )));
                    }
                    structural_match = Some((
                        Self::DocumentURLChunk(payload),
                        stringify!(DocumentURLChunk),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(ReferenceChunk),
                        )));
                    }
                    structural_match =
                        Some((Self::ReferenceChunk(payload), stringify!(ReferenceChunk)));
                }
                if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(FileChunk),
                        )));
                    }
                    structural_match = Some((Self::FileChunk(payload), stringify!(FileChunk)));
                }
                if let Ok(payload) = serde_json::from_value::<ThinkChunk>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(ThinkChunk),
                        )));
                    }
                    structural_match = Some((Self::ThinkChunk(payload), stringify!(ThinkChunk)));
                }
                if let Ok(payload) = serde_json::from_value::<AudioChunk>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(AudioChunk),
                        )));
                    }
                    structural_match = Some((Self::AudioChunk(payload), stringify!(AudioChunk)));
                }
                structural_match.map(|(payload, _)| payload).ok_or_else(|| {
                    serde::de::Error::custom(concat!(
                        "missing string discriminator `",
                        "type",
                        "` and no tagless branch matched",
                    ))
                })
            }
        }
    }
}
