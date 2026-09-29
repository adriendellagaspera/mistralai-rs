pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrganizationUsersCreateOut {
    /// Email addresses that could not be created.
    #[serde(default)]
    pub invalid_emails: Vec<String>,
    /// Mapping from created user email to user identifier.
    #[serde(default)]
    pub email_to_user_id: HashMap<String, String>,
}

impl OrganizationUsersCreateOut {
    pub fn builder() -> OrganizationUsersCreateOutBuilder {
        <OrganizationUsersCreateOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationUsersCreateOutBuilder {
    invalid_emails: Option<Vec<String>>,
    email_to_user_id: Option<HashMap<String, String>>,
}

impl OrganizationUsersCreateOutBuilder {
    pub fn invalid_emails(mut self, value: Vec<String>) -> Self {
        self.invalid_emails = Some(value);
        self
    }

    pub fn email_to_user_id(mut self, value: HashMap<String, String>) -> Self {
        self.email_to_user_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationUsersCreateOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invalid_emails`](OrganizationUsersCreateOutBuilder::invalid_emails)
    /// - [`email_to_user_id`](OrganizationUsersCreateOutBuilder::email_to_user_id)
    pub fn build(self) -> Result<OrganizationUsersCreateOut, BuildError> {
        Ok(OrganizationUsersCreateOut {
            invalid_emails: self
                .invalid_emails
                .ok_or_else(|| BuildError::missing_field("invalid_emails"))?,
            email_to_user_id: self
                .email_to_user_id
                .ok_or_else(|| BuildError::missing_field("email_to_user_id"))?,
        })
    }
}
