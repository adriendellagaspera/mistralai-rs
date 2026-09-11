impl<'de> serde::Deserialize<'de> for ConversationHistoryEntriesItemUnion {
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
                    if let Ok(payload) =
                        serde_json::from_value::<FunctionResultEntry>(value.clone())
                    {
                        return Ok(Self::FunctionResultEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEntry>(value.clone())
                    {
                        return Ok(Self::FunctionCallEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ToolExecutionEntry>(value.clone())
                    {
                        return Ok(Self::ToolExecutionEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<AgentHandoffEntry>(value.clone())
                    {
                        return Ok(Self::AgentHandoffEntry(payload));
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
                    if let Ok(payload) =
                        serde_json::from_value::<FunctionResultEntry>(value.clone())
                    {
                        return Ok(Self::FunctionResultEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEntry>(value.clone())
                    {
                        return Ok(Self::FunctionCallEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ToolExecutionEntry>(value.clone())
                    {
                        return Ok(Self::ToolExecutionEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<AgentHandoffEntry>(value.clone())
                    {
                        return Ok(Self::AgentHandoffEntry(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "function.result" => {
                    let primary_error =
                        match serde_json::from_value::<FunctionResultEntry>(value.clone()) {
                            Ok(payload) => return Ok(Self::FunctionResultEntry(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<MessageInputEntry>(value.clone())
                    {
                        return Ok(Self::MessageInputEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEntry>(value.clone())
                    {
                        return Ok(Self::MessageOutputEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEntry>(value.clone())
                    {
                        return Ok(Self::FunctionCallEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ToolExecutionEntry>(value.clone())
                    {
                        return Ok(Self::ToolExecutionEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<AgentHandoffEntry>(value.clone())
                    {
                        return Ok(Self::AgentHandoffEntry(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "function.call" => {
                    let primary_error =
                        match serde_json::from_value::<FunctionCallEntry>(value.clone()) {
                            Ok(payload) => return Ok(Self::FunctionCallEntry(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<MessageInputEntry>(value.clone())
                    {
                        return Ok(Self::MessageInputEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEntry>(value.clone())
                    {
                        return Ok(Self::MessageOutputEntry(payload));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<FunctionResultEntry>(value.clone())
                    {
                        return Ok(Self::FunctionResultEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ToolExecutionEntry>(value.clone())
                    {
                        return Ok(Self::ToolExecutionEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<AgentHandoffEntry>(value.clone())
                    {
                        return Ok(Self::AgentHandoffEntry(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "tool.execution" => {
                    let primary_error =
                        match serde_json::from_value::<ToolExecutionEntry>(value.clone()) {
                            Ok(payload) => return Ok(Self::ToolExecutionEntry(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<MessageInputEntry>(value.clone())
                    {
                        return Ok(Self::MessageInputEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEntry>(value.clone())
                    {
                        return Ok(Self::MessageOutputEntry(payload));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<FunctionResultEntry>(value.clone())
                    {
                        return Ok(Self::FunctionResultEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEntry>(value.clone())
                    {
                        return Ok(Self::FunctionCallEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<AgentHandoffEntry>(value.clone())
                    {
                        return Ok(Self::AgentHandoffEntry(payload));
                    }
                    Err(serde::de::Error::custom(primary_error))
                }
                "agent.handoff" => {
                    let primary_error =
                        match serde_json::from_value::<AgentHandoffEntry>(value.clone()) {
                            Ok(payload) => return Ok(Self::AgentHandoffEntry(payload)),
                            Err(error) => error,
                        };
                    if let Ok(payload) = serde_json::from_value::<MessageInputEntry>(value.clone())
                    {
                        return Ok(Self::MessageInputEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEntry>(value.clone())
                    {
                        return Ok(Self::MessageOutputEntry(payload));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<FunctionResultEntry>(value.clone())
                    {
                        return Ok(Self::FunctionResultEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEntry>(value.clone())
                    {
                        return Ok(Self::FunctionCallEntry(payload));
                    }
                    if let Ok(payload) = serde_json::from_value::<ToolExecutionEntry>(value.clone())
                    {
                        return Ok(Self::ToolExecutionEntry(payload));
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
                if let Ok(payload) = serde_json::from_value::<FunctionResultEntry>(value.clone()) {
                    return Ok(Self::FunctionResultEntry(payload));
                }
                if let Ok(payload) = serde_json::from_value::<FunctionCallEntry>(value.clone()) {
                    return Ok(Self::FunctionCallEntry(payload));
                }
                if let Ok(payload) = serde_json::from_value::<ToolExecutionEntry>(value.clone()) {
                    return Ok(Self::ToolExecutionEntry(payload));
                }
                if let Ok(payload) = serde_json::from_value::<AgentHandoffEntry>(value.clone()) {
                    return Ok(Self::AgentHandoffEntry(payload));
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
