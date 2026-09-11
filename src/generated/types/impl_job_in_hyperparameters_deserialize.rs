impl<'de> Deserialize<'de> for JobInHyperparameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        fn exact_json_integer(number: &serde_json::Number) -> Option<i128> {
            number
                .as_i64()
                .map(i128::from)
                .or_else(|| number.as_u64().map(i128::from))
        }
        fn json_numbers_have_same_value(
            encoded: &serde_json::Number,
            input: &serde_json::Number,
        ) -> bool {
            match (exact_json_integer(encoded), exact_json_integer(input)) {
                (Some(encoded), Some(input)) => encoded == input,
                (Some(encoded), None) => input.as_f64().is_some_and(|input| {
                    input.is_finite() && input.fract() == 0.0 && input as i128 == encoded
                }),
                (None, Some(input)) => encoded.as_f64().is_some_and(|encoded| {
                    encoded.is_finite() && encoded.fract() == 0.0 && encoded as i128 == input
                }),
                (None, None) => encoded.as_f64() == input.as_f64(),
            }
        }
        fn preserves_complete_json_input(
            encoded: &serde_json::Value,
            input: &serde_json::Value,
        ) -> bool {
            match (encoded, input) {
                (serde_json::Value::Object(encoded), serde_json::Value::Object(input)) => {
                    input.iter().all(|(key, value)| {
                        encoded.get(key).is_some_and(|encoded_value| {
                            preserves_complete_json_input(encoded_value, value)
                        })
                    })
                }
                (serde_json::Value::Array(encoded), serde_json::Value::Array(input)) => {
                    encoded.len() == input.len()
                        && encoded
                            .iter()
                            .zip(input)
                            .all(|(encoded, input)| preserves_complete_json_input(encoded, input))
                }
                (serde_json::Value::Number(encoded), serde_json::Value::Number(input)) => {
                    json_numbers_have_same_value(encoded, input)
                }
                _ => encoded == input,
            }
        }
        let input = <serde_json::Value as Deserialize>::deserialize(deserializer)?;
        if let Ok(candidate) =
            serde_json::from_value::<CompletionTrainingParametersIn>(input.clone())
        {
            let preserves_complete_input = serde_json::to_value(&candidate)
                .map(|encoded| preserves_complete_json_input(&encoded, &input))
                .unwrap_or(false);
            if preserves_complete_input {
                return Ok(Self::CompletionTrainingParametersIn(candidate));
            }
        }
        if let Ok(candidate) =
            serde_json::from_value::<ClassifierTrainingParametersIn>(input.clone())
        {
            let preserves_complete_input = serde_json::to_value(&candidate)
                .map(|encoded| preserves_complete_json_input(&encoded, &input))
                .unwrap_or(false);
            if preserves_complete_input {
                return Ok(Self::ClassifierTrainingParametersIn(candidate));
            }
        }
        Err(serde::de::Error::custom(concat!(
            "no anyOf branch for ",
            stringify!(JobInHyperparameters),
            " preserved the complete input",
        )))
    }
}
