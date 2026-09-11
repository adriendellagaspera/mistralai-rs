impl<'de> Deserialize<'de> for ListWorkflowEventResponseEventsItemUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = <serde_json::Value as Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null()
                    || matches!(value.to_string().as_str(), "\"WORKFLOW_EXECUTION_STARTED\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<WorkflowExecutionStartedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::WorkflowExecutionStartedResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null()
                    || matches!(
                        value.to_string().as_str(),
                        "\"WORKFLOW_EXECUTION_COMPLETED\""
                    )
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<WorkflowExecutionCompletedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::WorkflowExecutionCompletedResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null()
                    || matches!(value.to_string().as_str(), "\"WORKFLOW_EXECUTION_FAILED\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<WorkflowExecutionFailedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::WorkflowExecutionFailedResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null()
                    || matches!(
                        value.to_string().as_str(),
                        "\"WORKFLOW_EXECUTION_CANCELED\""
                    )
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<WorkflowExecutionCanceledResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::WorkflowExecutionCanceledResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null()
                    || matches!(
                        value.to_string().as_str(),
                        "\"WORKFLOW_EXECUTION_CONTINUED_AS_NEW\""
                    )
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<WorkflowExecutionContinuedAsNewResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::WorkflowExecutionContinuedAsNewResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null()
                    || matches!(value.to_string().as_str(), "\"WORKFLOW_TASK_TIMED_OUT\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<WorkflowTaskTimedOutResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::WorkflowTaskTimedOutResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null() || matches!(value.to_string().as_str(), "\"WORKFLOW_TASK_FAILED\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<WorkflowTaskFailedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::WorkflowTaskFailedResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null() || matches!(value.to_string().as_str(), "\"CUSTOM_TASK_STARTED\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<CustomTaskStartedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::CustomTaskStartedResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null()
                    || matches!(value.to_string().as_str(), "\"CUSTOM_TASK_IN_PROGRESS\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<CustomTaskInProgressResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::CustomTaskInProgressResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null() || matches!(value.to_string().as_str(), "\"CUSTOM_TASK_COMPLETED\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<CustomTaskCompletedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::CustomTaskCompletedResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null() || matches!(value.to_string().as_str(), "\"CUSTOM_TASK_FAILED\"")
            })
        }) {
            if let Ok(candidate) = serde_json::from_value::<CustomTaskFailedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::CustomTaskFailedResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null() || matches!(value.to_string().as_str(), "\"CUSTOM_TASK_TIMED_OUT\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<CustomTaskTimedOutResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::CustomTaskTimedOutResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null() || matches!(value.to_string().as_str(), "\"CUSTOM_TASK_CANCELED\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<CustomTaskCanceledResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::CustomTaskCanceledResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null() || matches!(value.to_string().as_str(), "\"ACTIVITY_TASK_STARTED\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<ActivityTaskStartedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::ActivityTaskStartedResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null()
                    || matches!(value.to_string().as_str(), "\"ACTIVITY_TASK_COMPLETED\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<ActivityTaskCompletedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::ActivityTaskCompletedResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null()
                    || matches!(value.to_string().as_str(), "\"ACTIVITY_TASK_RETRYING\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<ActivityTaskRetryingResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::ActivityTaskRetryingResponse(candidate));
                }
            }
        }
        if input.as_object().is_some_and(|object| {
            true && object.get("event_type").is_some_and(|value| {
                value.is_null() || matches!(value.to_string().as_str(), "\"ACTIVITY_TASK_FAILED\"")
            })
        }) {
            if let Ok(candidate) =
                serde_json::from_value::<ActivityTaskFailedResponse>(input.clone())
            {
                let preserves_complete_input = serde_json::to_value(&candidate)
                    .map(|encoded| encoded == input)
                    .unwrap_or(false);
                if preserves_complete_input {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!(
                            "ambiguous oneOf value for ",
                            stringify!(ListWorkflowEventResponseEventsItemUnion),
                            ": more than one branch preserved the complete input",
                        )));
                    }
                    matched = Some(Self::ActivityTaskFailedResponse(candidate));
                }
            }
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no oneOf branch for ",
                stringify!(ListWorkflowEventResponseEventsItemUnion),
                " preserved the complete input",
            ))
        })
    }
}
