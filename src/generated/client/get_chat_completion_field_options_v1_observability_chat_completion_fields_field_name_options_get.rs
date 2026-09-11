///Allowed values for the `operator` query parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetOperator
{
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lte")]
    Lte,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "gte")]
    Gte,
    #[serde(rename = "startswith")]
    Startswith,
    #[serde(rename = "istartswith")]
    Istartswith,
    #[serde(rename = "endswith")]
    Endswith,
    #[serde(rename = "iendswith")]
    Iendswith,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "icontains")]
    Icontains,
    #[serde(rename = "matches")]
    Matches,
    #[serde(rename = "notcontains")]
    Notcontains,
    #[serde(rename = "inotcontains")]
    Inotcontains,
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "neq")]
    Neq,
    #[serde(rename = "isnull")]
    Isnull,
    #[serde(rename = "includes")]
    Includes,
    #[serde(rename = "excludes")]
    Excludes,
    #[serde(rename = "len_eq")]
    LenEq,
}
