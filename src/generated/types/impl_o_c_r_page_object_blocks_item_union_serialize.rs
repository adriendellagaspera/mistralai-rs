impl serde::Serialize for OCRPageObjectBlocksItemUnion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::OCRTextBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRTextBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "text") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRTextBlock),
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
                            serde_json::Value::String("text".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRListBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRListBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "list") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRListBlock),
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
                            serde_json::Value::String("list".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRImageBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRImageBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "image") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRImageBlock),
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
                            serde_json::Value::String("image".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRTableBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRTableBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "table") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRTableBlock),
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
                            serde_json::Value::String("table".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRTitleBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRTitleBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "title") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRTitleBlock),
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
                            serde_json::Value::String("title".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCREquationBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCREquationBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "equation") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCREquationBlock),
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
                            serde_json::Value::String("equation".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRCaptionBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRCaptionBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "caption") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRCaptionBlock),
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
                            serde_json::Value::String("caption".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRCodeBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRCodeBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "code") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRCodeBlock),
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
                            serde_json::Value::String("code".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRReferencesBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRReferencesBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "references") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRReferencesBlock),
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
                            serde_json::Value::String("references".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRAsideTextBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRAsideTextBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "aside_text") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRAsideTextBlock),
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
                            serde_json::Value::String("aside_text".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRHeaderBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRHeaderBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "header") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRHeaderBlock),
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
                            serde_json::Value::String("header".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRFooterBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRFooterBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "footer") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRFooterBlock),
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
                            serde_json::Value::String("footer".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::OCRSignatureBlock(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(OCRSignatureBlock),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "signature") => {
                    }
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(OCRSignatureBlock),
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
                            serde_json::Value::String("signature".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
        }
    }
}
