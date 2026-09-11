impl serde::Serialize for AgentToolsItemUnion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::FunctionTool(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(FunctionTool),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "function") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(FunctionTool),
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
                            serde_json::Value::String("function".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::WebSearchTool(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(WebSearchTool),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "web_search") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(WebSearchTool),
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
                            serde_json::Value::String("web_search".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::WebSearchPremiumTool(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(WebSearchPremiumTool),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "web_search_premium") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(WebSearchPremiumTool),
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
                            serde_json::Value::String("web_search_premium".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::CodeInterpreterTool(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(CodeInterpreterTool),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "code_interpreter") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(CodeInterpreterTool),
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
                            serde_json::Value::String("code_interpreter".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::ImageGenerationTool(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(ImageGenerationTool),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "image_generation") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(ImageGenerationTool),
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
                            serde_json::Value::String("image_generation".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::DocumentLibraryTool(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(DocumentLibraryTool),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "document_library") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(DocumentLibraryTool),
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
                            serde_json::Value::String("document_library".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::CustomConnector(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(CustomConnector),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "connector") => {
                    }
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(CustomConnector),
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
                            serde_json::Value::String("connector".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
        }
    }
}
