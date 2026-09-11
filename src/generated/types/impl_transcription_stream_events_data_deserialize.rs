impl<'de> serde::Deserialize<'de> for TranscriptionStreamEventsData {
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
                "transcription.text.delta" => {
                    let primary_error =
                        match serde_json::from_value::<TranscriptionStreamTextDelta>(value.clone())
                        {
                            Ok(payload) => {
                                return Ok(Self::TranscriptionStreamTextDelta(payload));
                            }
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamLanguage>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.text.delta",
                                first_name,
                                stringify!(TranscriptionStreamLanguage),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamLanguage(payload),
                            stringify!(TranscriptionStreamLanguage),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamSegmentDelta>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.text.delta",
                                first_name,
                                stringify!(TranscriptionStreamSegmentDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamSegmentDelta(payload),
                            stringify!(TranscriptionStreamSegmentDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamDone>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.text.delta",
                                first_name,
                                stringify!(TranscriptionStreamDone),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamDone(payload),
                            stringify!(TranscriptionStreamDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "transcription.language" => {
                    let primary_error = match serde_json::from_value::<TranscriptionStreamLanguage>(
                        value.clone(),
                    ) {
                        Ok(payload) => {
                            return Ok(Self::TranscriptionStreamLanguage(payload));
                        }
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamTextDelta>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.language",
                                first_name,
                                stringify!(TranscriptionStreamTextDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamTextDelta(payload),
                            stringify!(TranscriptionStreamTextDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamSegmentDelta>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.language",
                                first_name,
                                stringify!(TranscriptionStreamSegmentDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamSegmentDelta(payload),
                            stringify!(TranscriptionStreamSegmentDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamDone>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.language",
                                first_name,
                                stringify!(TranscriptionStreamDone),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamDone(payload),
                            stringify!(TranscriptionStreamDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "transcription.segment" => {
                    let primary_error = match serde_json::from_value::<
                        TranscriptionStreamSegmentDelta,
                    >(value.clone())
                    {
                        Ok(payload) => {
                            return Ok(Self::TranscriptionStreamSegmentDelta(payload));
                        }
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamTextDelta>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.segment",
                                first_name,
                                stringify!(TranscriptionStreamTextDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamTextDelta(payload),
                            stringify!(TranscriptionStreamTextDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamLanguage>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.segment",
                                first_name,
                                stringify!(TranscriptionStreamLanguage),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamLanguage(payload),
                            stringify!(TranscriptionStreamLanguage),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamDone>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.segment",
                                first_name,
                                stringify!(TranscriptionStreamDone),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamDone(payload),
                            stringify!(TranscriptionStreamDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "transcription.done" => {
                    let primary_error =
                        match serde_json::from_value::<TranscriptionStreamDone>(value.clone()) {
                            Ok(payload) => {
                                return Ok(Self::TranscriptionStreamDone(payload));
                            }
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamTextDelta>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.done",
                                first_name,
                                stringify!(TranscriptionStreamTextDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamTextDelta(payload),
                            stringify!(TranscriptionStreamTextDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamLanguage>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.done",
                                first_name,
                                stringify!(TranscriptionStreamLanguage),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamLanguage(payload),
                            stringify!(TranscriptionStreamLanguage),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<TranscriptionStreamSegmentDelta>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "transcription.done",
                                first_name,
                                stringify!(TranscriptionStreamSegmentDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::TranscriptionStreamSegmentDelta(payload),
                            stringify!(TranscriptionStreamSegmentDelta),
                        ));
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
                if let Ok(payload) =
                    serde_json::from_value::<TranscriptionStreamTextDelta>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(TranscriptionStreamTextDelta),
                        )));
                    }
                    structural_match = Some((
                        Self::TranscriptionStreamTextDelta(payload),
                        stringify!(TranscriptionStreamTextDelta),
                    ));
                }
                if let Ok(payload) =
                    serde_json::from_value::<TranscriptionStreamLanguage>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(TranscriptionStreamLanguage),
                        )));
                    }
                    structural_match = Some((
                        Self::TranscriptionStreamLanguage(payload),
                        stringify!(TranscriptionStreamLanguage),
                    ));
                }
                if let Ok(payload) =
                    serde_json::from_value::<TranscriptionStreamSegmentDelta>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(TranscriptionStreamSegmentDelta),
                        )));
                    }
                    structural_match = Some((
                        Self::TranscriptionStreamSegmentDelta(payload),
                        stringify!(TranscriptionStreamSegmentDelta),
                    ));
                }
                if let Ok(payload) =
                    serde_json::from_value::<TranscriptionStreamDone>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(TranscriptionStreamDone),
                        )));
                    }
                    structural_match = Some((
                        Self::TranscriptionStreamDone(payload),
                        stringify!(TranscriptionStreamDone),
                    ));
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
