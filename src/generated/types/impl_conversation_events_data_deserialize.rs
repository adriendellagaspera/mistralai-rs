impl<'de> serde::Deserialize<'de> for ConversationEventsData {
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
                "conversation.response.started" => {
                    let primary_error =
                        match serde_json::from_value::<ResponseStartedEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::Started(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.started",
                                first_name,
                                stringify!(Done),
                            )));
                        }
                        structural_match = Some((Self::Done(payload), stringify!(Done)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.started",
                                first_name,
                                stringify!(Error),
                            )));
                        }
                        structural_match = Some((Self::Error(payload), stringify!(Error)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.started",
                                first_name,
                                stringify!(ToolExecutionStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionStarted(payload),
                            stringify!(ToolExecutionStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.started",
                                first_name,
                                stringify!(ToolExecutionDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDelta(payload),
                            stringify!(ToolExecutionDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.started",
                                first_name,
                                stringify!(ToolExecutionDone),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDone(payload),
                            stringify!(ToolExecutionDone),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.started",
                                first_name,
                                stringify!(MessageOutput),
                            )));
                        }
                        structural_match =
                            Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.started",
                                first_name,
                                stringify!(FunctionCall),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.started",
                                first_name,
                                stringify!(AgentHandoffStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffStarted(payload),
                            stringify!(AgentHandoffStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.started",
                                first_name,
                                stringify!(AgentHandoffDone),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffDone(payload),
                            stringify!(AgentHandoffDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "conversation.response.done" => {
                    let primary_error =
                        match serde_json::from_value::<ResponseDoneEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::Done(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ResponseStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.done",
                                first_name,
                                stringify!(Started),
                            )));
                        }
                        structural_match = Some((Self::Started(payload), stringify!(Started)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.done",
                                first_name,
                                stringify!(Error),
                            )));
                        }
                        structural_match = Some((Self::Error(payload), stringify!(Error)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.done",
                                first_name,
                                stringify!(ToolExecutionStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionStarted(payload),
                            stringify!(ToolExecutionStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.done",
                                first_name,
                                stringify!(ToolExecutionDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDelta(payload),
                            stringify!(ToolExecutionDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.done",
                                first_name,
                                stringify!(ToolExecutionDone),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDone(payload),
                            stringify!(ToolExecutionDone),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.done",
                                first_name,
                                stringify!(MessageOutput),
                            )));
                        }
                        structural_match =
                            Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.done",
                                first_name,
                                stringify!(FunctionCall),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.done",
                                first_name,
                                stringify!(AgentHandoffStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffStarted(payload),
                            stringify!(AgentHandoffStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.done",
                                first_name,
                                stringify!(AgentHandoffDone),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffDone(payload),
                            stringify!(AgentHandoffDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "conversation.response.error" => {
                    let primary_error =
                        match serde_json::from_value::<ResponseErrorEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::Error(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ResponseStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.error",
                                first_name,
                                stringify!(Started),
                            )));
                        }
                        structural_match = Some((Self::Started(payload), stringify!(Started)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.error",
                                first_name,
                                stringify!(Done),
                            )));
                        }
                        structural_match = Some((Self::Done(payload), stringify!(Done)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.error",
                                first_name,
                                stringify!(ToolExecutionStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionStarted(payload),
                            stringify!(ToolExecutionStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.error",
                                first_name,
                                stringify!(ToolExecutionDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDelta(payload),
                            stringify!(ToolExecutionDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.error",
                                first_name,
                                stringify!(ToolExecutionDone),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDone(payload),
                            stringify!(ToolExecutionDone),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.error",
                                first_name,
                                stringify!(MessageOutput),
                            )));
                        }
                        structural_match =
                            Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.error",
                                first_name,
                                stringify!(FunctionCall),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.error",
                                first_name,
                                stringify!(AgentHandoffStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffStarted(payload),
                            stringify!(AgentHandoffStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "conversation.response.error",
                                first_name,
                                stringify!(AgentHandoffDone),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffDone(payload),
                            stringify!(AgentHandoffDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "tool.execution.started" => {
                    let primary_error =
                        match serde_json::from_value::<ToolExecutionStartedEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::ToolExecutionStarted(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ResponseStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.started",
                                first_name,
                                stringify!(Started),
                            )));
                        }
                        structural_match = Some((Self::Started(payload), stringify!(Started)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.started",
                                first_name,
                                stringify!(Done),
                            )));
                        }
                        structural_match = Some((Self::Done(payload), stringify!(Done)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.started",
                                first_name,
                                stringify!(Error),
                            )));
                        }
                        structural_match = Some((Self::Error(payload), stringify!(Error)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.started",
                                first_name,
                                stringify!(ToolExecutionDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDelta(payload),
                            stringify!(ToolExecutionDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.started",
                                first_name,
                                stringify!(ToolExecutionDone),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDone(payload),
                            stringify!(ToolExecutionDone),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.started",
                                first_name,
                                stringify!(MessageOutput),
                            )));
                        }
                        structural_match =
                            Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.started",
                                first_name,
                                stringify!(FunctionCall),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.started",
                                first_name,
                                stringify!(AgentHandoffStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffStarted(payload),
                            stringify!(AgentHandoffStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.started",
                                first_name,
                                stringify!(AgentHandoffDone),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffDone(payload),
                            stringify!(AgentHandoffDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "tool.execution.delta" => {
                    let primary_error =
                        match serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::ToolExecutionDelta(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ResponseStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.delta",
                                first_name,
                                stringify!(Started),
                            )));
                        }
                        structural_match = Some((Self::Started(payload), stringify!(Started)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.delta",
                                first_name,
                                stringify!(Done),
                            )));
                        }
                        structural_match = Some((Self::Done(payload), stringify!(Done)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.delta",
                                first_name,
                                stringify!(Error),
                            )));
                        }
                        structural_match = Some((Self::Error(payload), stringify!(Error)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.delta",
                                first_name,
                                stringify!(ToolExecutionStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionStarted(payload),
                            stringify!(ToolExecutionStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.delta",
                                first_name,
                                stringify!(ToolExecutionDone),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDone(payload),
                            stringify!(ToolExecutionDone),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.delta",
                                first_name,
                                stringify!(MessageOutput),
                            )));
                        }
                        structural_match =
                            Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.delta",
                                first_name,
                                stringify!(FunctionCall),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.delta",
                                first_name,
                                stringify!(AgentHandoffStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffStarted(payload),
                            stringify!(AgentHandoffStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.delta",
                                first_name,
                                stringify!(AgentHandoffDone),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffDone(payload),
                            stringify!(AgentHandoffDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "tool.execution.done" => {
                    let primary_error =
                        match serde_json::from_value::<ToolExecutionDoneEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::ToolExecutionDone(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ResponseStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.done",
                                first_name,
                                stringify!(Started),
                            )));
                        }
                        structural_match = Some((Self::Started(payload), stringify!(Started)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.done",
                                first_name,
                                stringify!(Done),
                            )));
                        }
                        structural_match = Some((Self::Done(payload), stringify!(Done)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.done",
                                first_name,
                                stringify!(Error),
                            )));
                        }
                        structural_match = Some((Self::Error(payload), stringify!(Error)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.done",
                                first_name,
                                stringify!(ToolExecutionStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionStarted(payload),
                            stringify!(ToolExecutionStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.done",
                                first_name,
                                stringify!(ToolExecutionDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDelta(payload),
                            stringify!(ToolExecutionDelta),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.done",
                                first_name,
                                stringify!(MessageOutput),
                            )));
                        }
                        structural_match =
                            Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.done",
                                first_name,
                                stringify!(FunctionCall),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.done",
                                first_name,
                                stringify!(AgentHandoffStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffStarted(payload),
                            stringify!(AgentHandoffStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "tool.execution.done",
                                first_name,
                                stringify!(AgentHandoffDone),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffDone(payload),
                            stringify!(AgentHandoffDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "message.output.delta" => {
                    let primary_error =
                        match serde_json::from_value::<MessageOutputEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::MessageOutput(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ResponseStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "message.output.delta",
                                first_name,
                                stringify!(Started),
                            )));
                        }
                        structural_match = Some((Self::Started(payload), stringify!(Started)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "message.output.delta",
                                first_name,
                                stringify!(Done),
                            )));
                        }
                        structural_match = Some((Self::Done(payload), stringify!(Done)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "message.output.delta",
                                first_name,
                                stringify!(Error),
                            )));
                        }
                        structural_match = Some((Self::Error(payload), stringify!(Error)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "message.output.delta",
                                first_name,
                                stringify!(ToolExecutionStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionStarted(payload),
                            stringify!(ToolExecutionStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "message.output.delta",
                                first_name,
                                stringify!(ToolExecutionDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDelta(payload),
                            stringify!(ToolExecutionDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "message.output.delta",
                                first_name,
                                stringify!(ToolExecutionDone),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDone(payload),
                            stringify!(ToolExecutionDone),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "message.output.delta",
                                first_name,
                                stringify!(FunctionCall),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "message.output.delta",
                                first_name,
                                stringify!(AgentHandoffStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffStarted(payload),
                            stringify!(AgentHandoffStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "message.output.delta",
                                first_name,
                                stringify!(AgentHandoffDone),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffDone(payload),
                            stringify!(AgentHandoffDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "function.call.delta" => {
                    let primary_error =
                        match serde_json::from_value::<FunctionCallEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::FunctionCall(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ResponseStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function.call.delta",
                                first_name,
                                stringify!(Started),
                            )));
                        }
                        structural_match = Some((Self::Started(payload), stringify!(Started)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function.call.delta",
                                first_name,
                                stringify!(Done),
                            )));
                        }
                        structural_match = Some((Self::Done(payload), stringify!(Done)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function.call.delta",
                                first_name,
                                stringify!(Error),
                            )));
                        }
                        structural_match = Some((Self::Error(payload), stringify!(Error)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function.call.delta",
                                first_name,
                                stringify!(ToolExecutionStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionStarted(payload),
                            stringify!(ToolExecutionStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function.call.delta",
                                first_name,
                                stringify!(ToolExecutionDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDelta(payload),
                            stringify!(ToolExecutionDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function.call.delta",
                                first_name,
                                stringify!(ToolExecutionDone),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDone(payload),
                            stringify!(ToolExecutionDone),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function.call.delta",
                                first_name,
                                stringify!(MessageOutput),
                            )));
                        }
                        structural_match =
                            Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function.call.delta",
                                first_name,
                                stringify!(AgentHandoffStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffStarted(payload),
                            stringify!(AgentHandoffStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function.call.delta",
                                first_name,
                                stringify!(AgentHandoffDone),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffDone(payload),
                            stringify!(AgentHandoffDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "agent.handoff.started" => {
                    let primary_error =
                        match serde_json::from_value::<AgentHandoffStartedEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::AgentHandoffStarted(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ResponseStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.started",
                                first_name,
                                stringify!(Started),
                            )));
                        }
                        structural_match = Some((Self::Started(payload), stringify!(Started)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.started",
                                first_name,
                                stringify!(Done),
                            )));
                        }
                        structural_match = Some((Self::Done(payload), stringify!(Done)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.started",
                                first_name,
                                stringify!(Error),
                            )));
                        }
                        structural_match = Some((Self::Error(payload), stringify!(Error)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.started",
                                first_name,
                                stringify!(ToolExecutionStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionStarted(payload),
                            stringify!(ToolExecutionStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.started",
                                first_name,
                                stringify!(ToolExecutionDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDelta(payload),
                            stringify!(ToolExecutionDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.started",
                                first_name,
                                stringify!(ToolExecutionDone),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDone(payload),
                            stringify!(ToolExecutionDone),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.started",
                                first_name,
                                stringify!(MessageOutput),
                            )));
                        }
                        structural_match =
                            Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.started",
                                first_name,
                                stringify!(FunctionCall),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.started",
                                first_name,
                                stringify!(AgentHandoffDone),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffDone(payload),
                            stringify!(AgentHandoffDone),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "agent.handoff.done" => {
                    let primary_error =
                        match serde_json::from_value::<AgentHandoffDoneEvent>(value.clone()) {
                            Ok(payload) => return Ok(Self::AgentHandoffDone(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) =
                        serde_json::from_value::<ResponseStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.done",
                                first_name,
                                stringify!(Started),
                            )));
                        }
                        structural_match = Some((Self::Started(payload), stringify!(Started)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.done",
                                first_name,
                                stringify!(Done),
                            )));
                        }
                        structural_match = Some((Self::Done(payload), stringify!(Done)));
                    }
                    if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.done",
                                first_name,
                                stringify!(Error),
                            )));
                        }
                        structural_match = Some((Self::Error(payload), stringify!(Error)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.done",
                                first_name,
                                stringify!(ToolExecutionStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionStarted(payload),
                            stringify!(ToolExecutionStarted),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.done",
                                first_name,
                                stringify!(ToolExecutionDelta),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDelta(payload),
                            stringify!(ToolExecutionDelta),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.done",
                                first_name,
                                stringify!(ToolExecutionDone),
                            )));
                        }
                        structural_match = Some((
                            Self::ToolExecutionDone(payload),
                            stringify!(ToolExecutionDone),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.done",
                                first_name,
                                stringify!(MessageOutput),
                            )));
                        }
                        structural_match =
                            Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                    }
                    if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.done",
                                first_name,
                                stringify!(FunctionCall),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "agent.handoff.done",
                                first_name,
                                stringify!(AgentHandoffStarted),
                            )));
                        }
                        structural_match = Some((
                            Self::AgentHandoffStarted(payload),
                            stringify!(AgentHandoffStarted),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                other => Err(serde::de::Error::custom(format!(
                    "unknown discriminator value `{other}` for `{}`",
                    "type",
                ))),
            },
            None => {
                let mut structural_match: Option<(Self, &'static str)> = None;
                if let Ok(payload) = serde_json::from_value::<ResponseStartedEvent>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(Started),
                        )));
                    }
                    structural_match = Some((Self::Started(payload), stringify!(Started)));
                }
                if let Ok(payload) = serde_json::from_value::<ResponseDoneEvent>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(Done),
                        )));
                    }
                    structural_match = Some((Self::Done(payload), stringify!(Done)));
                }
                if let Ok(payload) = serde_json::from_value::<ResponseErrorEvent>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(Error),
                        )));
                    }
                    structural_match = Some((Self::Error(payload), stringify!(Error)));
                }
                if let Ok(payload) =
                    serde_json::from_value::<ToolExecutionStartedEvent>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(ToolExecutionStarted),
                        )));
                    }
                    structural_match = Some((
                        Self::ToolExecutionStarted(payload),
                        stringify!(ToolExecutionStarted),
                    ));
                }
                if let Ok(payload) =
                    serde_json::from_value::<ToolExecutionDeltaEvent>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(ToolExecutionDelta),
                        )));
                    }
                    structural_match = Some((
                        Self::ToolExecutionDelta(payload),
                        stringify!(ToolExecutionDelta),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<ToolExecutionDoneEvent>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(ToolExecutionDone),
                        )));
                    }
                    structural_match = Some((
                        Self::ToolExecutionDone(payload),
                        stringify!(ToolExecutionDone),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<MessageOutputEvent>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(MessageOutput),
                        )));
                    }
                    structural_match =
                        Some((Self::MessageOutput(payload), stringify!(MessageOutput)));
                }
                if let Ok(payload) = serde_json::from_value::<FunctionCallEvent>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(FunctionCall),
                        )));
                    }
                    structural_match =
                        Some((Self::FunctionCall(payload), stringify!(FunctionCall)));
                }
                if let Ok(payload) =
                    serde_json::from_value::<AgentHandoffStartedEvent>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(AgentHandoffStarted),
                        )));
                    }
                    structural_match = Some((
                        Self::AgentHandoffStarted(payload),
                        stringify!(AgentHandoffStarted),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<AgentHandoffDoneEvent>(value.clone())
                {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(AgentHandoffDone),
                        )));
                    }
                    structural_match = Some((
                        Self::AgentHandoffDone(payload),
                        stringify!(AgentHandoffDone),
                    ));
                }
                structural_match.map(|(payload, _)| payload).ok_or_else(|| {
                    serde::de::Error::custom(concat!(
                        "missing string discriminator `",
                        "type",
                        "` and no tagless branch matched",
                    ))
                })
            }
        }
    }
}
