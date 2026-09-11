impl serde::Serialize for JobsApiRoutesFineTuningUpdateFineTunedModelResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::CompletionFTModelOut(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(CompletionFTModelOut),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("model_type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "completion") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "model_type",
                            stringify!(CompletionFTModelOut),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "model_type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "model_type".to_string(),
                            serde_json::Value::String("completion".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::ClassifierFTModelOut(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(ClassifierFTModelOut),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("model_type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "classifier") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "model_type",
                            stringify!(ClassifierFTModelOut),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "model_type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "model_type".to_string(),
                            serde_json::Value::String("classifier".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
        }
    }
}
