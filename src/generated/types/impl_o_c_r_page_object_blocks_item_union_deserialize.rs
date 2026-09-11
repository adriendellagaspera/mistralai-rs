impl<'de> serde::Deserialize<'de> for OCRPageObjectBlocksItemUnion {
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
                    let primary_error = match serde_json::from_value::<OCRTextBlock>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::OCRTextBlock(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "text",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "list" => {
                    let primary_error = match serde_json::from_value::<OCRListBlock>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::OCRListBlock(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "list",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "image" => {
                    let primary_error = match serde_json::from_value::<OCRImageBlock>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::OCRImageBlock(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "image",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "table" => {
                    let primary_error = match serde_json::from_value::<OCRTableBlock>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::OCRTableBlock(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "table",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "title" => {
                    let primary_error = match serde_json::from_value::<OCRTitleBlock>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::OCRTitleBlock(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "title",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "equation" => {
                    let primary_error =
                        match serde_json::from_value::<OCREquationBlock>(value.clone()) {
                            Ok(payload) => return Ok(Self::OCREquationBlock(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "equation",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "caption" => {
                    let primary_error =
                        match serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                            Ok(payload) => return Ok(Self::OCRCaptionBlock(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "caption",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "code" => {
                    let primary_error = match serde_json::from_value::<OCRCodeBlock>(value.clone())
                    {
                        Ok(payload) => return Ok(Self::OCRCodeBlock(payload)),
                        Err(error) => error,
                    };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "code",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "references" => {
                    let primary_error =
                        match serde_json::from_value::<OCRReferencesBlock>(value.clone()) {
                            Ok(payload) => return Ok(Self::OCRReferencesBlock(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "references",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "aside_text" => {
                    let primary_error =
                        match serde_json::from_value::<OCRAsideTextBlock>(value.clone()) {
                            Ok(payload) => return Ok(Self::OCRAsideTextBlock(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "aside_text",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "header" => {
                    let primary_error =
                        match serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                            Ok(payload) => return Ok(Self::OCRHeaderBlock(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "header",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "footer" => {
                    let primary_error =
                        match serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                            Ok(payload) => return Ok(Self::OCRFooterBlock(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "footer",
                                first_name,
                                stringify!(OCRSignatureBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRSignatureBlock(payload),
                            stringify!(OCRSignatureBlock),
                        ));
                    }
                    match structural_match {
                        Some((payload, _)) => Ok(payload),
                        None => Err(serde::de::Error::custom(primary_error)),
                    }
                }
                "signature" => {
                    let primary_error =
                        match serde_json::from_value::<OCRSignatureBlock>(value.clone()) {
                            Ok(payload) => return Ok(Self::OCRSignatureBlock(payload)),
                            Err(error) => error,
                        };
                    let mut structural_match: Option<(Self, &'static str)> = None;
                    if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRTextBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRListBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRImageBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRTableBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRTitleBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCREquationBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCREquationBlock(payload),
                            stringify!(OCREquationBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRCaptionBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRCodeBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRReferencesBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRReferencesBlock(payload),
                            stringify!(OCRReferencesBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone())
                    {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRAsideTextBlock),
                            )));
                        }
                        structural_match = Some((
                            Self::OCRAsideTextBlock(payload),
                            stringify!(OCRAsideTextBlock),
                        ));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRHeaderBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                    }
                    if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                        if let Some((_, first_name)) = &structural_match {
                            return Err(serde::de::Error::custom(format!(
                                "discriminator `{}` value `{}` did not fit its mapped branch and structurally matched both `{}` and `{}`",
                                "type",
                                "signature",
                                first_name,
                                stringify!(OCRFooterBlock),
                            )));
                        }
                        structural_match =
                            Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
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
                if let Ok(payload) = serde_json::from_value::<OCRTextBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRTextBlock),
                        )));
                    }
                    structural_match =
                        Some((Self::OCRTextBlock(payload), stringify!(OCRTextBlock)));
                }
                if let Ok(payload) = serde_json::from_value::<OCRListBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRListBlock),
                        )));
                    }
                    structural_match =
                        Some((Self::OCRListBlock(payload), stringify!(OCRListBlock)));
                }
                if let Ok(payload) = serde_json::from_value::<OCRImageBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRImageBlock),
                        )));
                    }
                    structural_match =
                        Some((Self::OCRImageBlock(payload), stringify!(OCRImageBlock)));
                }
                if let Ok(payload) = serde_json::from_value::<OCRTableBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRTableBlock),
                        )));
                    }
                    structural_match =
                        Some((Self::OCRTableBlock(payload), stringify!(OCRTableBlock)));
                }
                if let Ok(payload) = serde_json::from_value::<OCRTitleBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRTitleBlock),
                        )));
                    }
                    structural_match =
                        Some((Self::OCRTitleBlock(payload), stringify!(OCRTitleBlock)));
                }
                if let Ok(payload) = serde_json::from_value::<OCREquationBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCREquationBlock),
                        )));
                    }
                    structural_match = Some((
                        Self::OCREquationBlock(payload),
                        stringify!(OCREquationBlock),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<OCRCaptionBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRCaptionBlock),
                        )));
                    }
                    structural_match =
                        Some((Self::OCRCaptionBlock(payload), stringify!(OCRCaptionBlock)));
                }
                if let Ok(payload) = serde_json::from_value::<OCRCodeBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRCodeBlock),
                        )));
                    }
                    structural_match =
                        Some((Self::OCRCodeBlock(payload), stringify!(OCRCodeBlock)));
                }
                if let Ok(payload) = serde_json::from_value::<OCRReferencesBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRReferencesBlock),
                        )));
                    }
                    structural_match = Some((
                        Self::OCRReferencesBlock(payload),
                        stringify!(OCRReferencesBlock),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<OCRAsideTextBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRAsideTextBlock),
                        )));
                    }
                    structural_match = Some((
                        Self::OCRAsideTextBlock(payload),
                        stringify!(OCRAsideTextBlock),
                    ));
                }
                if let Ok(payload) = serde_json::from_value::<OCRHeaderBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRHeaderBlock),
                        )));
                    }
                    structural_match =
                        Some((Self::OCRHeaderBlock(payload), stringify!(OCRHeaderBlock)));
                }
                if let Ok(payload) = serde_json::from_value::<OCRFooterBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRFooterBlock),
                        )));
                    }
                    structural_match =
                        Some((Self::OCRFooterBlock(payload), stringify!(OCRFooterBlock)));
                }
                if let Ok(payload) = serde_json::from_value::<OCRSignatureBlock>(value.clone()) {
                    if let Some((_, first_name)) = &structural_match {
                        return Err(serde::de::Error::custom(format!(
                            "missing discriminator `{}` structurally matched both `{}` and `{}`",
                            "type",
                            first_name,
                            stringify!(OCRSignatureBlock),
                        )));
                    }
                    structural_match = Some((
                        Self::OCRSignatureBlock(payload),
                        stringify!(OCRSignatureBlock),
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
