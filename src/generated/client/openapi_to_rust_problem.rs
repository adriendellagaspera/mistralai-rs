/// The generated validation-problem profile based on RFC 9457.
/// The distinctive namespace avoids collisions with user schemas.
pub mod openapi_to_rust_problem {
    #[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
    pub struct ProblemDetails {
        #[serde(rename = "type")]
        pub type_uri: String,
        pub title: String,
        pub status: u16,
        pub code: String,
        #[serde(default)]
        pub errors: Vec<InvalidParameter>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub detail: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub instance: Option<String>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
    pub struct InvalidParameter {
        pub code: String,
        pub location: String,
        pub message: String,
    }
}
