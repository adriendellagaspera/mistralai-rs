impl<'de> serde::Deserialize<'de> for RetrieveModelV1ModelsModelIdGetResponse {
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
                "base" => {
                    let primary_error = match serde_json::from_value::<BaseModelCard>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::BaseModelCard(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<FTModelCard>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "base",
                                first_name,
                                stringify!(FTModelCard),
                            )));
                        }
                        structural_match =
                            Some((Self::FTModelCard(payload), stringify!(FTModelCard)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "fine-tuned" => {
                    let primary_error = match serde_json::from_value::<FTModelCard>(value.clone()) {
                        Ok(payload) => return Ok(Self::FTModelCard(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<BaseModelCard>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "fine-tuned",
                                first_name,
                                stringify!(BaseModelCard),
                            )));
                        }
                        structural_match =
                            Some((Self::BaseModelCard(payload), stringify!(BaseModelCard)));
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
                if let Ok(payload) = serde_json::from_value::<BaseModelCard>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(BaseModelCard),
                        )));
                    }
                    structural_match =
                        Some((Self::BaseModelCard(payload), stringify!(BaseModelCard)));
                }
                if let Ok(payload) = serde_json::from_value::<FTModelCard>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(FTModelCard),
                        )));
                    }
                    structural_match = Some((Self::FTModelCard(payload), stringify!(FTModelCard)));
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
