impl<'de> serde::Deserialize<'de> for RealtimeTranscriptionClientMessage {
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
                "session.update" => {
                    let primary_error = match serde_json::from_value::<
                        RealtimeTranscriptionSessionUpdateMessage,
                    >(value.clone())
                    {
                        Ok(payload) => {
                            return Ok(Self::RealtimeTranscriptionSessionUpdateMessage(payload));
                        }
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<
                        RealtimeTranscriptionInputAudioAppend,
                    >(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "session.update",
                                first_name,
                                stringify!(RealtimeTranscriptionInputAudioAppend),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionInputAudioAppend(payload),
                            stringify!(RealtimeTranscriptionInputAudioAppend),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<
                        RealtimeTranscriptionInputAudioFlush,
                    >(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "session.update",
                                first_name,
                                stringify!(RealtimeTranscriptionInputAudioFlush),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionInputAudioFlush(payload),
                            stringify!(RealtimeTranscriptionInputAudioFlush),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<RealtimeTranscriptionInputAudioEnd>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "session.update",
                                first_name,
                                stringify!(RealtimeTranscriptionInputAudioEnd),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionInputAudioEnd(payload),
                            stringify!(RealtimeTranscriptionInputAudioEnd),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "input_audio.append" => {
                    let primary_error = match serde_json::from_value::<
                        RealtimeTranscriptionInputAudioAppend,
                    >(value.clone())
                    {
                        Ok(payload) => {
                            return Ok(Self::RealtimeTranscriptionInputAudioAppend(payload));
                        }
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<
                        RealtimeTranscriptionSessionUpdateMessage,
                    >(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio.append",
                                first_name,
                                stringify!(RealtimeTranscriptionSessionUpdateMessage),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionSessionUpdateMessage(payload),
                            stringify!(RealtimeTranscriptionSessionUpdateMessage),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<
                        RealtimeTranscriptionInputAudioFlush,
                    >(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio.append",
                                first_name,
                                stringify!(RealtimeTranscriptionInputAudioFlush),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionInputAudioFlush(payload),
                            stringify!(RealtimeTranscriptionInputAudioFlush),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<RealtimeTranscriptionInputAudioEnd>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio.append",
                                first_name,
                                stringify!(RealtimeTranscriptionInputAudioEnd),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionInputAudioEnd(payload),
                            stringify!(RealtimeTranscriptionInputAudioEnd),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "input_audio.flush" => {
                    let primary_error = match serde_json::from_value::<
                        RealtimeTranscriptionInputAudioFlush,
                    >(value.clone())
                    {
                        Ok(payload) => {
                            return Ok(Self::RealtimeTranscriptionInputAudioFlush(payload));
                        }
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<
                        RealtimeTranscriptionSessionUpdateMessage,
                    >(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio.flush",
                                first_name,
                                stringify!(RealtimeTranscriptionSessionUpdateMessage),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionSessionUpdateMessage(payload),
                            stringify!(RealtimeTranscriptionSessionUpdateMessage),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<
                        RealtimeTranscriptionInputAudioAppend,
                    >(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio.flush",
                                first_name,
                                stringify!(RealtimeTranscriptionInputAudioAppend),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionInputAudioAppend(payload),
                            stringify!(RealtimeTranscriptionInputAudioAppend),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<RealtimeTranscriptionInputAudioEnd>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio.flush",
                                first_name,
                                stringify!(RealtimeTranscriptionInputAudioEnd),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionInputAudioEnd(payload),
                            stringify!(RealtimeTranscriptionInputAudioEnd),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "input_audio.end" => {
                    let primary_error = match serde_json::from_value::<
                        RealtimeTranscriptionInputAudioEnd,
                    >(value.clone())
                    {
                        Ok(payload) => {
                            return Ok(Self::RealtimeTranscriptionInputAudioEnd(payload));
                        }
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<
                        RealtimeTranscriptionSessionUpdateMessage,
                    >(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio.end",
                                first_name,
                                stringify!(RealtimeTranscriptionSessionUpdateMessage),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionSessionUpdateMessage(payload),
                            stringify!(RealtimeTranscriptionSessionUpdateMessage),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<
                        RealtimeTranscriptionInputAudioAppend,
                    >(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio.end",
                                first_name,
                                stringify!(RealtimeTranscriptionInputAudioAppend),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionInputAudioAppend(payload),
                            stringify!(RealtimeTranscriptionInputAudioAppend),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<
                        RealtimeTranscriptionInputAudioFlush,
                    >(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "input_audio.end",
                                first_name,
                                stringify!(RealtimeTranscriptionInputAudioFlush),
                            )));
                        }
                        structural_match = Some((
                            Self::RealtimeTranscriptionInputAudioFlush(payload),
                            stringify!(RealtimeTranscriptionInputAudioFlush),
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
                if let Ok(payload) = serde_json::from_value::<
                    RealtimeTranscriptionSessionUpdateMessage,
                >(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(RealtimeTranscriptionSessionUpdateMessage),
                        )));
                    }
                    structural_match = Some((
                        Self::RealtimeTranscriptionSessionUpdateMessage(payload),
                        stringify!(RealtimeTranscriptionSessionUpdateMessage),
                    ));
                }
                if let Ok(payload) =
                    serde_json::from_value::<RealtimeTranscriptionInputAudioAppend>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(RealtimeTranscriptionInputAudioAppend),
                        )));
                    }
                    structural_match = Some((
                        Self::RealtimeTranscriptionInputAudioAppend(payload),
                        stringify!(RealtimeTranscriptionInputAudioAppend),
                    ));
                }
                if let Ok(payload) =
                    serde_json::from_value::<RealtimeTranscriptionInputAudioFlush>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(RealtimeTranscriptionInputAudioFlush),
                        )));
                    }
                    structural_match = Some((
                        Self::RealtimeTranscriptionInputAudioFlush(payload),
                        stringify!(RealtimeTranscriptionInputAudioFlush),
                    ));
                }
                if let Ok(payload) =
                    serde_json::from_value::<RealtimeTranscriptionInputAudioEnd>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(RealtimeTranscriptionInputAudioEnd),
                        )));
                    }
                    structural_match = Some((
                        Self::RealtimeTranscriptionInputAudioEnd(payload),
                        stringify!(RealtimeTranscriptionInputAudioEnd),
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
