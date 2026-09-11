impl<'de> serde::Deserialize<'de> for ModelConversationToolsItemUnion {
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
                "function" => {
                    let primary_error = match serde_json::from_value::<FunctionTool>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::FunctionTool(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<WebSearchTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function",
                                first_name,
                                stringify!(WebSearchTool),
                            )));
                        }
                        structural_match =
                            Some((Self::WebSearchTool(payload), stringify!(WebSearchTool)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<WebSearchPremiumTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function",
                                first_name,
                                stringify!(WebSearchPremiumTool),
                            )));
                        }
                        structural_match = Some((
                            Self::WebSearchPremiumTool(payload),
                            stringify!(WebSearchPremiumTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<CodeInterpreterTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function",
                                first_name,
                                stringify!(CodeInterpreterTool),
                            )));
                        }
                        structural_match = Some((
                            Self::CodeInterpreterTool(payload),
                            stringify!(CodeInterpreterTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ImageGenerationTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function",
                                first_name,
                                stringify!(ImageGenerationTool),
                            )));
                        }
                        structural_match = Some((
                            Self::ImageGenerationTool(payload),
                            stringify!(ImageGenerationTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<DocumentLibraryTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function",
                                first_name,
                                stringify!(DocumentLibraryTool),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentLibraryTool(payload),
                            stringify!(DocumentLibraryTool),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<CustomConnector>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "function",
                                first_name,
                                stringify!(CustomConnector),
                            )));
                        }
                        structural_match =
                            Some((Self::CustomConnector(payload), stringify!(CustomConnector)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "web_search" => {
                    let primary_error = match serde_json::from_value::<WebSearchTool>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::WebSearchTool(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<FunctionTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search",
                                first_name,
                                stringify!(FunctionTool),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionTool(payload), stringify!(FunctionTool)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<WebSearchPremiumTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search",
                                first_name,
                                stringify!(WebSearchPremiumTool),
                            )));
                        }
                        structural_match = Some((
                            Self::WebSearchPremiumTool(payload),
                            stringify!(WebSearchPremiumTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<CodeInterpreterTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search",
                                first_name,
                                stringify!(CodeInterpreterTool),
                            )));
                        }
                        structural_match = Some((
                            Self::CodeInterpreterTool(payload),
                            stringify!(CodeInterpreterTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ImageGenerationTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search",
                                first_name,
                                stringify!(ImageGenerationTool),
                            )));
                        }
                        structural_match = Some((
                            Self::ImageGenerationTool(payload),
                            stringify!(ImageGenerationTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<DocumentLibraryTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search",
                                first_name,
                                stringify!(DocumentLibraryTool),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentLibraryTool(payload),
                            stringify!(DocumentLibraryTool),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<CustomConnector>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search",
                                first_name,
                                stringify!(CustomConnector),
                            )));
                        }
                        structural_match =
                            Some((Self::CustomConnector(payload), stringify!(CustomConnector)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "web_search_premium" => {
                    let primary_error =
                        match serde_json::from_value::<WebSearchPremiumTool>(value.clone()) {
                            Ok(payload) => return Ok(Self::WebSearchPremiumTool(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<FunctionTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search_premium",
                                first_name,
                                stringify!(FunctionTool),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionTool(payload), stringify!(FunctionTool)));
                    }
                    if let Ok(payload) = serde_json::from_value::<WebSearchTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search_premium",
                                first_name,
                                stringify!(WebSearchTool),
                            )));
                        }
                        structural_match =
                            Some((Self::WebSearchTool(payload), stringify!(WebSearchTool)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<CodeInterpreterTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search_premium",
                                first_name,
                                stringify!(CodeInterpreterTool),
                            )));
                        }
                        structural_match = Some((
                            Self::CodeInterpreterTool(payload),
                            stringify!(CodeInterpreterTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ImageGenerationTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search_premium",
                                first_name,
                                stringify!(ImageGenerationTool),
                            )));
                        }
                        structural_match = Some((
                            Self::ImageGenerationTool(payload),
                            stringify!(ImageGenerationTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<DocumentLibraryTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search_premium",
                                first_name,
                                stringify!(DocumentLibraryTool),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentLibraryTool(payload),
                            stringify!(DocumentLibraryTool),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<CustomConnector>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "web_search_premium",
                                first_name,
                                stringify!(CustomConnector),
                            )));
                        }
                        structural_match =
                            Some((Self::CustomConnector(payload), stringify!(CustomConnector)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "code_interpreter" => {
                    let primary_error =
                        match serde_json::from_value::<CodeInterpreterTool>(value.clone()) {
                            Ok(payload) => return Ok(Self::CodeInterpreterTool(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<FunctionTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code_interpreter",
                                first_name,
                                stringify!(FunctionTool),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionTool(payload), stringify!(FunctionTool)));
                    }
                    if let Ok(payload) = serde_json::from_value::<WebSearchTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code_interpreter",
                                first_name,
                                stringify!(WebSearchTool),
                            )));
                        }
                        structural_match =
                            Some((Self::WebSearchTool(payload), stringify!(WebSearchTool)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<WebSearchPremiumTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code_interpreter",
                                first_name,
                                stringify!(WebSearchPremiumTool),
                            )));
                        }
                        structural_match = Some((
                            Self::WebSearchPremiumTool(payload),
                            stringify!(WebSearchPremiumTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ImageGenerationTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code_interpreter",
                                first_name,
                                stringify!(ImageGenerationTool),
                            )));
                        }
                        structural_match = Some((
                            Self::ImageGenerationTool(payload),
                            stringify!(ImageGenerationTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<DocumentLibraryTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code_interpreter",
                                first_name,
                                stringify!(DocumentLibraryTool),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentLibraryTool(payload),
                            stringify!(DocumentLibraryTool),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<CustomConnector>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code_interpreter",
                                first_name,
                                stringify!(CustomConnector),
                            )));
                        }
                        structural_match =
                            Some((Self::CustomConnector(payload), stringify!(CustomConnector)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "image_generation" => {
                    let primary_error =
                        match serde_json::from_value::<ImageGenerationTool>(value.clone()) {
                            Ok(payload) => return Ok(Self::ImageGenerationTool(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<FunctionTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_generation",
                                first_name,
                                stringify!(FunctionTool),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionTool(payload), stringify!(FunctionTool)));
                    }
                    if let Ok(payload) = serde_json::from_value::<WebSearchTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_generation",
                                first_name,
                                stringify!(WebSearchTool),
                            )));
                        }
                        structural_match =
                            Some((Self::WebSearchTool(payload), stringify!(WebSearchTool)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<WebSearchPremiumTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_generation",
                                first_name,
                                stringify!(WebSearchPremiumTool),
                            )));
                        }
                        structural_match = Some((
                            Self::WebSearchPremiumTool(payload),
                            stringify!(WebSearchPremiumTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<CodeInterpreterTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_generation",
                                first_name,
                                stringify!(CodeInterpreterTool),
                            )));
                        }
                        structural_match = Some((
                            Self::CodeInterpreterTool(payload),
                            stringify!(CodeInterpreterTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<DocumentLibraryTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_generation",
                                first_name,
                                stringify!(DocumentLibraryTool),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentLibraryTool(payload),
                            stringify!(DocumentLibraryTool),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<CustomConnector>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image_generation",
                                first_name,
                                stringify!(CustomConnector),
                            )));
                        }
                        structural_match =
                            Some((Self::CustomConnector(payload), stringify!(CustomConnector)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "document_library" => {
                    let primary_error =
                        match serde_json::from_value::<DocumentLibraryTool>(value.clone()) {
                            Ok(payload) => return Ok(Self::DocumentLibraryTool(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<FunctionTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_library",
                                first_name,
                                stringify!(FunctionTool),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionTool(payload), stringify!(FunctionTool)));
                    }
                    if let Ok(payload) = serde_json::from_value::<WebSearchTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_library",
                                first_name,
                                stringify!(WebSearchTool),
                            )));
                        }
                        structural_match =
                            Some((Self::WebSearchTool(payload), stringify!(WebSearchTool)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<WebSearchPremiumTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_library",
                                first_name,
                                stringify!(WebSearchPremiumTool),
                            )));
                        }
                        structural_match = Some((
                            Self::WebSearchPremiumTool(payload),
                            stringify!(WebSearchPremiumTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<CodeInterpreterTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_library",
                                first_name,
                                stringify!(CodeInterpreterTool),
                            )));
                        }
                        structural_match = Some((
                            Self::CodeInterpreterTool(payload),
                            stringify!(CodeInterpreterTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ImageGenerationTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_library",
                                first_name,
                                stringify!(ImageGenerationTool),
                            )));
                        }
                        structural_match = Some((
                            Self::ImageGenerationTool(payload),
                            stringify!(ImageGenerationTool),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<CustomConnector>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "document_library",
                                first_name,
                                stringify!(CustomConnector),
                            )));
                        }
                        structural_match =
                            Some((Self::CustomConnector(payload), stringify!(CustomConnector)));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "connector" => {
                    let primary_error =
                        match serde_json::from_value::<CustomConnector>(value.clone()) {
                            Ok(payload) => return Ok(Self::CustomConnector(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<FunctionTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "connector",
                                first_name,
                                stringify!(FunctionTool),
                            )));
                        }
                        structural_match =
                            Some((Self::FunctionTool(payload), stringify!(FunctionTool)));
                    }
                    if let Ok(payload) = serde_json::from_value::<WebSearchTool>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "connector",
                                first_name,
                                stringify!(WebSearchTool),
                            )));
                        }
                        structural_match =
                            Some((Self::WebSearchTool(payload), stringify!(WebSearchTool)));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<WebSearchPremiumTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "connector",
                                first_name,
                                stringify!(WebSearchPremiumTool),
                            )));
                        }
                        structural_match = Some((
                            Self::WebSearchPremiumTool(payload),
                            stringify!(WebSearchPremiumTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<CodeInterpreterTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "connector",
                                first_name,
                                stringify!(CodeInterpreterTool),
                            )));
                        }
                        structural_match = Some((
                            Self::CodeInterpreterTool(payload),
                            stringify!(CodeInterpreterTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<ImageGenerationTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "connector",
                                first_name,
                                stringify!(ImageGenerationTool),
                            )));
                        }
                        structural_match = Some((
                            Self::ImageGenerationTool(payload),
                            stringify!(ImageGenerationTool),
                        ));
                    }
                    if let Ok(payload) =
                        serde_json::from_value::<DocumentLibraryTool>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "connector",
                                first_name,
                                stringify!(DocumentLibraryTool),
                            )));
                        }
                        structural_match = Some((
                            Self::DocumentLibraryTool(payload),
                            stringify!(DocumentLibraryTool),
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
                if let Ok(payload) = serde_json::from_value::<FunctionTool>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(FunctionTool),
                        )));
                    }
                    structural_match =
                        Some((Self::FunctionTool(payload), stringify!(FunctionTool)));
                }
                if let Ok(payload) = serde_json::from_value::<WebSearchTool>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(WebSearchTool),
                        )));
                    }
                    structural_match =
                        Some((Self::WebSearchTool(payload), stringify!(WebSearchTool)));
                }
                if let Ok(payload) = serde_json::from_value::<WebSearchPremiumTool>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(WebSearchPremiumTool),
                        )));
                    }
                    structural_match = Some((
                        Self::WebSearchPremiumTool(payload),
                        stringify!(WebSearchPremiumTool),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<CodeInterpreterTool>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(CodeInterpreterTool),
                        )));
                    }
                    structural_match = Some((
                        Self::CodeInterpreterTool(payload),
                        stringify!(CodeInterpreterTool),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<ImageGenerationTool>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(ImageGenerationTool),
                        )));
                    }
                    structural_match = Some((
                        Self::ImageGenerationTool(payload),
                        stringify!(ImageGenerationTool),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<DocumentLibraryTool>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(DocumentLibraryTool),
                        )));
                    }
                    structural_match = Some((
                        Self::DocumentLibraryTool(payload),
                        stringify!(DocumentLibraryTool),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<CustomConnector>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(CustomConnector),
                        )));
                    }
                    structural_match =
                        Some((Self::CustomConnector(payload), stringify!(CustomConnector)));
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
