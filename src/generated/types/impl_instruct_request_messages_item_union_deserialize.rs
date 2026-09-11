impl<'de> serde::Deserialize<'de> for InstructRequestMessagesItemUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let discriminator = match value.get("role") {
            Some(serde_json::Value::String(discriminator)) => Some(discriminator.as_str()),
            Some(_) => {
                return Err(serde::de::Error::custom(concat!(
                    "non-string discriminator `",
                    "role",
                    "`",
                )));
            }
            None => None,
        };
        match discriminator {
            Some(discriminator) => match discriminator {
                "system" => {
                    let primary_error = match serde_json::from_value::<SystemMessage>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::SystemMessage(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<UserMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "system",
                                first_name,
                                stringify!(UserMessage),
                            )));
                        }
                        structural_match =
                            Some((Self::UserMessage(payload), stringify!(UserMessage)));
                    }
                    if let Ok(payload) = serde_json::from_value::<AssistantMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "system",
                                first_name,
                                stringify!(AssistantMessage),
                            )));
                        }
                        structural_match = Some((
                            Self::AssistantMessage(payload),
                            stringify!(AssistantMessage),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<ToolMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "system",
                                first_name,
                                stringify!(ToolMessage),
                            )));
                        }
                        structural_match =
                            Some((Self::ToolMessage(payload), stringify!(ToolMessage)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "user" => {
                    let primary_error = match serde_json::from_value::<UserMessage>(value.clone()) {
                        Ok(payload) => return Ok(Self::UserMessage(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<SystemMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "user",
                                first_name,
                                stringify!(SystemMessage),
                            )));
                        }
                        structural_match =
                            Some((Self::SystemMessage(payload), stringify!(SystemMessage)));
                    }
                    if let Ok(payload) = serde_json::from_value::<AssistantMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "user",
                                first_name,
                                stringify!(AssistantMessage),
                            )));
                        }
                        structural_match = Some((
                            Self::AssistantMessage(payload),
                            stringify!(AssistantMessage),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<ToolMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "user",
                                first_name,
                                stringify!(ToolMessage),
                            )));
                        }
                        structural_match =
                            Some((Self::ToolMessage(payload), stringify!(ToolMessage)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "assistant" => {
                    let primary_error =
                        match serde_json::from_value::<AssistantMessage>(value.clone()) {
                            Ok(payload) => return Ok(Self::AssistantMessage(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<SystemMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "assistant",
                                first_name,
                                stringify!(SystemMessage),
                            )));
                        }
                        structural_match =
                            Some((Self::SystemMessage(payload), stringify!(SystemMessage)));
                    }
                    if let Ok(payload) = serde_json::from_value::<UserMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "assistant",
                                first_name,
                                stringify!(UserMessage),
                            )));
                        }
                        structural_match =
                            Some((Self::UserMessage(payload), stringify!(UserMessage)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ToolMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "assistant",
                                first_name,
                                stringify!(ToolMessage),
                            )));
                        }
                        structural_match =
                            Some((Self::ToolMessage(payload), stringify!(ToolMessage)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "tool" => {
                    let primary_error = match serde_json::from_value::<ToolMessage>(value.clone()) {
                        Ok(payload) => return Ok(Self::ToolMessage(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<SystemMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "tool",
                                first_name,
                                stringify!(SystemMessage),
                            )));
                        }
                        structural_match =
                            Some((Self::SystemMessage(payload), stringify!(SystemMessage)));
                    }
                    if let Ok(payload) = serde_json::from_value::<UserMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "tool",
                                first_name,
                                stringify!(UserMessage),
                            )));
                        }
                        structural_match =
                            Some((Self::UserMessage(payload), stringify!(UserMessage)));
                    }
                    if let Ok(payload) = serde_json::from_value::<AssistantMessage>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "role",
                                "tool",
                                first_name,
                                stringify!(AssistantMessage),
                            )));
                        }
                        structural_match = Some((
                            Self::AssistantMessage(payload),
                            stringify!(AssistantMessage),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown discriminator value `{other}` for `{}`",
                    "role",
                ))),
            },
            None => {
                let mut structural_match: Option<(Self, &'static str)> = None;
                if let Ok(payload) = serde_json::from_value::<SystemMessage>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "role",
                            first_name,
                            stringify!(SystemMessage),
                        )));
                    }
                    structural_match =
                        Some((Self::SystemMessage(payload), stringify!(SystemMessage)));
                }
                if let Ok(payload) = serde_json::from_value::<UserMessage>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "role",
                            first_name,
                            stringify!(UserMessage),
                        )));
                    }
                    structural_match = Some((Self::UserMessage(payload), stringify!(UserMessage)));
                }
                if let Ok(payload) = serde_json::from_value::<AssistantMessage>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "role",
                            first_name,
                            stringify!(AssistantMessage),
                        )));
                    }
                    structural_match = Some((
                        Self::AssistantMessage(payload),
                        stringify!(AssistantMessage),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<ToolMessage>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "role",
                            first_name,
                            stringify!(ToolMessage),
                        )));
                    }
                    structural_match = Some((Self::ToolMessage(payload), stringify!(ToolMessage)));
                }
                structural_match.map(|(payload, _)| payload).ok_or_else(|| {
                    serde::de::Error::custom(concat!(
                        "missing string discriminator `",
                        "role",
                        "` and no tagless branch matched",
                    ))
                })
            }
        }
    }
}
