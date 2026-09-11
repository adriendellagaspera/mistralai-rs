impl serde::Serialize for TranscriptionStreamEventsData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::TranscriptionStreamTextDelta(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(TranscriptionStreamTextDelta),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "transcription.text.delta") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(TranscriptionStreamTextDelta),
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
                            serde_json::Value::String("transcription.text.delta".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::TranscriptionStreamLanguage(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(TranscriptionStreamLanguage),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "transcription.language") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(TranscriptionStreamLanguage),
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
                            serde_json::Value::String("transcription.language".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::TranscriptionStreamSegmentDelta(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(TranscriptionStreamSegmentDelta),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "transcription.segment") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(TranscriptionStreamSegmentDelta),
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
                            serde_json::Value::String("transcription.segment".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
            Self::TranscriptionStreamDone(payload) => {
                let mut value = serde_json::to_value(payload).map_err(serde::ser::Error::custom)?;
                let object = value.as_object_mut().ok_or_else(|| {
                    serde::ser::Error::custom(concat!(
                        "discriminated union variant `",
                        stringify!(TranscriptionStreamDone),
                        "` did not serialize as an object",
                    ))
                })?;
                match object.get("type") {
                    Some(serde_json::Value::String(tag))
                        if matches!(tag.as_str(), "transcription.done") => {}
                    Some(serde_json::Value::String(tag)) => {
                        return Err(serde::ser::Error::custom(format!(
                            "discriminator `{}` value `{tag}` is not valid for variant `{}`",
                            "type",
                            stringify!(TranscriptionStreamDone),
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
                            serde_json::Value::String("transcription.done".to_string()),
                        );
                    }
                }
                value.serialize(serializer)
            }
        }
    }
}
