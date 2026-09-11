impl<'de> serde::Deserialize<'de> for ThinkChunkThinkingItemUnion {
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
                "text" => {
                    let primary_error = match serde_json::from_value::<TextChunk>(value.clone()) {
                        Ok(payload) => return Ok(Self::TextChunk(payload)),
                        Err(error) => error,
                    };
                    if let Ok(payload) = serde_json::from_value::<ToolReferenceChunk>(value.clone())
                    {
                        return Ok(Self::ToolReferenceChunk(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                        return Ok(Self::ReferenceChunk(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "tool_reference" => {
                    let primary_error =
                        match serde_json::from_value::<ToolReferenceChunk>(value.clone()) {
                            Ok(payload) => return Ok(Self::ToolReferenceChunk(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                        return Ok(Self::TextChunk(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                        return Ok(Self::ReferenceChunk(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "reference" => {
                    let primary_error =
                        match serde_json::from_value::<ReferenceChunk>(value.clone()) {
                            Ok(payload) => return Ok(Self::ReferenceChunk(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                        return Ok(Self::TextChunk(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ToolReferenceChunk>(value.clone())
                    {
                        return Ok(Self::ToolReferenceChunk(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown discriminator value `{other}` for `{}`",
                    "type",
                ))),
            },
            None => {
                if let Ok(payload) = serde_json::from_value::<TextChunk>(value.clone()) {
                    return Ok(Self::TextChunk(payload));
                }
                if let Ok(payload) = serde_json::from_value::<ToolReferenceChunk>(value.clone()) {
                    return Ok(Self::ToolReferenceChunk(payload));
                }
                if let Ok(payload) = serde_json::from_value::<ReferenceChunk>(value.clone()) {
                    return Ok(Self::ReferenceChunk(payload));
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
