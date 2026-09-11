impl<'de> serde::Deserialize<'de> for CustomTaskInProgressAttributesResponsePayload {
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
                "json" => {
                    let primary_error =
                        match serde_json::from_value::<JSONPayloadResponse>(value.clone()) {
                            Ok(payload) => return Ok(Self::JSONPayloadResponse(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<JSONPatchPayloadResponse>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "json",
                                first_name,
                                stringify!(JSONPatchPayloadResponse),
                            )));
                        }
                        structural_match = Some((
                            Self::JSONPatchPayloadResponse(payload),
                            stringify!(JSONPatchPayloadResponse),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "json_patch" => {
                    let primary_error =
                        match serde_json::from_value::<JSONPatchPayloadResponse>(value.clone()) {
                            Ok(payload) => {
                                return Ok(Self::JSONPatchPayloadResponse(payload));
                            }
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<JSONPayloadResponse>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "json_patch",
                                first_name,
                                stringify!(JSONPayloadResponse),
                            )));
                        }
                        structural_match = Some((
                            Self::JSONPayloadResponse(payload),
                            stringify!(JSONPayloadResponse),
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
            None => Err(serde::de::Error::custom(concat!(
                "missing string discriminator `",
                "type",
                "`",
            ))),
        }
    }
}
