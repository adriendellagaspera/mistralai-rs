pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MemberSubscriptionOut {
    /// Whether the member currently has access to the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_has_access: Option<bool>,
    /// Plan assigned to the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<MemberSubscriptionOutPlan>,
    /// Whether the subscription was created through self-service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service: Option<bool>,
    /// Current status of the member subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatus>,
    /// Type of subscription assigned to the member.
    pub r#type: PlanType,
    /// User ID associated with the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl MemberSubscriptionOut {
    pub fn builder() -> MemberSubscriptionOutBuilder {
        <MemberSubscriptionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberSubscriptionOutBuilder {
    member_has_access: Option<bool>,
    plan: Option<MemberSubscriptionOutPlan>,
    self_service: Option<bool>,
    status: Option<SubscriptionStatus>,
    r#type: Option<PlanType>,
    user: Option<String>,
}

impl MemberSubscriptionOutBuilder {
    pub fn member_has_access(mut self, value: bool) -> Self {
        self.member_has_access = Some(value);
        self
    }

    pub fn plan(mut self, value: MemberSubscriptionOutPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn self_service(mut self, value: bool) -> Self {
        self.self_service = Some(value);
        self
    }

    pub fn status(mut self, value: SubscriptionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: PlanType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MemberSubscriptionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MemberSubscriptionOutBuilder::r#type)
    pub fn build(self) -> Result<MemberSubscriptionOut, BuildError> {
        Ok(MemberSubscriptionOut {
            member_has_access: self.member_has_access,
            plan: self.plan,
            self_service: self.self_service,
            status: self.status,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            user: self.user,
        })
    }
}
