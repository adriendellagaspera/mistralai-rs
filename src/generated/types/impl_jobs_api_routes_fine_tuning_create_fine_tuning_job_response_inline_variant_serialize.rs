impl serde::Serialize for JobsApiRoutesFineTuningCreateFineTuningJobResponseInlineVariant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::CompletionJobOut(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(CompletionJobOut),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("job_type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "completion") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "job_type",
                            stringify!(CompletionJobOut),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "job_type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "job_type".to_string(),
                            serde_json::Value::String("completion".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::ClassifierJobOut(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(ClassifierJobOut),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("job_type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "classifier") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "job_type",
                            stringify!(ClassifierJobOut),
                        )));
                    }
                    Some(_) => {
                        return Err(serde::ser::Error::custom(concat!(
                            "discriminator `",
                            "job_type",
                            "` did not serialize as a string",
                        )));
                    }
                    None => {
                        object.insert(
                            "job_type".to_string(),
                            serde_json::Value::String("classifier".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
        }
    }
}
