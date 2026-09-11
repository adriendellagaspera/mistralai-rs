impl<'de> serde::Deserialize<'de> for MCPToolCallResponseContentItemUnion {
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
                    let primary_error = match serde_json::from_value::<TextContent>(value.clone()) {
                        Ok(payload) => return Ok(Self::TextContent(payload)),
                        Err(error) => error,
                    };
                    if let Ok(payload) = serde_json::from_value::<ImageContent>(value.clone()) {
                        return Ok(Self::ImageContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioContent>(value.clone()) {
                        return Ok(Self::AudioContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResourceLink>(value.clone()) {
                        return Ok(Self::ResourceLink(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<EmbeddedResource>(value.clone()) {
                        return Ok(Self::EmbeddedResource(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "image" => {
                    let primary_error = match serde_json::from_value::<ImageContent>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::ImageContent(payload)),
                        Err(error) => error,
                    };
                    if let Ok(payload) = serde_json::from_value::<TextContent>(value.clone()) {
                        return Ok(Self::TextContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioContent>(value.clone()) {
                        return Ok(Self::AudioContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResourceLink>(value.clone()) {
                        return Ok(Self::ResourceLink(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<EmbeddedResource>(value.clone()) {
                        return Ok(Self::EmbeddedResource(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "audio" => {
                    let primary_error = match serde_json::from_value::<AudioContent>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::AudioContent(payload)),
                        Err(error) => error,
                    };
                    if let Ok(payload) = serde_json::from_value::<TextContent>(value.clone()) {
                        return Ok(Self::TextContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageContent>(value.clone()) {
                        return Ok(Self::ImageContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResourceLink>(value.clone()) {
                        return Ok(Self::ResourceLink(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<EmbeddedResource>(value.clone()) {
                        return Ok(Self::EmbeddedResource(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "resource_link" => {
                    let primary_error = match serde_json::from_value::<ResourceLink>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::ResourceLink(payload)),
                        Err(error) => error,
                    };
                    if let Ok(payload) = serde_json::from_value::<TextContent>(value.clone()) {
                        return Ok(Self::TextContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageContent>(value.clone()) {
                        return Ok(Self::ImageContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioContent>(value.clone()) {
                        return Ok(Self::AudioContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<EmbeddedResource>(value.clone()) {
                        return Ok(Self::EmbeddedResource(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "resource" => {
                    let primary_error =
                        match serde_json::from_value::<EmbeddedResource>(value.clone()) {
                            Ok(payload) => return Ok(Self::EmbeddedResource(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<TextContent>(value.clone()) {
                        return Ok(Self::TextContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ImageContent>(value.clone()) {
                        return Ok(Self::ImageContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<AudioContent>(value.clone()) {
                        return Ok(Self::AudioContent(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResourceLink>(value.clone()) {
                        return Ok(Self::ResourceLink(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown discriminator value `{other}` for `{}`",
                    "type",
                ))),
            },
            None => Err(serde::de::Error::custom(concat!(
                "missing string discriminator `",
                "type",
                "`",
            ))),
        }
    }
}
