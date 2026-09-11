impl<'de> serde::Deserialize<'de> for SpeechStreamEventsData {
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
                "speech.audio.delta" => {
                    let primary_error =
                        match serde_json::from_value::<SpeechStreamAudioDelta>(value.clone()) {
                            Ok(payload) => {
                                return Ok(Self::SpeechStreamAudioDelta(payload));
                            }
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<SpeechStreamDone>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "speech.audio.delta",
                                first_name,
                                stringify!(SpeechStreamDone),
                            )));
                        }
                        structural_match = Some((
                            Self::SpeechStreamDone(payload),
                            stringify!(SpeechStreamDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "speech.audio.done" => {
                    let primary_error =
                        match serde_json::from_value::<SpeechStreamDone>(value.clone()) {
                            Ok(payload) => return Ok(Self::SpeechStreamDone(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<SpeechStreamAudioDelta>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "speech.audio.done",
                                first_name,
                                stringify!(SpeechStreamAudioDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::SpeechStreamAudioDelta(payload),
                            stringify!(SpeechStreamAudioDelta),
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
                if let Ok(payload) = serde_json::from_value::<SpeechStreamAudioDelta>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(SpeechStreamAudioDelta),
                        )));
                    }
                    structural_match = Some((
                        Self::SpeechStreamAudioDelta(payload),
                        stringify!(SpeechStreamAudioDelta),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<SpeechStreamDone>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(SpeechStreamDone),
                        )));
                    }
                    structural_match = Some((
                        Self::SpeechStreamDone(payload),
                        stringify!(SpeechStreamDone),
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
