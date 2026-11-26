# Mistral upstream synchronization

**Human review required** | **Generation: FAILED / INCOMPLETE**

> Automated candidate only. CI verifies technical consistency; a separate
> GitHub PR approval is required to accept API and compatibility changes.

## Summary

| Source | State | Previously pinned → observed |
| --- | --- | --- |
| OpenAPI catalog | **Changed** | `7652f1307b3b` → `acf0efcbee46` |
| Python SDK | **Changed** | [`e3444997d526`](https://github.com/mistralai/client-python/commit/e3444997d52684aaf612ee8fed98f9b8025b95d6) → [`6e9e31ac6f98`](https://github.com/mistralai/client-python/commit/6e9e31ac6f982c1fdfe177787ded223f458c9f69) |
| TypeScript SDK | **Unchanged** | [`3e87497eea3a`](https://github.com/mistralai/client-ts/commit/3e87497eea3ae73f482cd683b043469e72f12389) → [`3e87497eea3a`](https://github.com/mistralai/client-ts/commit/3e87497eea3ae73f482cd683b043469e72f12389) |

**Endpoints:** 347 → 345 (3 added, 5 removed, 26 modified).

## Review checklist

- [ ] Verify the public OpenAPI catalog and official SDK source revisions.
- [ ] Inspect removed/modified operations and schemas for breaking changes.
- [ ] Require regenerated Rust sources and a green CI `gate` check.
- [ ] Approve this PR in GitHub after the final push (never via a lockfile flag).

**Generation attempt:** FAILURE; the source update is not validated as a Rust SDK.

## OpenAPI surface changes

| | Before | After | Added | Removed | Modified |
| --- | ---: | ---: | ---: | ---: | ---: |
| Endpoints | 347 | 345 | 3 | 5 | 26 |
| Schemas | 1106 | 1117 | 30 | 19 | 46 |

A modified schema fingerprint does not necessarily indicate a breaking change.
Removed operations and changed schemas should be reviewed first.

### Most affected API areas

| Area | Added | Removed | Modified |
| --- | ---: | ---: | ---: |
| `/v1/context` | 0 | 0 | 15 |
| `/v1/admin` | 1 | 0 | 5 |
| `/v1/rag` | 0 | 4 | 0 |
| `/v1/observability` | 1 | 1 | 0 |
| `/v1/workflows` | 1 | 0 | 1 |
| `/v1/audio` | 0 | 0 | 1 |
| `/v1/chat` | 0 | 0 | 1 |
| `/v1/connectors` | 0 | 0 | 1 |
| `/v1/files` | 0 | 0 | 1 |
| `/v1/moderations` | 0 | 0 | 1 |

### Changes needing compatibility review

<details>
<summary>Removed endpoints (5)</summary>

- `DELETE /v1/rag/deployments/{deployment_id}`
- `GET /v1/rag/deployments`
- `POST /v1/observability/datasets/{dataset_id}/imports/from-playground`
- `PUT /v1/rag/deployments`
- `PUT /v1/rag/deployments/{deployment_id}/metrics`

</details>

<details>
<summary>Modified endpoints (26)</summary>

- `DELETE /v1/context/search-indexes/{index_name}`
- `DELETE /v1/context/search-indexes/{index_name}/documents`
- `GET /v1/admin/user-groups`
- `GET /v1/admin/users/{user_id}/workspaces`
- `GET /v1/admin/workspaces`
- `GET /v1/connectors/{connector_id_or_name}/auth_url`
- `GET /v1/context/search-indexes`
- `GET /v1/context/search-indexes/{index_name}`
- `GET /v1/context/search-indexes/{index_name}/chunks/{chunk_id}`
- `GET /v1/context/search-indexes/{index_name}/summary_field/{language}`
- `GET /v1/workflows/runs`
- `PATCH /v1/admin/user-groups/{group_uuid}/workspaces/{workspace_uuid}`
- `PATCH /v1/admin/workspaces/{workspace_uuid}/users`
- `POST /v1/audio/speech`
- `POST /v1/chat/completions`
- `POST /v1/context/search-indexes`
- `POST /v1/context/search-indexes/{index_name}/documents`
- `POST /v1/context/search-indexes/{index_name}/grep`
- `POST /v1/context/search-indexes/{index_name}/navigate`
- `POST /v1/context/search-indexes/{index_name}/read`
- `POST /v1/context/search-indexes/{index_name}/search`
- `POST /v1/context/search-indexes/{index_name}/summary_field/{language}`
- `POST /v1/files`
- `POST /v1/moderations`
- `PUT /v1/context/search-indexes/{index_name}`
- `PUT /v1/context/search-indexes/{index_name}/summary_field/{language}`

</details>

<details>
<summary>Removed schemas (19)</summary>

- `GetDeploymentSummariesResponse`
- `GetDeploymentSummariesResponseDeployment`
- `GetDeploymentSummariesResponseVespaDeployment`
- `GetDeploymentSummariesResponseVespaIndex`
- `ImportDatasetFromPlaygroundRequest`
- `IngestionPipelineTargetIndexRef`
- `RegisterDeploymentRequestDeployment`
- `RegisterDeploymentRequestVespaDeployment`
- `RegisterDeploymentRequestVespaField`
- `RegisterDeploymentRequestVespaIndex`
- `RegisterSearchIndexResponseIndex`
- `SchemaFieldDataType`
- `SchemaFieldIndex`
- `SchemaFieldRankingType`
- `SchemaFieldStorage`
- `ToolChoice`
- `UpdateMetricsRequestDeploymentMetricsOffline`
- `UpdateMetricsRequestDeploymentMetricsOnline`
- `UpdateMetricsRequestIndexMetrics`

</details>

<details>
<summary>Modified schemas (46)</summary>

- `AdminAssignGroupToWorkspaceIn`
- `AdminProvisionGroupToWorkspaceIn`
- `AdminUpdateGroupWorkspaceAssignmentIn`
- `AdminUsageQueryFilter`
- `AdminWorkspaceMemberSingleIN`
- `AgentsCompletionRequest`
- `AudioChunk`
- `AudioTranscriptionRequest`
- `AudioTranscriptionRequestStream`
- `AuditLogEventType`
- `ChatCompletionRequest`
- `ChatModerationRequest`
- `ConnectorErrorCode`
- `CreateDatasetRequest`
- `CreateIngestionPipelineConfigurationRequest`
- `CreateWebhook`
- `Dataset`
- `DatasetPreview`
- `DeploymentMistralCloudBackendSpec`
- `DeploymentSecretBinding`
- `DeploymentWorkerSpecInput`
- `EmbeddingRequest`
- `EvaluationRunV2Response`
- `EvaluationRunV2WithRefsResponse`
- `EvaluatorV2`
- `FIMCompletionRequest`
- `GoalResult`
- `ObservabilityErrorCode`
- `PercentileStatisticSpec`
- `ResponseFormat`
- `ResponsesStreamFrame`
- `ScalarStatisticValue`
- `SignalInvocationBody`
- `SimpleNumericStatisticSpec`
- `StaticWorkspaceRoles`
- `ToolCall`
- `TranscriptionStreamEventTypes`
- `TranscriptionStreamEvents`
- `UpdateDatasetRequest`
- `UpdateInvocationBody`
- `UsageInfo`
- `UsageQueryIN`
- `UsageQueryParamsOUT`
- `UsageQueryRowOUT`
- `Webhook`
- `WorkflowExecutionRequest`

</details>

### Additions

<details>
<summary>Added endpoints (3)</summary>

- `GET /v1/admin/workspaces/{workspace_uuid}/users`
- `POST /v1/observability/datasets/{dataset_id}/records/bulk`
- `POST /v1/workflows/deployments/archives`

</details>

<details>
<summary>Added schemas (30)</summary>

- `AdminWorkspaceGroupGrantOUT`
- `AdminWorkspaceMemberOUT`
- `AdminWorkspaceMemberUpsertIN`
- `AdminWorkspaceMemberUpsertSingleIN`
- `AdminWorkspaceUsersOUT`
- `CreateDatasetRecordsItem`
- `CreateDatasetRecordsRequest`
- `CreateDatasetRecordsResponse`
- `CreateDeploymentArchiveRequest`
- `DeploymentArchiveUploadResponse`
- `DisplayFormat`
- `DisplaySpec`
- `EmbeddingInput`
- `EvaluationRunRunScoreV2`
- `MetricDirectionSpec`
- `MissingGoalItem`
- `ModerationRequest`
- `NamedToolChoice`
- `PromptTokensDetails`
- `ScoreMetric`
- `SpeechStreamEventTypes`
- `SummaryChunk`
- `SummaryError`
- `SummaryMetadata`
- `SummaryStatus1`
- `TurnDetectionEagerEndEvent`
- `TurnDetectionEndEvent`
- `TurnDetectionEndReason`
- `TurnDetectionResumeEvent`
- `TurnDetectionStartEvent`

</details>

## Provenance

- OpenAPI source: <https://docs.mistral.ai/openapi.yaml>
- Proposed SHA-256: `acf0efcbee4617601df77fc40185516ed6aae4369ce090034099317d5866b2db`
- Previously pinned public SHA-256: `7652f1307b3bc6cb7a10526c28788a430246f641cb9785a9f1b7b0dd912cdfae`
- Catalog size: 1,564,240 bytes
- Python SDK: [compare commits](https://github.com/mistralai/client-python/compare/e3444997d52684aaf612ee8fed98f9b8025b95d6...6e9e31ac6f982c1fdfe177787ded223f458c9f69)
- TypeScript SDK: [compare commits](https://github.com/mistralai/client-ts/compare/3e87497eea3ae73f482cd683b043469e72f12389...3e87497eea3ae73f482cd683b043469e72f12389)

## Acceptance

The protected main branch requires a GitHub approving review and the CI `gate` check.
If source generation fails, this PR is a **drift report, not a mergeable SDK update**.
