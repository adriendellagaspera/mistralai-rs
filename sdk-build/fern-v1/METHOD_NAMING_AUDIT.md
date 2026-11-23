# Rust SDK method naming audit

Audit source: `sdk-build/fern/inventory.json` and `sdk-build/fern/policy.yaml` at commit `077f0b3c573763d647748d8b75867ca23158fb49`.

## Scope

- 347 OpenAPI operations in the current source catalog.
- 347 published operations; 0 excluded.
- 349 public method paths, because streaming operations can expose both a regular and a stream method.
- 1770 public types.

This audit covers the complete reviewed SDK surface and the 16 high-confidence method-name corrections applied to the source policy, inventory, and production-generated Rust SDK. The Fern v1 overlay mirrors the same corrections for diagnostic validation.

## Naming convention proposed

- Use `list` for collection retrieval and `get` for a single identified object or scalar resource.
- Use concise verbs in resource-scoped modules: `create`, `get`, `list`, `update`, `delete`, `cancel`, `terminate`, `stream`, `execute`, `search`, `complete`, etc.
- Avoid repeating the resource hierarchy already expressed by the module path (`workflows.executions.get_logs`, not `get_workflow_execution_logs`).
- Do not expose OpenAPI implementation artefacts such as `_v1`, HTTP verbs, or numeric collision suffixes in Rust method names.
- Keep semantically meaningful distinctions (e.g. `cancel` vs `terminate`, `update` vs `update_metadata`, JSON request vs SSE stream).

## High-confidence naming corrections

| Endpoint | Current method | Proposed method | Rationale |
|---|---|---|---|
| PUT `/v1/libraries/{library_id}` | `libraries_update_v1` | `update_legacy` | Deprecated PUT endpoint; PATCH already owns the canonical `update` method. |
| GET `/v1/workflows` | `get_workflows` | `list` | Collection endpoint; module is already `workflows`. |
| GET `/v1/workflows/registrations` | `get_workflow_registrations` | `list_registrations` | Collection endpoint; avoid repeating `workflow`. |
| GET `/v1/workflows/schedules` | `get_schedules` | `list` | Collection endpoint; module is already `workflows.schedules`. |
| GET `/v1/workflows/events/list` | `get_workflow_events` | `list` | Collection operation in the `events` resource. |
| GET `/v1/workflows/events/stream` | `get_stream_events` | `stream` | Stream operation; current verb order is awkward. |
| GET `/v1/observability/traces/fields` | `get_trace_fields` | `list_fields` | Collection of trace field descriptors. |
| GET `/v1/rag/deployments` | `get_deployment_summaries` | `list_deployment_summaries` | Collection endpoint; explicit list semantics. |
| GET `/v1/workflows/deployments/{name}/logs` | `get_deployment_logs` | `get_logs` | The deployment context is already in the module. |
| GET `/v1/workflows/deployments/{name}/logs/stream` | `stream_deployment_logs` | `stream_logs` | The deployment context is already in the module. |
| GET `/v1/workflows/executions/{execution_id}/logs` | `get_workflow_execution_logs` | `get_logs` | The execution context is already in the module. |
| GET `/v1/workflows/executions/{execution_id}/logs/stream` | `stream_workflow_execution_logs` | `stream_logs` | The execution context is already in the module. |
| GET `/v1/workflows/executions/{execution_id}/trace/events` | `get_workflow_execution_trace_events` | `list_trace_events` | Remove repeated resource hierarchy while retaining the trace distinction. |
| GET `/v1/workflows/executions/{execution_id}/trace/info` | `get_workflow_execution_trace_info` | `get_trace_info` | Remove repeated resource hierarchy. |
| GET `/v1/workflows/executions/{execution_id}/trace/otel` | `get_workflow_execution_trace_otel` | `get_trace_otel` | Remove repeated resource hierarchy. |
| GET `/v1/workflows/executions/{execution_id}/trace/summary` | `get_workflow_execution_trace_summary` | `get_trace_summary` | Remove repeated resource hierarchy. |

These 16 naming corrections are now applied to the reviewed SDK policy and production Fern 0.48 sources. The legacy library PUT operation is named `update_legacy` because the current PATCH endpoint already owns `update`; Mistral documents PUT as deprecated and recommends PATCH. The collection operations retain `list` naming even when the response is paginated: the verb describes collection semantics, not whether the SDK transparently fetches every page. `get_budget`, `get_subscription`, `get_identity`, and `get_invite` remain `get` because they address singular resources or scalar state. Trace event retrieval is named `list_trace_events` to distinguish the collection from the singular trace metadata operations.

## Complete operation inventory

| HTTP | Path | OpenAPI operationId | SDK method path(s) from current policy |
|---|---|---|---|
| POST | `/v1/agents/completions` | `agents_completion_v1_agents_completions_post` | `agents.complete` |
| POST | `/v1/audio/agents` | `create_agent_v1_audio_agents_post` | `audio.agents.create` |
| DELETE | `/v1/audio/agents/{agent_id}` | `delete_agent_v1_audio_agents__agent_id__delete` | `audio.agents.delete` |
| GET | `/v1/audio/agents/{agent_id}` | `get_agent_v1_audio_agents__agent_id__get` | `audio.agents.get` |
| GET | `/v1/audio/agents` | `list_agents_v1_audio_agents_get` | `audio.agents.list` |
| GET | `/v1/audio/agents/{agent_id}/sessions` | `list_agent_sessions_v1_audio_agents__agent_id__sessions_get` | `audio.agents.list_sessions` |
| GET | `/v1/audio/agents/{agent_id}/versions` | `list_agent_versions_v1_audio_agents__agent_id__versions_get` | `audio.agents.list_versions` |
| PUT | `/v1/audio/agents/{agent_id}` | `update_agent_v1_audio_agents__agent_id__put` | `audio.agents.update` |
| DELETE | `/v1/audio/sessions/{session_id}` | `delete_session_v1_audio_sessions__session_id__delete` | `audio.sessions.close` |
| POST | `/v1/audio/sessions` | `create_session_v1_audio_sessions_post` | `audio.sessions.create` |
| GET | `/v1/audio/sessions/{session_id}` | `get_session_v1_audio_sessions__session_id__get` | `audio.sessions.get` |
| PATCH | `/v1/audio/sessions/{session_id}` | `update_session_v1_audio_sessions__session_id__patch` | `audio.sessions.update` |
| POST | `/v1/audio/speech` | `speech_v1_audio_speech_post` | `audio.speech.complete` |
| POST | `/v1/audio/transcriptions` | `audio_api_v1_transcriptions_post` | `audio.transcriptions.complete` |
| POST | `/v1/audio/transcriptions#stream` | `audio_api_v1_transcriptions_post_stream` | `audio.transcriptions.stream` |
| POST | `/v1/audio/voices` | `create_voice_v1_audio_voices_post` | `audio.voices.create` |
| DELETE | `/v1/audio/voices/{voice_id}` | `delete_voice_v1_audio_voices__voice_id__delete` | `audio.voices.delete` |
| GET | `/v1/audio/voices/{voice_id}` | `get_voice_v1_audio_voices__voice_id__get` | `audio.voices.get` |
| GET | `/v1/audio/voices/{voice_id}/sample` | `get_voice_sample_audio_v1_audio_voices__voice_id__sample_get` | `audio.voices.get_sample_audio` |
| GET | `/v1/audio/voices` | `list_voices_v1_audio_voices_get` | `audio.voices.list` |
| GET | `/v2/audio/voices` | `audio_v2_voices_list` | `audio.voices.search` |
| PATCH | `/v1/audio/voices/{voice_id}` | `update_voice_v1_audio_voices__voice_id__patch` | `audio.voices.update` |
| POST | `/v1/batch/jobs/{job_id}/cancel` | `jobs_api_routes_batch_cancel_batch_job` | `batch.jobs.cancel` |
| POST | `/v1/batch/jobs` | `jobs_api_routes_batch_create_batch_job` | `batch.jobs.create` |
| DELETE | `/v1/batch/jobs/{job_id}` | `jobs_api_routes_batch_delete_batch_job` | `batch.jobs.delete` |
| GET | `/v1/batch/jobs/{job_id}` | `jobs_api_routes_batch_get_batch_job` | `batch.jobs.get` |
| GET | `/v1/batch/jobs` | `jobs_api_routes_batch_get_batch_jobs` | `batch.jobs.list` |
| POST | `/v1/admin/api-keys` | `users_api_admin_api_keys_create_api_key` | `beta.admin.api_keys.create` |
| DELETE | `/v1/admin/api-keys/{key_id}` | `users_api_admin_api_keys_delete_api_key` | `beta.admin.api_keys.delete` |
| GET | `/v1/admin/api-keys` | `users_api_admin_api_keys_get_api_keys` | `beta.admin.api_keys.list` |
| GET | `/v1/admin/audit-logs` | `users_api_admin_audit_logs_get_audit_logs` | `beta.admin.audit_logs.list` |
| GET | `/v1/admin/rate-limit` | `users_api_admin_rate_limits_get_rate_limits` | `beta.admin.rate_limits.get` |
| GET | `/v1/admin/roles` | `users_api_admin_roles_get_roles` | `beta.admin.roles.list` |
| GET | `/v1/admin/scim/sync/{run_id}` | `users_api_admin_scim_sync_get_scim_sync_run` | `beta.admin.scim.get_sync_run` |
| POST | `/v1/admin/scim/sync` | `users_api_admin_scim_sync_trigger_scim_sync` | `beta.admin.scim.trigger_sync` |
| GET | `/v1/admin/spend-limit` | `users_api_admin_spend_limits_get_spend_limits` | `beta.admin.spend_limits.get` |
| POST | `/v1/admin/spend-limit` | `users_api_admin_spend_limits_update_spend_limits` | `beta.admin.spend_limits.update` |
| GET | `/v1/admin/usage` | `users_api_admin_usage_get_usage` | `beta.admin.usage.get` |
| POST | `/v1/admin/usage/query` | `users_api_admin_usage_query_usage` | `beta.admin.usage.query` |
| POST | `/v1/admin/user-groups/{group_uuid}/members` | `users_api_admin_user_groups_assign_users_to_group` | `beta.admin.user_groups.add_users` |
| POST | `/v1/admin/user-groups/{group_uuid}/workspaces` | `users_api_admin_user_groups_assign_group_to_workspace` | `beta.admin.user_groups.assign_to_workspace` |
| POST | `/v1/admin/user-groups` | `users_api_admin_user_groups_create_user_group` | `beta.admin.user_groups.create` |
| DELETE | `/v1/admin/user-groups/{group_uuid}` | `users_api_admin_user_groups_delete_user_group` | `beta.admin.user_groups.delete` |
| GET | `/v1/admin/user-groups/{group_uuid}` | `users_api_admin_user_groups_get_user_group` | `beta.admin.user_groups.get` |
| GET | `/v1/admin/user-groups` | `users_api_admin_user_groups_get_user_groups` | `beta.admin.user_groups.list` |
| GET | `/v1/admin/user-groups/{group_uuid}/members` | `users_api_admin_user_groups_get_user_group_members` | `beta.admin.user_groups.list_members` |
| GET | `/v1/admin/user-groups/{group_uuid}/workspaces` | `users_admin_user_groups_get_group_workspace_assignments` | `beta.admin.user_groups.list_workspace_assignments` |
| POST | `/v1/admin/user-groups/provision-workspace` | `users_api_admin_user_groups_provision_group_to_workspace` | `beta.admin.user_groups.provision_to_workspace` |
| DELETE | `/v1/admin/user-groups/{group_uuid}/workspaces/{workspace_uuid}` | `users_admin_user_groups_remove_group_from_workspace` | `beta.admin.user_groups.remove_from_workspace` |
| DELETE | `/v1/admin/user-groups/{group_uuid}/members` | `users_api_admin_user_groups_remove_users_from_group` | `beta.admin.user_groups.remove_users` |
| PATCH | `/v1/admin/user-groups/{group_uuid}` | `users_api_admin_user_groups_update_user_group` | `beta.admin.user_groups.update` |
| PATCH | `/v1/admin/user-groups/{group_uuid}/organization-role` | `users_admin_user_groups_update_user_group_organization_role` | `beta.admin.user_groups.update_organization_role` |
| PATCH | `/v1/admin/user-groups/{group_uuid}/workspaces/{workspace_uuid}` | `users_admin_user_groups_update_group_workspace_assignment` | `beta.admin.user_groups.update_workspace_assignment` |
| POST | `/v1/admin/users` | `users_api_admin_users_create_users` | `beta.admin.users.create` |
| POST | `/v1/admin/users/{user_id}/deactivate` | `users_api_admin_users_deactivate_user` | `beta.admin.users.deactivate` |
| DELETE | `/v1/admin/users/{user_id}` | `users_api_admin_users_delete_user` | `beta.admin.users.delete` |
| DELETE | `/v1/admin/users-invite/{invite_uuid}` | `users_api_admin_users_delete_invite` | `beta.admin.users.delete_invite` |
| GET | `/v1/admin/users/{user_id}` | `users_api_admin_users_get_user` | `beta.admin.users.get` |
| GET | `/v1/admin/users-invite` | `users_api_admin_users_get_invite` | `beta.admin.users.get_invite` |
| POST | `/v1/admin/users-invite` | `users_api_admin_users_invite_users` | `beta.admin.users.invite` |
| GET | `/v1/admin/users` | `users_api_admin_users_get_users` | `beta.admin.users.list` |
| GET | `/v1/admin/users/{user_id}/workspaces` | `users_api_admin_users_get_user_workspaces` | `beta.admin.users.list_workspaces` |
| POST | `/v1/admin/users/{user_id}/reactivate` | `users_api_admin_users_reactivate_user` | `beta.admin.users.reactivate` |
| PATCH | `/v1/admin/users/{user_id}` | `users_api_admin_users_update_user` | `beta.admin.users.update` |
| GET | `/v1/admin/analytics/vibe/code/usage/by_organization` | `get_organization_stats_v1_admin_analytics_vibe_code_usage_by_organization` | `beta.admin.vibe_code_analytics.organization_stats` |
| GET | `/v1/admin/analytics/vibe/code/usage/by_workspace` | `get_workspace_stats_v1_admin_analytics_vibe_code_usage_by_workspace` | `beta.admin.vibe_code_analytics.workspace_stats` |
| GET | `/v1/admin/analytics/vibe/work/usage/by_agent_stats` | `get_by_agent_stats_v1_admin_analytics_vibe_work_usage_by_agent_stats` | `beta.admin.vibe_work_analytics.agent_stats` |
| GET | `/v1/admin/analytics/vibe/work/usage/by_time_stats` | `get_by_time_stats_v1_admin_analytics_vibe_work_usage_by_time_stats` | `beta.admin.vibe_work_analytics.time_stats` |
| GET | `/v1/admin/analytics/vibe/work/usage/by_user_stats` | `get_by_user_stats_v1_admin_analytics_vibe_work_usage_by_user_stats` | `beta.admin.vibe_work_analytics.user_stats` |
| POST | `/v1/admin/workspaces/{workspace_uuid}/add-users` | `users_api_admin_workspaces_add_users_workspaces` | `beta.admin.workspaces.add_users` |
| POST | `/v1/admin/workspaces` | `users_api_admin_workspaces_create_workspace` | `beta.admin.workspaces.create` |
| DELETE | `/v1/admin/workspaces/{workspace_uuid}` | `users_api_admin_workspaces_delete_workspaces` | `beta.admin.workspaces.delete` |
| GET | `/v1/admin/workspaces` | `users_api_admin_workspaces_get_workspaces` | `beta.admin.workspaces.list` |
| DELETE | `/v1/admin/workspaces/{workspace_uuid}/remove-users` | `users_api_admin_workspaces_remove_users_workspaces` | `beta.admin.workspaces.remove_users` |
| PATCH | `/v1/admin/workspaces/{workspace_uuid}` | `users_api_admin_workspaces_update_workspaces` | `beta.admin.workspaces.update` |
| PATCH | `/v1/admin/workspaces/{workspace_uuid}/users` | `users_api_admin_workspaces_add_or_update_users_workspaces` | `beta.admin.workspaces.upsert_users` |
| POST | `/v1/agents` | `agents_api_v1_agents_create` | `beta.agents.create` |
| PUT | `/v1/agents/{agent_id}/aliases` | `agents_api_v1_agents_create_or_update_alias` | `beta.agents.create_version_alias` |
| DELETE | `/v1/agents/{agent_id}` | `agents_api_v1_agents_delete` | `beta.agents.delete` |
| DELETE | `/v1/agents/{agent_id}/aliases` | `agents_api_v1_agents_delete_alias` | `beta.agents.delete_version_alias` |
| GET | `/v1/agents/{agent_id}` | `agents_api_v1_agents_get` | `beta.agents.get` |
| GET | `/v1/agents/{agent_id}/versions/{version}` | `agents_api_v1_agents_get_version` | `beta.agents.get_version` |
| GET | `/v1/agents` | `agents_api_v1_agents_list` | `beta.agents.list` |
| GET | `/v1/agents/pages` | `agents_api_v1_agents_list_pages` | `beta.agents.list_pages` |
| GET | `/v1/agents/{agent_id}/aliases` | `agents_api_v1_agents_list_version_aliases` | `beta.agents.list_version_aliases` |
| GET | `/v1/agents/{agent_id}/versions` | `agents_api_v1_agents_list_versions` | `beta.agents.list_versions` |
| PATCH | `/v1/agents/{agent_id}` | `agents_api_v1_agents_update` | `beta.agents.update` |
| PATCH | `/v1/agents/{agent_id}/version` | `agents_api_v1_agents_update_version` | `beta.agents.update_version` |
| GET | `/v1/billing/budget` | `billing_api_public_get_budget` | `beta.billing.get_budget` |
| GET | `/v1/billing/subscription` | `billing_api_public_get_subscription` | `beta.billing.get_subscription` |
| POST | `/v1/connectors/{connector_id}/{consumer_scope}/activate` | `connector_activate_for_consumer_v1` | `beta.connectors.activate_for_consumer` |
| POST | `/v1/connectors/{connector_id_or_name}/tools/{tool_name}/call` | `connector_call_tool_v1` | `beta.connectors.call_tool` |
| POST | `/v1/connectors` | `connector_create_v1` | `beta.connectors.create` |
| POST | `/v1/connectors/{connector_id_or_name}/{consumer_scope}/credentials` | `connector_create_credentials_v1` | `beta.connectors.create_credentials` |
| POST | `/v1/connectors/{connector_id}/{consumer_scope}/deactivate` | `connector_deactivate_for_consumer_v1` | `beta.connectors.deactivate_for_consumer` |
| DELETE | `/v1/connectors/{connector_id}#id` | `connector_delete_v1` | `beta.connectors.delete` |
| DELETE | `/v1/connectors/{connector_id_or_name}/user/credentials` | `connector_delete_all_user_credentials_v1` | `beta.connectors.delete_all_user_credentials` |
| DELETE | `/v1/connectors/{connector_id_or_name}/{consumer_scope}/credentials/{credentials_name}` | `connector_delete_credentials` | `beta.connectors.delete_credentials` |
| GET | `/v1/connectors/{connector_id_or_name}#idOrName` | `connector_get_v1` | `beta.connectors.get` |
| GET | `/v1/connectors/{connector_id_or_name}/auth_url` | `connector_get_auth_url_v1` | `beta.connectors.get_auth_url` |
| GET | `/v1/connectors/{connector_id_or_name}/authentication_methods` | `connector_get_authentication_methods_v1` | `beta.connectors.get_authentication_methods` |
| GET | `/v1/connectors` | `connector_list_v1` | `beta.connectors.list` |
| GET | `/v1/connectors/{connector_id_or_name}/organization/credentials` | `connector_list_organization_credentials_v1` | `beta.connectors.list_organization_credentials` |
| GET | `/v1/connectors/{connector_id_or_name}/tools` | `connector_list_tools_v1` | `beta.connectors.list_tools` |
| GET | `/v1/connectors/{connector_id_or_name}/user/credentials` | `connector_list_user_credentials_v1` | `beta.connectors.list_user_credentials` |
| GET | `/v1/connectors/{connector_id_or_name}/workspace/credentials` | `connector_list_workspace_credentials_v1` | `beta.connectors.list_workspace_credentials` |
| PUT | `/v1/connectors/{connector_id}/share` | `connector_share_v1` | `beta.connectors.share` |
| PUT | `/v1/connectors/{connector_id}/organization/share` | `connector_share_to_organization_v1` | `beta.connectors.share_to_organization` |
| DELETE | `/v1/connectors/{connector_id}/share` | `connector_unshare_v1` | `beta.connectors.unshare` |
| DELETE | `/v1/connectors/{connector_id}/organization/share` | `connector_unshare_from_organization_v1` | `beta.connectors.unshare_from_organization` |
| PATCH | `/v1/connectors/{connector_id}#id` | `connector_update_v1` | `beta.connectors.update` |
| PATCH | `/v1/connectors/{connector_id_or_name}/{consumer_scope}/credentials` | `connector_update_credentials` | `beta.connectors.update_credentials` |
| POST | `/v1/context/search-indexes` | `create_index_v1_context_search_indexes_post` | `beta.context.search_indexes.create` |
| DELETE | `/v1/context/search-indexes/{index_name}` | `delete_index_v1_context_search_indexes__index_name__delete` | `beta.context.search_indexes.delete` |
| DELETE | `/v1/context/search-indexes/{index_name}/documents` | `delete_documents_v1_context_search_indexes__index_name__documents_delete` | `beta.context.search_indexes.delete_documents` |
| POST | `/v1/context/search-indexes/{index_name}/summary_field/{language}` | `generate_managed_index_summary_post_v1_context_search_indexes__index_name__summary_field__language__post` | `beta.context.search_indexes.generate_index_summary` |
| GET | `/v1/context/search-indexes/{index_name}` | `get_index_v1_context_search_indexes__index_name__get` | `beta.context.search_indexes.get` |
| GET | `/v1/context/search-indexes/{index_name}/chunks/{chunk_id}` | `get_chunk_index_v1_context_search_indexes__index_name__chunks__chunk_id__get` | `beta.context.search_indexes.get_chunk` |
| GET | `/v1/context/search-indexes/{index_name}/summary_field/{language}` | `get_managed_index_summary_v1_context_search_indexes__index_name__summary_field__language__get` | `beta.context.search_indexes.get_index_summary` |
| POST | `/v1/context/search-indexes/{index_name}/grep` | `grep_index_v1_context_search_indexes__index_name__grep_post` | `beta.context.search_indexes.grep` |
| POST | `/v1/context/search-indexes/{index_name}/documents` | `ingest_documents_v1_context_search_indexes__index_name__documents_post` | `beta.context.search_indexes.ingest_documents` |
| GET | `/v1/context/search-indexes` | `list_indexes_v1_context_search_indexes_get` | `beta.context.search_indexes.list` |
| POST | `/v1/context/search-indexes/{index_name}/navigate` | `navigate_index_v1_context_search_indexes__index_name__navigate_post` | `beta.context.search_indexes.navigate` |
| POST | `/v1/context/search-indexes/{index_name}/read` | `read_index_v1_context_search_indexes__index_name__read_post` | `beta.context.search_indexes.read` |
| POST | `/v1/context/search-indexes/{index_name}/search` | `search_index_v1_context_search_indexes__index_name__search_post` | `beta.context.search_indexes.search` |
| PUT | `/v1/context/search-indexes/{index_name}/summary_field/{language}` | `set_managed_index_summary_v1_context_search_indexes__index_name__summary_field__language__put` | `beta.context.search_indexes.set_index_summary` |
| PUT | `/v1/context/search-indexes/{index_name}` | `update_index_v1_context_search_indexes__index_name__put` | `beta.context.search_indexes.update` |
| POST | `/v1/conversations/{conversation_id}` | `agents_api_v1_conversations_append` | `beta.conversations.append` |
| POST | `/v1/conversations/{conversation_id}#stream` | `agents_api_v1_conversations_append_stream` | `beta.conversations.append_stream` |
| DELETE | `/v1/conversations/{conversation_id}` | `agents_api_v1_conversations_delete` | `beta.conversations.delete` |
| GET | `/v1/conversations/{conversation_id}` | `agents_api_v1_conversations_get` | `beta.conversations.get` |
| GET | `/v1/conversations/{conversation_id}/history` | `agents_api_v1_conversations_history` | `beta.conversations.get_history` |
| GET | `/v1/conversations/{conversation_id}/messages` | `agents_api_v1_conversations_messages` | `beta.conversations.get_messages` |
| GET | `/v1/conversations` | `agents_api_v1_conversations_list` | `beta.conversations.list` |
| POST | `/v1/conversations/{conversation_id}/restart` | `agents_api_v1_conversations_restart` | `beta.conversations.restart` |
| POST | `/v1/conversations/{conversation_id}/restart#stream` | `agents_api_v1_conversations_restart_stream` | `beta.conversations.restart_stream` |
| POST | `/v1/conversations` | `agents_api_v1_conversations_start` | `beta.conversations.start` |
| POST | `/v1/conversations#stream` | `agents_api_v1_conversations_start_stream` | `beta.conversations.start_stream` |
| DELETE | `/v1/libraries/{library_id}/share` | `libraries_share_delete_v1` | `beta.libraries.accesses.delete` |
| GET | `/v1/libraries/{library_id}/share` | `libraries_share_list_v1` | `beta.libraries.accesses.list` |
| PUT | `/v1/libraries/{library_id}/share` | `libraries_share_create_v1` | `beta.libraries.accesses.update_or_create` |
| POST | `/v1/libraries` | `libraries_create_v1` | `beta.libraries.create` |
| DELETE | `/v1/libraries/{library_id}` | `libraries_delete_v1` | `beta.libraries.delete` |
| DELETE | `/v1/libraries/{library_id}/documents/{document_id}` | `libraries_documents_delete_v1` | `beta.libraries.documents.delete` |
| GET | `/v1/libraries/{library_id}/documents/{document_id}/extracted-text-signed-url` | `libraries_documents_get_extracted_text_signed_url_v1` | `beta.libraries.documents.extracted_text_signed_url` |
| GET | `/v1/libraries/{library_id}/documents/{document_id}` | `libraries_documents_get_v1` | `beta.libraries.documents.get` |
| GET | `/v1/libraries/{library_id}/documents/{document_id}/signed-url` | `libraries_documents_get_signed_url_v1` | `beta.libraries.documents.get_signed_url` |
| GET | `/v1/libraries/{library_id}/documents` | `libraries_documents_list_v1` | `beta.libraries.documents.list` |
| PUT | `/v1/libraries/{library_id}/documents/{document_id}` | `libraries_documents_update_v1` | `beta.libraries.documents.rename` |
| POST | `/v1/libraries/{library_id}/documents/{document_id}/reprocess` | `libraries_documents_reprocess_v1` | `beta.libraries.documents.reprocess` |
| GET | `/v1/libraries/{library_id}/documents/{document_id}/status` | `libraries_documents_get_status_v1` | `beta.libraries.documents.status` |
| GET | `/v1/libraries/{library_id}/documents/{document_id}/text_content` | `libraries_documents_get_text_content_v1` | `beta.libraries.documents.text_content` |
| PATCH | `/v1/libraries/{library_id}/documents/{document_id}` | `libraries_documents_patch_v1` | `beta.libraries.documents.update` |
| POST | `/v1/libraries/{library_id}/documents` | `libraries_documents_upload_v1` | `beta.libraries.documents.upload` |
| GET | `/v1/libraries/{library_id}` | `libraries_get_v1` | `beta.libraries.get` |
| PUT | `/v1/libraries/{library_id}` | `libraries_update_v1` | `beta.libraries.libraries_update_v1` |
| GET | `/v1/libraries` | `libraries_list_v1` | `beta.libraries.list` |
| PATCH | `/v1/libraries/{library_id}` | `libraries_patch_v1` | `beta.libraries.update` |
| POST | `/v1/observability/datasets` | `create_dataset_v1_observability_datasets_post` | `beta.observability.datasets.create` |
| POST | `/v1/observability/datasets/{dataset_id}/records` | `create_dataset_record_v1_observability_datasets__dataset_id__records_post` | `beta.observability.datasets.create_record` |
| DELETE | `/v1/observability/datasets/{dataset_id}` | `delete_dataset_v1_observability_datasets__dataset_id__delete` | `beta.observability.datasets.delete` |
| GET | `/v1/observability/datasets/{dataset_id}/exports/to-jsonl` | `export_dataset_to_jsonl_v1_observability_datasets__dataset_id__exports_to_jsonl_get` | `beta.observability.datasets.export_to_jsonl` |
| GET | `/v1/observability/datasets/{dataset_id}` | `get_dataset_by_id_v1_observability_datasets__dataset_id__get` | `beta.observability.datasets.fetch` |
| GET | `/v1/observability/datasets/{dataset_id}/tasks/{task_id}` | `get_dataset_import_task_v1_observability_datasets__dataset_id__tasks__task_id__get` | `beta.observability.datasets.fetch_task` |
| POST | `/v1/observability/datasets/{dataset_id}/imports/from-dataset` | `post_dataset_records_from_dataset_v1_observability_datasets__dataset_id__imports_from_dataset_post` | `beta.observability.datasets.import_from_dataset_records` |
| POST | `/v1/observability/datasets/{dataset_id}/imports/from-file` | `post_dataset_records_from_file_v1_observability_datasets__dataset_id__imports_from_file_post` | `beta.observability.datasets.import_from_file` |
| POST | `/v1/observability/datasets/{dataset_id}/imports/from-playground` | `post_dataset_records_from_playground_v1_observability_datasets__dataset_id__imports_from_playground_post` | `beta.observability.datasets.import_from_playground` |
| POST | `/v1/observability/datasets/{dataset_id}/imports/from-spans` | `post_dataset_records_from_spans_v1_observability_datasets__dataset_id__imports_from_spans_post` | `beta.observability.datasets.import_from_spans` |
| GET | `/v1/observability/datasets` | `get_datasets_v1_observability_datasets_get` | `beta.observability.datasets.list` |
| GET | `/v1/observability/datasets/{dataset_id}/records` | `get_dataset_records_v1_observability_datasets__dataset_id__records_get` | `beta.observability.datasets.list_records` |
| GET | `/v1/observability/datasets/{dataset_id}/tasks` | `get_dataset_import_tasks_v1_observability_datasets__dataset_id__tasks_get` | `beta.observability.datasets.list_tasks` |
| POST | `/v1/observability/dataset-records/bulk-delete` | `delete_dataset_records_v1_observability_dataset_records_bulk_delete_post` | `beta.observability.datasets.records.bulk_delete` |
| DELETE | `/v1/observability/dataset-records/{dataset_record_id}` | `delete_dataset_record_v1_observability_dataset_records__dataset_record_id__delete` | `beta.observability.datasets.records.delete` |
| GET | `/v1/observability/dataset-records/{dataset_record_id}` | `get_dataset_record_v1_observability_dataset_records__dataset_record_id__get` | `beta.observability.datasets.records.fetch` |
| PUT | `/v1/observability/dataset-records/{dataset_record_id}/payload` | `update_dataset_record_payload_v1_observability_dataset_records__dataset_record_id__payload_put` | `beta.observability.datasets.records.update_payload` |
| PUT | `/v1/observability/dataset-records/{dataset_record_id}/properties` | `update_dataset_record_properties_v1_observability_dataset_records__dataset_record_id__properties_put` | `beta.observability.datasets.records.update_properties` |
| PATCH | `/v1/observability/datasets/{dataset_id}` | `update_dataset_v1_observability_datasets__dataset_id__patch` | `beta.observability.datasets.update` |
| POST | `/v1/observability/pipelines` | `create_pipeline_v1_observability_pipelines_post` | `beta.observability.evaluations.create_pipeline` |
| DELETE | `/v1/observability/pipelines/{pipeline_id}` | `delete_pipeline_v1_observability_pipelines__pipeline_id__delete` | `beta.observability.evaluations.delete_pipeline` |
| GET | `/v1/observability/evaluations_v2/{evaluation_slug}` | `get_evaluation_v2_v1_observability_evaluations_v2__evaluation_slug__get` | `beta.observability.evaluations.get` |
| GET | `/v1/observability/pipelines/{pipeline_id}` | `get_pipeline_v1_observability_pipelines__pipeline_id__get` | `beta.observability.evaluations.get_pipeline` |
| GET | `/v1/observability/projects/{project_slug}/evaluations_v2` | `get_project_evaluations_v2_v1_observability_projects__project_slug__evaluations_v2_get` | `beta.observability.evaluations.list` |
| GET | `/v1/observability/pipelines` | `list_pipelines_v1_observability_pipelines_get` | `beta.observability.evaluations.list_pipelines` |
| GET | `/v1/observability/optimizations/{optimization_id}` | `get_optimization_v1_observability_optimizations__optimization_id__get` | `beta.observability.evaluations.optimizations.get` |
| GET | `/v1/observability/optimizations` | `list_optimizations_v1_observability_optimizations_get` | `beta.observability.evaluations.optimizations.list` |
| GET | `/v1/observability/optimizations/{optimization_id}/trials` | `list_optimization_trials_v1_observability_optimizations__optimization_id__trials_get` | `beta.observability.evaluations.optimizations.list_trials` |
| GET | `/v1/observability/projects/{project_slug}` | `get_project_v1_observability_projects__project_slug__get` | `beta.observability.evaluations.projects.get` |
| GET | `/v1/observability/projects` | `get_projects_v1_observability_projects_get` | `beta.observability.evaluations.projects.list` |
| GET | `/v1/observability/evaluation-runs_v2/{run_id}/records/{record_id}` | `get_evaluation_run_record_v2_v1_observability_evaluation_runs_v2__run_id__records__record_id__get` | `beta.observability.evaluations.records.get` |
| POST | `/v1/observability/evaluation-runs_v2/{run_id}/records/search` | `search_evaluation_run_records_v2_v1_observability_evaluation_runs_v2__run_id__records_search_post` | `beta.observability.evaluations.records.search` |
| POST | `/v1/observability/evaluation-runs_v2/{run_id}/statistics/compute` | `compute_filtered_run_statistics_v2_v1_observability_evaluation_runs_v2__run_id__statistics_compute_post` | `beta.observability.evaluations.runs.compute_statistics` |
| GET | `/v1/observability/evaluation-runs_v2/{run_id}` | `get_evaluation_run_v2_v1_observability_evaluation_runs_v2__run_id__get` | `beta.observability.evaluations.runs.get` |
| GET | `/v1/observability/evaluation-runs_v2/{run_id}/statistics` | `get_single_evaluation_run_statistics_v2_v1_observability_evaluation_runs_v2__run_id__statistics_get` | `beta.observability.evaluations.runs.get_statistics` |
| POST | `/v1/observability/evaluations_v2/{evaluation_slug}/runs/search` | `search_evaluation_runs_v2_v1_observability_evaluations_v2__evaluation_slug__runs_search_post` | `beta.observability.evaluations.runs.search` |
| POST | `/v1/observability/evaluation-runs_v2/search` | `search_all_evaluation_runs_v2_v1_observability_evaluation_runs_v2_search_post` | `beta.observability.evaluations.runs.search_all` |
| POST | `/v1/observability/scorers` | `create_scorer_v1_observability_scorers_post` | `beta.observability.evaluations.scorers.create` |
| GET | `/v1/observability/scorers/{scorer_slug}` | `get_scorer_v1_observability_scorers__scorer_slug__get` | `beta.observability.evaluations.scorers.get` |
| GET | `/v1/observability/scorers` | `list_scorers_v1_observability_scorers_get` | `beta.observability.evaluations.scorers.list` |
| PATCH | `/v1/observability/scorers/{scorer_slug}` | `edit_scorer_v1_observability_scorers__scorer_slug__patch` | `beta.observability.evaluations.scorers.update` |
| PUT | `/v1/observability/pipelines/{pipeline_id}` | `update_pipeline_v1_observability_pipelines__pipeline_id__put` | `beta.observability.evaluations.update_pipeline` |
| GET | `/v1/observability/logs/fields/{field_name}/options` | `get_log_field_options_v1_observability_logs_fields__field_name__options_get` | `beta.observability.logs.fetch_options` |
| GET | `/v1/observability/logs/fields` | `get_log_fields_v1_observability_logs_fields_get` | `beta.observability.logs.list` |
| POST | `/v1/observability/logs/search` | `search_logs_v1_observability_logs_search_post` | `beta.observability.logs.search` |
| POST | `/v1/observability/spans/aggregate` | `aggregate_spans_v1_observability_spans_aggregate_post` | `beta.observability.spans.aggregate` |
| POST | `/v1/observability/spans/evaluations/aggregate` | `aggregate_span_evaluations_v1_observability_spans_evaluations_aggregate_post` | `beta.observability.spans.aggregate_span_evaluations` |
| GET | `/v1/observability/spans/evaluations/fields/{field_name}/options` | `get_span_evaluation_field_options_v1_observability_spans_evaluations_fields__field_name__options_get` | `beta.observability.spans.fetch_span_eval_field_options` |
| GET | `/v1/observability/spans/fields/{field_name}/options` | `get_span_field_options_v1_observability_spans_fields__field_name__options_get` | `beta.observability.spans.fetch_span_field_options` |
| GET | `/v1/observability/pipelines/{pipeline_id}/results/fields` | `get_pipeline_result_fields_v1_observability_pipelines__pipeline_id__results_fields_get` | `beta.observability.spans.list_pipeline_result_fields` |
| GET | `/v1/observability/spans/evaluations/fields` | `get_span_evaluation_fields_v1_observability_spans_evaluations_fields_get` | `beta.observability.spans.list_span_eval_fields` |
| GET | `/v1/observability/spans/fields` | `get_span_fields_v1_observability_spans_fields_get` | `beta.observability.spans.list_span_fields` |
| POST | `/v1/observability/spans/evaluations/search/latest` | `search_latest_span_evaluations_v1_observability_spans_evaluations_search_latest_post` | `beta.observability.spans.search_latest_span_evaluations` |
| POST | `/v1/observability/pipeline-results/search` | `search_pipeline_results_v1_observability_pipeline_results_search_post` | `beta.observability.spans.search_pipeline_results` |
| POST | `/v1/observability/spans/evaluations/search` | `search_span_evaluations_v1_observability_spans_evaluations_search_post` | `beta.observability.spans.search_span_evaluations` |
| POST | `/v1/observability/spans/search` | `search_spans_v1_observability_spans_search_post` | `beta.observability.spans.search_spans` |
| POST | `/v1/observability/traces/aggregate` | `aggregate_traces_v1_observability_traces_aggregate_post` | `beta.observability.traces.aggregate` |
| GET | `/v1/observability/traces/fields/{field_name}/options` | `get_trace_field_options_v1_observability_traces_fields__field_name__options_get` | `beta.observability.traces.fetch_options` |
| GET | `/v1/observability/traces/{trace_id}/spans/{span_id}` | `get_span_by_id_v1_observability_traces__trace_id__spans__span_id__get` | `beta.observability.traces.get_span_by_id` |
| GET | `/v1/observability/traces/{trace_id}` | `get_trace_by_id_v1_observability_traces__trace_id__get` | `beta.observability.traces.get_trace_by_id` |
| GET | `/v1/observability/traces/fields` | `get_trace_fields_v1_observability_traces_fields_get` | `beta.observability.traces.get_trace_fields` |
| GET | `/v1/observability/traces/{trace_id}/spans` | `get_trace_spans_v1_observability_traces__trace_id__spans_get` | `beta.observability.traces.get_trace_spans` |
| POST | `/v1/observability/traces/search` | `search_traces_v1_observability_traces_search_post` | `beta.observability.traces.search` |
| POST | `/v2/prompts` | `prompts_create` | `beta.prompts.create` |
| POST | `/v2/prompts/{prompt_id}/versions` | `prompts_create_version` | `beta.prompts.create_version` |
| DELETE | `/v2/prompts/{prompt_id}` | `prompts_delete` | `beta.prompts.delete` |
| GET | `/v2/prompts/{prompt_id}` | `prompts_get` | `beta.prompts.get` |
| GET | `/v2/prompts/{prompt_id}/versions/{version}` | `prompts_get_version` | `beta.prompts.get_version` |
| GET | `/v2/prompts` | `prompts_list` | `beta.prompts.list` |
| GET | `/v2/prompts/{prompt_id}/versions` | `prompts_list_versions` | `beta.prompts.list_versions` |
| PATCH | `/v2/prompts/{prompt_id}` | `prompts_update` | `beta.prompts.update_metadata` |
| PATCH | `/v2/prompts/{prompt_id}/versions/{version}` | `prompts_update_version_metadata` | `beta.prompts.update_version_metadata` |
| GET | `/v1/rag/ingestion_pipeline_configurations` | `get_configs_v1_rag_ingestion_pipeline_configurations_get` | `beta.rag.ingestion_pipeline_configurations.list` |
| PUT | `/v1/rag/ingestion_pipeline_configurations` | `register_config_v1_rag_ingestion_pipeline_configurations_put` | `beta.rag.ingestion_pipeline_configurations.register` |
| PUT | `/v1/rag/ingestion_pipeline_configurations/{id}/run_info` | `update_run_info_v1_rag_ingestion_pipeline_configurations__id__run_info_put` | `beta.rag.ingestion_pipeline_configurations.update_run_info` |
| GET | `/v1/rag/deployments` | `get_deployment_summaries_v1_rag_deployments_get` | `beta.rag.search_indexes.get_deployment_summaries` |
| PUT | `/v1/rag/deployments` | `register_deployment_v1_rag_deployments_put` | `beta.rag.search_indexes.register_deployment` |
| DELETE | `/v1/rag/deployments/{deployment_id}` | `unregister_deployment_v1_rag_deployments__deployment_id__delete` | `beta.rag.search_indexes.unregister_deployment` |
| PUT | `/v1/rag/deployments/{deployment_id}/metrics` | `update_index_metrics_v1_rag_deployments__deployment_id__metrics_put` | `beta.rag.search_indexes.update_index_metrics` |
| POST | `/v1/service-accounts` | `create_service_account_v1_service_accounts_post` | `beta.service_accounts.create` |
| DELETE | `/v1/service-accounts/{service_account_id}` | `delete_service_account_v1_service_accounts__service_account_id__delete` | `beta.service_accounts.delete` |
| GET | `/v1/service-accounts/{service_account_id}` | `get_service_account_v1_service_accounts__service_account_id__get` | `beta.service_accounts.get` |
| GET | `/v1/service-accounts` | `list_service_accounts_v1_service_accounts_get` | `beta.service_accounts.list` |
| GET | `/v1/service-accounts/assignable-roles` | `list_assignable_service_account_roles_v1_service_accounts_assignable_roles_get` | `beta.service_accounts.list_assignable_roles` |
| GET | `/v1/service-accounts/{service_account_id}/roles` | `list_service_account_roles_v1_service_accounts__service_account_id__roles_get` | `beta.service_accounts.list_roles` |
| PUT | `/v1/service-accounts/{service_account_id}/roles` | `set_service_account_roles_v1_service_accounts__service_account_id__roles_put` | `beta.service_accounts.set_roles` |
| PATCH | `/v1/service-accounts/{service_account_id}` | `update_service_account_v1_service_accounts__service_account_id__patch` | `beta.service_accounts.update` |
| POST | `/v2/skills` | `skills_create` | `beta.skills.create` |
| POST | `/v2/skills/{skill_id}/versions` | `skills_create_version` | `beta.skills.create_version` |
| DELETE | `/v2/skills/{skill_id}` | `skills_delete` | `beta.skills.delete` |
| GET | `/v2/skills/{skill_id}` | `skills_get` | `beta.skills.get` |
| GET | `/v2/skills/{skill_id}/versions/{version}` | `skills_get_version` | `beta.skills.get_version` |
| GET | `/v2/skills` | `skills_list` | `beta.skills.list` |
| GET | `/v2/skills/{skill_id}/versions` | `skills_list_versions` | `beta.skills.list_versions` |
| PATCH | `/v2/skills/{skill_id}` | `skills_update` | `beta.skills.update_metadata` |
| PATCH | `/v2/skills/{skill_id}/versions/{version}` | `skills_update_version_metadata` | `beta.skills.update_version_metadata` |
| POST | `/v1/triggers` | `create_trigger_v1_triggers_post` | `beta.triggers.create` |
| DELETE | `/v1/triggers/{trigger_id}` | `delete_trigger_v1_triggers__trigger_id__delete` | `beta.triggers.delete` |
| GET | `/v1/triggers/{trigger_id}` | `get_trigger_v1_triggers__trigger_id__get` | `beta.triggers.get` |
| GET | `/v1/triggers` | `list_triggers_v1_triggers_get` | `beta.triggers.list` |
| GET | `/v1/triggers/{trigger_id}/events` | `list_trigger_events_v1_triggers__trigger_id__events_get` | `beta.triggers.list_events` |
| GET | `/v1/users/me` | `users_api_get_identity` | `beta.users.get_identity` |
| GET | `/v1/users/me/organizations` | `users_api_list_organizations` | `beta.users.list_organizations` |
| GET | `/v1/users/me/workspaces` | `users_api_list_workspaces` | `beta.users.list_workspaces` |
| GET | `/v1/whoami` | `users_api_whoami` | `beta.users.whoami` |
| POST | `/v1/webhooks` | `create_webhook_v1_webhooks_post` | `beta.webhooks.create` |
| DELETE | `/v1/webhooks/{webhook_id}` | `delete_webhook_v1_webhooks__webhook_id__delete` | `beta.webhooks.delete` |
| GET | `/v1/webhooks/{webhook_id}` | `get_webhook_v1_webhooks__webhook_id__get` | `beta.webhooks.get` |
| GET | `/v1/webhooks/{webhook_id}/signing-secret` | `get_webhook_signing_secret_v1_webhooks__webhook_id__signing_secret_get` | `beta.webhooks.get_signing_secret` |
| GET | `/v1/webhooks` | `list_webhooks_v1_webhooks_get` | `beta.webhooks.list` |
| POST | `/v1/chat/completions` | `chat_completion_v1_chat_completions_post` | `chat.complete`, `chat.complete_stream` |
| POST | `/v1/classifications` | `classifications_v1_classifications_post` | `classifiers.classify` |
| POST | `/v1/chat/classifications` | `chat_classifications_v1_chat_classifications_post` | `classifiers.classify_chat` |
| POST | `/v1/moderations` | `moderations_v1_moderations_post` | `classifiers.moderate` |
| POST | `/v1/chat/moderations` | `chat_moderations_v1_chat_moderations_post` | `classifiers.moderate_chat` |
| POST | `/v1/embeddings` | `embeddings_v1_embeddings_post` | `embeddings.create` |
| GET | `/v1/workflows/events/stream` | `get_stream_events_v1_workflows_events_stream_get` | `events.get_stream_events` |
| GET | `/v1/workflows/events/list` | `get_workflow_events_v1_workflows_events_list_get` | `events.get_workflow_events` |
| DELETE | `/v1/files/{file_id}` | `files_api_routes_delete_file` | `files.delete` |
| GET | `/v1/files/{file_id}/content` | `files_api_routes_download_file` | `files.download` |
| GET | `/v1/files/{file_id}/url` | `files_api_routes_get_signed_url` | `files.get_signed_url` |
| GET | `/v1/files` | `files_api_routes_list_files` | `files.list` |
| GET | `/v1/files/{file_id}` | `files_api_routes_retrieve_file` | `files.retrieve` |
| POST | `/v1/files` | `files_api_routes_upload_file` | `files.upload` |
| POST | `/v1/fim/completions` | `fim_completion_v1_fim_completions_post` | `fim.complete`, `fim.complete_stream` |
| POST | `/v1/fine_tuning/models/{model_id}/archive` | `jobs_api_routes_fine_tuning_archive_fine_tuned_model` | `models.archive` |
| DELETE | `/v1/models/{model_id}` | `delete_model_v1_models__model_id__delete` | `models.delete` |
| GET | `/v1/models` | `list_models_v1_models_get` | `models.list` |
| GET | `/v1/models/{model_id}` | `retrieve_model_v1_models__model_id__get` | `models.retrieve` |
| DELETE | `/v1/fine_tuning/models/{model_id}/archive` | `jobs_api_routes_fine_tuning_unarchive_fine_tuned_model` | `models.unarchive` |
| PATCH | `/v1/fine_tuning/models/{model_id}` | `jobs_api_routes_fine_tuning_update_fine_tuned_model` | `models.update` |
| POST | `/v1/ocr` | `ocr_v1_ocr_post` | `ocr.process` |
| POST | `/v1/client/sessions` | `create_client_session_v1_client_sessions_post` | `realtime.sessions.create` |
| POST | `/v1/responses/input_tokens` | `count_input_tokens_v1_responses_input_tokens_post` | `responses.count_input_tokens` |
| POST | `/v1/responses` | `create_response_v1_responses_post` | `responses.create` |
| POST | `/v1/workflows/{workflow_identifier}/tags` | `add_workflow_tags_v1_workflows__workflow_identifier__tags_post` | `workflows.add_workflow_tags` |
| PUT | `/v1/workflows/{workflow_identifier}/archive` | `archive_workflow_v1_workflows__workflow_identifier__archive_put` | `workflows.archive_workflow` |
| PUT | `/v1/workflows/archive` | `bulk_archive_workflows_v1_workflows_archive_put` | `workflows.bulk_archive_workflows` |
| PUT | `/v1/workflows/unarchive` | `bulk_unarchive_workflows_v1_workflows_unarchive_put` | `workflows.bulk_unarchive_workflows` |
| POST | `/v1/workflows/deployments` | `create_deployment_v1_workflows_deployments_post` | `workflows.deployments.create_deployment` |
| DELETE | `/v1/workflows/deployments/{name}` | `delete_deployment_v1_workflows_deployments__name__delete` | `workflows.deployments.delete_deployment` |
| GET | `/v1/workflows/deployments/{name}` | `get_deployment_v1_workflows_deployments__name__get` | `workflows.deployments.get_deployment` |
| GET | `/v1/workflows/deployments/{name}/archive` | `get_deployment_archive_v1_workflows_deployments__name__archive_get` | `workflows.deployments.get_deployment_archive` |
| GET | `/v1/workflows/deployments/{name}/logs` | `get_deployment_logs` | `workflows.deployments.get_deployment_logs` |
| GET | `/v1/workflows/deployments/{name}/workers` | `list_deployment_workers_v1_workflows_deployments__name__workers_get` | `workflows.deployments.list_deployment_workers` |
| GET | `/v1/workflows/deployments` | `list_deployments_v1_workflows_deployments_get` | `workflows.deployments.list_deployments` |
| POST | `/v1/workflows/deployments/{name}/restart` | `restart_deployment_v1_workflows_deployments__name__restart_post` | `workflows.deployments.restart_deployment` |
| POST | `/v1/workflows/deployments/{name}/start` | `start_deployment_v1_workflows_deployments__name__start_post` | `workflows.deployments.start_deployment` |
| POST | `/v1/workflows/deployments/{name}/stop` | `stop_deployment_v1_workflows_deployments__name__stop_post` | `workflows.deployments.stop_deployment` |
| GET | `/v1/workflows/deployments/{name}/logs/stream` | `stream_deployment_logs` | `workflows.deployments.stream_deployment_logs` |
| POST | `/v1/workflows/deployments/{deployment_id}/unharden` | `unharden_deployment_v1_workflows_deployments__deployment_id__unharden_post` | `workflows.deployments.unharden_deployment` |
| PATCH | `/v1/workflows/deployments/{name}` | `update_deployment_v1_workflows_deployments__name__patch` | `workflows.deployments.update_deployment` |
| POST | `/v1/workflows/{workflow_identifier}/execute` | `execute_workflow_v1_workflows__workflow_identifier__execute_post` | `workflows.execute_workflow` |
| POST | `/v1/workflows/registrations/{workflow_registration_id}/execute` | `execute_workflow_registration_v1_workflows_registrations__workflow_registration_id__execute_post` | `workflows.execute_workflow_registration` |
| POST | `/v1/workflows/executions/cancel` | `batch_cancel_workflow_executions_v1_workflows_executions_cancel_post` | `workflows.executions.batch_cancel_workflow_executions` |
| POST | `/v1/workflows/executions/terminate` | `batch_terminate_workflow_executions_v1_workflows_executions_terminate_post` | `workflows.executions.batch_terminate_workflow_executions` |
| POST | `/v1/workflows/executions/{execution_id}/cancel` | `cancel_workflow_execution_v1_workflows_executions__execution_id__cancel_post` | `workflows.executions.cancel_workflow_execution` |
| GET | `/v1/workflows/executions/{execution_id}` | `get_workflow_execution_v1_workflows_executions__execution_id__get` | `workflows.executions.get_workflow_execution` |
| GET | `/v1/workflows/executions/{execution_id}/history` | `get_workflow_execution_history_v1_workflows_executions__execution_id__history_get` | `workflows.executions.get_workflow_execution_history` |
| GET | `/v1/workflows/executions/{execution_id}/logs` | `get_workflow_execution_logs` | `workflows.executions.get_workflow_execution_logs` |
| GET | `/v1/workflows/executions/{execution_id}/trace/events` | `get_workflow_execution_trace_events` | `workflows.executions.get_workflow_execution_trace_events` |
| GET | `/v1/workflows/executions/{execution_id}/trace/info` | `get_workflow_execution_trace_info` | `workflows.executions.get_workflow_execution_trace_info` |
| GET | `/v1/workflows/executions/{execution_id}/trace/otel` | `get_workflow_execution_trace_otel` | `workflows.executions.get_workflow_execution_trace_otel` |
| GET | `/v1/workflows/executions/{execution_id}/trace/summary` | `get_workflow_execution_trace_summary` | `workflows.executions.get_workflow_execution_trace_summary` |
| POST | `/v1/workflows/executions/{execution_id}/queries` | `query_workflow_execution_v1_workflows_executions__execution_id__queries_post` | `workflows.executions.query_workflow_execution` |
| POST | `/v1/workflows/executions/{execution_id}/reset` | `reset_workflow_v1_workflows_executions__execution_id__reset_post` | `workflows.executions.reset_workflow` |
| POST | `/v1/workflows/executions/{execution_id}/signals` | `signal_workflow_execution_v1_workflows_executions__execution_id__signals_post` | `workflows.executions.signal_workflow_execution` |
| GET | `/v1/workflows/executions/{execution_id}/stream` | `stream_v1_workflows_executions__execution_id__stream_get` | `workflows.executions.stream` |
| GET | `/v1/workflows/executions/{execution_id}/logs/stream` | `stream_workflow_execution_logs` | `workflows.executions.stream_workflow_execution_logs` |
| POST | `/v1/workflows/executions/{execution_id}/terminate` | `terminate_workflow_execution_v1_workflows_executions__execution_id__terminate_post` | `workflows.executions.terminate_workflow_execution` |
| POST | `/v1/workflows/executions/{execution_id}/updates` | `update_workflow_execution_v1_workflows_executions__execution_id__updates_post` | `workflows.executions.update_workflow_execution` |
| GET | `/v1/workflows/{workflow_identifier}` | `get_workflow_v1_workflows__workflow_identifier__get` | `workflows.get_workflow` |
| GET | `/v1/workflows/registrations/{workflow_registration_id}` | `get_workflow_registration_v1_workflows_registrations__workflow_registration_id__get` | `workflows.get_workflow_registration` |
| GET | `/v1/workflows/registrations` | `get_workflow_registrations_v1_workflows_registrations_get` | `workflows.get_workflow_registrations` |
| GET | `/v1/workflows` | `get_workflows_v1_workflows_get` | `workflows.get_workflows` |
| GET | `/v1/workflows/{workflow_name}/metrics` | `get_workflow_metrics_v1_workflows__workflow_name__metrics_get` | `workflows.metrics.get_workflow_metrics` |
| GET | `/v1/workflows/runs/{run_id}` | `get_run_v1_workflows_runs__run_id__get` | `workflows.runs.get_run` |
| GET | `/v1/workflows/runs/{run_id}/history` | `get_run_history_v1_workflows_runs__run_id__history_get` | `workflows.runs.get_run_history` |
| GET | `/v1/workflows/runs` | `list_runs_v1_workflows_runs_get` | `workflows.runs.list_runs` |
| GET | `/v1/workflows/schedules/{schedule_id}` | `get_schedule_v1_workflows_schedules__schedule_id__get` | `workflows.schedules.get_schedule` |
| GET | `/v1/workflows/schedules` | `get_schedules_v1_workflows_schedules_get` | `workflows.schedules.get_schedules` |
| POST | `/v1/workflows/schedules/{schedule_id}/pause` | `pause_schedule_v1_workflows_schedules__schedule_id__pause_post` | `workflows.schedules.pause_schedule` |
| POST | `/v1/workflows/schedules/{schedule_id}/resume` | `resume_schedule_v1_workflows_schedules__schedule_id__resume_post` | `workflows.schedules.resume_schedule` |
| POST | `/v1/workflows/schedules` | `schedule_workflow_v1_workflows_schedules_post` | `workflows.schedules.schedule_workflow` |
| POST | `/v1/workflows/schedules/{schedule_id}/trigger` | `trigger_schedule_v1_workflows_schedules__schedule_id__trigger_post` | `workflows.schedules.trigger_schedule` |
| DELETE | `/v1/workflows/schedules/{schedule_id}` | `unschedule_workflow_v1_workflows_schedules__schedule_id__delete` | `workflows.schedules.unschedule_workflow` |
| PATCH | `/v1/workflows/schedules/{schedule_id}` | `update_schedule_v1_workflows_schedules__schedule_id__patch` | `workflows.schedules.update_schedule` |
| PUT | `/v1/workflows/{workflow_identifier}/unarchive` | `unarchive_workflow_v1_workflows__workflow_identifier__unarchive_put` | `workflows.unarchive_workflow` |
| PUT | `/v1/workflows/{workflow_identifier}` | `update_workflow_v1_workflows__workflow_identifier__put` | `workflows.update_workflow` |

## Approved naming decisions

All 16 corrections are encoded in `policy.yaml` and `inventory.json`, and mirrored in `fern/sdk-method-naming.overlay.yml` using native OpenAPI Overlay actions and `x-fern-sdk-method-name`. The production Fern 0.48 pipeline and the Fern v1 diagnostic both consume the same reviewed naming policy. No operationId, path, HTTP verb, request schema, or response schema is changed.

The naming review checked the public resource grouping and semantic role of each operation. Shortened names are scoped by the generated resource module; therefore `get_logs` and `stream_logs` can coexist in the separate deployment and execution modules. `list_trace_events` is intentionally distinct from `get_trace_info`, `get_trace_otel`, and `get_trace_summary`.

The Fern v1 Rust generator flattens nested resource groups and can therefore append numeric suffixes to otherwise identical leaf-resource methods (for example, `audio.agents` versus `beta.agents`). Production Fern 0.48 preserves the nested group modules (`audio/agents`, `beta/agents`, `realtime/sessions`, `beta/users`), so these are v1-specific naming collisions. We deliberately keep the concise production names rather than add redundant domain prefixes to work around a diagnostic generator's flattening behavior.

## Generation validation

### Fern v1 diagnostic

Run 45142402366 (commit `5011520`) completed successfully, including the `generation-v1` job and final gate. Its artifact demonstrated native OpenAPI Overlay renaming against the no-overlay baseline:

- Fern resolves 347 published operations into 349 public method paths; the generated Rust SDK contains 347 public async endpoint methods. The two additional paths are the streaming/non-streaming variants of chat and FIM completions.
- All 16 overlay overrides appear in the generated resource modules. Fern v1 flattens nested resource groups and therefore still adds numeric suffixes to some cross-group collisions; this is specific to the v1 Rust generator.
- Ten query-bearing operations have their synthetic wrapper names updated consistently (for example, `GetWorkflowEventsQueryRequest` → `EventsListQueryRequest` and `GetWorkflowExecutionTraceEventsQueryRequest` → `ListTraceEventsQueryRequest`). The remaining six overrides do not have synthetic query wrappers.
- The corrected type inventory contains 93 public `*QueryRequest` types. Two v1-only wrapper names (`AgentsGetQueryRequest2` and `AgentsListQueryRequest2`) retain numeric suffixes because the v1 generator flattens the audio and beta agent groups; production Fern 0.48 keeps those groups separate.
- The generated public function inventories contain 9,796 declarations in each parameter style. Exactly 165 signature lines change from the baseline and 165 are added, corresponding to the 16 endpoint renames and Fern's derived request/response naming changes. Wrapped-vs-inline function and query-type diffs are both empty.

Both Fern v1 generation modes succeed. Cargo checks are diagnostic and report 52 crate errors in each mode (missing generated types, unsupported `Display` implementations, and a missing `Default` implementation). This does not establish that the v1 generated crate compiles, so v1 is not the production generator.

### Production Fern 0.48

The production `generation` job in run 45155086336 passed. It ran `generate.sh` to materialize the SDK from the reviewed policy, then `check.sh` to regenerate and verify the committed source tree. The generated-source commit is `4cdbc250ef8cc90daf0ced898cb70b16a5e15a0a`; the final production output retains nested resource modules and the concise method names.

No generated Rust was edited by hand. The v1 guest-mode diagnostic is now opt-in because Fern's guest service enforces a daily session quota; ordinary pull-request CI validates the production Fern 0.48 generator.
