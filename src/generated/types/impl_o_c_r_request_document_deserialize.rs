impl<'de> serde::Deserialize<'de> for OCRRequestDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let discriminator = match value.get("type") {
            Some(serde_json::Value::String(discriminator)) => Some(discriminator.as_str()),
            Some(_) => {
                return Err(serde::de::Error::custom(concat!(
                    "non-string discriminator `",
                    "type",
                    "`",
                )));
            }
            None => None,
        };
        match discriminator {
            Some(discriminator) => match discriminator {
                "file" => {
                    let primary_error = match serde_json::from_value::<FileChunk>(value.clone()) {
                        Ok(payload) => return Ok(Self::FileChunk(payload)),
                        Err(error) => error,
                    };
                    if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                        return Ok(Self::DocumentURLChunk(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                        return Ok(Self::ImageURLChunk(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "document_url" => {
                    let primary_error =
                        match serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                            Ok(payload) => return Ok(Self::DocumentURLChunk(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                        return Ok(Self::FileChunk(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                        return Ok(Self::ImageURLChunk(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "image_url" => {
                    let primary_error = match serde_json::from_value::<ImageURLChunk>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::ImageURLChunk(payload)),
                        Err(error) => error,
                    };
                    if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                        return Ok(Self::FileChunk(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                        return Ok(Self::DocumentURLChunk(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown discriminator value `{other}` for `{}`",
                    "type",
                ))),
            },
            None => {
                if let Ok(payload) = serde_json::from_value::<FileChunk>(value.clone()) {
                    return Ok(Self::FileChunk(payload));
                }
                if let Ok(payload) = serde_json::from_value::<DocumentURLChunk>(value.clone()) {
                    return Ok(Self::DocumentURLChunk(payload));
                }
                if let Ok(payload) = serde_json::from_value::<ImageURLChunk>(value.clone()) {
                    return Ok(Self::ImageURLChunk(payload));
                }
                Err(serde::de::Error::custom(concat!(
                    "missing string discriminator `",
                    "type",
                    "` and no tagless branch matched",
                )))
            }
        }
    }
}
