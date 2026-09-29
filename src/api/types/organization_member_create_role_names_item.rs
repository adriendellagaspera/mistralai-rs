pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrganizationMemberCreateRoleNamesItem {
    Member,
    BillingManager,
    OrganizationAdmin,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OrganizationMemberCreateRoleNamesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Member => serializer.serialize_str("member"),
            Self::BillingManager => serializer.serialize_str("billing_manager"),
            Self::OrganizationAdmin => serializer.serialize_str("organization_admin"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OrganizationMemberCreateRoleNamesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "member" => Ok(Self::Member),
            "billing_manager" => Ok(Self::BillingManager),
            "organization_admin" => Ok(Self::OrganizationAdmin),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OrganizationMemberCreateRoleNamesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Member => write!(f, "member"),
            Self::BillingManager => write!(f, "billing_manager"),
            Self::OrganizationAdmin => write!(f, "organization_admin"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
