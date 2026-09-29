pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuditLogEventType {
    UserCreate,
    UserDelete,
    UserLogIn,
    UserInfoUpdate,
    UserPasswordUpdate,
    UserPhoneNumberVerify,
    UserOrganizationLeave,
    UserOrganizationRoleUpdate,
    UserOrganizationDelete,
    UserOrganizationJoin,
    OrganizationCreate,
    OrganizationUpdate,
    OrganizationInviteSend,
    OrganizationInviteResend,
    OrganizationInviteAccepted,
    OrganizationInviteRevoked,
    OrganizationJoinByEmailDomain,
    OrganizationDomainVerificationDisable,
    OrganizationDomainVerificationEnable,
    OrganizationEmailDomainAuthenticationEnable,
    OrganizationEmailDomainAuthenticationDisable,
    OrganizationSamlAuthenticationEnable,
    OrganizationSamlAuthenticationDisable,
    OrganizationSamlAuthenticationSsoUserProvisioningUpdate,
    OrganizationScimSyncTrigger,
    OrganizationSeatAutoAssignEnable,
    OrganizationSeatAutoAssignDisable,
    OrganizationKindUpdate,
    OrganizationSsoSeatAutoAssignEnable,
    OrganizationSsoSeatAutoAssignDisable,
    WorkspaceCreate,
    WorkspaceUpdate,
    WorkspaceDelete,
    WorkspaceMemberAdd,
    WorkspaceMemberRemove,
    WorkspaceMemberRoleUpdate,
    UserGroupCreate,
    UserGroupUpdate,
    UserGroupDelete,
    UserGroupMemberAdd,
    UserGroupMemberRemove,
    UserGroupWorkspaceProvision,
    UserGroupWorkspaceDeprovision,
    UserGroupWorkspaceUpdate,
    BillingInformationUpdated,
    BillingPaymentMethodAdded,
    BillingPaymentMethodRemoved,
    BillingPaymentMethodDefaultChanged,
    BillingInvoiceRetried,
    BillingSubscriptionSubscribe,
    BillingSubscriptionUnsubscribe,
    BillingSubscriptionCancelUnsubscribe,
    BillingSubscriptionAddSeats,
    BillingSubscriptionRemoveSeats,
    BillingSubscriptionTerminationDateUpdated,
    BillingCreditsAdded,
    BillingGiftCodeUsed,
    BillingMonthlyLimitUpdated,
    BillingWorkspaceMonthlyLimitUpdated,
    BillingSharedBudgetOverrideUpdated,
    BillingAutoRechargeUpdated,
    BillingSeatGrant,
    BillingSeatRevoke,
    BillingPriorityServiceTierUpdated,
    BillingPriorityServiceTierRemoved,
    LeChatConversationCreated,
    LeChatConversationDeleted,
    LeChatConversationBatchDeleted,
    LeChatConversationPublicSharingEnabled,
    LeChatConversationPublicSharingDisabled,
    LeChatFlashAnswersEnabled,
    LeChatFlashAnswersDisabled,
    LeChatLocalisationSharingEnabled,
    LeChatLocalisationSharingDisabled,
    LeChatMemoriesEnabled,
    LeChatMemoriesDisabled,
    LeChatDataTrainingEnabled,
    LeChatDataTrainingDisabled,
    LeChatActionsExternalLink,
    AdminApiKeyCreated,
    AdminApiKeyDelete,
    ApiKeyCreate,
    ApiKeyRotate,
    ApiKeyDelete,
    ApiKeyPolicyUpdate,
    SecretStoreEntryCreate,
    SecretStoreEntryUpdate,
    SecretStoreEntryDelete,
    ServiceAccountCreate,
    ServiceAccountUpdate,
    ServiceAccountDelete,
    ServiceAccountClientSecretCreate,
    ServiceAccountClientSecretDelete,
    ServiceAccountRolesSet,
    WorkloadIdentityCredentialRegistrationCreate,
    WorkloadIdentityCredentialCreate,
    TrustedIssuerCreate,
    TrustedIssuerUpdate,
    TrustedIssuerDelete,
    AgentCreate,
    AgentDelete,
    AgentUpdate,
    SkillCreate,
    SkillDelete,
    SkillEnable,
    SkillDisable,
    SkillUpdate,
    SkillShare,
    SkillUnshare,
    SkillLoad,
    SkillForceLoad,
    SkillVersionCreate,
    PromptCreate,
    PromptDelete,
    PromptUpdate,
    PromptVersionCreate,
    KnowledgeBaseCreate,
    KnowledgeBaseDelete,
    KnowledgeBaseUpdate,
    KnowledgeBaseVersionCreate,
    CustomVoiceCreate,
    CustomVoiceUpdate,
    CustomVoiceDelete,
    FeaturePermissionOverrideCreated,
    FeaturePermissionOverrideDeleted,
    ResourceShare,
    ResourceUnshare,
    LaPlateformeTrainingEnabled,
    LaPlateformeTrainingDisabled,
    CloudRuntimeAppCreated,
    CloudRuntimeAppUpdated,
    CloudRuntimeAppDeleted,
    CloudRuntimeAppPaused,
    CloudRuntimeAppResumed,
    CloudRuntimeDeploymentCreated,
    CloudRuntimeDeploymentSucceeded,
    CloudRuntimeDeploymentCanceled,
    CloudRuntimeDeploymentStopped,
    CloudRuntimeDeploymentFailed,
    CloudRuntimeDeploymentAutoscaled,
    CloudRuntimeDomainCreated,
    CloudRuntimeDomainUpdated,
    CloudRuntimeDomainDeleted,
    CloudRuntimeSecretCreated,
    CloudRuntimeSecretUpdated,
    CloudRuntimeSecretDeleted,
    CloudRuntimeServiceCreated,
    CloudRuntimeServicePaused,
    CloudRuntimeServiceResumed,
    CloudRuntimeServiceDeleted,
    CloudRuntimeServiceManuallyScaled,
    CloudRuntimeServiceManualScalingDeleted,
    CloudRuntimePersistentVolumeCreated,
    CloudRuntimePersistentVolumeDeleted,
    CloudRuntimePersistentVolumeAttached,
    CloudRuntimePersistentVolumeDetached,
    FineTuningJobCreate,
    FineTuningJobCancel,
    BatchJobCreate,
    BatchJobCancel,
    BatchJobDelete,
    DataCaptureExtractJobCreate,
    DataCaptureExtractJobCancel,
    DatasetCreate,
    DatasetDelete,
    LibraryCreate,
    LibraryDelete,
    LibraryUpdate,
    LibraryShare,
    LibraryUnshare,
    LibraryDocumentCreate,
    LibraryDocumentDelete,
    LibraryDocumentBulkDelete,
    LibraryDocumentUpdate,
    LibraryDocumentReprocess,
    IntegrationConnected,
    IntegrationDisconnected,
    IndexingWorkflowCompleted,
    IndexingDeleted,
    ConnectionAdminSetupIndex,
    ConnectionAdminDeleted,
    IntegrationActivatedForOrg,
    IntegrationDeactivatedForOrg,
    IntegrationActivatedForWorkspace,
    IntegrationDeactivatedForWorkspace,
    IntegrationActivatedForUser,
    IntegrationDeactivatedForUser,
    IntegrationCreated,
    IntegrationUpdated,
    IntegrationDeleted,
    IntegrationToolCalled,
    IntegrationCredentialsCreatedOrUpdated,
    IntegrationCredentialsDeleted,
    IntegrationCredentialsRevoked,
    IntegrationCredentialsRevocationFailed,
    IntegrationPreferencesCreatedOrUpdated,
    IntegrationPreferencesDeleted,
    IntegrationAuthenticationMethodCreatedOrUpdated,
    IntegrationConnectionCreated,
    IntegrationShared,
    IntegrationUnshared,
    ConnectorsGatewayToolCalled,
    ConnectorsDebuggerToolCalled,
    CrawlerConfigCreate,
    CrawlerConfigUpdate,
    CrawlerConfigDelete,
    CrawlerRunCreate,
    CrawlerRunCancel,
    RateLimitRuleCreate,
    RateLimitRuleUpdate,
    RateLimitRuleDelete,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AuditLogEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UserCreate => serializer.serialize_str("user.create"),
            Self::UserDelete => serializer.serialize_str("user.delete"),
            Self::UserLogIn => serializer.serialize_str("user.log_in"),
            Self::UserInfoUpdate => serializer.serialize_str("user.info.update"),
            Self::UserPasswordUpdate => serializer.serialize_str("user.password.update"),
            Self::UserPhoneNumberVerify => serializer.serialize_str("user.phone_number.verify"),
            Self::UserOrganizationLeave => serializer.serialize_str("user.organization.leave"),
            Self::UserOrganizationRoleUpdate => {
                serializer.serialize_str("user.organization.role.update")
            }
            Self::UserOrganizationDelete => serializer.serialize_str("user.organization.delete"),
            Self::UserOrganizationJoin => serializer.serialize_str("user.organization.join"),
            Self::OrganizationCreate => serializer.serialize_str("organization.create"),
            Self::OrganizationUpdate => serializer.serialize_str("organization.update"),
            Self::OrganizationInviteSend => serializer.serialize_str("organization.invite.send"),
            Self::OrganizationInviteResend => {
                serializer.serialize_str("organization.invite.resend")
            }
            Self::OrganizationInviteAccepted => {
                serializer.serialize_str("organization.invite.accepted")
            }
            Self::OrganizationInviteRevoked => {
                serializer.serialize_str("organization.invite.revoked")
            }
            Self::OrganizationJoinByEmailDomain => {
                serializer.serialize_str("organization.join_by_email_domain")
            }
            Self::OrganizationDomainVerificationDisable => {
                serializer.serialize_str("organization.domain_verification.disable")
            }
            Self::OrganizationDomainVerificationEnable => {
                serializer.serialize_str("organization.domain_verification.enable")
            }
            Self::OrganizationEmailDomainAuthenticationEnable => {
                serializer.serialize_str("organization.email_domain_authentication.enable")
            }
            Self::OrganizationEmailDomainAuthenticationDisable => {
                serializer.serialize_str("organization.email_domain_authentication.disable")
            }
            Self::OrganizationSamlAuthenticationEnable => {
                serializer.serialize_str("organization.saml_authentication.enable")
            }
            Self::OrganizationSamlAuthenticationDisable => {
                serializer.serialize_str("organization.saml_authentication.disable")
            }
            Self::OrganizationSamlAuthenticationSsoUserProvisioningUpdate => serializer
                .serialize_str("organization.saml_authentication.sso_user_provisioning.update"),
            Self::OrganizationScimSyncTrigger => {
                serializer.serialize_str("organization.scim_sync.trigger")
            }
            Self::OrganizationSeatAutoAssignEnable => {
                serializer.serialize_str("organization.seat_auto_assign.enable")
            }
            Self::OrganizationSeatAutoAssignDisable => {
                serializer.serialize_str("organization.seat_auto_assign.disable")
            }
            Self::OrganizationKindUpdate => serializer.serialize_str("organization.kind.update"),
            Self::OrganizationSsoSeatAutoAssignEnable => {
                serializer.serialize_str("organization.sso_seat_auto_assign.enable")
            }
            Self::OrganizationSsoSeatAutoAssignDisable => {
                serializer.serialize_str("organization.sso_seat_auto_assign.disable")
            }
            Self::WorkspaceCreate => serializer.serialize_str("workspace.create"),
            Self::WorkspaceUpdate => serializer.serialize_str("workspace.update"),
            Self::WorkspaceDelete => serializer.serialize_str("workspace.delete"),
            Self::WorkspaceMemberAdd => serializer.serialize_str("workspace.member.add"),
            Self::WorkspaceMemberRemove => serializer.serialize_str("workspace.member.remove"),
            Self::WorkspaceMemberRoleUpdate => {
                serializer.serialize_str("workspace.member.role.update")
            }
            Self::UserGroupCreate => serializer.serialize_str("user_group.create"),
            Self::UserGroupUpdate => serializer.serialize_str("user_group.update"),
            Self::UserGroupDelete => serializer.serialize_str("user_group.delete"),
            Self::UserGroupMemberAdd => serializer.serialize_str("user_group.member.add"),
            Self::UserGroupMemberRemove => serializer.serialize_str("user_group.member.remove"),
            Self::UserGroupWorkspaceProvision => {
                serializer.serialize_str("user_group.workspace.provision")
            }
            Self::UserGroupWorkspaceDeprovision => {
                serializer.serialize_str("user_group.workspace.deprovision")
            }
            Self::UserGroupWorkspaceUpdate => {
                serializer.serialize_str("user_group.workspace.update")
            }
            Self::BillingInformationUpdated => {
                serializer.serialize_str("billing.information.updated")
            }
            Self::BillingPaymentMethodAdded => {
                serializer.serialize_str("billing.payment_method.added")
            }
            Self::BillingPaymentMethodRemoved => {
                serializer.serialize_str("billing.payment_method.removed")
            }
            Self::BillingPaymentMethodDefaultChanged => {
                serializer.serialize_str("billing.payment_method.default_changed")
            }
            Self::BillingInvoiceRetried => serializer.serialize_str("billing.invoice.retried"),
            Self::BillingSubscriptionSubscribe => {
                serializer.serialize_str("billing.subscription.subscribe")
            }
            Self::BillingSubscriptionUnsubscribe => {
                serializer.serialize_str("billing.subscription.unsubscribe")
            }
            Self::BillingSubscriptionCancelUnsubscribe => {
                serializer.serialize_str("billing.subscription.cancel_unsubscribe")
            }
            Self::BillingSubscriptionAddSeats => {
                serializer.serialize_str("billing.subscription.add_seats")
            }
            Self::BillingSubscriptionRemoveSeats => {
                serializer.serialize_str("billing.subscription.remove_seats")
            }
            Self::BillingSubscriptionTerminationDateUpdated => {
                serializer.serialize_str("billing.subscription.termination_date.updated")
            }
            Self::BillingCreditsAdded => serializer.serialize_str("billing.credits.added"),
            Self::BillingGiftCodeUsed => serializer.serialize_str("billing.gift_code.used"),
            Self::BillingMonthlyLimitUpdated => {
                serializer.serialize_str("billing.monthly_limit.updated")
            }
            Self::BillingWorkspaceMonthlyLimitUpdated => {
                serializer.serialize_str("billing.workspace_monthly_limit.updated")
            }
            Self::BillingSharedBudgetOverrideUpdated => {
                serializer.serialize_str("billing.shared_budget_override.updated")
            }
            Self::BillingAutoRechargeUpdated => {
                serializer.serialize_str("billing.auto_recharge.updated")
            }
            Self::BillingSeatGrant => serializer.serialize_str("billing.seat.grant"),
            Self::BillingSeatRevoke => serializer.serialize_str("billing.seat.revoke"),
            Self::BillingPriorityServiceTierUpdated => {
                serializer.serialize_str("billing.priority_service_tier.updated")
            }
            Self::BillingPriorityServiceTierRemoved => {
                serializer.serialize_str("billing.priority_service_tier.removed")
            }
            Self::LeChatConversationCreated => {
                serializer.serialize_str("le_chat.conversation.created")
            }
            Self::LeChatConversationDeleted => {
                serializer.serialize_str("le_chat.conversation.deleted")
            }
            Self::LeChatConversationBatchDeleted => {
                serializer.serialize_str("le_chat.conversation_batch.deleted")
            }
            Self::LeChatConversationPublicSharingEnabled => {
                serializer.serialize_str("le_chat.conversation.public_sharing.enabled")
            }
            Self::LeChatConversationPublicSharingDisabled => {
                serializer.serialize_str("le_chat.conversation.public_sharing.disabled")
            }
            Self::LeChatFlashAnswersEnabled => {
                serializer.serialize_str("le_chat.flash_answers.enabled")
            }
            Self::LeChatFlashAnswersDisabled => {
                serializer.serialize_str("le_chat.flash_answers.disabled")
            }
            Self::LeChatLocalisationSharingEnabled => {
                serializer.serialize_str("le_chat.localisation_sharing.enabled")
            }
            Self::LeChatLocalisationSharingDisabled => {
                serializer.serialize_str("le_chat.localisation_sharing.disabled")
            }
            Self::LeChatMemoriesEnabled => serializer.serialize_str("le_chat.memories.enabled"),
            Self::LeChatMemoriesDisabled => serializer.serialize_str("le_chat.memories.disabled"),
            Self::LeChatDataTrainingEnabled => {
                serializer.serialize_str("le_chat.data.training_enabled")
            }
            Self::LeChatDataTrainingDisabled => {
                serializer.serialize_str("le_chat.data.training_disabled")
            }
            Self::LeChatActionsExternalLink => {
                serializer.serialize_str("le_chat.actions.external_link")
            }
            Self::AdminApiKeyCreated => serializer.serialize_str("admin_api_key.created"),
            Self::AdminApiKeyDelete => serializer.serialize_str("admin_api_key.delete"),
            Self::ApiKeyCreate => serializer.serialize_str("api_key.create"),
            Self::ApiKeyRotate => serializer.serialize_str("api_key.rotate"),
            Self::ApiKeyDelete => serializer.serialize_str("api_key.delete"),
            Self::ApiKeyPolicyUpdate => serializer.serialize_str("api_key_policy.update"),
            Self::SecretStoreEntryCreate => serializer.serialize_str("secret_store.entry.create"),
            Self::SecretStoreEntryUpdate => serializer.serialize_str("secret_store.entry.update"),
            Self::SecretStoreEntryDelete => serializer.serialize_str("secret_store.entry.delete"),
            Self::ServiceAccountCreate => serializer.serialize_str("service_account.create"),
            Self::ServiceAccountUpdate => serializer.serialize_str("service_account.update"),
            Self::ServiceAccountDelete => serializer.serialize_str("service_account.delete"),
            Self::ServiceAccountClientSecretCreate => {
                serializer.serialize_str("service_account.client_secret.create")
            }
            Self::ServiceAccountClientSecretDelete => {
                serializer.serialize_str("service_account.client_secret.delete")
            }
            Self::ServiceAccountRolesSet => serializer.serialize_str("service_account.roles.set"),
            Self::WorkloadIdentityCredentialRegistrationCreate => {
                serializer.serialize_str("workload_identity.credential_registration.create")
            }
            Self::WorkloadIdentityCredentialCreate => {
                serializer.serialize_str("workload_identity.credential.create")
            }
            Self::TrustedIssuerCreate => serializer.serialize_str("trusted_issuer.create"),
            Self::TrustedIssuerUpdate => serializer.serialize_str("trusted_issuer.update"),
            Self::TrustedIssuerDelete => serializer.serialize_str("trusted_issuer.delete"),
            Self::AgentCreate => serializer.serialize_str("agent.create"),
            Self::AgentDelete => serializer.serialize_str("agent.delete"),
            Self::AgentUpdate => serializer.serialize_str("agent.update"),
            Self::SkillCreate => serializer.serialize_str("skill.create"),
            Self::SkillDelete => serializer.serialize_str("skill.delete"),
            Self::SkillEnable => serializer.serialize_str("skill.enable"),
            Self::SkillDisable => serializer.serialize_str("skill.disable"),
            Self::SkillUpdate => serializer.serialize_str("skill.update"),
            Self::SkillShare => serializer.serialize_str("skill.share"),
            Self::SkillUnshare => serializer.serialize_str("skill.unshare"),
            Self::SkillLoad => serializer.serialize_str("skill.load"),
            Self::SkillForceLoad => serializer.serialize_str("skill.force_load"),
            Self::SkillVersionCreate => serializer.serialize_str("skill.version.create"),
            Self::PromptCreate => serializer.serialize_str("prompt.create"),
            Self::PromptDelete => serializer.serialize_str("prompt.delete"),
            Self::PromptUpdate => serializer.serialize_str("prompt.update"),
            Self::PromptVersionCreate => serializer.serialize_str("prompt.version.create"),
            Self::KnowledgeBaseCreate => serializer.serialize_str("knowledge_base.create"),
            Self::KnowledgeBaseDelete => serializer.serialize_str("knowledge_base.delete"),
            Self::KnowledgeBaseUpdate => serializer.serialize_str("knowledge_base.update"),
            Self::KnowledgeBaseVersionCreate => {
                serializer.serialize_str("knowledge_base.version.create")
            }
            Self::CustomVoiceCreate => serializer.serialize_str("custom_voice.create"),
            Self::CustomVoiceUpdate => serializer.serialize_str("custom_voice.update"),
            Self::CustomVoiceDelete => serializer.serialize_str("custom_voice.delete"),
            Self::FeaturePermissionOverrideCreated => {
                serializer.serialize_str("feature_permission.override.created")
            }
            Self::FeaturePermissionOverrideDeleted => {
                serializer.serialize_str("feature_permission.override.deleted")
            }
            Self::ResourceShare => serializer.serialize_str("resource.share"),
            Self::ResourceUnshare => serializer.serialize_str("resource.unshare"),
            Self::LaPlateformeTrainingEnabled => {
                serializer.serialize_str("la_plateforme.training.enabled")
            }
            Self::LaPlateformeTrainingDisabled => {
                serializer.serialize_str("la_plateforme.training.disabled")
            }
            Self::CloudRuntimeAppCreated => serializer.serialize_str("cloud_runtime.app.created"),
            Self::CloudRuntimeAppUpdated => serializer.serialize_str("cloud_runtime.app.updated"),
            Self::CloudRuntimeAppDeleted => serializer.serialize_str("cloud_runtime.app.deleted"),
            Self::CloudRuntimeAppPaused => serializer.serialize_str("cloud_runtime.app.paused"),
            Self::CloudRuntimeAppResumed => serializer.serialize_str("cloud_runtime.app.resumed"),
            Self::CloudRuntimeDeploymentCreated => {
                serializer.serialize_str("cloud_runtime.deployment.created")
            }
            Self::CloudRuntimeDeploymentSucceeded => {
                serializer.serialize_str("cloud_runtime.deployment.succeeded")
            }
            Self::CloudRuntimeDeploymentCanceled => {
                serializer.serialize_str("cloud_runtime.deployment.canceled")
            }
            Self::CloudRuntimeDeploymentStopped => {
                serializer.serialize_str("cloud_runtime.deployment.stopped")
            }
            Self::CloudRuntimeDeploymentFailed => {
                serializer.serialize_str("cloud_runtime.deployment.failed")
            }
            Self::CloudRuntimeDeploymentAutoscaled => {
                serializer.serialize_str("cloud_runtime.deployment.autoscaled")
            }
            Self::CloudRuntimeDomainCreated => {
                serializer.serialize_str("cloud_runtime.domain.created")
            }
            Self::CloudRuntimeDomainUpdated => {
                serializer.serialize_str("cloud_runtime.domain.updated")
            }
            Self::CloudRuntimeDomainDeleted => {
                serializer.serialize_str("cloud_runtime.domain.deleted")
            }
            Self::CloudRuntimeSecretCreated => {
                serializer.serialize_str("cloud_runtime.secret.created")
            }
            Self::CloudRuntimeSecretUpdated => {
                serializer.serialize_str("cloud_runtime.secret.updated")
            }
            Self::CloudRuntimeSecretDeleted => {
                serializer.serialize_str("cloud_runtime.secret.deleted")
            }
            Self::CloudRuntimeServiceCreated => {
                serializer.serialize_str("cloud_runtime.service.created")
            }
            Self::CloudRuntimeServicePaused => {
                serializer.serialize_str("cloud_runtime.service.paused")
            }
            Self::CloudRuntimeServiceResumed => {
                serializer.serialize_str("cloud_runtime.service.resumed")
            }
            Self::CloudRuntimeServiceDeleted => {
                serializer.serialize_str("cloud_runtime.service.deleted")
            }
            Self::CloudRuntimeServiceManuallyScaled => {
                serializer.serialize_str("cloud_runtime.service.manually_scaled")
            }
            Self::CloudRuntimeServiceManualScalingDeleted => {
                serializer.serialize_str("cloud_runtime.service.manual_scaling_deleted")
            }
            Self::CloudRuntimePersistentVolumeCreated => {
                serializer.serialize_str("cloud_runtime.persistent_volume.created")
            }
            Self::CloudRuntimePersistentVolumeDeleted => {
                serializer.serialize_str("cloud_runtime.persistent_volume.deleted")
            }
            Self::CloudRuntimePersistentVolumeAttached => {
                serializer.serialize_str("cloud_runtime.persistent_volume.attached")
            }
            Self::CloudRuntimePersistentVolumeDetached => {
                serializer.serialize_str("cloud_runtime.persistent_volume.detached")
            }
            Self::FineTuningJobCreate => serializer.serialize_str("fine_tuning_job.create"),
            Self::FineTuningJobCancel => serializer.serialize_str("fine_tuning_job.cancel"),
            Self::BatchJobCreate => serializer.serialize_str("batch_job.create"),
            Self::BatchJobCancel => serializer.serialize_str("batch_job.cancel"),
            Self::BatchJobDelete => serializer.serialize_str("batch_job.delete"),
            Self::DataCaptureExtractJobCreate => {
                serializer.serialize_str("data_capture.extract_job.create")
            }
            Self::DataCaptureExtractJobCancel => {
                serializer.serialize_str("data_capture.extract_job.cancel")
            }
            Self::DatasetCreate => serializer.serialize_str("dataset.create"),
            Self::DatasetDelete => serializer.serialize_str("dataset.delete"),
            Self::LibraryCreate => serializer.serialize_str("library.create"),
            Self::LibraryDelete => serializer.serialize_str("library.delete"),
            Self::LibraryUpdate => serializer.serialize_str("library.update"),
            Self::LibraryShare => serializer.serialize_str("library.share"),
            Self::LibraryUnshare => serializer.serialize_str("library.unshare"),
            Self::LibraryDocumentCreate => serializer.serialize_str("library.document.create"),
            Self::LibraryDocumentDelete => serializer.serialize_str("library.document.delete"),
            Self::LibraryDocumentBulkDelete => {
                serializer.serialize_str("library.document.bulk_delete")
            }
            Self::LibraryDocumentUpdate => serializer.serialize_str("library.document.update"),
            Self::LibraryDocumentReprocess => {
                serializer.serialize_str("library.document.reprocess")
            }
            Self::IntegrationConnected => serializer.serialize_str("integration.connected"),
            Self::IntegrationDisconnected => serializer.serialize_str("integration.disconnected"),
            Self::IndexingWorkflowCompleted => {
                serializer.serialize_str("indexing.workflow.completed")
            }
            Self::IndexingDeleted => serializer.serialize_str("indexing.deleted"),
            Self::ConnectionAdminSetupIndex => {
                serializer.serialize_str("connection.admin.setup_index")
            }
            Self::ConnectionAdminDeleted => serializer.serialize_str("connection.admin.deleted"),
            Self::IntegrationActivatedForOrg => {
                serializer.serialize_str("integration.activated_for_org")
            }
            Self::IntegrationDeactivatedForOrg => {
                serializer.serialize_str("integration.deactivated_for_org")
            }
            Self::IntegrationActivatedForWorkspace => {
                serializer.serialize_str("integration.activated_for_workspace")
            }
            Self::IntegrationDeactivatedForWorkspace => {
                serializer.serialize_str("integration.deactivated_for_workspace")
            }
            Self::IntegrationActivatedForUser => {
                serializer.serialize_str("integration.activated_for_user")
            }
            Self::IntegrationDeactivatedForUser => {
                serializer.serialize_str("integration.deactivated_for_user")
            }
            Self::IntegrationCreated => serializer.serialize_str("integration.created"),
            Self::IntegrationUpdated => serializer.serialize_str("integration.updated"),
            Self::IntegrationDeleted => serializer.serialize_str("integration.deleted"),
            Self::IntegrationToolCalled => serializer.serialize_str("integration.tool_called"),
            Self::IntegrationCredentialsCreatedOrUpdated => {
                serializer.serialize_str("integration.credentials.created_or_updated")
            }
            Self::IntegrationCredentialsDeleted => {
                serializer.serialize_str("integration.credentials.deleted")
            }
            Self::IntegrationCredentialsRevoked => {
                serializer.serialize_str("integration.credentials.revoked")
            }
            Self::IntegrationCredentialsRevocationFailed => {
                serializer.serialize_str("integration.credentials.revocation_failed")
            }
            Self::IntegrationPreferencesCreatedOrUpdated => {
                serializer.serialize_str("integration.preferences.created_or_updated")
            }
            Self::IntegrationPreferencesDeleted => {
                serializer.serialize_str("integration.preferences.deleted")
            }
            Self::IntegrationAuthenticationMethodCreatedOrUpdated => {
                serializer.serialize_str("integration.authentication_method.created_or_updated")
            }
            Self::IntegrationConnectionCreated => {
                serializer.serialize_str("integration.connection.created")
            }
            Self::IntegrationShared => serializer.serialize_str("integration.shared"),
            Self::IntegrationUnshared => serializer.serialize_str("integration.unshared"),
            Self::ConnectorsGatewayToolCalled => {
                serializer.serialize_str("connectors_gateway.tool_called")
            }
            Self::ConnectorsDebuggerToolCalled => {
                serializer.serialize_str("connectors_debugger.tool_called")
            }
            Self::CrawlerConfigCreate => serializer.serialize_str("crawler.config.create"),
            Self::CrawlerConfigUpdate => serializer.serialize_str("crawler.config.update"),
            Self::CrawlerConfigDelete => serializer.serialize_str("crawler.config.delete"),
            Self::CrawlerRunCreate => serializer.serialize_str("crawler.run.create"),
            Self::CrawlerRunCancel => serializer.serialize_str("crawler.run.cancel"),
            Self::RateLimitRuleCreate => serializer.serialize_str("rate_limit.rule.create"),
            Self::RateLimitRuleUpdate => serializer.serialize_str("rate_limit.rule.update"),
            Self::RateLimitRuleDelete => serializer.serialize_str("rate_limit.rule.delete"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AuditLogEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user.create" => Ok(Self::UserCreate),
            "user.delete" => Ok(Self::UserDelete),
            "user.log_in" => Ok(Self::UserLogIn),
            "user.info.update" => Ok(Self::UserInfoUpdate),
            "user.password.update" => Ok(Self::UserPasswordUpdate),
            "user.phone_number.verify" => Ok(Self::UserPhoneNumberVerify),
            "user.organization.leave" => Ok(Self::UserOrganizationLeave),
            "user.organization.role.update" => Ok(Self::UserOrganizationRoleUpdate),
            "user.organization.delete" => Ok(Self::UserOrganizationDelete),
            "user.organization.join" => Ok(Self::UserOrganizationJoin),
            "organization.create" => Ok(Self::OrganizationCreate),
            "organization.update" => Ok(Self::OrganizationUpdate),
            "organization.invite.send" => Ok(Self::OrganizationInviteSend),
            "organization.invite.resend" => Ok(Self::OrganizationInviteResend),
            "organization.invite.accepted" => Ok(Self::OrganizationInviteAccepted),
            "organization.invite.revoked" => Ok(Self::OrganizationInviteRevoked),
            "organization.join_by_email_domain" => Ok(Self::OrganizationJoinByEmailDomain),
            "organization.domain_verification.disable" => {
                Ok(Self::OrganizationDomainVerificationDisable)
            }
            "organization.domain_verification.enable" => {
                Ok(Self::OrganizationDomainVerificationEnable)
            }
            "organization.email_domain_authentication.enable" => {
                Ok(Self::OrganizationEmailDomainAuthenticationEnable)
            }
            "organization.email_domain_authentication.disable" => {
                Ok(Self::OrganizationEmailDomainAuthenticationDisable)
            }
            "organization.saml_authentication.enable" => {
                Ok(Self::OrganizationSamlAuthenticationEnable)
            }
            "organization.saml_authentication.disable" => {
                Ok(Self::OrganizationSamlAuthenticationDisable)
            }
            "organization.saml_authentication.sso_user_provisioning.update" => {
                Ok(Self::OrganizationSamlAuthenticationSsoUserProvisioningUpdate)
            }
            "organization.scim_sync.trigger" => Ok(Self::OrganizationScimSyncTrigger),
            "organization.seat_auto_assign.enable" => Ok(Self::OrganizationSeatAutoAssignEnable),
            "organization.seat_auto_assign.disable" => Ok(Self::OrganizationSeatAutoAssignDisable),
            "organization.kind.update" => Ok(Self::OrganizationKindUpdate),
            "organization.sso_seat_auto_assign.enable" => {
                Ok(Self::OrganizationSsoSeatAutoAssignEnable)
            }
            "organization.sso_seat_auto_assign.disable" => {
                Ok(Self::OrganizationSsoSeatAutoAssignDisable)
            }
            "workspace.create" => Ok(Self::WorkspaceCreate),
            "workspace.update" => Ok(Self::WorkspaceUpdate),
            "workspace.delete" => Ok(Self::WorkspaceDelete),
            "workspace.member.add" => Ok(Self::WorkspaceMemberAdd),
            "workspace.member.remove" => Ok(Self::WorkspaceMemberRemove),
            "workspace.member.role.update" => Ok(Self::WorkspaceMemberRoleUpdate),
            "user_group.create" => Ok(Self::UserGroupCreate),
            "user_group.update" => Ok(Self::UserGroupUpdate),
            "user_group.delete" => Ok(Self::UserGroupDelete),
            "user_group.member.add" => Ok(Self::UserGroupMemberAdd),
            "user_group.member.remove" => Ok(Self::UserGroupMemberRemove),
            "user_group.workspace.provision" => Ok(Self::UserGroupWorkspaceProvision),
            "user_group.workspace.deprovision" => Ok(Self::UserGroupWorkspaceDeprovision),
            "user_group.workspace.update" => Ok(Self::UserGroupWorkspaceUpdate),
            "billing.information.updated" => Ok(Self::BillingInformationUpdated),
            "billing.payment_method.added" => Ok(Self::BillingPaymentMethodAdded),
            "billing.payment_method.removed" => Ok(Self::BillingPaymentMethodRemoved),
            "billing.payment_method.default_changed" => {
                Ok(Self::BillingPaymentMethodDefaultChanged)
            }
            "billing.invoice.retried" => Ok(Self::BillingInvoiceRetried),
            "billing.subscription.subscribe" => Ok(Self::BillingSubscriptionSubscribe),
            "billing.subscription.unsubscribe" => Ok(Self::BillingSubscriptionUnsubscribe),
            "billing.subscription.cancel_unsubscribe" => {
                Ok(Self::BillingSubscriptionCancelUnsubscribe)
            }
            "billing.subscription.add_seats" => Ok(Self::BillingSubscriptionAddSeats),
            "billing.subscription.remove_seats" => Ok(Self::BillingSubscriptionRemoveSeats),
            "billing.subscription.termination_date.updated" => {
                Ok(Self::BillingSubscriptionTerminationDateUpdated)
            }
            "billing.credits.added" => Ok(Self::BillingCreditsAdded),
            "billing.gift_code.used" => Ok(Self::BillingGiftCodeUsed),
            "billing.monthly_limit.updated" => Ok(Self::BillingMonthlyLimitUpdated),
            "billing.workspace_monthly_limit.updated" => {
                Ok(Self::BillingWorkspaceMonthlyLimitUpdated)
            }
            "billing.shared_budget_override.updated" => {
                Ok(Self::BillingSharedBudgetOverrideUpdated)
            }
            "billing.auto_recharge.updated" => Ok(Self::BillingAutoRechargeUpdated),
            "billing.seat.grant" => Ok(Self::BillingSeatGrant),
            "billing.seat.revoke" => Ok(Self::BillingSeatRevoke),
            "billing.priority_service_tier.updated" => Ok(Self::BillingPriorityServiceTierUpdated),
            "billing.priority_service_tier.removed" => Ok(Self::BillingPriorityServiceTierRemoved),
            "le_chat.conversation.created" => Ok(Self::LeChatConversationCreated),
            "le_chat.conversation.deleted" => Ok(Self::LeChatConversationDeleted),
            "le_chat.conversation_batch.deleted" => Ok(Self::LeChatConversationBatchDeleted),
            "le_chat.conversation.public_sharing.enabled" => {
                Ok(Self::LeChatConversationPublicSharingEnabled)
            }
            "le_chat.conversation.public_sharing.disabled" => {
                Ok(Self::LeChatConversationPublicSharingDisabled)
            }
            "le_chat.flash_answers.enabled" => Ok(Self::LeChatFlashAnswersEnabled),
            "le_chat.flash_answers.disabled" => Ok(Self::LeChatFlashAnswersDisabled),
            "le_chat.localisation_sharing.enabled" => Ok(Self::LeChatLocalisationSharingEnabled),
            "le_chat.localisation_sharing.disabled" => Ok(Self::LeChatLocalisationSharingDisabled),
            "le_chat.memories.enabled" => Ok(Self::LeChatMemoriesEnabled),
            "le_chat.memories.disabled" => Ok(Self::LeChatMemoriesDisabled),
            "le_chat.data.training_enabled" => Ok(Self::LeChatDataTrainingEnabled),
            "le_chat.data.training_disabled" => Ok(Self::LeChatDataTrainingDisabled),
            "le_chat.actions.external_link" => Ok(Self::LeChatActionsExternalLink),
            "admin_api_key.created" => Ok(Self::AdminApiKeyCreated),
            "admin_api_key.delete" => Ok(Self::AdminApiKeyDelete),
            "api_key.create" => Ok(Self::ApiKeyCreate),
            "api_key.rotate" => Ok(Self::ApiKeyRotate),
            "api_key.delete" => Ok(Self::ApiKeyDelete),
            "api_key_policy.update" => Ok(Self::ApiKeyPolicyUpdate),
            "secret_store.entry.create" => Ok(Self::SecretStoreEntryCreate),
            "secret_store.entry.update" => Ok(Self::SecretStoreEntryUpdate),
            "secret_store.entry.delete" => Ok(Self::SecretStoreEntryDelete),
            "service_account.create" => Ok(Self::ServiceAccountCreate),
            "service_account.update" => Ok(Self::ServiceAccountUpdate),
            "service_account.delete" => Ok(Self::ServiceAccountDelete),
            "service_account.client_secret.create" => Ok(Self::ServiceAccountClientSecretCreate),
            "service_account.client_secret.delete" => Ok(Self::ServiceAccountClientSecretDelete),
            "service_account.roles.set" => Ok(Self::ServiceAccountRolesSet),
            "workload_identity.credential_registration.create" => {
                Ok(Self::WorkloadIdentityCredentialRegistrationCreate)
            }
            "workload_identity.credential.create" => Ok(Self::WorkloadIdentityCredentialCreate),
            "trusted_issuer.create" => Ok(Self::TrustedIssuerCreate),
            "trusted_issuer.update" => Ok(Self::TrustedIssuerUpdate),
            "trusted_issuer.delete" => Ok(Self::TrustedIssuerDelete),
            "agent.create" => Ok(Self::AgentCreate),
            "agent.delete" => Ok(Self::AgentDelete),
            "agent.update" => Ok(Self::AgentUpdate),
            "skill.create" => Ok(Self::SkillCreate),
            "skill.delete" => Ok(Self::SkillDelete),
            "skill.enable" => Ok(Self::SkillEnable),
            "skill.disable" => Ok(Self::SkillDisable),
            "skill.update" => Ok(Self::SkillUpdate),
            "skill.share" => Ok(Self::SkillShare),
            "skill.unshare" => Ok(Self::SkillUnshare),
            "skill.load" => Ok(Self::SkillLoad),
            "skill.force_load" => Ok(Self::SkillForceLoad),
            "skill.version.create" => Ok(Self::SkillVersionCreate),
            "prompt.create" => Ok(Self::PromptCreate),
            "prompt.delete" => Ok(Self::PromptDelete),
            "prompt.update" => Ok(Self::PromptUpdate),
            "prompt.version.create" => Ok(Self::PromptVersionCreate),
            "knowledge_base.create" => Ok(Self::KnowledgeBaseCreate),
            "knowledge_base.delete" => Ok(Self::KnowledgeBaseDelete),
            "knowledge_base.update" => Ok(Self::KnowledgeBaseUpdate),
            "knowledge_base.version.create" => Ok(Self::KnowledgeBaseVersionCreate),
            "custom_voice.create" => Ok(Self::CustomVoiceCreate),
            "custom_voice.update" => Ok(Self::CustomVoiceUpdate),
            "custom_voice.delete" => Ok(Self::CustomVoiceDelete),
            "feature_permission.override.created" => Ok(Self::FeaturePermissionOverrideCreated),
            "feature_permission.override.deleted" => Ok(Self::FeaturePermissionOverrideDeleted),
            "resource.share" => Ok(Self::ResourceShare),
            "resource.unshare" => Ok(Self::ResourceUnshare),
            "la_plateforme.training.enabled" => Ok(Self::LaPlateformeTrainingEnabled),
            "la_plateforme.training.disabled" => Ok(Self::LaPlateformeTrainingDisabled),
            "cloud_runtime.app.created" => Ok(Self::CloudRuntimeAppCreated),
            "cloud_runtime.app.updated" => Ok(Self::CloudRuntimeAppUpdated),
            "cloud_runtime.app.deleted" => Ok(Self::CloudRuntimeAppDeleted),
            "cloud_runtime.app.paused" => Ok(Self::CloudRuntimeAppPaused),
            "cloud_runtime.app.resumed" => Ok(Self::CloudRuntimeAppResumed),
            "cloud_runtime.deployment.created" => Ok(Self::CloudRuntimeDeploymentCreated),
            "cloud_runtime.deployment.succeeded" => Ok(Self::CloudRuntimeDeploymentSucceeded),
            "cloud_runtime.deployment.canceled" => Ok(Self::CloudRuntimeDeploymentCanceled),
            "cloud_runtime.deployment.stopped" => Ok(Self::CloudRuntimeDeploymentStopped),
            "cloud_runtime.deployment.failed" => Ok(Self::CloudRuntimeDeploymentFailed),
            "cloud_runtime.deployment.autoscaled" => Ok(Self::CloudRuntimeDeploymentAutoscaled),
            "cloud_runtime.domain.created" => Ok(Self::CloudRuntimeDomainCreated),
            "cloud_runtime.domain.updated" => Ok(Self::CloudRuntimeDomainUpdated),
            "cloud_runtime.domain.deleted" => Ok(Self::CloudRuntimeDomainDeleted),
            "cloud_runtime.secret.created" => Ok(Self::CloudRuntimeSecretCreated),
            "cloud_runtime.secret.updated" => Ok(Self::CloudRuntimeSecretUpdated),
            "cloud_runtime.secret.deleted" => Ok(Self::CloudRuntimeSecretDeleted),
            "cloud_runtime.service.created" => Ok(Self::CloudRuntimeServiceCreated),
            "cloud_runtime.service.paused" => Ok(Self::CloudRuntimeServicePaused),
            "cloud_runtime.service.resumed" => Ok(Self::CloudRuntimeServiceResumed),
            "cloud_runtime.service.deleted" => Ok(Self::CloudRuntimeServiceDeleted),
            "cloud_runtime.service.manually_scaled" => Ok(Self::CloudRuntimeServiceManuallyScaled),
            "cloud_runtime.service.manual_scaling_deleted" => {
                Ok(Self::CloudRuntimeServiceManualScalingDeleted)
            }
            "cloud_runtime.persistent_volume.created" => {
                Ok(Self::CloudRuntimePersistentVolumeCreated)
            }
            "cloud_runtime.persistent_volume.deleted" => {
                Ok(Self::CloudRuntimePersistentVolumeDeleted)
            }
            "cloud_runtime.persistent_volume.attached" => {
                Ok(Self::CloudRuntimePersistentVolumeAttached)
            }
            "cloud_runtime.persistent_volume.detached" => {
                Ok(Self::CloudRuntimePersistentVolumeDetached)
            }
            "fine_tuning_job.create" => Ok(Self::FineTuningJobCreate),
            "fine_tuning_job.cancel" => Ok(Self::FineTuningJobCancel),
            "batch_job.create" => Ok(Self::BatchJobCreate),
            "batch_job.cancel" => Ok(Self::BatchJobCancel),
            "batch_job.delete" => Ok(Self::BatchJobDelete),
            "data_capture.extract_job.create" => Ok(Self::DataCaptureExtractJobCreate),
            "data_capture.extract_job.cancel" => Ok(Self::DataCaptureExtractJobCancel),
            "dataset.create" => Ok(Self::DatasetCreate),
            "dataset.delete" => Ok(Self::DatasetDelete),
            "library.create" => Ok(Self::LibraryCreate),
            "library.delete" => Ok(Self::LibraryDelete),
            "library.update" => Ok(Self::LibraryUpdate),
            "library.share" => Ok(Self::LibraryShare),
            "library.unshare" => Ok(Self::LibraryUnshare),
            "library.document.create" => Ok(Self::LibraryDocumentCreate),
            "library.document.delete" => Ok(Self::LibraryDocumentDelete),
            "library.document.bulk_delete" => Ok(Self::LibraryDocumentBulkDelete),
            "library.document.update" => Ok(Self::LibraryDocumentUpdate),
            "library.document.reprocess" => Ok(Self::LibraryDocumentReprocess),
            "integration.connected" => Ok(Self::IntegrationConnected),
            "integration.disconnected" => Ok(Self::IntegrationDisconnected),
            "indexing.workflow.completed" => Ok(Self::IndexingWorkflowCompleted),
            "indexing.deleted" => Ok(Self::IndexingDeleted),
            "connection.admin.setup_index" => Ok(Self::ConnectionAdminSetupIndex),
            "connection.admin.deleted" => Ok(Self::ConnectionAdminDeleted),
            "integration.activated_for_org" => Ok(Self::IntegrationActivatedForOrg),
            "integration.deactivated_for_org" => Ok(Self::IntegrationDeactivatedForOrg),
            "integration.activated_for_workspace" => Ok(Self::IntegrationActivatedForWorkspace),
            "integration.deactivated_for_workspace" => Ok(Self::IntegrationDeactivatedForWorkspace),
            "integration.activated_for_user" => Ok(Self::IntegrationActivatedForUser),
            "integration.deactivated_for_user" => Ok(Self::IntegrationDeactivatedForUser),
            "integration.created" => Ok(Self::IntegrationCreated),
            "integration.updated" => Ok(Self::IntegrationUpdated),
            "integration.deleted" => Ok(Self::IntegrationDeleted),
            "integration.tool_called" => Ok(Self::IntegrationToolCalled),
            "integration.credentials.created_or_updated" => {
                Ok(Self::IntegrationCredentialsCreatedOrUpdated)
            }
            "integration.credentials.deleted" => Ok(Self::IntegrationCredentialsDeleted),
            "integration.credentials.revoked" => Ok(Self::IntegrationCredentialsRevoked),
            "integration.credentials.revocation_failed" => {
                Ok(Self::IntegrationCredentialsRevocationFailed)
            }
            "integration.preferences.created_or_updated" => {
                Ok(Self::IntegrationPreferencesCreatedOrUpdated)
            }
            "integration.preferences.deleted" => Ok(Self::IntegrationPreferencesDeleted),
            "integration.authentication_method.created_or_updated" => {
                Ok(Self::IntegrationAuthenticationMethodCreatedOrUpdated)
            }
            "integration.connection.created" => Ok(Self::IntegrationConnectionCreated),
            "integration.shared" => Ok(Self::IntegrationShared),
            "integration.unshared" => Ok(Self::IntegrationUnshared),
            "connectors_gateway.tool_called" => Ok(Self::ConnectorsGatewayToolCalled),
            "connectors_debugger.tool_called" => Ok(Self::ConnectorsDebuggerToolCalled),
            "crawler.config.create" => Ok(Self::CrawlerConfigCreate),
            "crawler.config.update" => Ok(Self::CrawlerConfigUpdate),
            "crawler.config.delete" => Ok(Self::CrawlerConfigDelete),
            "crawler.run.create" => Ok(Self::CrawlerRunCreate),
            "crawler.run.cancel" => Ok(Self::CrawlerRunCancel),
            "rate_limit.rule.create" => Ok(Self::RateLimitRuleCreate),
            "rate_limit.rule.update" => Ok(Self::RateLimitRuleUpdate),
            "rate_limit.rule.delete" => Ok(Self::RateLimitRuleDelete),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AuditLogEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserCreate => write!(f, "user.create"),
            Self::UserDelete => write!(f, "user.delete"),
            Self::UserLogIn => write!(f, "user.log_in"),
            Self::UserInfoUpdate => write!(f, "user.info.update"),
            Self::UserPasswordUpdate => write!(f, "user.password.update"),
            Self::UserPhoneNumberVerify => write!(f, "user.phone_number.verify"),
            Self::UserOrganizationLeave => write!(f, "user.organization.leave"),
            Self::UserOrganizationRoleUpdate => write!(f, "user.organization.role.update"),
            Self::UserOrganizationDelete => write!(f, "user.organization.delete"),
            Self::UserOrganizationJoin => write!(f, "user.organization.join"),
            Self::OrganizationCreate => write!(f, "organization.create"),
            Self::OrganizationUpdate => write!(f, "organization.update"),
            Self::OrganizationInviteSend => write!(f, "organization.invite.send"),
            Self::OrganizationInviteResend => write!(f, "organization.invite.resend"),
            Self::OrganizationInviteAccepted => write!(f, "organization.invite.accepted"),
            Self::OrganizationInviteRevoked => write!(f, "organization.invite.revoked"),
            Self::OrganizationJoinByEmailDomain => write!(f, "organization.join_by_email_domain"),
            Self::OrganizationDomainVerificationDisable => {
                write!(f, "organization.domain_verification.disable")
            }
            Self::OrganizationDomainVerificationEnable => {
                write!(f, "organization.domain_verification.enable")
            }
            Self::OrganizationEmailDomainAuthenticationEnable => {
                write!(f, "organization.email_domain_authentication.enable")
            }
            Self::OrganizationEmailDomainAuthenticationDisable => {
                write!(f, "organization.email_domain_authentication.disable")
            }
            Self::OrganizationSamlAuthenticationEnable => {
                write!(f, "organization.saml_authentication.enable")
            }
            Self::OrganizationSamlAuthenticationDisable => {
                write!(f, "organization.saml_authentication.disable")
            }
            Self::OrganizationSamlAuthenticationSsoUserProvisioningUpdate => write!(
                f,
                "organization.saml_authentication.sso_user_provisioning.update"
            ),
            Self::OrganizationScimSyncTrigger => write!(f, "organization.scim_sync.trigger"),
            Self::OrganizationSeatAutoAssignEnable => {
                write!(f, "organization.seat_auto_assign.enable")
            }
            Self::OrganizationSeatAutoAssignDisable => {
                write!(f, "organization.seat_auto_assign.disable")
            }
            Self::OrganizationKindUpdate => write!(f, "organization.kind.update"),
            Self::OrganizationSsoSeatAutoAssignEnable => {
                write!(f, "organization.sso_seat_auto_assign.enable")
            }
            Self::OrganizationSsoSeatAutoAssignDisable => {
                write!(f, "organization.sso_seat_auto_assign.disable")
            }
            Self::WorkspaceCreate => write!(f, "workspace.create"),
            Self::WorkspaceUpdate => write!(f, "workspace.update"),
            Self::WorkspaceDelete => write!(f, "workspace.delete"),
            Self::WorkspaceMemberAdd => write!(f, "workspace.member.add"),
            Self::WorkspaceMemberRemove => write!(f, "workspace.member.remove"),
            Self::WorkspaceMemberRoleUpdate => write!(f, "workspace.member.role.update"),
            Self::UserGroupCreate => write!(f, "user_group.create"),
            Self::UserGroupUpdate => write!(f, "user_group.update"),
            Self::UserGroupDelete => write!(f, "user_group.delete"),
            Self::UserGroupMemberAdd => write!(f, "user_group.member.add"),
            Self::UserGroupMemberRemove => write!(f, "user_group.member.remove"),
            Self::UserGroupWorkspaceProvision => write!(f, "user_group.workspace.provision"),
            Self::UserGroupWorkspaceDeprovision => write!(f, "user_group.workspace.deprovision"),
            Self::UserGroupWorkspaceUpdate => write!(f, "user_group.workspace.update"),
            Self::BillingInformationUpdated => write!(f, "billing.information.updated"),
            Self::BillingPaymentMethodAdded => write!(f, "billing.payment_method.added"),
            Self::BillingPaymentMethodRemoved => write!(f, "billing.payment_method.removed"),
            Self::BillingPaymentMethodDefaultChanged => {
                write!(f, "billing.payment_method.default_changed")
            }
            Self::BillingInvoiceRetried => write!(f, "billing.invoice.retried"),
            Self::BillingSubscriptionSubscribe => write!(f, "billing.subscription.subscribe"),
            Self::BillingSubscriptionUnsubscribe => write!(f, "billing.subscription.unsubscribe"),
            Self::BillingSubscriptionCancelUnsubscribe => {
                write!(f, "billing.subscription.cancel_unsubscribe")
            }
            Self::BillingSubscriptionAddSeats => write!(f, "billing.subscription.add_seats"),
            Self::BillingSubscriptionRemoveSeats => write!(f, "billing.subscription.remove_seats"),
            Self::BillingSubscriptionTerminationDateUpdated => {
                write!(f, "billing.subscription.termination_date.updated")
            }
            Self::BillingCreditsAdded => write!(f, "billing.credits.added"),
            Self::BillingGiftCodeUsed => write!(f, "billing.gift_code.used"),
            Self::BillingMonthlyLimitUpdated => write!(f, "billing.monthly_limit.updated"),
            Self::BillingWorkspaceMonthlyLimitUpdated => {
                write!(f, "billing.workspace_monthly_limit.updated")
            }
            Self::BillingSharedBudgetOverrideUpdated => {
                write!(f, "billing.shared_budget_override.updated")
            }
            Self::BillingAutoRechargeUpdated => write!(f, "billing.auto_recharge.updated"),
            Self::BillingSeatGrant => write!(f, "billing.seat.grant"),
            Self::BillingSeatRevoke => write!(f, "billing.seat.revoke"),
            Self::BillingPriorityServiceTierUpdated => {
                write!(f, "billing.priority_service_tier.updated")
            }
            Self::BillingPriorityServiceTierRemoved => {
                write!(f, "billing.priority_service_tier.removed")
            }
            Self::LeChatConversationCreated => write!(f, "le_chat.conversation.created"),
            Self::LeChatConversationDeleted => write!(f, "le_chat.conversation.deleted"),
            Self::LeChatConversationBatchDeleted => write!(f, "le_chat.conversation_batch.deleted"),
            Self::LeChatConversationPublicSharingEnabled => {
                write!(f, "le_chat.conversation.public_sharing.enabled")
            }
            Self::LeChatConversationPublicSharingDisabled => {
                write!(f, "le_chat.conversation.public_sharing.disabled")
            }
            Self::LeChatFlashAnswersEnabled => write!(f, "le_chat.flash_answers.enabled"),
            Self::LeChatFlashAnswersDisabled => write!(f, "le_chat.flash_answers.disabled"),
            Self::LeChatLocalisationSharingEnabled => {
                write!(f, "le_chat.localisation_sharing.enabled")
            }
            Self::LeChatLocalisationSharingDisabled => {
                write!(f, "le_chat.localisation_sharing.disabled")
            }
            Self::LeChatMemoriesEnabled => write!(f, "le_chat.memories.enabled"),
            Self::LeChatMemoriesDisabled => write!(f, "le_chat.memories.disabled"),
            Self::LeChatDataTrainingEnabled => write!(f, "le_chat.data.training_enabled"),
            Self::LeChatDataTrainingDisabled => write!(f, "le_chat.data.training_disabled"),
            Self::LeChatActionsExternalLink => write!(f, "le_chat.actions.external_link"),
            Self::AdminApiKeyCreated => write!(f, "admin_api_key.created"),
            Self::AdminApiKeyDelete => write!(f, "admin_api_key.delete"),
            Self::ApiKeyCreate => write!(f, "api_key.create"),
            Self::ApiKeyRotate => write!(f, "api_key.rotate"),
            Self::ApiKeyDelete => write!(f, "api_key.delete"),
            Self::ApiKeyPolicyUpdate => write!(f, "api_key_policy.update"),
            Self::SecretStoreEntryCreate => write!(f, "secret_store.entry.create"),
            Self::SecretStoreEntryUpdate => write!(f, "secret_store.entry.update"),
            Self::SecretStoreEntryDelete => write!(f, "secret_store.entry.delete"),
            Self::ServiceAccountCreate => write!(f, "service_account.create"),
            Self::ServiceAccountUpdate => write!(f, "service_account.update"),
            Self::ServiceAccountDelete => write!(f, "service_account.delete"),
            Self::ServiceAccountClientSecretCreate => {
                write!(f, "service_account.client_secret.create")
            }
            Self::ServiceAccountClientSecretDelete => {
                write!(f, "service_account.client_secret.delete")
            }
            Self::ServiceAccountRolesSet => write!(f, "service_account.roles.set"),
            Self::WorkloadIdentityCredentialRegistrationCreate => {
                write!(f, "workload_identity.credential_registration.create")
            }
            Self::WorkloadIdentityCredentialCreate => {
                write!(f, "workload_identity.credential.create")
            }
            Self::TrustedIssuerCreate => write!(f, "trusted_issuer.create"),
            Self::TrustedIssuerUpdate => write!(f, "trusted_issuer.update"),
            Self::TrustedIssuerDelete => write!(f, "trusted_issuer.delete"),
            Self::AgentCreate => write!(f, "agent.create"),
            Self::AgentDelete => write!(f, "agent.delete"),
            Self::AgentUpdate => write!(f, "agent.update"),
            Self::SkillCreate => write!(f, "skill.create"),
            Self::SkillDelete => write!(f, "skill.delete"),
            Self::SkillEnable => write!(f, "skill.enable"),
            Self::SkillDisable => write!(f, "skill.disable"),
            Self::SkillUpdate => write!(f, "skill.update"),
            Self::SkillShare => write!(f, "skill.share"),
            Self::SkillUnshare => write!(f, "skill.unshare"),
            Self::SkillLoad => write!(f, "skill.load"),
            Self::SkillForceLoad => write!(f, "skill.force_load"),
            Self::SkillVersionCreate => write!(f, "skill.version.create"),
            Self::PromptCreate => write!(f, "prompt.create"),
            Self::PromptDelete => write!(f, "prompt.delete"),
            Self::PromptUpdate => write!(f, "prompt.update"),
            Self::PromptVersionCreate => write!(f, "prompt.version.create"),
            Self::KnowledgeBaseCreate => write!(f, "knowledge_base.create"),
            Self::KnowledgeBaseDelete => write!(f, "knowledge_base.delete"),
            Self::KnowledgeBaseUpdate => write!(f, "knowledge_base.update"),
            Self::KnowledgeBaseVersionCreate => write!(f, "knowledge_base.version.create"),
            Self::CustomVoiceCreate => write!(f, "custom_voice.create"),
            Self::CustomVoiceUpdate => write!(f, "custom_voice.update"),
            Self::CustomVoiceDelete => write!(f, "custom_voice.delete"),
            Self::FeaturePermissionOverrideCreated => {
                write!(f, "feature_permission.override.created")
            }
            Self::FeaturePermissionOverrideDeleted => {
                write!(f, "feature_permission.override.deleted")
            }
            Self::ResourceShare => write!(f, "resource.share"),
            Self::ResourceUnshare => write!(f, "resource.unshare"),
            Self::LaPlateformeTrainingEnabled => write!(f, "la_plateforme.training.enabled"),
            Self::LaPlateformeTrainingDisabled => write!(f, "la_plateforme.training.disabled"),
            Self::CloudRuntimeAppCreated => write!(f, "cloud_runtime.app.created"),
            Self::CloudRuntimeAppUpdated => write!(f, "cloud_runtime.app.updated"),
            Self::CloudRuntimeAppDeleted => write!(f, "cloud_runtime.app.deleted"),
            Self::CloudRuntimeAppPaused => write!(f, "cloud_runtime.app.paused"),
            Self::CloudRuntimeAppResumed => write!(f, "cloud_runtime.app.resumed"),
            Self::CloudRuntimeDeploymentCreated => write!(f, "cloud_runtime.deployment.created"),
            Self::CloudRuntimeDeploymentSucceeded => {
                write!(f, "cloud_runtime.deployment.succeeded")
            }
            Self::CloudRuntimeDeploymentCanceled => write!(f, "cloud_runtime.deployment.canceled"),
            Self::CloudRuntimeDeploymentStopped => write!(f, "cloud_runtime.deployment.stopped"),
            Self::CloudRuntimeDeploymentFailed => write!(f, "cloud_runtime.deployment.failed"),
            Self::CloudRuntimeDeploymentAutoscaled => {
                write!(f, "cloud_runtime.deployment.autoscaled")
            }
            Self::CloudRuntimeDomainCreated => write!(f, "cloud_runtime.domain.created"),
            Self::CloudRuntimeDomainUpdated => write!(f, "cloud_runtime.domain.updated"),
            Self::CloudRuntimeDomainDeleted => write!(f, "cloud_runtime.domain.deleted"),
            Self::CloudRuntimeSecretCreated => write!(f, "cloud_runtime.secret.created"),
            Self::CloudRuntimeSecretUpdated => write!(f, "cloud_runtime.secret.updated"),
            Self::CloudRuntimeSecretDeleted => write!(f, "cloud_runtime.secret.deleted"),
            Self::CloudRuntimeServiceCreated => write!(f, "cloud_runtime.service.created"),
            Self::CloudRuntimeServicePaused => write!(f, "cloud_runtime.service.paused"),
            Self::CloudRuntimeServiceResumed => write!(f, "cloud_runtime.service.resumed"),
            Self::CloudRuntimeServiceDeleted => write!(f, "cloud_runtime.service.deleted"),
            Self::CloudRuntimeServiceManuallyScaled => {
                write!(f, "cloud_runtime.service.manually_scaled")
            }
            Self::CloudRuntimeServiceManualScalingDeleted => {
                write!(f, "cloud_runtime.service.manual_scaling_deleted")
            }
            Self::CloudRuntimePersistentVolumeCreated => {
                write!(f, "cloud_runtime.persistent_volume.created")
            }
            Self::CloudRuntimePersistentVolumeDeleted => {
                write!(f, "cloud_runtime.persistent_volume.deleted")
            }
            Self::CloudRuntimePersistentVolumeAttached => {
                write!(f, "cloud_runtime.persistent_volume.attached")
            }
            Self::CloudRuntimePersistentVolumeDetached => {
                write!(f, "cloud_runtime.persistent_volume.detached")
            }
            Self::FineTuningJobCreate => write!(f, "fine_tuning_job.create"),
            Self::FineTuningJobCancel => write!(f, "fine_tuning_job.cancel"),
            Self::BatchJobCreate => write!(f, "batch_job.create"),
            Self::BatchJobCancel => write!(f, "batch_job.cancel"),
            Self::BatchJobDelete => write!(f, "batch_job.delete"),
            Self::DataCaptureExtractJobCreate => write!(f, "data_capture.extract_job.create"),
            Self::DataCaptureExtractJobCancel => write!(f, "data_capture.extract_job.cancel"),
            Self::DatasetCreate => write!(f, "dataset.create"),
            Self::DatasetDelete => write!(f, "dataset.delete"),
            Self::LibraryCreate => write!(f, "library.create"),
            Self::LibraryDelete => write!(f, "library.delete"),
            Self::LibraryUpdate => write!(f, "library.update"),
            Self::LibraryShare => write!(f, "library.share"),
            Self::LibraryUnshare => write!(f, "library.unshare"),
            Self::LibraryDocumentCreate => write!(f, "library.document.create"),
            Self::LibraryDocumentDelete => write!(f, "library.document.delete"),
            Self::LibraryDocumentBulkDelete => write!(f, "library.document.bulk_delete"),
            Self::LibraryDocumentUpdate => write!(f, "library.document.update"),
            Self::LibraryDocumentReprocess => write!(f, "library.document.reprocess"),
            Self::IntegrationConnected => write!(f, "integration.connected"),
            Self::IntegrationDisconnected => write!(f, "integration.disconnected"),
            Self::IndexingWorkflowCompleted => write!(f, "indexing.workflow.completed"),
            Self::IndexingDeleted => write!(f, "indexing.deleted"),
            Self::ConnectionAdminSetupIndex => write!(f, "connection.admin.setup_index"),
            Self::ConnectionAdminDeleted => write!(f, "connection.admin.deleted"),
            Self::IntegrationActivatedForOrg => write!(f, "integration.activated_for_org"),
            Self::IntegrationDeactivatedForOrg => write!(f, "integration.deactivated_for_org"),
            Self::IntegrationActivatedForWorkspace => {
                write!(f, "integration.activated_for_workspace")
            }
            Self::IntegrationDeactivatedForWorkspace => {
                write!(f, "integration.deactivated_for_workspace")
            }
            Self::IntegrationActivatedForUser => write!(f, "integration.activated_for_user"),
            Self::IntegrationDeactivatedForUser => write!(f, "integration.deactivated_for_user"),
            Self::IntegrationCreated => write!(f, "integration.created"),
            Self::IntegrationUpdated => write!(f, "integration.updated"),
            Self::IntegrationDeleted => write!(f, "integration.deleted"),
            Self::IntegrationToolCalled => write!(f, "integration.tool_called"),
            Self::IntegrationCredentialsCreatedOrUpdated => {
                write!(f, "integration.credentials.created_or_updated")
            }
            Self::IntegrationCredentialsDeleted => write!(f, "integration.credentials.deleted"),
            Self::IntegrationCredentialsRevoked => write!(f, "integration.credentials.revoked"),
            Self::IntegrationCredentialsRevocationFailed => {
                write!(f, "integration.credentials.revocation_failed")
            }
            Self::IntegrationPreferencesCreatedOrUpdated => {
                write!(f, "integration.preferences.created_or_updated")
            }
            Self::IntegrationPreferencesDeleted => write!(f, "integration.preferences.deleted"),
            Self::IntegrationAuthenticationMethodCreatedOrUpdated => {
                write!(f, "integration.authentication_method.created_or_updated")
            }
            Self::IntegrationConnectionCreated => write!(f, "integration.connection.created"),
            Self::IntegrationShared => write!(f, "integration.shared"),
            Self::IntegrationUnshared => write!(f, "integration.unshared"),
            Self::ConnectorsGatewayToolCalled => write!(f, "connectors_gateway.tool_called"),
            Self::ConnectorsDebuggerToolCalled => write!(f, "connectors_debugger.tool_called"),
            Self::CrawlerConfigCreate => write!(f, "crawler.config.create"),
            Self::CrawlerConfigUpdate => write!(f, "crawler.config.update"),
            Self::CrawlerConfigDelete => write!(f, "crawler.config.delete"),
            Self::CrawlerRunCreate => write!(f, "crawler.run.create"),
            Self::CrawlerRunCancel => write!(f, "crawler.run.cancel"),
            Self::RateLimitRuleCreate => write!(f, "rate_limit.rule.create"),
            Self::RateLimitRuleUpdate => write!(f, "rate_limit.rule.update"),
            Self::RateLimitRuleDelete => write!(f, "rate_limit.rule.delete"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
