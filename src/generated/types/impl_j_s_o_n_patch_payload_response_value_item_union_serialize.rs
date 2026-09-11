impl serde::Serialize for JSONPatchPayloadResponseValueItemUnion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::JSONPatchAppend(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(JSONPatchAppend),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("op") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "append") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "op",
                            stringify!(JSONPatchAppend),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "op",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "op".to_string(),
                            serde_json::Value::String("append".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::JSONPatchAdd(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(JSONPatchAdd),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("op") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "add") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "op",
                            stringify!(JSONPatchAdd),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "op",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "op".to_string(),
                            serde_json::Value::String("add".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::JSONPatchReplace(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(JSONPatchReplace),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("op") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "replace") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "op",
                            stringify!(JSONPatchReplace),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "op",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "op".to_string(),
                            serde_json::Value::String("replace".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::JSONPatchRemove(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(JSONPatchRemove),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("op") {
                    Some(serde_json::Value::String(tag)) if matches!(tag.as_str(), "remove") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "op",
                            stringify!(JSONPatchRemove),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "op",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "op".to_string(),
                            serde_json::Value::String("remove".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
        }
    }
}
