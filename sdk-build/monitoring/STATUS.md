# Mistral upstream synchronization

**Human review required** | **Generation: FAILED / INCOMPLETE**

> Automated candidate only. CI verifies technical consistency; a separate
> GitHub PR approval is required to accept API and compatibility changes.

## Summary

| Source | State | Previously pinned → observed |
| --- | --- | --- |
| OpenAPI catalog | **Changed** | `86b89916f38b` → `08a585fb75bc` |
| Python SDK | **Changed** | [`e8dfa1c8a2d0`](https://github.com/mistralai/client-python/commit/e8dfa1c8a2d0975cebfc0136b2d9060ff555494d) → [`8fdbff0b1bcb`](https://github.com/mistralai/client-python/commit/8fdbff0b1bcb0c9c3e5b803080a20ac820c2905c) |
| TypeScript SDK | **Changed** | [`d8f2d2539cf0`](https://github.com/mistralai/client-ts/commit/d8f2d2539cf00d13d429653ab891b66a3524a2a9) → [`50b2bc105cce`](https://github.com/mistralai/client-ts/commit/50b2bc105cce327d3bd2ccee86240c4b0f6f2d6c) |

**Endpoints:** 288 → 321 (55 added, 22 removed, 69 modified).

## Review checklist

- [ ] Verify the public OpenAPI catalog and official SDK source revisions.
- [ ] Inspect removed/modified operations and schemas for breaking changes.
- [ ] Require regenerated Rust sources and a green CI `gate` check.
- [ ] Approve this PR in GitHub after the final push (never via a lockfile flag).

**Generation attempt:** FAILURE; the source update is not validated as a Rust SDK.

## OpenAPI surface changes

| | Before | After | Added | Removed | Modified |
| --- | ---: | ---: | ---: | ---: | ---: |
| Endpoints | 288 | 321 | 55 | 22 | 69 |
| Schemas | 756 | 1021 | 306 | 41 | 134 |

A modified schema fingerprint does not necessarily indicate a breaking change.
Removed operations and changed schemas should be reviewed first.

### Most affected API areas

| Area | Added | Removed | Modified |
| --- | ---: | ---: | ---: |
| `/v1/observability` | 22 | 16 | 18 |
| `/v1/connectors` | 5 | 6 | 10 |
| `/v1/admin` | 2 | 0 | 12 |
| `/v1/workflows` | 1 | 0 | 13 |
| `/v1/rag` | 12 | 0 | 0 |
| `/v1/service-accounts` | 8 | 0 | 0 |
| `/v1/audio` | 0 | 0 | 3 |
| `/v1/users` | 0 | 0 | 3 |
| `/v1/agents` | 0 | 0 | 2 |
| `/v1/libraries` | 0 | 0 | 2 |
| `/v1/responses` | 2 | 0 | 0 |
| `/v2/prompts` | 0 | 0 | 2 |
| `/v2/skills` | 0 | 0 | 2 |
| `/v1/batch` | 0 | 0 | 1 |
| `/v1/client` | 1 | 0 | 0 |

### Changes needing compatibility review

<details>
<summary>Removed endpoints (22)</summary>

- `DELETE /v1/connectors/{connector_id_or_name}/organization/credentials/{credentials_name}`
- `DELETE /v1/connectors/{connector_id_or_name}/user/credentials/{credentials_name}`
- `DELETE /v1/connectors/{connector_id_or_name}/workspace/credentials/{credentials_name}`
- `DELETE /v1/observability/campaigns/{campaign_id}`
- `GET /v1/observability/campaigns`
- `GET /v1/observability/campaigns/{campaign_id}`
- `GET /v1/observability/campaigns/{campaign_id}/selected-events`
- `GET /v1/observability/campaigns/{campaign_id}/status`
- `GET /v1/observability/chat-completion-events/{event_id}`
- `GET /v1/observability/chat-completion-events/{event_id}/similar-events`
- `GET /v1/observability/chat-completion-fields`
- `GET /v1/observability/chat-completion-fields/{field_name}/options`
- `POST /v1/connectors/{connector_id_or_name}/organization/credentials`
- `POST /v1/connectors/{connector_id_or_name}/user/credentials`
- `POST /v1/connectors/{connector_id_or_name}/workspace/credentials`
- `POST /v1/observability/campaigns`
- `POST /v1/observability/chat-completion-events/search`
- `POST /v1/observability/chat-completion-events/search-ids`
- `POST /v1/observability/chat-completion-events/{event_id}/live-judging`
- `POST /v1/observability/chat-completion-fields/{field_name}/options-counts`
- `POST /v1/observability/datasets/{dataset_id}/imports/from-campaign`
- `POST /v1/observability/datasets/{dataset_id}/imports/from-explorer`

</details>

<details>
<summary>Modified endpoints (69)</summary>

- `DELETE /v1/observability/datasets/{dataset_id}`
- `GET /v1/admin/audit-logs`
- `GET /v1/admin/usage`
- `GET /v1/admin/user-groups/{group_uuid}/members`
- `GET /v1/admin/users`
- `GET /v1/admin/workspaces`
- `GET /v1/agents`
- `GET /v1/agents/pages`
- `GET /v1/audio/voices`
- `GET /v1/batch/jobs`
- `GET /v1/connectors`
- `GET /v1/connectors/{connector_id_or_name}#idOrName`
- `GET /v1/connectors/{connector_id_or_name}/auth_url`
- `GET /v1/connectors/{connector_id_or_name}/organization/credentials`
- `GET /v1/connectors/{connector_id_or_name}/tools`
- `GET /v1/connectors/{connector_id_or_name}/user/credentials`
- `GET /v1/connectors/{connector_id_or_name}/workspace/credentials`
- `GET /v1/files`
- `GET /v1/libraries`
- `GET /v1/libraries/{library_id}/documents`
- `GET /v1/observability/datasets/{dataset_id}`
- `GET /v1/observability/datasets/{dataset_id}/exports/to-jsonl`
- `GET /v1/observability/datasets/{dataset_id}/records`
- `GET /v1/observability/datasets/{dataset_id}/tasks`
- `GET /v1/observability/datasets/{dataset_id}/tasks/{task_id}`
- `GET /v1/observability/judges`
- `GET /v1/observability/traces/{trace_id}/spans`
- `GET /v1/users/me`
- `GET /v1/users/me/organizations`
- `GET /v1/users/me/workspaces`
- `GET /v1/workflows`
- `GET /v1/workflows/deployments`
- `GET /v1/workflows/deployments/{name}/logs`
- `GET /v1/workflows/deployments/{name}/logs/stream`
- `GET /v1/workflows/events/list`
- `GET /v1/workflows/events/stream`
- `GET /v1/workflows/executions/{execution_id}`
- `GET /v1/workflows/executions/{execution_id}/logs`
- `GET /v1/workflows/executions/{execution_id}/logs/stream`
- `GET /v1/workflows/registrations`
- `GET /v1/workflows/runs`
- `GET /v1/workflows/schedules`
- `GET /v1/workflows/{workflow_name}/metrics`
- `GET /v2/prompts`
- `GET /v2/skills`
- `PATCH /v1/admin/user-groups/{group_uuid}`
- `PATCH /v1/admin/user-groups/{group_uuid}/organization-role`
- `PATCH /v1/admin/user-groups/{group_uuid}/workspaces/{workspace_uuid}`
- `PATCH /v1/admin/workspaces/{workspace_uuid}/users`
- `PATCH /v1/audio/voices/{voice_id}`
- `PATCH /v1/connectors/{connector_id}#id`
- `PATCH /v1/observability/datasets/{dataset_id}`
- `PATCH /v2/prompts/{prompt_id}`
- `PATCH /v2/skills/{skill_id}`
- `POST /v1/admin/user-groups/provision-workspace`
- `POST /v1/admin/user-groups/{group_uuid}/workspaces`
- `POST /v1/admin/workspaces/{workspace_uuid}/add-users`
- `POST /v1/audio/speech`
- `POST /v1/connectors`
- `POST /v1/connectors/{connector_id_or_name}/tools/{tool_name}/call`
- `POST /v1/observability/datasets/{dataset_id}/imports/from-dataset`
- `POST /v1/observability/datasets/{dataset_id}/imports/from-file`
- `POST /v1/observability/datasets/{dataset_id}/imports/from-playground`
- `POST /v1/observability/datasets/{dataset_id}/records`
- `POST /v1/observability/logs/search`
- `POST /v1/observability/spans/evaluations/search`
- `POST /v1/observability/spans/evaluations/search/latest`
- `POST /v1/observability/spans/search`
- `POST /v1/observability/traces/search`

</details>

<details>
<summary>Removed schemas (41)</summary>

- `AssignGroupToWorkspaceIn`
- `AuthDirection`
- `BaseFieldDefinition`
- `Campaign`
- `ChatCompletionEvent`
- `ChatCompletionEventPreview`
- `ChatTranscriptionEvent`
- `ConnectionCredentials`
- `ConnectorProtocol`
- `CreateCampaignRequest`
- `FeedResultChatCompletionEventPreview`
- `FetchCampaignStatusResponse`
- `FetchChatCompletionFieldOptionsResponse`
- `FetchFieldOptionCountsRequest`
- `FetchFieldOptionCountsResponse`
- `FieldGroup`
- `FieldOptionCountItem`
- `FilterCondition`
- `FilterGroup`
- `FilterPayload`
- `ImportDatasetFromCampaignRequest`
- `ImportDatasetFromExplorerRequest`
- `InboundAuthenticationType`
- `JudgeChatCompletionEventRequest`
- `ListCampaignSelectedEventsResponse`
- `ListCampaignsResponse`
- `ListChatCompletionFieldsResponse`
- `OAuth2Token`
- `OutboundAuthenticationType`
- `PaginatedResultCampaignPreview`
- `PaginatedResultChatCompletionEventPreview`
- `SearchChatCompletionEventIdsRequest`
- `SearchChatCompletionEventIdsResponse`
- `SearchChatCompletionEventsRequest`
- `SearchChatCompletionEventsResponse`
- `UpdateConnectorRequest`
- `UpdateGroupWorkspaceAssignmentIn`
- `UpdateUserGroupOrganizationRoleIn`
- `UserGroupTargetType`
- `WorkspaceMemberIN`
- `WorkspaceMemberSingleIN`

</details>

<details>
<summary>Modified schemas (134)</summary>

- `ActivityTaskCompletedResponse`
- `ActivityTaskFailedResponse`
- `ActivityTaskRetryingResponse`
- `ActivityTaskStartedResponse`
- `ActorType`
- `AdminProvisionGroupToWorkspaceIn`
- `AdminUpdateUserGroupIn`
- `AdminUserGroupIn`
- `AdminUserGroupOut`
- `Agent`
- `AgentsCompletionRequest`
- `Annotations`
- `AudioContent`
- `AuditLogEventType`
- `AuthData`
- `AuthenticationConfiguration`
- `AuthenticationMethodCreateOrUpdateRequest`
- `BaseModelCard`
- `BlobResourceContents`
- `BuiltInConnectors`
- `ChatCompletionChoice`
- `ChatCompletionRequest`
- `ClientCapabilities`
- `ClientTasksCapability`
- `ClientTasksRequestsCapability`
- `CompletionResponseStreamChoice`
- `CompletionsCapability`
- `Connector`
- `ConnectorSupportedLanguage`
- `ConnectorToolCallResponse`
- `ConnectorsQueryFilters`
- `ContentChunk`
- `CreateConnectorRequest`
- `CreateDatasetRecordRequest`
- `CreateIngestionPipelineConfigurationRequest`
- `CreatePromptRequest`
- `CreateSkillRequest`
- `CredentialsCreateOrUpdate`
- `CredentialsResponse`
- `CredentialsStatusErrorReason`
- `CustomTaskCanceledResponse`
- `CustomTaskCompletedResponse`
- `CustomTaskFailedResponse`
- `CustomTaskInProgressResponse`
- `CustomTaskStartedResponse`
- `CustomTaskTimedOutResponse`
- `Dataset`
- `DatasetImportTask`
- `DatasetPreview`
- `DatasetRecordSource`
- `DeploymentDetailResponse`
- `DeploymentResourceConfig`
- `DeploymentResourceConfigUpdate`
- `DeploymentResponse`
- `DeploymentWorkerResponse`
- `DeploymentWorkerSpecInput`
- `DeploymentWorkerSpecResponse`
- `ElicitationCapability`
- `EmbeddedResource`
- `ExtendedOAuthServerMetadata`
- `FTModelCard`
- `FilePurpose`
- `FormElicitationCapability`
- `GroupWorkspaceAssignmentOut`
- `ImageContent`
- `ListLibrariesResponse`
- `LoggingCapability`
- `MCPResource`
- `MCPServerIcon`
- `MCPSupportedLanguage`
- `ManagedDeploymentResponse`
- `ModelCapabilities`
- `ObservabilityErrorCode`
- `OrganizationInviteIN`
- `OtelFieldDefinition`
- `Prompt`
- `PromptArgument`
- `PromptsCapability`
- `PublicAuthenticationMethod`
- `PublicConnectorExecutionData`
- `PublicResourceVisibility`
- `RegisterDeploymentRequestVespaIndex`
- `ResourceLink`
- `ResourcesCapability`
- `RootsCapability`
- `SamplingCapability`
- `SamplingContextCapability`
- `SamplingToolsCapability`
- `ScimSyncUserRefOut`
- `ServerCapabilities`
- `ServerTasksCapability`
- `ServerTasksRequestsCapability`
- `SpeechRequest`
- `StaticWorkspaceRoles`
- `SubscriptionStatus`
- `TargetType`
- `TasksCallCapability`
- `TasksCancelCapability`
- `TasksCreateElicitationCapability`
- `TasksCreateMessageCapability`
- `TasksElicitationCapability`
- `TasksListCapability`
- `TasksSamplingCapability`
- `TasksToolsCapability`
- `TempoTraceAttribute`
- `TempoTraceAttributeArrayElement`
- `TextContent`
- `TextResourceContents`
- `ToolAnnotations`
- `ToolExecution`
- `ToolsCapability`
- `TranscriptionSegmentChunk`
- `TranscriptionStreamSegmentDelta`
- `UrlElicitationCapability`
- `UsageOUTJSON`
- `VoiceCreateRequest`
- `VoiceResponse`
- `VoiceUpdateRequest`
- `WorkflowExecutionCanceledResponse`
- `WorkflowExecutionCompletedResponse`
- `WorkflowExecutionContinuedAsNewResponse`
- `WorkflowExecutionFailedResponse`
- `WorkflowExecutionRequest`
- `WorkflowExecutionResponse`
- `WorkflowExecutionStartedResponse`
- `WorkflowExecutionTraceEventsResponse`
- `WorkflowExecutionTraceOTelResponse`
- `WorkflowExecutionTraceSummaryResponse`
- `WorkflowMetrics`
- `WorkflowTaskFailedResponse`
- `WorkflowTaskTimedOutResponse`
- `WorkflowsWorkerSpecUpdate`
- `WorkspaceEnrichedOUT`
- `WorkspaceOUT`

</details>

### Additions

<details>
<summary>Added endpoints (55)</summary>

- `DELETE /v1/connectors/{connector_id_or_name}/{consumer_scope}/credentials/{credentials_name}`
- `DELETE /v1/connectors/{connector_id}/organization/share`
- `DELETE /v1/observability/pipelines/{pipeline_id}`
- `DELETE /v1/rag/managed_indexes/{index_name}`
- `DELETE /v1/rag/managed_indexes/{index_name}/documents`
- `DELETE /v1/service-accounts/{service_account_id}`
- `GET /v1/observability/evaluation-runs_v2/{run_id}`
- `GET /v1/observability/evaluation-runs_v2/{run_id}/records/{record_id}`
- `GET /v1/observability/evaluation-runs_v2/{run_id}/statistics`
- `GET /v1/observability/evaluations_v2/{evaluation_slug}`
- `GET /v1/observability/optimizations`
- `GET /v1/observability/optimizations/{optimization_id}`
- `GET /v1/observability/optimizations/{optimization_id}/trials`
- `GET /v1/observability/pipelines`
- `GET /v1/observability/pipelines/{pipeline_id}`
- `GET /v1/observability/projects`
- `GET /v1/observability/projects/{project_slug}`
- `GET /v1/observability/projects/{project_slug}/evaluations_v2`
- `GET /v1/rag/managed_indexes`
- `GET /v1/rag/managed_indexes/{index_name}`
- `GET /v1/rag/managed_indexes/{index_name}/chunks/{chunk_id}`
- `GET /v1/service-accounts`
- `GET /v1/service-accounts/assignable-roles`
- `GET /v1/service-accounts/{service_account_id}`
- `GET /v1/service-accounts/{service_account_id}/roles`
- `GET /v1/whoami`
- `GET /v2/audio/voices`
- `PATCH /v1/connectors/{connector_id_or_name}/{consumer_scope}/credentials`
- `PATCH /v1/service-accounts/{service_account_id}`
- `POST /v1/admin/usage/query`
- `POST /v1/admin/users/{user_id}/deactivate`
- `POST /v1/client/sessions`
- `POST /v1/connectors/{connector_id_or_name}/{consumer_scope}/credentials`
- `POST /v1/observability/datasets/{dataset_id}/imports/from-spans`
- `POST /v1/observability/evaluation-runs_v2/search`
- `POST /v1/observability/evaluation-runs_v2/{run_id}/records/search`
- `POST /v1/observability/evaluation-runs_v2/{run_id}/statistics/compute`
- `POST /v1/observability/evaluations_v2/{evaluation_slug}/runs/search`
- `POST /v1/observability/pipeline-results/search`
- `POST /v1/observability/pipelines`
- `POST /v1/observability/spans/evaluations/aggregate`
- `POST /v1/rag/managed_indexes`
- `POST /v1/rag/managed_indexes/{index_name}/documents`
- `POST /v1/rag/managed_indexes/{index_name}/grep`
- `POST /v1/rag/managed_indexes/{index_name}/navigate`
- `POST /v1/rag/managed_indexes/{index_name}/read`
- `POST /v1/rag/managed_indexes/{index_name}/search`
- `POST /v1/responses`
- `POST /v1/responses/input_tokens`
- `POST /v1/service-accounts`
- `POST /v1/workflows/deployments/{deployment_id}/unharden`
- `PUT /v1/connectors/{connector_id}/organization/share`
- `PUT /v1/observability/pipelines/{pipeline_id}`
- `PUT /v1/rag/managed_indexes/{index_name}`
- `PUT /v1/service-accounts/{service_account_id}/roles`

</details>

<details>
<summary>Added schemas (306)</summary>

- `AcceptedDocumentResponse`
- `AdminAssignGroupToWorkspaceIn`
- `AdminUpdateGroupWorkspaceAssignmentIn`
- `AdminUpdateUserGroupOrganizationRoleIn`
- `AdminUsageQueryFilter`
- `AdminWorkspaceMemberIN`
- `AdminWorkspaceMemberSingleIN`
- `And`
- `AssignableServiceAccountRole`
- `AttributeHashSampling`
- `AuthenticationType`
- `BearerAuthMethod`
- `BoolArrayFieldDefinition`
- `BoolFieldDefinition`
- `CategoricalStatistics`
- `ChatCompletionResponseChoiceLogprobs`
- `ChatCompletionResponseTokenLogprobs`
- `ChatCompletionResponseTokenLogprobsTopK`
- `ClassificationStatisticSpec`
- `ClientSecret`
- `ClientSessionPurpose`
- `ConfusionMatrix`
- `ConnectionCredentialsInput`
- `ConnectorMCPPublicUpdate`
- `ContentSearchFilter`
- `ContentSearchFilterGroup`
- `ContentSearchOperator`
- `CountResponseInputTokensRequest`
- `CountResponseInputTokensResponse`
- `CreateHTTPConnectorRequest`
- `CreateManagedIndexRequest`
- `CreatePipelineRequest`
- `CreateRealtimeSessionRequest`
- `CreateRealtimeSessionResponse`
- `CreateResponseRequest`
- `CreateResponseResponse`
- `CreateServiceAccountRequest`
- `CustomEmbeddingModel`
- `DatasetImportResult`
- `DeactivateUserOUT`
- `DeleteDocumentsRequest`
- `DeleteDocumentsResponse`
- `DeleteManagedIndexResponse`
- `DenseVectorFieldDefinition`
- `DeploymentK8sBackendSpec`
- `DeploymentLogType`
- `DeploymentMistralCloudBackendSpec`
- `DeploymentSecretBinding`
- `DetectionDefinitionV1`
- `DetectionPattern`
- `DistanceMetric`
- `DocumentFieldErrorResponse`
- `EmbeddingMismatchResponse`
- `Equal`
- `EvaluationBooleanResultSpec`
- `EvaluationCategoricalResultSpec`
- `EvaluationNumericResultSpec`
- `EvaluationResultSpec`
- `EvaluationRunFilterInput`
- `EvaluationRunGenerationV2`
- `EvaluationRunOutputRecordV2`
- `EvaluationRunRecordFilterInput`
- `EvaluationRunRecordV2Response`
- `EvaluationRunScoreV2`
- `EvaluationRunV2Response`
- `EvaluationRunV2WithRefsResponse`
- `EvaluationV2`
- `EvaluationsV2`
- `EvaluatorStatistics`
- `EvaluatorV2`
- `ExportDefinitionV1`
- `FeedResultGetPipelineResult`
- `FieldSortInput`
- `FilterCombinator`
- `FloatArrayFieldDefinition`
- `FloatFieldDefinition`
- `GetPipelineResults`
- `GoalResult`
- `GoalResultItem`
- `GoalSpec`
- `GoalStatus`
- `GoalStatusFilter`
- `GoalStatusFilterGroup`
- `GrepMode`
- `GrepRequest`
- `HTTPConnector`
- `IPAPhoneme`
- `ImportDatasetFromSpansRequest`
- `In`
- `IncorrectFieldTypeResponse`
- `IngestDocumentsRequest`
- `IngestDocumentsResponse`
- `IngestionPipelineTargetIndexRef`
- `IntArrayFieldDefinition`
- `IntFieldDefinition`
- `JsonFieldDefinition`
- `JsonValue`
- `JudgeDefinitionV1`
- `KeywordArrayFieldDefinition`
- `KeywordFieldDefinition`
- `KeywordRetriever`
- `ListAssignableServiceAccountRolesResponse`
- `ListEvaluationRunRecordsResponse`
- `ListEvaluationRunsResponse`
- `ListManagedIndexesResponse`
- `ListOptimizationTrialsResponse`
- `ListOptimizationsResponse`
- `ListServiceAccountRolesResponse`
- `ListServiceAccountsResponse`
- `LlmJudgeReference`
- `LongArrayFieldDefinition`
- `LongFieldDefinition`
- `MCPConnector`
- `ManagedIndexConfig`
- `ManagedIndexFields`
- `ManagedIndexResponse`
- `ManagedIndexStatus`
- `MaxCandidatesTooSmallResponse`
- `MetadataFilter`
- `MetadataFilterGroup`
- `MissingEmbeddingResponse`
- `MissingFieldResponse`
- `MistralEmbeddingModel`
- `ModerationDefinitionV1`
- `NavigateRequest`
- `NavigationChunkResponse`
- `NavigationDirection`
- `NavigationResponse`
- `NearestNeighbourRetriever`
- `NoneAuthMethod`
- `Not`
- `NumericScoreMetric`
- `NumericStatistics`
- `OAuth2AuthMethod`
- `OAuth2AuthorizationCodeAuthMethod`
- `OAuth2ClientCredentialsAuthMethod`
- `OAuth2ClientCredentialsInput`
- `OAuth2GrantType`
- `OAuth2TokenEndpointAuthMethod`
- `OTLPDestination`
- `Optimization`
- `OptimizationDetail`
- `OptimizationReference`
- `OptimizationTrialObservation`
- `OptimizationTrialWithObservations`
- `Or`
- `PaginatedResultEvaluationRunRecordV2Response`
- `PaginatedResultEvaluationRunV2Response`
- `PaginatedResultEvaluationRunV2WithRefsResponse`
- `PaginatedResultEvaluationV2`
- `PaginatedResultOptimization`
- `PaginatedResultOptimizationTrialWithObservations`
- `PaginatedResultPipeline`
- `PaginatedResultProject`
- `PercentileStatisticSpec`
- `Pipeline`
- `PipelineConfig`
- `PipelineConfigDefinition`
- `PipelineConfigHeader`
- `PipelineConfigSampling`
- `PipelineConfigScope`
- `PipelineConfigSelector`
- `PipelineKind`
- `PipelineResult`
- `PipelineResultsRequest`
- `PipelinesResponse`
- `PrincipalType`
- `ProbabilisticSampling`
- `Project`
- `ProjectEvaluationSummary`
- `Projects`
- `RRFRetriever`
- `Range`
- `ReadRequest`
- `RecordSortField`
- `RejectedDocumentResponse`
- `ResourceChunk`
- `ResourceLinkChunk`
- `ResponsesAnnotationContainerFileCitation`
- `ResponsesAnnotationFileCitation`
- `ResponsesAnnotationFilePath`
- `ResponsesAnnotationURLCitation`
- `ResponsesCallerDirect`
- `ResponsesCallerProgram`
- `ResponsesCompletedEvent`
- `ResponsesContent`
- `ResponsesContentPartAddedEvent`
- `ResponsesContentPartDoneEvent`
- `ResponsesCreatedEvent`
- `ResponsesCustomTool`
- `ResponsesEasyInputMessage`
- `ResponsesError`
- `ResponsesErrorEvent`
- `ResponsesFailedEvent`
- `ResponsesFormatJSONObject`
- `ResponsesFormatText`
- `ResponsesFormatTextJSONSchemaConfig`
- `ResponsesFunctionCallArgumentsDeltaEvent`
- `ResponsesFunctionCallArgumentsDoneEvent`
- `ResponsesFunctionCallOutput`
- `ResponsesFunctionCallOutputCallerDirect`
- `ResponsesFunctionCallOutputCallerProgram`
- `ResponsesFunctionTool`
- `ResponsesFunctionToolCall`
- `ResponsesGrammar`
- `ResponsesInProgressEvent`
- `ResponsesIncompleteDetails`
- `ResponsesIncompleteEvent`
- `ResponsesInputFile`
- `ResponsesInputFileContent`
- `ResponsesInputImage`
- `ResponsesInputImageContent`
- `ResponsesInputText`
- `ResponsesInputTextContent`
- `ResponsesInputTokensDetails`
- `ResponsesMessage`
- `ResponsesNamespaceTool`
- `ResponsesOutputItemAddedEvent`
- `ResponsesOutputItemDoneEvent`
- `ResponsesOutputMessage`
- `ResponsesOutputRefusal`
- `ResponsesOutputText`
- `ResponsesOutputTextLogprob`
- `ResponsesOutputTextLogprobTopLogprob`
- `ResponsesOutputTokensDetails`
- `ResponsesPartReasoningText`
- `ResponsesPromptCacheBreakpoint`
- `ResponsesReasoningItem`
- `ResponsesReasoningTextDeltaEvent`
- `ResponsesReasoningTextDoneEvent`
- `ResponsesStreamFrame`
- `ResponsesSummary`
- `ResponsesSupportedReasoning`
- `ResponsesText`
- `ResponsesTextConfig`
- `ResponsesTextDeltaEvent`
- `ResponsesTextDeltaEventLogprob`
- `ResponsesTextDeltaEventLogprobTopLogprob`
- `ResponsesTextDoneEvent`
- `ResponsesTextDoneEventLogprob`
- `ResponsesTextDoneEventLogprobTopLogprob`
- `ResponsesToolChoiceFunction`
- `ResponsesToolFunction`
- `ResponsesUsage`
- `RunEvaluationSummary`
- `RunOptimizationRef`
- `RunOwnershipFilter`
- `RunProjectSummary`
- `RunStatisticsResponse`
- `ScalarStatisticValue`
- `ScoreFilter`
- `ScoreFilterGroup`
- `ScoreOperator`
- `ScoreSortInput`
- `SearchAllEvaluationRunsResponse`
- `SearchChunkResponse`
- `SearchHitResponse`
- `SearchRequest`
- `SearchResponse`
- `ServiceAccount`
- `ServiceAccountRole`
- `SetServiceAccountRolesRequest`
- `ShareRelation`
- `SimpleNumericStatisticSpec`
- `SortDirection`
- `SortOrder`
- `SourceKind`
- `SpanDatasetMapping`
- `SpanDatasetMappingContract`
- `SpanDatasetSourceNamespace`
- `SpanDatasetSourceReference`
- `SystemFilter`
- `SystemNameFilter`
- `SystemNameFilterGroup`
- `SystemParamFilter`
- `SystemParamFilterGroup`
- `SystemParamOperator`
- `SystemV2`
- `TagFilter`
- `TelemetrySpanReference`
- `TempoTraceAttributeDoubleValue`
- `TextArrayFieldDefinition`
- `TextFieldDefinition`
- `TimestampArrayFieldDefinition`
- `TimestampFieldDefinition`
- `TopKTooLargeResponse`
- `UnsupportedStatistics`
- `UpdateHTTPConnectorRequest`
- `UpdateManagedIndexRequest`
- `UpdatePipelineRequest`
- `UpdateServiceAccountRequest`
- `UsageQueryAggregatesOUT`
- `UsageQueryIN`
- `UsageQueryMetadataOUT`
- `UsageQueryOUT`
- `UsageQueryParamsOUT`
- `UsageQueryRowOUT`
- `UsageType`
- `VectorDType`
- `VoiceGender`
- `VoiceKind`
- `VoiceListPage`
- `WhoamiOrganization`
- `WhoamiResponse`
- `WhoamiServiceAccountPrincipal`
- `WhoamiUserPrincipal`

</details>

## Provenance

- OpenAPI source: <https://docs.mistral.ai/openapi.yaml>
- Proposed SHA-256: `08a585fb75bcfa6ff968773134ee00060e0db283fc6b16f421f544e78ba8db92`
- Previously pinned public SHA-256: `4cb0b5382d4f7fe4071e6328caa765f55344615916d6fc9db2d1480460c9d41c`
- Catalog size: 1,427,215 bytes
- Python SDK: [compare commits](https://github.com/mistralai/client-python/compare/e8dfa1c8a2d0975cebfc0136b2d9060ff555494d...8fdbff0b1bcb0c9c3e5b803080a20ac820c2905c)
- TypeScript SDK: [compare commits](https://github.com/mistralai/client-ts/compare/d8f2d2539cf00d13d429653ab891b66a3524a2a9...50b2bc105cce327d3bd2ccee86240c4b0f6f2d6c)

## Acceptance

The protected main branch requires a GitHub approving review and the CI `gate` check.
If source generation fails, this PR is a **drift report, not a mergeable SDK update**.
