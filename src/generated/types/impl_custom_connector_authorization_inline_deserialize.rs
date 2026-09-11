impl<'de> serde::Deserialize<'de> for CustomConnectorAuthorizationInline {
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
                "oauth2-token" => {
                    let primary_error =
                        match serde_json::from_value::<OAuth2TokenAuth>(value.clone()) {
                            Ok(payload) => return Ok(Self::OAuth2TokenAuth(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<APIKeyAuth>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "oauth2-token",
                                first_name,
                                stringify!(APIKeyAuth),
                            )));
                        }
                        structural_match =
                            Some((Self::APIKeyAuth(payload), stringify!(APIKeyAuth)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "api-key" => {
                    let primary_error = match serde_json::from_value::<APIKeyAuth>(value.clone()) {
                        Ok(payload) => return Ok(Self::APIKeyAuth(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OAuth2TokenAuth>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "api-key",
                                first_name,
                                stringify!(OAuth2TokenAuth),
                            )));
                        }
                        structural_match =
                            Some((Self::OAuth2TokenAuth(payload), stringify!(OAuth2TokenAuth)));
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
                if let Ok(payload) = serde_json::from_value::<OAuth2TokenAuth>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OAuth2TokenAuth),
                        )));
                    }
                    structural_match =
                        Some((Self::OAuth2TokenAuth(payload), stringify!(OAuth2TokenAuth)));
                }
                if let Ok(payload) = serde_json::from_value::<APIKeyAuth>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(APIKeyAuth),
                        )));
                    }
                    structural_match = Some((Self::APIKeyAuth(payload), stringify!(APIKeyAuth)));
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
