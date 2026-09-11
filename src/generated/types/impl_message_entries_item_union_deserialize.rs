impl<'de> serde::Deserialize<'de> for MessageEntriesItemUnion {
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
                "message.input" => {
                    let primary_error =
                        match serde_json::from_value::<MessageInputEntry>(value.clone()) {
                            Ok(payload) => return Ok(Self::MessageInputEntry(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEntry>(value.clone())
                    {
                        return Ok(Self::MessageOutputEntry(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "message.output" => {
                    let primary_error =
                        match serde_json::from_value::<MessageOutputEntry>(value.clone()) {
                            Ok(payload) => return Ok(Self::MessageOutputEntry(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<MessageInputEntry>(value.clone())
                    {
                        return Ok(Self::MessageInputEntry(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown discriminator value `{other}` for `{}`",
                    "type",
                ))),
            },
            None => {
                if let Ok(payload) = serde_json::from_value::<MessageInputEntry>(value.clone()) {
                    return Ok(Self::MessageInputEntry(payload));
                }
                if let Ok(payload) = serde_json::from_value::<MessageOutputEntry>(value.clone()) {
                    return Ok(Self::MessageOutputEntry(payload));
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
