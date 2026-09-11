impl<'de> serde::Deserialize<'de> for SystemMessageContentChunks {
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
