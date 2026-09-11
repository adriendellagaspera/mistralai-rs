impl<'de> serde::Deserialize<'de> for JSONPatchPayloadResponseValueItemUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let discriminator = match value.get("op") {
            Some(serde_json::Value::String(discriminator)) => Some(discriminator.as_str()),
            Some(_) => {
                return Err(serde::de::Error::custom(concat!(
                    "non-string discriminator `",
                    "op",
                    "`",
                )));
            }
            None => None,
        };
        match discriminator {
            Some(discriminator) => match discriminator {
                "append" => {
                    let primary_error =
                        match serde_json::from_value::<JSONPatchAppend>(value.clone()) {
                            Ok(payload) => return Ok(Self::JSONPatchAppend(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<JSONPatchAdd>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "append",
                                first_name,
                                stringify!(JSONPatchAdd),
                            )));
                        }
                        structural_match =
                            Some((Self::JSONPatchAdd(payload), stringify!(JSONPatchAdd)));
                    }
                    if let Ok(payload) = serde_json::from_value::<JSONPatchReplace>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "append",
                                first_name,
                                stringify!(JSONPatchReplace),
                            )));
                        }
                        structural_match = Some((
                            Self::JSONPatchReplace(payload),
                            stringify!(JSONPatchReplace),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<JSONPatchRemove>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "append",
                                first_name,
                                stringify!(JSONPatchRemove),
                            )));
                        }
                        structural_match =
                            Some((Self::JSONPatchRemove(payload), stringify!(JSONPatchRemove)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "add" => {
                    let primary_error = match serde_json::from_value::<JSONPatchAdd>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::JSONPatchAdd(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<JSONPatchAppend>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "add",
                                first_name,
                                stringify!(JSONPatchAppend),
                            )));
                        }
                        structural_match =
                            Some((Self::JSONPatchAppend(payload), stringify!(JSONPatchAppend)));
                    }
                    if let Ok(payload) = serde_json::from_value::<JSONPatchReplace>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "add",
                                first_name,
                                stringify!(JSONPatchReplace),
                            )));
                        }
                        structural_match = Some((
                            Self::JSONPatchReplace(payload),
                            stringify!(JSONPatchReplace),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<JSONPatchRemove>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "add",
                                first_name,
                                stringify!(JSONPatchRemove),
                            )));
                        }
                        structural_match =
                            Some((Self::JSONPatchRemove(payload), stringify!(JSONPatchRemove)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "replace" => {
                    let primary_error =
                        match serde_json::from_value::<JSONPatchReplace>(value.clone()) {
                            Ok(payload) => return Ok(Self::JSONPatchReplace(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<JSONPatchAppend>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "replace",
                                first_name,
                                stringify!(JSONPatchAppend),
                            )));
                        }
                        structural_match =
                            Some((Self::JSONPatchAppend(payload), stringify!(JSONPatchAppend)));
                    }
                    if let Ok(payload) = serde_json::from_value::<JSONPatchAdd>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "replace",
                                first_name,
                                stringify!(JSONPatchAdd),
                            )));
                        }
                        structural_match =
                            Some((Self::JSONPatchAdd(payload), stringify!(JSONPatchAdd)));
                    }
                    if let Ok(payload) = serde_json::from_value::<JSONPatchRemove>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "replace",
                                first_name,
                                stringify!(JSONPatchRemove),
                            )));
                        }
                        structural_match =
                            Some((Self::JSONPatchRemove(payload), stringify!(JSONPatchRemove)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "remove" => {
                    let primary_error =
                        match serde_json::from_value::<JSONPatchRemove>(value.clone()) {
                            Ok(payload) => return Ok(Self::JSONPatchRemove(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<JSONPatchAppend>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "remove",
                                first_name,
                                stringify!(JSONPatchAppend),
                            )));
                        }
                        structural_match =
                            Some((Self::JSONPatchAppend(payload), stringify!(JSONPatchAppend)));
                    }
                    if let Ok(payload) = serde_json::from_value::<JSONPatchAdd>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "remove",
                                first_name,
                                stringify!(JSONPatchAdd),
                            )));
                        }
                        structural_match =
                            Some((Self::JSONPatchAdd(payload), stringify!(JSONPatchAdd)));
                    }
                    if let Ok(payload) = serde_json::from_value::<JSONPatchReplace>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "op",
                                "remove",
                                first_name,
                                stringify!(JSONPatchReplace),
                            )));
                        }
                        structural_match = Some((
                            Self::JSONPatchReplace(payload),
                            stringify!(JSONPatchReplace),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown discriminator value `{other}` for `{}`",
                    "op",
                ))),
            },
            None => Err(serde::de::Error::custom(concat!(
                "missing string discriminator `",
                "op",
                "`",
            ))),
        }
    }
}
