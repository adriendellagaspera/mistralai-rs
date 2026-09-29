pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkspaceMemberSingleInRoleNamesItem {
    Billing,
    User,
    Contributor,
    Dev,
    DevContributor,
    MistralCodeUser,
    CloudUser,
    WorkspaceContributor,
    WorkspaceAdmin,
    ObservabilityViewer,
    WorkflowExecutor,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkspaceMemberSingleInRoleNamesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Billing => serializer.serialize_str("billing"),
            Self::User => serializer.serialize_str("user"),
            Self::Contributor => serializer.serialize_str("contributor"),
            Self::Dev => serializer.serialize_str("dev"),
            Self::DevContributor => serializer.serialize_str("dev_contributor"),
            Self::MistralCodeUser => serializer.serialize_str("mistral_code_user"),
            Self::CloudUser => serializer.serialize_str("cloud_user"),
            Self::WorkspaceContributor => serializer.serialize_str("workspace_contributor"),
            Self::WorkspaceAdmin => serializer.serialize_str("workspace_admin"),
            Self::ObservabilityViewer => serializer.serialize_str("observability_viewer"),
            Self::WorkflowExecutor => serializer.serialize_str("workflow_executor"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkspaceMemberSingleInRoleNamesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "billing" => Ok(Self::Billing),
            "user" => Ok(Self::User),
            "contributor" => Ok(Self::Contributor),
            "dev" => Ok(Self::Dev),
            "dev_contributor" => Ok(Self::DevContributor),
            "mistral_code_user" => Ok(Self::MistralCodeUser),
            "cloud_user" => Ok(Self::CloudUser),
            "workspace_contributor" => Ok(Self::WorkspaceContributor),
            "workspace_admin" => Ok(Self::WorkspaceAdmin),
            "observability_viewer" => Ok(Self::ObservabilityViewer),
            "workflow_executor" => Ok(Self::WorkflowExecutor),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkspaceMemberSingleInRoleNamesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Billing => write!(f, "billing"),
            Self::User => write!(f, "user"),
            Self::Contributor => write!(f, "contributor"),
            Self::Dev => write!(f, "dev"),
            Self::DevContributor => write!(f, "dev_contributor"),
            Self::MistralCodeUser => write!(f, "mistral_code_user"),
            Self::CloudUser => write!(f, "cloud_user"),
            Self::WorkspaceContributor => write!(f, "workspace_contributor"),
            Self::WorkspaceAdmin => write!(f, "workspace_admin"),
            Self::ObservabilityViewer => write!(f, "observability_viewer"),
            Self::WorkflowExecutor => write!(f, "workflow_executor"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
