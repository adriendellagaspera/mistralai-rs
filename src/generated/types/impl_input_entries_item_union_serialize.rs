impl serde::Serialize for InputEntriesItemUnion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::MessageInputEntry(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(MessageInputEntry),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "message.input") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(MessageInputEntry),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "type".to_string(),
                            serde_json::Value::String("message.input".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::MessageOutputEntry(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(MessageOutputEntry),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "message.output") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(MessageOutputEntry),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "type".to_string(),
                            serde_json::Value::String("message.output".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::FunctionResultEntry(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(FunctionResultEntry),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "function.result") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(FunctionResultEntry),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "type".to_string(),
                            serde_json::Value::String("function.result".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::FunctionCallEntry(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(FunctionCallEntry),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "function.call") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(FunctionCallEntry),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "type".to_string(),
                            serde_json::Value::String("function.call".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::ToolExecutionEntry(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(ToolExecutionEntry),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "tool.execution") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(ToolExecutionEntry),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "type".to_string(),
                            serde_json::Value::String("tool.execution".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::AgentHandoffEntry(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(AgentHandoffEntry),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "agent.handoff") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(AgentHandoffEntry),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "type".to_string(),
                            serde_json::Value::String("agent.handoff".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
        }
    }
}
