# Mistral upstream synchronization

**Human review required** | **Generation: PASSED**

> Automated candidate only. CI verifies technical consistency; a separate
> GitHub PR approval is required to accept API and compatibility changes.

## Summary

| Source | State | Previously pinned → observed |
| --- | --- | --- |
| OpenAPI catalog | **Changed** | `08a585fb75bc` → `7652f1307b3b` |
| Python SDK | **Changed** | [`8fdbff0b1bcb`](https://github.com/mistralai/client-python/commit/8fdbff0b1bcb0c9c3e5b803080a20ac820c2905c) → [`e3444997d526`](https://github.com/mistralai/client-python/commit/e3444997d52684aaf612ee8fed98f9b8025b95d6) |
| TypeScript SDK | **Changed** | [`50b2bc105cce`](https://github.com/mistralai/client-ts/commit/50b2bc105cce327d3bd2ccee86240c4b0f6f2d6c) → [`3e87497eea3a`](https://github.com/mistralai/client-ts/commit/3e87497eea3ae73f482cd683b043469e72f12389) |

**Endpoints:** 321 → 347 (47 added, 21 removed, 22 modified).

## Review checklist

- [ ] Verify the public OpenAPI catalog and official SDK source revisions.
- [ ] Inspect removed/modified operations and schemas for breaking changes.
- [ ] Require regenerated Rust sources and a green CI `gate` check.
- [ ] Approve this PR in GitHub after the final push (never via a lockfile flag).

**Generation attempt:** successful.

## OpenAPI surface changes

| | Before | After | Added | Removed | Modified |
| --- | ---: | ---: | ---: | ---: | ---: |
| Endpoints | 321 | 347 | 47 | 21 | 22 |
| Schemas | 1021 | 1106 | 107 | 22 | 96 |

A modified schema fingerprint does not necessarily indicate a breaking change.
Removed operations and changed schemas should be reviewed first.

### Most affected API areas

| Area | Added | Removed | Modified |
| --- | ---: | ---: | ---: |
| `/v1/context` | 15 | 0 | 0 |
| `/v1/observability` | 5 | 7 | 1 |
| `/v1/audio` | 11 | 0 | 1 |
| `/v1/rag` | 0 | 12 | 0 |
| `/v1/admin` | 2 | 2 | 5 |
| `/v1/workflows` | 2 | 0 | 5 |
| `/v1/triggers` | 5 | 0 | 0 |
| `/v1/webhooks` | 5 | 0 | 0 |
| `/v1/connectors` | 0 | 0 | 4 |
| `/v1/billing` | 2 | 0 | 0 |
| `/v1/service-accounts` | 0 | 0 | 2 |
| `/v1/files` | 0 | 0 | 1 |
| `/v1/responses` | 0 | 0 | 1 |
| `/v2/audio` | 0 | 0 | 1 |
| `/v2/skills` | 0 | 0 | 1 |

### Changes needing compatibility review

<details>
<summary>Removed endpoints (21)</summary>

- `DELETE /v1/observability/judges/{judge_id}`
- `DELETE /v1/rag/managed_indexes/{index_name}`
- `DELETE /v1/rag/managed_indexes/{index_name}/documents`
- `GET /v1/admin/user-groups/{group_uuid}/nested`
- `GET /v1/observability/judges`
- `GET /v1/observability/judges/{judge_id}`
- `GET /v1/rag/managed_indexes`
- `GET /v1/rag/managed_indexes/{index_name}`
- `GET /v1/rag/managed_indexes/{index_name}/chunks/{chunk_id}`
- `PATCH /v1/admin/user-groups/{group_uuid}/nested`
- `POST /v1/observability/dataset-records/{dataset_record_id}/live-judging`
- `POST /v1/observability/judges`
- `POST /v1/observability/judges/{judge_id}/live-judging`
- `POST /v1/rag/managed_indexes`
- `POST /v1/rag/managed_indexes/{index_name}/documents`
- `POST /v1/rag/managed_indexes/{index_name}/grep`
- `POST /v1/rag/managed_indexes/{index_name}/navigate`
- `POST /v1/rag/managed_indexes/{index_name}/read`
- `POST /v1/rag/managed_indexes/{index_name}/search`
- `PUT /v1/observability/judges/{judge_id}`
- `PUT /v1/rag/managed_indexes/{index_name}`

</details>

<details>
<summary>Modified endpoints (22)</summary>

- `DELETE /v1/admin/users/{user_id}`
- `GET /v1/admin/user-groups`
- `GET /v1/admin/users`
- `GET /v1/admin/users/{user_id}`
- `GET /v1/observability/pipelines`
- `GET /v1/service-accounts`
- `GET /v1/service-accounts/{service_account_id}`
- `GET /v1/workflows`
- `GET /v1/workflows/deployments`
- `GET /v1/workflows/registrations`
- `GET /v1/workflows/runs`
- `GET /v1/workflows/schedules`
- `GET /v2/audio/voices`
- `GET /v2/skills`
- `PATCH /v1/connectors/{connector_id_or_name}/{consumer_scope}/credentials`
- `PATCH /v1/connectors/{connector_id}#id`
- `POST /v1/admin/users/{user_id}/deactivate`
- `POST /v1/audio/speech`
- `POST /v1/connectors`
- `POST /v1/connectors/{connector_id_or_name}/{consumer_scope}/credentials`
- `POST /v1/files`
- `POST /v1/responses`

</details>

<details>
<summary>Removed schemas (22)</summary>

- `ConnectionConfigType`
- `CreateJudgeRequest`
- `ExecutionTool`
- `Judge`
- `JudgeClassificationOutput`
- `JudgeClassificationOutputOption`
- `JudgeConversationRequest`
- `JudgeDatasetRecordRequest`
- `JudgeOutput`
- `JudgeOutputType`
- `JudgeRegressionOutput`
- `ListJudgesResponse`
- `NestedGroupRef`
- `NestedGroupsOut`
- `PaginatedResultJudgePreview`
- `PublicConnectionConfig`
- `PublicConnectorExecutionData`
- `PublicExecutionConnectionConfig`
- `PublicExecutionConnector`
- `PublicExecutionEnv`
- `SetNestedGroupsIn`
- `UpdateJudgeRequest`

</details>

<details>
<summary>Modified schemas (96)</summary>

- `APIKeyExtendedOUT`
- `AdminAssignGroupToWorkspaceIn`
- `AdminOrganizationMemberOUT`
- `AdminOrganizationMemberUpdate`
- `AdminProvisionGroupToWorkspaceIn`
- `AdminScimSyncRunOUT`
- `AdminUpdateGroupWorkspaceAssignmentIn`
- `AdminUpdateUserGroupIn`
- `AdminUsageQueryFilter`
- `AdminUserGroupIn`
- `AdminUserGroupOut`
- `AdminUserOUT`
- `AdminWorkspaceMemberSingleIN`
- `AggregationRow`
- `AudioTranscriptionRequest`
- `AudioTranscriptionRequestStream`
- `AuditLogEventType`
- `BaseModelCard`
- `ConnectionCredentialsInput`
- `ConnectionPreference`
- `ConnectorMCPPublicUpdate`
- `ConnectorToolResultMetadata`
- `CreateDatasetRequest`
- `CreatePipelineRequest`
- `CreateRealtimeSessionRequest`
- `CreateResponseResponse`
- `CreateServiceAccountRequest`
- `CredentialsCreateOrUpdate`
- `CustomConnector`
- `Dataset`
- `DatasetImportTask`
- `DatasetPreview`
- `DeploymentObservedState`
- `DeploymentWorkerSpecInput`
- `DeploymentWorkerSpecResponse`
- `EntityType`
- `EvaluationRunV2Response`
- `EvaluationRunV2WithRefsResponse`
- `EvaluationV2`
- `GetLog`
- `GetSpan`
- `GetSpanEvaluation`
- `GetTrace`
- `HTTPConnector`
- `IPAPhoneme`
- `KeywordRetriever`
- `LagoEventType`
- `MCPConnector`
- `MemberWorkspaceInfo`
- `ObservabilityErrorCode`
- `Optimization`
- `OptimizationDetail`
- `OrganizationInviteIN`
- `OrganizationInviteOUT`
- `OrganizationMemberCreate`
- `OrganizationUserInviteOUT`
- `PipelineResult`
- `Project`
- `PublicAuthenticationMethod`
- `RRFRetriever`
- `ReasoningEffort`
- `ResponsesCustomTool`
- `ResponsesError`
- `ResponsesFunctionCallArgumentsDoneEvent`
- `ResponsesFunctionCallOutput`
- `ResponsesFunctionTool`
- `ResponsesFunctionToolCall`
- `ResponsesIncompleteDetails`
- `ResponsesPromptCacheBreakpoint`
- `ResponsesToolFunction`
- `RotateUnavailableReason`
- `ScheduleDefinitionOutput`
- `ScimSyncGroupRefOut`
- `ScimSyncGroupUpdateActionOut`
- `ScimSyncUserRefOut`
- `SetServiceAccountRolesRequest`
- `SharingDelete`
- `SharingRequest`
- `SpeechRequest`
- `StaticWorkspaceRoles`
- `SystemV2`
- `TargetType`
- `TracesRequest`
- `UpdateDatasetRequest`
- `UpdateHTTPConnectorRequest`
- `UpdatePipelineRequest`
- `UsageQueryIN`
- `UsageQueryParamsOUT`
- `UsageQueryRowOUT`
- `UsageType`
- `VoiceCreateRequest`
- `VoiceResponse`
- `VoiceUpdateRequest`
- `WhoamiResponse`
- `WorkflowsWorkerSpecUpdate`
- `WorkspaceEnrichedOUT`

</details>

### Additions

<details>
<summary>Added endpoints (47)</summary>

- `DELETE /v1/audio/agents/{agent_id}`
- `DELETE /v1/audio/sessions/{session_id}`
- `DELETE /v1/context/search-indexes/{index_name}`
- `DELETE /v1/context/search-indexes/{index_name}/documents`
- `DELETE /v1/triggers/{trigger_id}`
- `DELETE /v1/webhooks/{webhook_id}`
- `GET /v1/admin/users/{user_id}/workspaces`
- `GET /v1/audio/agents`
- `GET /v1/audio/agents/{agent_id}`
- `GET /v1/audio/agents/{agent_id}/sessions`
- `GET /v1/audio/agents/{agent_id}/versions`
- `GET /v1/audio/sessions/{session_id}`
- `GET /v1/billing/budget`
- `GET /v1/billing/subscription`
- `GET /v1/context/search-indexes`
- `GET /v1/context/search-indexes/{index_name}`
- `GET /v1/context/search-indexes/{index_name}/chunks/{chunk_id}`
- `GET /v1/context/search-indexes/{index_name}/summary_field/{language}`
- `GET /v1/observability/pipelines/{pipeline_id}/results/fields`
- `GET /v1/observability/scorers`
- `GET /v1/observability/scorers/{scorer_slug}`
- `GET /v1/triggers`
- `GET /v1/triggers/{trigger_id}`
- `GET /v1/triggers/{trigger_id}/events`
- `GET /v1/webhooks`
- `GET /v1/webhooks/{webhook_id}`
- `GET /v1/webhooks/{webhook_id}/signing-secret`
- `GET /v1/workflows/deployments/{name}/archive`
- `PATCH /v1/audio/sessions/{session_id}`
- `PATCH /v1/observability/scorers/{scorer_slug}`
- `POST /v1/admin/users/{user_id}/reactivate`
- `POST /v1/audio/agents`
- `POST /v1/audio/sessions`
- `POST /v1/context/search-indexes`
- `POST /v1/context/search-indexes/{index_name}/documents`
- `POST /v1/context/search-indexes/{index_name}/grep`
- `POST /v1/context/search-indexes/{index_name}/navigate`
- `POST /v1/context/search-indexes/{index_name}/read`
- `POST /v1/context/search-indexes/{index_name}/search`
- `POST /v1/context/search-indexes/{index_name}/summary_field/{language}`
- `POST /v1/observability/scorers`
- `POST /v1/triggers`
- `POST /v1/webhooks`
- `POST /v1/workflows/{workflow_identifier}/tags`
- `PUT /v1/audio/agents/{agent_id}`
- `PUT /v1/context/search-indexes/{index_name}`
- `PUT /v1/context/search-indexes/{index_name}/summary_field/{language}`

</details>

<details>
<summary>Added schemas (107)</summary>

- `AdminUserWorkspacesOUT`
- `AmazonConnectContact`
- `AudioAgentConfigurationRequest`
- `AudioAgentConfigurationResponseConfig`
- `AudioAgentCreateRequest`
- `AudioAgentListPage`
- `AudioAgentResponse`
- `AudioAgentSessionListPage`
- `AudioAgentSessionSummary`
- `AudioAgentUpdateRequest`
- `AudioAssistantTranscriptTurn`
- `AudioInactivityConfig`
- `AudioLLMConfig`
- `AudioLatencyBreakdownAssistant`
- `AudioLatencyBreakdownUser`
- `AudioSTTConfig`
- `AudioSessionCreateRequest`
- `AudioSessionCreateResponse`
- `AudioSessionPhoneCall`
- `AudioSessionPhoneNumber`
- `AudioSessionResponse`
- `AudioSessionTurnUpdate`
- `AudioSessionUpdateRequest`
- `AudioSessionUpdateResponse`
- `AudioSpeechCapabilities`
- `AudioTTSConfig`
- `AudioToolCall`
- `AudioToolTranscriptTurn`
- `AudioToolTranscriptTurnStatus`
- `AudioTranscriptionCapabilities`
- `AudioTurnConfig`
- `AudioTurnDetectionConfig`
- `AudioUserTranscriptTurn`
- `BudgetDetail`
- `ConnectorErrorCode`
- `ConnectorErrorResponse`
- `ConnectorOpenApiSpec`
- `CreateScorerRequest`
- `CreateTrigger`
- `CreateTriggerAction`
- `CreateWebhook`
- `DeploymentArchiveResponse`
- `EditScorerRequest`
- `EnumConstraints`
- `FloatConstraints`
- `FunctionRequest`
- `GeneratedToolFiller`
- `GetBudgetResponse`
- `GetPipelineResultFields`
- `GetSubscriptionResponse`
- `GetSummaryResponseSummary`
- `HangUpTool`
- `ListScorersResponse`
- `LlmJudgeConfig`
- `LlmJudgeParams`
- `MemberStatus`
- `OpenApiFileSource`
- `OpenApiSpecSource`
- `OpenApiUrlSource`
- `OtelEnumFieldDefinition`
- `OtelFloatFieldDefinition`
- `PaginatedResultScorerSummary`
- `PhoneDestination`
- `RawAudio`
- `ReactivateUserOUT`
- `ResponsesMisalignment`
- `ResponsesMisalignmentSteer`
- `Scorer`
- `ScorerKind`
- `ScorerSummary`
- `ScorerVersion`
- `SipDestination`
- `SpeechAssistantTurn`
- `SpeechStreamAudioDelta`
- `SpeechStreamDone`
- `SpeechUserTurn`
- `StaticToolFiller`
- `StreamingDelayRange`
- `SummaryStatus`
- `TransferCallTool`
- `TransferTarget`
- `Trigger`
- `TriggerAction`
- `TriggerActionType`
- `TriggerEvent`
- `TriggerEventList`
- `TriggerHandler`
- `TriggerStatusValue`
- `TriggersOwnerType`
- `UpdateSummaryRequestSummary`
- `VoiceAgentCustomConnector`
- `VoiceAgentDocumentLibraryTool`
- `VoiceAgentErrorEvent`
- `VoiceAgentEvent`
- `VoiceAgentFunctionTool`
- `VoiceAgentFunctionToolRequest`
- `VoiceAgentStateChangedEvent`
- `VoiceAgentToolCallEndedEvent`
- `VoiceAgentToolCallStartedEvent`
- `VoiceAgentTranscriptEvent`
- `VoiceAgentWebSearchTool`
- `Webhook`
- `WebhookProvider`
- `WebhookSignatureEncoding`
- `WebhookSignatureHashAlgorithm`
- `WebhookSigningSecret`
- `WorkflowTagsAddRequest`

</details>

## Provenance

- OpenAPI source: <https://docs.mistral.ai/openapi.yaml>
- Proposed SHA-256: `7652f1307b3bc6cb7a10526c28788a430246f641cb9785a9f1b7b0dd912cdfae`
- Previously pinned public SHA-256: `08a585fb75bcfa6ff968773134ee00060e0db283fc6b16f421f544e78ba8db92`
- Catalog size: 1,548,910 bytes
- Python SDK: [compare commits](https://github.com/mistralai/client-python/compare/8fdbff0b1bcb0c9c3e5b803080a20ac820c2905c...e3444997d52684aaf612ee8fed98f9b8025b95d6)
- TypeScript SDK: [compare commits](https://github.com/mistralai/client-ts/compare/50b2bc105cce327d3bd2ccee86240c4b0f6f2d6c...3e87497eea3ae73f482cd683b043469e72f12389)

## Acceptance

The protected main branch requires a GitHub approving review and the CI `gate` check.
If source generation fails, this PR is a **drift report, not a mergeable SDK update**.
