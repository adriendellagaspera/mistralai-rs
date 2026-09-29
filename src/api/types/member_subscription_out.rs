pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MemberSubscriptionOut {
    /// Type of subscription assigned to the member.
    pub r#type: PlanType,
    /// Plan assigned to the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<MemberSubscriptionOutPlan>,
    /// Current status of the member subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatus>,
    /// User ID associated with the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Whether the subscription was created through self-service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service: Option<bool>,
    /// Whether the member currently has access to the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_has_access: Option<bool>,
}

impl MemberSubscriptionOut {
    pub fn builder() -> MemberSubscriptionOutBuilder {
        <MemberSubscriptionOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberSubscriptionOutBuilder {
    r#type: Option<PlanType>,
    plan: Option<MemberSubscriptionOutPlan>,
    status: Option<SubscriptionStatus>,
    user: Option<String>,
    self_service: Option<bool>,
    member_has_access: Option<bool>,
}

impl MemberSubscriptionOutBuilder {
    pub fn r#type(mut self, value: PlanType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn plan(mut self, value: MemberSubscriptionOutPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn status(mut self, value: SubscriptionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    pub fn self_service(mut self, value: bool) -> Self {
        self.self_service = Some(value);
        self
    }

    pub fn member_has_access(mut self, value: bool) -> Self {
        self.member_has_access = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemberSubscriptionOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MemberSubscriptionOutBuilder::r#type)
    pub fn build(self) -> Result<MemberSubscriptionOut, BuildError> {
        Ok(MemberSubscriptionOut {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            plan: self.plan,
            status: self.status,
            user: self.user,
            self_service: self.self_service,
            member_has_access: self.member_has_access,
        })
    }
}
