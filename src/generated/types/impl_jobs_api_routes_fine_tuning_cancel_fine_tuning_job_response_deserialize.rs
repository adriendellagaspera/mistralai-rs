impl<'de> serde::Deserialize<'de> for JobsApiRoutesFineTuningCancelFineTuningJobResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let discriminator = match value.get("job_type") {
            Some(serde_json::Value::String(discriminator)) => Some(discriminator.as_str()),
            Some(_) => {
                return Err(serde::de::Error::custom(concat!(
                    "non-string discriminator `",
                    "job_type",
                    "`",
                )));
            }
            None => None,
        };
        match discriminator {
            Some(discriminator) => match discriminator {
                "completion" => {
                    let primary_error =
                        match serde_json::from_value::<CompletionDetailedJobOut>(value.clone()) {
                            Ok(payload) => {
                                return Ok(Self::CompletionDetailedJobOut(payload));
                            }
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ClassifierDetailedJobOut>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "job_type",
                                "completion",
                                first_name,
                                stringify!(ClassifierDetailedJobOut),
                            )));
                        }
                        structural_match = Some((
                            Self::ClassifierDetailedJobOut(payload),
                            stringify!(ClassifierDetailedJobOut),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "classifier" => {
                    let primary_error =
                        match serde_json::from_value::<ClassifierDetailedJobOut>(value.clone()) {
                            Ok(payload) => {
                                return Ok(Self::ClassifierDetailedJobOut(payload));
                            }
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<CompletionDetailedJobOut>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "job_type",
                                "classifier",
                                first_name,
                                stringify!(CompletionDetailedJobOut),
                            )));
                        }
                        structural_match = Some((
                            Self::CompletionDetailedJobOut(payload),
                            stringify!(CompletionDetailedJobOut),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown discriminator value `{other}` for `{}`",
                    "job_type",
                ))),
            },
            None => {
                let mut structural_match: Option<(Self, &'static str)> = None;
                if let Ok(payload) =
                    serde_json::from_value::<CompletionDetailedJobOut>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "job_type",
                            first_name,
                            stringify!(CompletionDetailedJobOut),
                        )));
                    }
                    structural_match = Some((
                        Self::CompletionDetailedJobOut(payload),
                        stringify!(CompletionDetailedJobOut),
                    ));
                }
                if let Ok(payload) =
                    serde_json::from_value::<ClassifierDetailedJobOut>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "job_type",
                            first_name,
                            stringify!(ClassifierDetailedJobOut),
                        )));
                    }
                    structural_match = Some((
                        Self::ClassifierDetailedJobOut(payload),
                        stringify!(ClassifierDetailedJobOut),
                    ));
                }
                structural_match.map(|(payload, _)| payload).ok_or_else(|| {
                    serde::de::Error::custom(concat!(
                        "missing string discriminator `",
                        "job_type",
                        "` and no tagless branch matched",
                    ))
                })
            }
        }
    }
}
