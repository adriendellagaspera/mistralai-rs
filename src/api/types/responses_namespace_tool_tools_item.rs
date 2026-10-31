pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ResponsesNamespaceToolToolsItem {
    #[serde(rename = "function")]
    #[non_exhaustive]
    Function {
        #[serde(skip_serializing_if = "Option::is_none")]
        allowed_callers: Option<Vec<ResponsesToolFunctionAllowedCallersItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        defer_loading: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_schema: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },

    #[serde(rename = "custom")]
    #[non_exhaustive]
    Custom {
        #[serde(skip_serializing_if = "Option::is_none")]
        allowed_callers: Option<Vec<ResponsesCustomToolAllowedCallersItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        defer_loading: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<ResponsesCustomToolFormat>,
        #[serde(default)]
        name: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ResponsesNamespaceToolToolsItem {
    pub fn function(name: String) -> Self {
        Self::Function {
            allowed_callers: None,
            defer_loading: None,
            description: None,
            name,
            output_schema: None,
            parameters: None,
            strict: None,
        }
    }

    pub fn custom(name: String) -> Self {
        Self::Custom {
            allowed_callers: None,
            defer_loading: None,
            description: None,
            format: None,
            name,
        }
    }

    pub fn function_with_allowed_callers(
        allowed_callers: Vec<ResponsesToolFunctionAllowedCallersItem>,
        defer_loading: Option<bool>,
        description: Option<String>,
        name: String,
        output_schema: Option<HashMap<String, serde_json::Value>>,
        parameters: Option<serde_json::Value>,
        strict: Option<bool>,
    ) -> Self {
        Self::Function {
            allowed_callers: Some(allowed_callers),
            defer_loading,
            description,
            name,
            output_schema,
            parameters,
            strict,
        }
    }

    pub fn function_with_defer_loading(
        allowed_callers: Option<Vec<ResponsesToolFunctionAllowedCallersItem>>,
        defer_loading: bool,
        description: Option<String>,
        name: String,
        output_schema: Option<HashMap<String, serde_json::Value>>,
        parameters: Option<serde_json::Value>,
        strict: Option<bool>,
    ) -> Self {
        Self::Function {
            allowed_callers,
            defer_loading: Some(defer_loading),
            description,
            name,
            output_schema,
            parameters,
            strict,
        }
    }

    pub fn function_with_description(
        allowed_callers: Option<Vec<ResponsesToolFunctionAllowedCallersItem>>,
        defer_loading: Option<bool>,
        description: String,
        name: String,
        output_schema: Option<HashMap<String, serde_json::Value>>,
        parameters: Option<serde_json::Value>,
        strict: Option<bool>,
    ) -> Self {
        Self::Function {
            allowed_callers,
            defer_loading,
            description: Some(description),
            name,
            output_schema,
            parameters,
            strict,
        }
    }

    pub fn function_with_output_schema(
        allowed_callers: Option<Vec<ResponsesToolFunctionAllowedCallersItem>>,
        defer_loading: Option<bool>,
        description: Option<String>,
        name: String,
        output_schema: HashMap<String, serde_json::Value>,
        parameters: Option<serde_json::Value>,
        strict: Option<bool>,
    ) -> Self {
        Self::Function {
            allowed_callers,
            defer_loading,
            description,
            name,
            output_schema: Some(output_schema),
            parameters,
            strict,
        }
    }

    pub fn function_with_parameters(
        allowed_callers: Option<Vec<ResponsesToolFunctionAllowedCallersItem>>,
        defer_loading: Option<bool>,
        description: Option<String>,
        name: String,
        output_schema: Option<HashMap<String, serde_json::Value>>,
        parameters: serde_json::Value,
        strict: Option<bool>,
    ) -> Self {
        Self::Function {
            allowed_callers,
            defer_loading,
            description,
            name,
            output_schema,
            parameters: Some(parameters),
            strict,
        }
    }

    pub fn function_with_strict(
        allowed_callers: Option<Vec<ResponsesToolFunctionAllowedCallersItem>>,
        defer_loading: Option<bool>,
        description: Option<String>,
        name: String,
        output_schema: Option<HashMap<String, serde_json::Value>>,
        parameters: Option<serde_json::Value>,
        strict: bool,
    ) -> Self {
        Self::Function {
            allowed_callers,
            defer_loading,
            description,
            name,
            output_schema,
            parameters,
            strict: Some(strict),
        }
    }

    pub fn custom_with_allowed_callers(
        allowed_callers: Vec<ResponsesCustomToolAllowedCallersItem>,
        defer_loading: Option<bool>,
        description: Option<String>,
        format: Option<ResponsesCustomToolFormat>,
        name: String,
    ) -> Self {
        Self::Custom {
            allowed_callers: Some(allowed_callers),
            defer_loading,
            description,
            format,
            name,
        }
    }

    pub fn custom_with_defer_loading(
        allowed_callers: Option<Vec<ResponsesCustomToolAllowedCallersItem>>,
        defer_loading: bool,
        description: Option<String>,
        format: Option<ResponsesCustomToolFormat>,
        name: String,
    ) -> Self {
        Self::Custom {
            allowed_callers,
            defer_loading: Some(defer_loading),
            description,
            format,
            name,
        }
    }

    pub fn custom_with_description(
        allowed_callers: Option<Vec<ResponsesCustomToolAllowedCallersItem>>,
        defer_loading: Option<bool>,
        description: String,
        format: Option<ResponsesCustomToolFormat>,
        name: String,
    ) -> Self {
        Self::Custom {
            allowed_callers,
            defer_loading,
            description: Some(description),
            format,
            name,
        }
    }

    pub fn custom_with_format(
        allowed_callers: Option<Vec<ResponsesCustomToolAllowedCallersItem>>,
        defer_loading: Option<bool>,
        description: Option<String>,
        format: ResponsesCustomToolFormat,
        name: String,
    ) -> Self {
        Self::Custom {
            allowed_callers,
            defer_loading,
            description,
            format: Some(format),
            name,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
