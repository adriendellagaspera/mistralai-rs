impl serde::Serialize for ChatModerationRequestInputItemUnion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::SystemMessage(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(SystemMessage),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("role") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "system") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "role",
                            stringify!(SystemMessage),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "role",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "role".to_string(),
                            serde_json::Value::String("system".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::UserMessage(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(UserMessage),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("role") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "user") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "role",
                            stringify!(UserMessage),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "role",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "role".to_string(),
                            serde_json::Value::String("user".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::AssistantMessage(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(AssistantMessage),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("role") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "assistant") => {
                    }
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "role",
                            stringify!(AssistantMessage),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "role",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "role".to_string(),
                            serde_json::Value::String("assistant".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::ToolMessage(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(ToolMessage),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("role") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "tool") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "role",
                            stringify!(ToolMessage),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "role",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "role".to_string(),
                            serde_json::Value::String("tool".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
        }
    }
}
