//! generated response wrappers

use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemResponse {
    #[serde(rename = "CoreMenuItem")]
    pub core_menu_item: Box<PaginatedCoreMenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionResponse {
    #[serde(rename = "CoreGroupAction")]
    pub core_group_action: Box<PaginatedCoreGroupAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupResponse {
    #[serde(rename = "CoreStandardGroup")]
    pub core_standard_group: Box<PaginatedCoreStandardGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupResponse {
    #[serde(rename = "CoreGeneratorGroup")]
    pub core_generator_group: Box<PaginatedCoreGeneratorGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupResponse {
    #[serde(rename = "CoreGeneratorAwareGroup")]
    pub core_generator_aware_group: Box<PaginatedCoreGeneratorAwareGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupResponse {
    #[serde(rename = "CoreGraphQLQueryGroup")]
    pub core_graph_ql_query_group: Box<PaginatedCoreGraphQLQueryGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupResponse {
    #[serde(rename = "CoreRepositoryGroup")]
    pub core_repository_group: Box<PaginatedCoreRepositoryGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagResponse {
    #[serde(rename = "BuiltinTag")]
    pub builtin_tag: Box<PaginatedBuiltinTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountResponse {
    #[serde(rename = "CoreAccount")]
    pub core_account: Box<PaginatedCoreAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionResponse {
    #[serde(rename = "CoreGeneratorAction")]
    pub core_generator_action: Box<PaginatedCoreGeneratorAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleResponse {
    #[serde(rename = "CoreGroupTriggerRule")]
    pub core_group_trigger_rule: Box<PaginatedCoreGroupTriggerRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleResponse {
    #[serde(rename = "CoreNodeTriggerRule")]
    pub core_node_trigger_rule: Box<PaginatedCoreNodeTriggerRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchResponse {
    #[serde(rename = "CoreNodeTriggerAttributeMatch")]
    pub core_node_trigger_attribute_match: Box<PaginatedCoreNodeTriggerAttributeMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchResponse {
    #[serde(rename = "CoreNodeTriggerRelationshipMatch")]
    pub core_node_trigger_relationship_match: Box<PaginatedCoreNodeTriggerRelationshipMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialResponse {
    #[serde(rename = "CorePasswordCredential")]
    pub core_password_credential: Box<PaginatedCorePasswordCredential>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeResponse {
    #[serde(rename = "CoreProposedChange")]
    pub core_proposed_change: Box<PaginatedCoreProposedChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadResponse {
    #[serde(rename = "CoreChangeThread")]
    pub core_change_thread: Box<PaginatedCoreChangeThread>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadResponse {
    #[serde(rename = "CoreFileThread")]
    pub core_file_thread: Box<PaginatedCoreFileThread>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadResponse {
    #[serde(rename = "CoreArtifactThread")]
    pub core_artifact_thread: Box<PaginatedCoreArtifactThread>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadResponse {
    #[serde(rename = "CoreObjectThread")]
    pub core_object_thread: Box<PaginatedCoreObjectThread>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentResponse {
    #[serde(rename = "CoreChangeComment")]
    pub core_change_comment: Box<PaginatedCoreChangeComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentResponse {
    #[serde(rename = "CoreThreadComment")]
    pub core_thread_comment: Box<PaginatedCoreThreadComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryResponse {
    #[serde(rename = "CoreRepository")]
    pub core_repository: Box<PaginatedCoreRepository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryResponse {
    #[serde(rename = "CoreReadOnlyRepository")]
    pub core_read_only_repository: Box<PaginatedCoreReadOnlyRepository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2Response {
    #[serde(rename = "CoreTransformJinja2")]
    pub core_transform_jinja2: Box<PaginatedCoreTransformJinja2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckResponse {
    #[serde(rename = "CoreDataCheck")]
    pub core_data_check: Box<PaginatedCoreDataCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckResponse {
    #[serde(rename = "CoreStandardCheck")]
    pub core_standard_check: Box<PaginatedCoreStandardCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckResponse {
    #[serde(rename = "CoreSchemaCheck")]
    pub core_schema_check: Box<PaginatedCoreSchemaCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckResponse {
    #[serde(rename = "CoreFileCheck")]
    pub core_file_check: Box<PaginatedCoreFileCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckResponse {
    #[serde(rename = "CoreArtifactCheck")]
    pub core_artifact_check: Box<PaginatedCoreArtifactCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckResponse {
    #[serde(rename = "CoreGeneratorCheck")]
    pub core_generator_check: Box<PaginatedCoreGeneratorCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorResponse {
    #[serde(rename = "CoreDataValidator")]
    pub core_data_validator: Box<PaginatedCoreDataValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorResponse {
    #[serde(rename = "CoreRepositoryValidator")]
    pub core_repository_validator: Box<PaginatedCoreRepositoryValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorResponse {
    #[serde(rename = "CoreUserValidator")]
    pub core_user_validator: Box<PaginatedCoreUserValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorResponse {
    #[serde(rename = "CoreSchemaValidator")]
    pub core_schema_validator: Box<PaginatedCoreSchemaValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorResponse {
    #[serde(rename = "CoreArtifactValidator")]
    pub core_artifact_validator: Box<PaginatedCoreArtifactValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorResponse {
    #[serde(rename = "CoreGeneratorValidator")]
    pub core_generator_validator: Box<PaginatedCoreGeneratorValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionResponse {
    #[serde(rename = "CoreCheckDefinition")]
    pub core_check_definition: Box<PaginatedCoreCheckDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonResponse {
    #[serde(rename = "CoreTransformPython")]
    pub core_transform_python: Box<PaginatedCoreTransformPython>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryResponse {
    #[serde(rename = "CoreGraphQLQuery")]
    pub core_graph_ql_query: Box<PaginatedCoreGraphQLQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactResponse {
    #[serde(rename = "CoreArtifact")]
    pub core_artifact: Box<PaginatedCoreArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionResponse {
    #[serde(rename = "CoreArtifactDefinition")]
    pub core_artifact_definition: Box<PaginatedCoreArtifactDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionResponse {
    #[serde(rename = "CoreGeneratorDefinition")]
    pub core_generator_definition: Box<PaginatedCoreGeneratorDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceResponse {
    #[serde(rename = "CoreGeneratorInstance")]
    pub core_generator_instance: Box<PaginatedCoreGeneratorInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueResponse {
    #[serde(rename = "CoreStaticKeyValue")]
    pub core_static_key_value: Box<PaginatedCoreStaticKeyValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueResponse {
    #[serde(rename = "CoreEnvKeyValue")]
    pub core_env_key_value: Box<PaginatedCoreEnvKeyValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookResponse {
    #[serde(rename = "CoreStandardWebhook")]
    pub core_standard_webhook: Box<PaginatedCoreStandardWebhook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookResponse {
    #[serde(rename = "CoreCustomWebhook")]
    pub core_custom_webhook: Box<PaginatedCoreCustomWebhook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceResponse {
    #[serde(rename = "IpamNamespace")]
    pub ipam_namespace: Box<PaginatedIpamNamespace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolResponse {
    #[serde(rename = "CoreIPPrefixPool")]
    pub core_ip_prefix_pool: Box<PaginatedCoreIPPrefixPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolResponse {
    #[serde(rename = "CoreIPAddressPool")]
    pub core_ip_address_pool: Box<PaginatedCoreIPAddressPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolResponse {
    #[serde(rename = "CoreNumberPool")]
    pub core_number_pool: Box<PaginatedCoreNumberPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionResponse {
    #[serde(rename = "CoreGlobalPermission")]
    pub core_global_permission: Box<PaginatedCoreGlobalPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionResponse {
    #[serde(rename = "CoreObjectPermission")]
    pub core_object_permission: Box<PaginatedCoreObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleResponse {
    #[serde(rename = "CoreAccountRole")]
    pub core_account_role: Box<PaginatedCoreAccountRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupResponse {
    #[serde(rename = "CoreAccountGroup")]
    pub core_account_group: Box<PaginatedCoreAccountGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProfileResponse {
    #[serde(rename = "CoreProfile")]
    pub core_profile: Box<PaginatedCoreProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreActionResponse {
    #[serde(rename = "CoreAction")]
    pub core_action: Box<PaginatedCoreAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTriggerRuleResponse {
    #[serde(rename = "CoreTriggerRule")]
    pub core_trigger_rule: Box<PaginatedCoreTriggerRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerMatchResponse {
    #[serde(rename = "CoreNodeTriggerMatch")]
    pub core_node_trigger_match: Box<PaginatedCoreNodeTriggerMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeResponse {
    #[serde(rename = "CoreNode")]
    pub core_node: Box<PaginatedCoreNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageOwnerResponse {
    #[serde(rename = "LineageOwner")]
    pub lineage_owner: Box<PaginatedLineageOwner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageSourceResponse {
    #[serde(rename = "LineageSource")]
    pub lineage_source: Box<PaginatedLineageSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCommentResponse {
    #[serde(rename = "CoreComment")]
    pub core_comment: Box<PaginatedCoreComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadResponse {
    #[serde(rename = "CoreThread")]
    pub core_thread: Box<PaginatedCoreThread>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupResponse {
    #[serde(rename = "CoreGroup")]
    pub core_group: Box<PaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreValidatorResponse {
    #[serde(rename = "CoreValidator")]
    pub core_validator: Box<PaginatedCoreValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckResponse {
    #[serde(rename = "CoreCheck")]
    pub core_check: Box<PaginatedCoreCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformationResponse {
    #[serde(rename = "CoreTransformation")]
    pub core_transformation: Box<PaginatedCoreTransformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactTargetResponse {
    #[serde(rename = "CoreArtifactTarget")]
    pub core_artifact_target: Box<PaginatedCoreArtifactTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileObjectResponse {
    #[serde(rename = "CoreFileObject")]
    pub core_file_object: Box<PaginatedCoreFileObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTaskTargetResponse {
    #[serde(rename = "CoreTaskTarget")]
    pub core_task_target: Box<PaginatedCoreTaskTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreKeyValueResponse {
    #[serde(rename = "CoreKeyValue")]
    pub core_key_value: Box<PaginatedCoreKeyValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWebhookResponse {
    #[serde(rename = "CoreWebhook")]
    pub core_webhook: Box<PaginatedCoreWebhook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericRepositoryResponse {
    #[serde(rename = "CoreGenericRepository")]
    pub core_generic_repository: Box<PaginatedCoreGenericRepository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPNamespaceResponse {
    #[serde(rename = "BuiltinIPNamespace")]
    pub builtin_ip_namespace: Box<PaginatedBuiltinIPNamespace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPPrefixResponse {
    #[serde(rename = "BuiltinIPPrefix")]
    pub builtin_ip_prefix: Box<PaginatedBuiltinIPPrefix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPAddressResponse {
    #[serde(rename = "BuiltinIPAddress")]
    pub builtin_ip_address: Box<PaginatedBuiltinIPAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreResourcePoolResponse {
    #[serde(rename = "CoreResourcePool")]
    pub core_resource_pool: Box<PaginatedCoreResourcePool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWeightedPoolResourceResponse {
    #[serde(rename = "CoreWeightedPoolResource")]
    pub core_weighted_pool_resource: Box<PaginatedCoreWeightedPoolResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPoolResponse {
    #[serde(rename = "CoreIPPool")]
    pub core_ip_pool: Box<PaginatedCoreIPPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericAccountResponse {
    #[serde(rename = "CoreGenericAccount")]
    pub core_generic_account: Box<PaginatedCoreGenericAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProfileResponse {
    #[serde(rename = "AccountProfile")]
    pub account_profile: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreBasePermissionResponse {
    #[serde(rename = "CoreBasePermission")]
    pub core_base_permission: Box<PaginatedCoreBasePermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCredentialResponse {
    #[serde(rename = "CoreCredential")]
    pub core_credential: Box<PaginatedCoreCredential>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectTemplateResponse {
    #[serde(rename = "CoreObjectTemplate")]
    pub core_object_template: Box<PaginatedCoreObjectTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectComponentTemplateResponse {
    #[serde(rename = "CoreObjectComponentTemplate")]
    pub core_object_component_template: Box<PaginatedCoreObjectComponentTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuResponse {
    #[serde(rename = "CoreMenu")]
    pub core_menu: Box<PaginatedCoreMenu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagResponse {
    #[serde(rename = "ProfileBuiltinTag")]
    pub profile_builtin_tag: Box<PaginatedProfileBuiltinTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceResponse {
    #[serde(rename = "ProfileIpamNamespace")]
    pub profile_ipam_namespace: Box<PaginatedProfileIpamNamespace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixResponse {
    #[serde(rename = "ProfileBuiltinIPPrefix")]
    pub profile_builtin_ip_prefix: Box<PaginatedProfileBuiltinIPPrefix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressResponse {
    #[serde(rename = "ProfileBuiltinIPAddress")]
    pub profile_builtin_ip_address: Box<PaginatedProfileBuiltinIPAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenResponse {
    #[serde(rename = "InfrahubAccountToken")]
    pub infrahub_account_token: Box<AccountTokenEdges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubPermissionsResponse {
    #[serde(rename = "InfrahubPermissions")]
    pub infrahub_permissions: Box<AccountPermissionsEdges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchResponse {
    #[serde(rename = "Branch")]
    pub branch: Vec<Branch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubBranchResponse {
    #[serde(rename = "InfrahubBranch")]
    pub infrahub_branch: Box<InfrahubBranchType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubGraphQLQueryReportResponse {
    #[serde(rename = "InfrahubGraphQLQueryReport")]
    pub infrahub_graph_ql_query_report: Box<GraphQLQueryReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubInfoResponse {
    #[serde(rename = "InfrahubInfo")]
    pub infrahub_info: Box<Info>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubIPAddressGetNextAvailableResponse {
    #[serde(rename = "InfrahubIPAddressGetNextAvailable")]
    pub infrahub_ip_address_get_next_available: Box<IPAddressGetNextAvailable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubIPPrefixGetNextAvailableResponse {
    #[serde(rename = "InfrahubIPPrefixGetNextAvailable")]
    pub infrahub_ip_prefix_get_next_available: Box<IPPrefixGetNextAvailable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubPathTraversalResponse {
    #[serde(rename = "InfrahubPathTraversal")]
    pub infrahub_path_traversal: Box<PathTraversalResultType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubEffectivePreferencesResponse {
    #[serde(rename = "InfrahubEffectivePreferences")]
    pub infrahub_effective_preferences: Box<EffectivePreferencesType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubUserPreferencesResponse {
    #[serde(rename = "InfrahubUserPreferences")]
    pub infrahub_user_preferences: Box<RawPreferencesType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubGlobalPreferencesResponse {
    #[serde(rename = "InfrahubGlobalPreferences")]
    pub infrahub_global_preferences: Box<RawPreferencesType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeAvailableActionsResponse {
    #[serde(rename = "CoreProposedChangeAvailableActions")]
    pub core_proposed_change_available_actions: Box<AvailableActions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubReachableNodesResponse {
    #[serde(rename = "InfrahubReachableNodes")]
    pub infrahub_reachable_nodes: Box<ReachableNodesResultType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipResponse {
    #[serde(rename = "Relationship")]
    pub relationship: Box<Relationships>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubResourcePoolAllocatedResponse {
    #[serde(rename = "InfrahubResourcePoolAllocated")]
    pub infrahub_resource_pool_allocated: Box<PoolAllocated>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubResourcePoolUtilizationResponse {
    #[serde(rename = "InfrahubResourcePoolUtilization")]
    pub infrahub_resource_pool_utilization: Box<PoolUtilization>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubSearchAnywhereResponse {
    #[serde(rename = "InfrahubSearchAnywhere")]
    pub infrahub_search_anywhere: Box<NodeEdges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubStatusResponse {
    #[serde(rename = "InfrahubStatus")]
    pub infrahub_status: Box<Status>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubTaskResponse {
    #[serde(rename = "InfrahubTask")]
    pub infrahub_task: Box<Tasks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubTaskBranchStatusResponse {
    #[serde(rename = "InfrahubTaskBranchStatus")]
    pub infrahub_task_branch_status: Box<Tasks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldsMappingTypeConversionResponse {
    #[serde(rename = "FieldsMappingTypeConversion")]
    pub fields_mapping_type_conversion: Box<FieldsMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffTreeResponse {
    #[serde(rename = "DiffTree")]
    pub diff_tree: Option<Box<DiffTree>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffTreeSummaryResponse {
    #[serde(rename = "DiffTreeSummary")]
    pub diff_tree_summary: Option<Box<DiffTreeSummary>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubEventResponse {
    #[serde(rename = "InfrahubEvent")]
    pub infrahub_event: Box<Events>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemCreateResponse {
    #[serde(rename = "CoreMenuItemCreate")]
    pub core_menu_item_create: Option<Box<CoreMenuItemCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemUpdateResponse {
    #[serde(rename = "CoreMenuItemUpdate")]
    pub core_menu_item_update: Option<Box<CoreMenuItemUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemUpsertResponse {
    #[serde(rename = "CoreMenuItemUpsert")]
    pub core_menu_item_upsert: Option<Box<CoreMenuItemUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemDeleteResponse {
    #[serde(rename = "CoreMenuItemDelete")]
    pub core_menu_item_delete: Option<Box<CoreMenuItemDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionCreateResponse {
    #[serde(rename = "CoreGroupActionCreate")]
    pub core_group_action_create: Option<Box<CoreGroupActionCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionUpdateResponse {
    #[serde(rename = "CoreGroupActionUpdate")]
    pub core_group_action_update: Option<Box<CoreGroupActionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionUpsertResponse {
    #[serde(rename = "CoreGroupActionUpsert")]
    pub core_group_action_upsert: Option<Box<CoreGroupActionUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionDeleteResponse {
    #[serde(rename = "CoreGroupActionDelete")]
    pub core_group_action_delete: Option<Box<CoreGroupActionDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupCreateResponse {
    #[serde(rename = "CoreStandardGroupCreate")]
    pub core_standard_group_create: Option<Box<CoreStandardGroupCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupUpdateResponse {
    #[serde(rename = "CoreStandardGroupUpdate")]
    pub core_standard_group_update: Option<Box<CoreStandardGroupUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupUpsertResponse {
    #[serde(rename = "CoreStandardGroupUpsert")]
    pub core_standard_group_upsert: Option<Box<CoreStandardGroupUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupDeleteResponse {
    #[serde(rename = "CoreStandardGroupDelete")]
    pub core_standard_group_delete: Option<Box<CoreStandardGroupDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupCreateResponse {
    #[serde(rename = "CoreGeneratorGroupCreate")]
    pub core_generator_group_create: Option<Box<CoreGeneratorGroupCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupUpdateResponse {
    #[serde(rename = "CoreGeneratorGroupUpdate")]
    pub core_generator_group_update: Option<Box<CoreGeneratorGroupUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupUpsertResponse {
    #[serde(rename = "CoreGeneratorGroupUpsert")]
    pub core_generator_group_upsert: Option<Box<CoreGeneratorGroupUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupDeleteResponse {
    #[serde(rename = "CoreGeneratorGroupDelete")]
    pub core_generator_group_delete: Option<Box<CoreGeneratorGroupDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupCreateResponse {
    #[serde(rename = "CoreGeneratorAwareGroupCreate")]
    pub core_generator_aware_group_create: Option<Box<CoreGeneratorAwareGroupCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupUpdateResponse {
    #[serde(rename = "CoreGeneratorAwareGroupUpdate")]
    pub core_generator_aware_group_update: Option<Box<CoreGeneratorAwareGroupUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupUpsertResponse {
    #[serde(rename = "CoreGeneratorAwareGroupUpsert")]
    pub core_generator_aware_group_upsert: Option<Box<CoreGeneratorAwareGroupUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupDeleteResponse {
    #[serde(rename = "CoreGeneratorAwareGroupDelete")]
    pub core_generator_aware_group_delete: Option<Box<CoreGeneratorAwareGroupDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupCreateResponse {
    #[serde(rename = "CoreGraphQLQueryGroupCreate")]
    pub core_graph_ql_query_group_create: Option<Box<CoreGraphQLQueryGroupCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupUpdateResponse {
    #[serde(rename = "CoreGraphQLQueryGroupUpdate")]
    pub core_graph_ql_query_group_update: Option<Box<CoreGraphQLQueryGroupUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupUpsertResponse {
    #[serde(rename = "CoreGraphQLQueryGroupUpsert")]
    pub core_graph_ql_query_group_upsert: Option<Box<CoreGraphQLQueryGroupUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupDeleteResponse {
    #[serde(rename = "CoreGraphQLQueryGroupDelete")]
    pub core_graph_ql_query_group_delete: Option<Box<CoreGraphQLQueryGroupDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupCreateResponse {
    #[serde(rename = "CoreRepositoryGroupCreate")]
    pub core_repository_group_create: Option<Box<CoreRepositoryGroupCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupUpdateResponse {
    #[serde(rename = "CoreRepositoryGroupUpdate")]
    pub core_repository_group_update: Option<Box<CoreRepositoryGroupUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupUpsertResponse {
    #[serde(rename = "CoreRepositoryGroupUpsert")]
    pub core_repository_group_upsert: Option<Box<CoreRepositoryGroupUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupDeleteResponse {
    #[serde(rename = "CoreRepositoryGroupDelete")]
    pub core_repository_group_delete: Option<Box<CoreRepositoryGroupDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagCreateResponse {
    #[serde(rename = "BuiltinTagCreate")]
    pub builtin_tag_create: Option<Box<BuiltinTagCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagUpdateResponse {
    #[serde(rename = "BuiltinTagUpdate")]
    pub builtin_tag_update: Option<Box<BuiltinTagUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagUpsertResponse {
    #[serde(rename = "BuiltinTagUpsert")]
    pub builtin_tag_upsert: Option<Box<BuiltinTagUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagDeleteResponse {
    #[serde(rename = "BuiltinTagDelete")]
    pub builtin_tag_delete: Option<Box<BuiltinTagDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountCreateResponse {
    #[serde(rename = "CoreAccountCreate")]
    pub core_account_create: Option<Box<CoreAccountCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountUpdateResponse {
    #[serde(rename = "CoreAccountUpdate")]
    pub core_account_update: Option<Box<CoreAccountUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountUpsertResponse {
    #[serde(rename = "CoreAccountUpsert")]
    pub core_account_upsert: Option<Box<CoreAccountUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountDeleteResponse {
    #[serde(rename = "CoreAccountDelete")]
    pub core_account_delete: Option<Box<CoreAccountDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionCreateResponse {
    #[serde(rename = "CoreGeneratorActionCreate")]
    pub core_generator_action_create: Option<Box<CoreGeneratorActionCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionUpdateResponse {
    #[serde(rename = "CoreGeneratorActionUpdate")]
    pub core_generator_action_update: Option<Box<CoreGeneratorActionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionUpsertResponse {
    #[serde(rename = "CoreGeneratorActionUpsert")]
    pub core_generator_action_upsert: Option<Box<CoreGeneratorActionUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionDeleteResponse {
    #[serde(rename = "CoreGeneratorActionDelete")]
    pub core_generator_action_delete: Option<Box<CoreGeneratorActionDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleCreateResponse {
    #[serde(rename = "CoreGroupTriggerRuleCreate")]
    pub core_group_trigger_rule_create: Option<Box<CoreGroupTriggerRuleCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleUpdateResponse {
    #[serde(rename = "CoreGroupTriggerRuleUpdate")]
    pub core_group_trigger_rule_update: Option<Box<CoreGroupTriggerRuleUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleUpsertResponse {
    #[serde(rename = "CoreGroupTriggerRuleUpsert")]
    pub core_group_trigger_rule_upsert: Option<Box<CoreGroupTriggerRuleUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleDeleteResponse {
    #[serde(rename = "CoreGroupTriggerRuleDelete")]
    pub core_group_trigger_rule_delete: Option<Box<CoreGroupTriggerRuleDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleCreateResponse {
    #[serde(rename = "CoreNodeTriggerRuleCreate")]
    pub core_node_trigger_rule_create: Option<Box<CoreNodeTriggerRuleCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleUpdateResponse {
    #[serde(rename = "CoreNodeTriggerRuleUpdate")]
    pub core_node_trigger_rule_update: Option<Box<CoreNodeTriggerRuleUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleUpsertResponse {
    #[serde(rename = "CoreNodeTriggerRuleUpsert")]
    pub core_node_trigger_rule_upsert: Option<Box<CoreNodeTriggerRuleUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleDeleteResponse {
    #[serde(rename = "CoreNodeTriggerRuleDelete")]
    pub core_node_trigger_rule_delete: Option<Box<CoreNodeTriggerRuleDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchCreateResponse {
    #[serde(rename = "CoreNodeTriggerAttributeMatchCreate")]
    pub core_node_trigger_attribute_match_create: Option<Box<CoreNodeTriggerAttributeMatchCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchUpdateResponse {
    #[serde(rename = "CoreNodeTriggerAttributeMatchUpdate")]
    pub core_node_trigger_attribute_match_update: Option<Box<CoreNodeTriggerAttributeMatchUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchUpsertResponse {
    #[serde(rename = "CoreNodeTriggerAttributeMatchUpsert")]
    pub core_node_trigger_attribute_match_upsert: Option<Box<CoreNodeTriggerAttributeMatchUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchDeleteResponse {
    #[serde(rename = "CoreNodeTriggerAttributeMatchDelete")]
    pub core_node_trigger_attribute_match_delete: Option<Box<CoreNodeTriggerAttributeMatchDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchCreateResponse {
    #[serde(rename = "CoreNodeTriggerRelationshipMatchCreate")]
    pub core_node_trigger_relationship_match_create: Option<Box<CoreNodeTriggerRelationshipMatchCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchUpdateResponse {
    #[serde(rename = "CoreNodeTriggerRelationshipMatchUpdate")]
    pub core_node_trigger_relationship_match_update: Option<Box<CoreNodeTriggerRelationshipMatchUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchUpsertResponse {
    #[serde(rename = "CoreNodeTriggerRelationshipMatchUpsert")]
    pub core_node_trigger_relationship_match_upsert: Option<Box<CoreNodeTriggerRelationshipMatchUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchDeleteResponse {
    #[serde(rename = "CoreNodeTriggerRelationshipMatchDelete")]
    pub core_node_trigger_relationship_match_delete: Option<Box<CoreNodeTriggerRelationshipMatchDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialCreateResponse {
    #[serde(rename = "CorePasswordCredentialCreate")]
    pub core_password_credential_create: Option<Box<CorePasswordCredentialCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialUpdateResponse {
    #[serde(rename = "CorePasswordCredentialUpdate")]
    pub core_password_credential_update: Option<Box<CorePasswordCredentialUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialUpsertResponse {
    #[serde(rename = "CorePasswordCredentialUpsert")]
    pub core_password_credential_upsert: Option<Box<CorePasswordCredentialUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialDeleteResponse {
    #[serde(rename = "CorePasswordCredentialDelete")]
    pub core_password_credential_delete: Option<Box<CorePasswordCredentialDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeCreateResponse {
    #[serde(rename = "CoreProposedChangeCreate")]
    pub core_proposed_change_create: Option<Box<CoreProposedChangeCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeUpdateResponse {
    #[serde(rename = "CoreProposedChangeUpdate")]
    pub core_proposed_change_update: Option<Box<CoreProposedChangeUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeUpsertResponse {
    #[serde(rename = "CoreProposedChangeUpsert")]
    pub core_proposed_change_upsert: Option<Box<CoreProposedChangeUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeDeleteResponse {
    #[serde(rename = "CoreProposedChangeDelete")]
    pub core_proposed_change_delete: Option<Box<CoreProposedChangeDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadCreateResponse {
    #[serde(rename = "CoreChangeThreadCreate")]
    pub core_change_thread_create: Option<Box<CoreChangeThreadCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadUpdateResponse {
    #[serde(rename = "CoreChangeThreadUpdate")]
    pub core_change_thread_update: Option<Box<CoreChangeThreadUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadUpsertResponse {
    #[serde(rename = "CoreChangeThreadUpsert")]
    pub core_change_thread_upsert: Option<Box<CoreChangeThreadUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadDeleteResponse {
    #[serde(rename = "CoreChangeThreadDelete")]
    pub core_change_thread_delete: Option<Box<CoreChangeThreadDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadCreateResponse {
    #[serde(rename = "CoreFileThreadCreate")]
    pub core_file_thread_create: Option<Box<CoreFileThreadCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadUpdateResponse {
    #[serde(rename = "CoreFileThreadUpdate")]
    pub core_file_thread_update: Option<Box<CoreFileThreadUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadUpsertResponse {
    #[serde(rename = "CoreFileThreadUpsert")]
    pub core_file_thread_upsert: Option<Box<CoreFileThreadUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadDeleteResponse {
    #[serde(rename = "CoreFileThreadDelete")]
    pub core_file_thread_delete: Option<Box<CoreFileThreadDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadCreateResponse {
    #[serde(rename = "CoreArtifactThreadCreate")]
    pub core_artifact_thread_create: Option<Box<CoreArtifactThreadCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadUpdateResponse {
    #[serde(rename = "CoreArtifactThreadUpdate")]
    pub core_artifact_thread_update: Option<Box<CoreArtifactThreadUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadUpsertResponse {
    #[serde(rename = "CoreArtifactThreadUpsert")]
    pub core_artifact_thread_upsert: Option<Box<CoreArtifactThreadUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadDeleteResponse {
    #[serde(rename = "CoreArtifactThreadDelete")]
    pub core_artifact_thread_delete: Option<Box<CoreArtifactThreadDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadCreateResponse {
    #[serde(rename = "CoreObjectThreadCreate")]
    pub core_object_thread_create: Option<Box<CoreObjectThreadCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadUpdateResponse {
    #[serde(rename = "CoreObjectThreadUpdate")]
    pub core_object_thread_update: Option<Box<CoreObjectThreadUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadUpsertResponse {
    #[serde(rename = "CoreObjectThreadUpsert")]
    pub core_object_thread_upsert: Option<Box<CoreObjectThreadUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadDeleteResponse {
    #[serde(rename = "CoreObjectThreadDelete")]
    pub core_object_thread_delete: Option<Box<CoreObjectThreadDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentCreateResponse {
    #[serde(rename = "CoreChangeCommentCreate")]
    pub core_change_comment_create: Option<Box<CoreChangeCommentCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentUpdateResponse {
    #[serde(rename = "CoreChangeCommentUpdate")]
    pub core_change_comment_update: Option<Box<CoreChangeCommentUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentUpsertResponse {
    #[serde(rename = "CoreChangeCommentUpsert")]
    pub core_change_comment_upsert: Option<Box<CoreChangeCommentUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentDeleteResponse {
    #[serde(rename = "CoreChangeCommentDelete")]
    pub core_change_comment_delete: Option<Box<CoreChangeCommentDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentCreateResponse {
    #[serde(rename = "CoreThreadCommentCreate")]
    pub core_thread_comment_create: Option<Box<CoreThreadCommentCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentUpdateResponse {
    #[serde(rename = "CoreThreadCommentUpdate")]
    pub core_thread_comment_update: Option<Box<CoreThreadCommentUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentUpsertResponse {
    #[serde(rename = "CoreThreadCommentUpsert")]
    pub core_thread_comment_upsert: Option<Box<CoreThreadCommentUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentDeleteResponse {
    #[serde(rename = "CoreThreadCommentDelete")]
    pub core_thread_comment_delete: Option<Box<CoreThreadCommentDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryCreateResponse {
    #[serde(rename = "CoreRepositoryCreate")]
    pub core_repository_create: Option<Box<CoreRepositoryCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryUpdateResponse {
    #[serde(rename = "CoreRepositoryUpdate")]
    pub core_repository_update: Option<Box<CoreRepositoryUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryUpsertResponse {
    #[serde(rename = "CoreRepositoryUpsert")]
    pub core_repository_upsert: Option<Box<CoreRepositoryUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryDeleteResponse {
    #[serde(rename = "CoreRepositoryDelete")]
    pub core_repository_delete: Option<Box<CoreRepositoryDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryCreateResponse {
    #[serde(rename = "CoreReadOnlyRepositoryCreate")]
    pub core_read_only_repository_create: Option<Box<CoreReadOnlyRepositoryCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryUpdateResponse {
    #[serde(rename = "CoreReadOnlyRepositoryUpdate")]
    pub core_read_only_repository_update: Option<Box<CoreReadOnlyRepositoryUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryUpsertResponse {
    #[serde(rename = "CoreReadOnlyRepositoryUpsert")]
    pub core_read_only_repository_upsert: Option<Box<CoreReadOnlyRepositoryUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryDeleteResponse {
    #[serde(rename = "CoreReadOnlyRepositoryDelete")]
    pub core_read_only_repository_delete: Option<Box<CoreReadOnlyRepositoryDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2CreateResponse {
    #[serde(rename = "CoreTransformJinja2Create")]
    pub core_transform_jinja2_create: Option<Box<CoreTransformJinja2Create>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2UpdateResponse {
    #[serde(rename = "CoreTransformJinja2Update")]
    pub core_transform_jinja2_update: Option<Box<CoreTransformJinja2Update>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2UpsertResponse {
    #[serde(rename = "CoreTransformJinja2Upsert")]
    pub core_transform_jinja2_upsert: Option<Box<CoreTransformJinja2Upsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2DeleteResponse {
    #[serde(rename = "CoreTransformJinja2Delete")]
    pub core_transform_jinja2_delete: Option<Box<CoreTransformJinja2Delete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckCreateResponse {
    #[serde(rename = "CoreDataCheckCreate")]
    pub core_data_check_create: Option<Box<CoreDataCheckCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckUpdateResponse {
    #[serde(rename = "CoreDataCheckUpdate")]
    pub core_data_check_update: Option<Box<CoreDataCheckUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckUpsertResponse {
    #[serde(rename = "CoreDataCheckUpsert")]
    pub core_data_check_upsert: Option<Box<CoreDataCheckUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckDeleteResponse {
    #[serde(rename = "CoreDataCheckDelete")]
    pub core_data_check_delete: Option<Box<CoreDataCheckDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckCreateResponse {
    #[serde(rename = "CoreStandardCheckCreate")]
    pub core_standard_check_create: Option<Box<CoreStandardCheckCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckUpdateResponse {
    #[serde(rename = "CoreStandardCheckUpdate")]
    pub core_standard_check_update: Option<Box<CoreStandardCheckUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckUpsertResponse {
    #[serde(rename = "CoreStandardCheckUpsert")]
    pub core_standard_check_upsert: Option<Box<CoreStandardCheckUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckDeleteResponse {
    #[serde(rename = "CoreStandardCheckDelete")]
    pub core_standard_check_delete: Option<Box<CoreStandardCheckDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckCreateResponse {
    #[serde(rename = "CoreSchemaCheckCreate")]
    pub core_schema_check_create: Option<Box<CoreSchemaCheckCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckUpdateResponse {
    #[serde(rename = "CoreSchemaCheckUpdate")]
    pub core_schema_check_update: Option<Box<CoreSchemaCheckUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckUpsertResponse {
    #[serde(rename = "CoreSchemaCheckUpsert")]
    pub core_schema_check_upsert: Option<Box<CoreSchemaCheckUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckDeleteResponse {
    #[serde(rename = "CoreSchemaCheckDelete")]
    pub core_schema_check_delete: Option<Box<CoreSchemaCheckDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckCreateResponse {
    #[serde(rename = "CoreFileCheckCreate")]
    pub core_file_check_create: Option<Box<CoreFileCheckCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckUpdateResponse {
    #[serde(rename = "CoreFileCheckUpdate")]
    pub core_file_check_update: Option<Box<CoreFileCheckUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckUpsertResponse {
    #[serde(rename = "CoreFileCheckUpsert")]
    pub core_file_check_upsert: Option<Box<CoreFileCheckUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckDeleteResponse {
    #[serde(rename = "CoreFileCheckDelete")]
    pub core_file_check_delete: Option<Box<CoreFileCheckDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckCreateResponse {
    #[serde(rename = "CoreArtifactCheckCreate")]
    pub core_artifact_check_create: Option<Box<CoreArtifactCheckCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckUpdateResponse {
    #[serde(rename = "CoreArtifactCheckUpdate")]
    pub core_artifact_check_update: Option<Box<CoreArtifactCheckUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckUpsertResponse {
    #[serde(rename = "CoreArtifactCheckUpsert")]
    pub core_artifact_check_upsert: Option<Box<CoreArtifactCheckUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckDeleteResponse {
    #[serde(rename = "CoreArtifactCheckDelete")]
    pub core_artifact_check_delete: Option<Box<CoreArtifactCheckDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckCreateResponse {
    #[serde(rename = "CoreGeneratorCheckCreate")]
    pub core_generator_check_create: Option<Box<CoreGeneratorCheckCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckUpdateResponse {
    #[serde(rename = "CoreGeneratorCheckUpdate")]
    pub core_generator_check_update: Option<Box<CoreGeneratorCheckUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckUpsertResponse {
    #[serde(rename = "CoreGeneratorCheckUpsert")]
    pub core_generator_check_upsert: Option<Box<CoreGeneratorCheckUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckDeleteResponse {
    #[serde(rename = "CoreGeneratorCheckDelete")]
    pub core_generator_check_delete: Option<Box<CoreGeneratorCheckDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorCreateResponse {
    #[serde(rename = "CoreDataValidatorCreate")]
    pub core_data_validator_create: Option<Box<CoreDataValidatorCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorUpdateResponse {
    #[serde(rename = "CoreDataValidatorUpdate")]
    pub core_data_validator_update: Option<Box<CoreDataValidatorUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorUpsertResponse {
    #[serde(rename = "CoreDataValidatorUpsert")]
    pub core_data_validator_upsert: Option<Box<CoreDataValidatorUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorDeleteResponse {
    #[serde(rename = "CoreDataValidatorDelete")]
    pub core_data_validator_delete: Option<Box<CoreDataValidatorDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorCreateResponse {
    #[serde(rename = "CoreRepositoryValidatorCreate")]
    pub core_repository_validator_create: Option<Box<CoreRepositoryValidatorCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorUpdateResponse {
    #[serde(rename = "CoreRepositoryValidatorUpdate")]
    pub core_repository_validator_update: Option<Box<CoreRepositoryValidatorUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorUpsertResponse {
    #[serde(rename = "CoreRepositoryValidatorUpsert")]
    pub core_repository_validator_upsert: Option<Box<CoreRepositoryValidatorUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorDeleteResponse {
    #[serde(rename = "CoreRepositoryValidatorDelete")]
    pub core_repository_validator_delete: Option<Box<CoreRepositoryValidatorDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorCreateResponse {
    #[serde(rename = "CoreUserValidatorCreate")]
    pub core_user_validator_create: Option<Box<CoreUserValidatorCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorUpdateResponse {
    #[serde(rename = "CoreUserValidatorUpdate")]
    pub core_user_validator_update: Option<Box<CoreUserValidatorUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorUpsertResponse {
    #[serde(rename = "CoreUserValidatorUpsert")]
    pub core_user_validator_upsert: Option<Box<CoreUserValidatorUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorDeleteResponse {
    #[serde(rename = "CoreUserValidatorDelete")]
    pub core_user_validator_delete: Option<Box<CoreUserValidatorDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorCreateResponse {
    #[serde(rename = "CoreSchemaValidatorCreate")]
    pub core_schema_validator_create: Option<Box<CoreSchemaValidatorCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorUpdateResponse {
    #[serde(rename = "CoreSchemaValidatorUpdate")]
    pub core_schema_validator_update: Option<Box<CoreSchemaValidatorUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorUpsertResponse {
    #[serde(rename = "CoreSchemaValidatorUpsert")]
    pub core_schema_validator_upsert: Option<Box<CoreSchemaValidatorUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorDeleteResponse {
    #[serde(rename = "CoreSchemaValidatorDelete")]
    pub core_schema_validator_delete: Option<Box<CoreSchemaValidatorDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorCreateResponse {
    #[serde(rename = "CoreArtifactValidatorCreate")]
    pub core_artifact_validator_create: Option<Box<CoreArtifactValidatorCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorUpdateResponse {
    #[serde(rename = "CoreArtifactValidatorUpdate")]
    pub core_artifact_validator_update: Option<Box<CoreArtifactValidatorUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorUpsertResponse {
    #[serde(rename = "CoreArtifactValidatorUpsert")]
    pub core_artifact_validator_upsert: Option<Box<CoreArtifactValidatorUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorDeleteResponse {
    #[serde(rename = "CoreArtifactValidatorDelete")]
    pub core_artifact_validator_delete: Option<Box<CoreArtifactValidatorDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorCreateResponse {
    #[serde(rename = "CoreGeneratorValidatorCreate")]
    pub core_generator_validator_create: Option<Box<CoreGeneratorValidatorCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorUpdateResponse {
    #[serde(rename = "CoreGeneratorValidatorUpdate")]
    pub core_generator_validator_update: Option<Box<CoreGeneratorValidatorUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorUpsertResponse {
    #[serde(rename = "CoreGeneratorValidatorUpsert")]
    pub core_generator_validator_upsert: Option<Box<CoreGeneratorValidatorUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorDeleteResponse {
    #[serde(rename = "CoreGeneratorValidatorDelete")]
    pub core_generator_validator_delete: Option<Box<CoreGeneratorValidatorDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionCreateResponse {
    #[serde(rename = "CoreCheckDefinitionCreate")]
    pub core_check_definition_create: Option<Box<CoreCheckDefinitionCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionUpdateResponse {
    #[serde(rename = "CoreCheckDefinitionUpdate")]
    pub core_check_definition_update: Option<Box<CoreCheckDefinitionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionUpsertResponse {
    #[serde(rename = "CoreCheckDefinitionUpsert")]
    pub core_check_definition_upsert: Option<Box<CoreCheckDefinitionUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionDeleteResponse {
    #[serde(rename = "CoreCheckDefinitionDelete")]
    pub core_check_definition_delete: Option<Box<CoreCheckDefinitionDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonCreateResponse {
    #[serde(rename = "CoreTransformPythonCreate")]
    pub core_transform_python_create: Option<Box<CoreTransformPythonCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonUpdateResponse {
    #[serde(rename = "CoreTransformPythonUpdate")]
    pub core_transform_python_update: Option<Box<CoreTransformPythonUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonUpsertResponse {
    #[serde(rename = "CoreTransformPythonUpsert")]
    pub core_transform_python_upsert: Option<Box<CoreTransformPythonUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonDeleteResponse {
    #[serde(rename = "CoreTransformPythonDelete")]
    pub core_transform_python_delete: Option<Box<CoreTransformPythonDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryCreateResponse {
    #[serde(rename = "CoreGraphQLQueryCreate")]
    pub core_graph_ql_query_create: Option<Box<CoreGraphQLQueryCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryUpdateResponse {
    #[serde(rename = "CoreGraphQLQueryUpdate")]
    pub core_graph_ql_query_update: Option<Box<CoreGraphQLQueryUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryUpsertResponse {
    #[serde(rename = "CoreGraphQLQueryUpsert")]
    pub core_graph_ql_query_upsert: Option<Box<CoreGraphQLQueryUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryDeleteResponse {
    #[serde(rename = "CoreGraphQLQueryDelete")]
    pub core_graph_ql_query_delete: Option<Box<CoreGraphQLQueryDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCreateResponse {
    #[serde(rename = "CoreArtifactCreate")]
    pub core_artifact_create: Option<Box<CoreArtifactCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactUpdateResponse {
    #[serde(rename = "CoreArtifactUpdate")]
    pub core_artifact_update: Option<Box<CoreArtifactUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactUpsertResponse {
    #[serde(rename = "CoreArtifactUpsert")]
    pub core_artifact_upsert: Option<Box<CoreArtifactUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDeleteResponse {
    #[serde(rename = "CoreArtifactDelete")]
    pub core_artifact_delete: Option<Box<CoreArtifactDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionCreateResponse {
    #[serde(rename = "CoreArtifactDefinitionCreate")]
    pub core_artifact_definition_create: Option<Box<CoreArtifactDefinitionCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionUpdateResponse {
    #[serde(rename = "CoreArtifactDefinitionUpdate")]
    pub core_artifact_definition_update: Option<Box<CoreArtifactDefinitionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionUpsertResponse {
    #[serde(rename = "CoreArtifactDefinitionUpsert")]
    pub core_artifact_definition_upsert: Option<Box<CoreArtifactDefinitionUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionDeleteResponse {
    #[serde(rename = "CoreArtifactDefinitionDelete")]
    pub core_artifact_definition_delete: Option<Box<CoreArtifactDefinitionDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionCreateResponse {
    #[serde(rename = "CoreGeneratorDefinitionCreate")]
    pub core_generator_definition_create: Option<Box<CoreGeneratorDefinitionCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionUpdateResponse {
    #[serde(rename = "CoreGeneratorDefinitionUpdate")]
    pub core_generator_definition_update: Option<Box<CoreGeneratorDefinitionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionUpsertResponse {
    #[serde(rename = "CoreGeneratorDefinitionUpsert")]
    pub core_generator_definition_upsert: Option<Box<CoreGeneratorDefinitionUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionDeleteResponse {
    #[serde(rename = "CoreGeneratorDefinitionDelete")]
    pub core_generator_definition_delete: Option<Box<CoreGeneratorDefinitionDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceCreateResponse {
    #[serde(rename = "CoreGeneratorInstanceCreate")]
    pub core_generator_instance_create: Option<Box<CoreGeneratorInstanceCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceUpdateResponse {
    #[serde(rename = "CoreGeneratorInstanceUpdate")]
    pub core_generator_instance_update: Option<Box<CoreGeneratorInstanceUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceUpsertResponse {
    #[serde(rename = "CoreGeneratorInstanceUpsert")]
    pub core_generator_instance_upsert: Option<Box<CoreGeneratorInstanceUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceDeleteResponse {
    #[serde(rename = "CoreGeneratorInstanceDelete")]
    pub core_generator_instance_delete: Option<Box<CoreGeneratorInstanceDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueCreateResponse {
    #[serde(rename = "CoreStaticKeyValueCreate")]
    pub core_static_key_value_create: Option<Box<CoreStaticKeyValueCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueUpdateResponse {
    #[serde(rename = "CoreStaticKeyValueUpdate")]
    pub core_static_key_value_update: Option<Box<CoreStaticKeyValueUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueUpsertResponse {
    #[serde(rename = "CoreStaticKeyValueUpsert")]
    pub core_static_key_value_upsert: Option<Box<CoreStaticKeyValueUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueDeleteResponse {
    #[serde(rename = "CoreStaticKeyValueDelete")]
    pub core_static_key_value_delete: Option<Box<CoreStaticKeyValueDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueCreateResponse {
    #[serde(rename = "CoreEnvKeyValueCreate")]
    pub core_env_key_value_create: Option<Box<CoreEnvKeyValueCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueUpdateResponse {
    #[serde(rename = "CoreEnvKeyValueUpdate")]
    pub core_env_key_value_update: Option<Box<CoreEnvKeyValueUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueUpsertResponse {
    #[serde(rename = "CoreEnvKeyValueUpsert")]
    pub core_env_key_value_upsert: Option<Box<CoreEnvKeyValueUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueDeleteResponse {
    #[serde(rename = "CoreEnvKeyValueDelete")]
    pub core_env_key_value_delete: Option<Box<CoreEnvKeyValueDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookCreateResponse {
    #[serde(rename = "CoreStandardWebhookCreate")]
    pub core_standard_webhook_create: Option<Box<CoreStandardWebhookCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookUpdateResponse {
    #[serde(rename = "CoreStandardWebhookUpdate")]
    pub core_standard_webhook_update: Option<Box<CoreStandardWebhookUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookUpsertResponse {
    #[serde(rename = "CoreStandardWebhookUpsert")]
    pub core_standard_webhook_upsert: Option<Box<CoreStandardWebhookUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookDeleteResponse {
    #[serde(rename = "CoreStandardWebhookDelete")]
    pub core_standard_webhook_delete: Option<Box<CoreStandardWebhookDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookCreateResponse {
    #[serde(rename = "CoreCustomWebhookCreate")]
    pub core_custom_webhook_create: Option<Box<CoreCustomWebhookCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookUpdateResponse {
    #[serde(rename = "CoreCustomWebhookUpdate")]
    pub core_custom_webhook_update: Option<Box<CoreCustomWebhookUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookUpsertResponse {
    #[serde(rename = "CoreCustomWebhookUpsert")]
    pub core_custom_webhook_upsert: Option<Box<CoreCustomWebhookUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookDeleteResponse {
    #[serde(rename = "CoreCustomWebhookDelete")]
    pub core_custom_webhook_delete: Option<Box<CoreCustomWebhookDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceCreateResponse {
    #[serde(rename = "IpamNamespaceCreate")]
    pub ipam_namespace_create: Option<Box<IpamNamespaceCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceUpdateResponse {
    #[serde(rename = "IpamNamespaceUpdate")]
    pub ipam_namespace_update: Option<Box<IpamNamespaceUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceUpsertResponse {
    #[serde(rename = "IpamNamespaceUpsert")]
    pub ipam_namespace_upsert: Option<Box<IpamNamespaceUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceDeleteResponse {
    #[serde(rename = "IpamNamespaceDelete")]
    pub ipam_namespace_delete: Option<Box<IpamNamespaceDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolCreateResponse {
    #[serde(rename = "CoreIPPrefixPoolCreate")]
    pub core_ip_prefix_pool_create: Option<Box<CoreIPPrefixPoolCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolUpdateResponse {
    #[serde(rename = "CoreIPPrefixPoolUpdate")]
    pub core_ip_prefix_pool_update: Option<Box<CoreIPPrefixPoolUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolUpsertResponse {
    #[serde(rename = "CoreIPPrefixPoolUpsert")]
    pub core_ip_prefix_pool_upsert: Option<Box<CoreIPPrefixPoolUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolDeleteResponse {
    #[serde(rename = "CoreIPPrefixPoolDelete")]
    pub core_ip_prefix_pool_delete: Option<Box<CoreIPPrefixPoolDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolCreateResponse {
    #[serde(rename = "CoreIPAddressPoolCreate")]
    pub core_ip_address_pool_create: Option<Box<CoreIPAddressPoolCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolUpdateResponse {
    #[serde(rename = "CoreIPAddressPoolUpdate")]
    pub core_ip_address_pool_update: Option<Box<CoreIPAddressPoolUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolUpsertResponse {
    #[serde(rename = "CoreIPAddressPoolUpsert")]
    pub core_ip_address_pool_upsert: Option<Box<CoreIPAddressPoolUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolDeleteResponse {
    #[serde(rename = "CoreIPAddressPoolDelete")]
    pub core_ip_address_pool_delete: Option<Box<CoreIPAddressPoolDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolCreateResponse {
    #[serde(rename = "CoreNumberPoolCreate")]
    pub core_number_pool_create: Option<Box<CoreNumberPoolCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolUpdateResponse {
    #[serde(rename = "CoreNumberPoolUpdate")]
    pub core_number_pool_update: Option<Box<CoreNumberPoolUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolUpsertResponse {
    #[serde(rename = "CoreNumberPoolUpsert")]
    pub core_number_pool_upsert: Option<Box<CoreNumberPoolUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolDeleteResponse {
    #[serde(rename = "CoreNumberPoolDelete")]
    pub core_number_pool_delete: Option<Box<CoreNumberPoolDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionCreateResponse {
    #[serde(rename = "CoreGlobalPermissionCreate")]
    pub core_global_permission_create: Option<Box<CoreGlobalPermissionCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionUpdateResponse {
    #[serde(rename = "CoreGlobalPermissionUpdate")]
    pub core_global_permission_update: Option<Box<CoreGlobalPermissionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionUpsertResponse {
    #[serde(rename = "CoreGlobalPermissionUpsert")]
    pub core_global_permission_upsert: Option<Box<CoreGlobalPermissionUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionDeleteResponse {
    #[serde(rename = "CoreGlobalPermissionDelete")]
    pub core_global_permission_delete: Option<Box<CoreGlobalPermissionDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionCreateResponse {
    #[serde(rename = "CoreObjectPermissionCreate")]
    pub core_object_permission_create: Option<Box<CoreObjectPermissionCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionUpdateResponse {
    #[serde(rename = "CoreObjectPermissionUpdate")]
    pub core_object_permission_update: Option<Box<CoreObjectPermissionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionUpsertResponse {
    #[serde(rename = "CoreObjectPermissionUpsert")]
    pub core_object_permission_upsert: Option<Box<CoreObjectPermissionUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionDeleteResponse {
    #[serde(rename = "CoreObjectPermissionDelete")]
    pub core_object_permission_delete: Option<Box<CoreObjectPermissionDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleCreateResponse {
    #[serde(rename = "CoreAccountRoleCreate")]
    pub core_account_role_create: Option<Box<CoreAccountRoleCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleUpdateResponse {
    #[serde(rename = "CoreAccountRoleUpdate")]
    pub core_account_role_update: Option<Box<CoreAccountRoleUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleUpsertResponse {
    #[serde(rename = "CoreAccountRoleUpsert")]
    pub core_account_role_upsert: Option<Box<CoreAccountRoleUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleDeleteResponse {
    #[serde(rename = "CoreAccountRoleDelete")]
    pub core_account_role_delete: Option<Box<CoreAccountRoleDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupCreateResponse {
    #[serde(rename = "CoreAccountGroupCreate")]
    pub core_account_group_create: Option<Box<CoreAccountGroupCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupUpdateResponse {
    #[serde(rename = "CoreAccountGroupUpdate")]
    pub core_account_group_update: Option<Box<CoreAccountGroupUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupUpsertResponse {
    #[serde(rename = "CoreAccountGroupUpsert")]
    pub core_account_group_upsert: Option<Box<CoreAccountGroupUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupDeleteResponse {
    #[serde(rename = "CoreAccountGroupDelete")]
    pub core_account_group_delete: Option<Box<CoreAccountGroupDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProfileUpdateResponse {
    #[serde(rename = "CoreProfileUpdate")]
    pub core_profile_update: Option<Box<CoreProfileUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreActionUpdateResponse {
    #[serde(rename = "CoreActionUpdate")]
    pub core_action_update: Option<Box<CoreActionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTriggerRuleUpdateResponse {
    #[serde(rename = "CoreTriggerRuleUpdate")]
    pub core_trigger_rule_update: Option<Box<CoreTriggerRuleUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerMatchUpdateResponse {
    #[serde(rename = "CoreNodeTriggerMatchUpdate")]
    pub core_node_trigger_match_update: Option<Box<CoreNodeTriggerMatchUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeUpdateResponse {
    #[serde(rename = "CoreNodeUpdate")]
    pub core_node_update: Option<Box<CoreNodeUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCommentUpdateResponse {
    #[serde(rename = "CoreCommentUpdate")]
    pub core_comment_update: Option<Box<CoreCommentUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadUpdateResponse {
    #[serde(rename = "CoreThreadUpdate")]
    pub core_thread_update: Option<Box<CoreThreadUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupUpdateResponse {
    #[serde(rename = "CoreGroupUpdate")]
    pub core_group_update: Option<Box<CoreGroupUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreValidatorUpdateResponse {
    #[serde(rename = "CoreValidatorUpdate")]
    pub core_validator_update: Option<Box<CoreValidatorUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckUpdateResponse {
    #[serde(rename = "CoreCheckUpdate")]
    pub core_check_update: Option<Box<CoreCheckUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformationUpdateResponse {
    #[serde(rename = "CoreTransformationUpdate")]
    pub core_transformation_update: Option<Box<CoreTransformationUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactTargetUpdateResponse {
    #[serde(rename = "CoreArtifactTargetUpdate")]
    pub core_artifact_target_update: Option<Box<CoreArtifactTargetUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileObjectUpdateResponse {
    #[serde(rename = "CoreFileObjectUpdate")]
    pub core_file_object_update: Option<Box<CoreFileObjectUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTaskTargetUpdateResponse {
    #[serde(rename = "CoreTaskTargetUpdate")]
    pub core_task_target_update: Option<Box<CoreTaskTargetUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreKeyValueUpdateResponse {
    #[serde(rename = "CoreKeyValueUpdate")]
    pub core_key_value_update: Option<Box<CoreKeyValueUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWebhookUpdateResponse {
    #[serde(rename = "CoreWebhookUpdate")]
    pub core_webhook_update: Option<Box<CoreWebhookUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericRepositoryUpdateResponse {
    #[serde(rename = "CoreGenericRepositoryUpdate")]
    pub core_generic_repository_update: Option<Box<CoreGenericRepositoryUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPNamespaceUpdateResponse {
    #[serde(rename = "BuiltinIPNamespaceUpdate")]
    pub builtin_ip_namespace_update: Option<Box<BuiltinIPNamespaceUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPPrefixUpdateResponse {
    #[serde(rename = "BuiltinIPPrefixUpdate")]
    pub builtin_ip_prefix_update: Option<Box<BuiltinIPPrefixUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPAddressUpdateResponse {
    #[serde(rename = "BuiltinIPAddressUpdate")]
    pub builtin_ip_address_update: Option<Box<BuiltinIPAddressUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreResourcePoolUpdateResponse {
    #[serde(rename = "CoreResourcePoolUpdate")]
    pub core_resource_pool_update: Option<Box<CoreResourcePoolUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWeightedPoolResourceUpdateResponse {
    #[serde(rename = "CoreWeightedPoolResourceUpdate")]
    pub core_weighted_pool_resource_update: Option<Box<CoreWeightedPoolResourceUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPoolUpdateResponse {
    #[serde(rename = "CoreIPPoolUpdate")]
    pub core_ip_pool_update: Option<Box<CoreIPPoolUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericAccountUpdateResponse {
    #[serde(rename = "CoreGenericAccountUpdate")]
    pub core_generic_account_update: Option<Box<CoreGenericAccountUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreBasePermissionUpdateResponse {
    #[serde(rename = "CoreBasePermissionUpdate")]
    pub core_base_permission_update: Option<Box<CoreBasePermissionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCredentialUpdateResponse {
    #[serde(rename = "CoreCredentialUpdate")]
    pub core_credential_update: Option<Box<CoreCredentialUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectTemplateUpdateResponse {
    #[serde(rename = "CoreObjectTemplateUpdate")]
    pub core_object_template_update: Option<Box<CoreObjectTemplateUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectComponentTemplateUpdateResponse {
    #[serde(rename = "CoreObjectComponentTemplateUpdate")]
    pub core_object_component_template_update: Option<Box<CoreObjectComponentTemplateUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuUpdateResponse {
    #[serde(rename = "CoreMenuUpdate")]
    pub core_menu_update: Option<Box<CoreMenuUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagCreateResponse {
    #[serde(rename = "ProfileBuiltinTagCreate")]
    pub profile_builtin_tag_create: Option<Box<ProfileBuiltinTagCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagUpdateResponse {
    #[serde(rename = "ProfileBuiltinTagUpdate")]
    pub profile_builtin_tag_update: Option<Box<ProfileBuiltinTagUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagUpsertResponse {
    #[serde(rename = "ProfileBuiltinTagUpsert")]
    pub profile_builtin_tag_upsert: Option<Box<ProfileBuiltinTagUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagDeleteResponse {
    #[serde(rename = "ProfileBuiltinTagDelete")]
    pub profile_builtin_tag_delete: Option<Box<ProfileBuiltinTagDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceCreateResponse {
    #[serde(rename = "ProfileIpamNamespaceCreate")]
    pub profile_ipam_namespace_create: Option<Box<ProfileIpamNamespaceCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceUpdateResponse {
    #[serde(rename = "ProfileIpamNamespaceUpdate")]
    pub profile_ipam_namespace_update: Option<Box<ProfileIpamNamespaceUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceUpsertResponse {
    #[serde(rename = "ProfileIpamNamespaceUpsert")]
    pub profile_ipam_namespace_upsert: Option<Box<ProfileIpamNamespaceUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceDeleteResponse {
    #[serde(rename = "ProfileIpamNamespaceDelete")]
    pub profile_ipam_namespace_delete: Option<Box<ProfileIpamNamespaceDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixCreateResponse {
    #[serde(rename = "ProfileBuiltinIPPrefixCreate")]
    pub profile_builtin_ip_prefix_create: Option<Box<ProfileBuiltinIPPrefixCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixUpdateResponse {
    #[serde(rename = "ProfileBuiltinIPPrefixUpdate")]
    pub profile_builtin_ip_prefix_update: Option<Box<ProfileBuiltinIPPrefixUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixUpsertResponse {
    #[serde(rename = "ProfileBuiltinIPPrefixUpsert")]
    pub profile_builtin_ip_prefix_upsert: Option<Box<ProfileBuiltinIPPrefixUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixDeleteResponse {
    #[serde(rename = "ProfileBuiltinIPPrefixDelete")]
    pub profile_builtin_ip_prefix_delete: Option<Box<ProfileBuiltinIPPrefixDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressCreateResponse {
    #[serde(rename = "ProfileBuiltinIPAddressCreate")]
    pub profile_builtin_ip_address_create: Option<Box<ProfileBuiltinIPAddressCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressUpdateResponse {
    #[serde(rename = "ProfileBuiltinIPAddressUpdate")]
    pub profile_builtin_ip_address_update: Option<Box<ProfileBuiltinIPAddressUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressUpsertResponse {
    #[serde(rename = "ProfileBuiltinIPAddressUpsert")]
    pub profile_builtin_ip_address_upsert: Option<Box<ProfileBuiltinIPAddressUpsert>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressDeleteResponse {
    #[serde(rename = "ProfileBuiltinIPAddressDelete")]
    pub profile_builtin_ip_address_delete: Option<Box<ProfileBuiltinIPAddressDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenCreateResponse {
    #[serde(rename = "InfrahubAccountTokenCreate")]
    pub infrahub_account_token_create: Option<Box<InfrahubAccountTokenCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountSelfUpdateResponse {
    #[serde(rename = "InfrahubAccountSelfUpdate")]
    pub infrahub_account_self_update: Option<Box<InfrahubAccountSelfUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenDeleteResponse {
    #[serde(rename = "InfrahubAccountTokenDelete")]
    pub infrahub_account_token_delete: Option<Box<InfrahubAccountTokenDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeRunCheckResponse {
    #[serde(rename = "CoreProposedChangeRunCheck")]
    pub core_proposed_change_run_check: Option<Box<ProposedChangeRequestRunCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeMergeResponse {
    #[serde(rename = "CoreProposedChangeMerge")]
    pub core_proposed_change_merge: Option<Box<ProposedChangeMerge>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeReviewResponse {
    #[serde(rename = "CoreProposedChangeReview")]
    pub core_proposed_change_review: Option<Box<ProposedChangeReview>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionRunResponse {
    #[serde(rename = "CoreGeneratorDefinitionRun")]
    pub core_generator_definition_run: Option<Box<GeneratorDefinitionRequestRun>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubIPPrefixPoolGetResourceResponse {
    #[serde(rename = "InfrahubIPPrefixPoolGetResource")]
    pub infrahub_ip_prefix_pool_get_resource: Option<Box<IPPrefixPoolGetResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubIPAddressPoolGetResourceResponse {
    #[serde(rename = "InfrahubIPAddressPoolGetResource")]
    pub infrahub_ip_address_pool_get_resource: Option<Box<IPAddressPoolGetResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCreateResponse {
    #[serde(rename = "BranchCreate")]
    pub branch_create: Option<Box<BranchCreate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchDeleteResponse {
    #[serde(rename = "BranchDelete")]
    pub branch_delete: Option<Box<BranchDelete>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchRebaseResponse {
    #[serde(rename = "BranchRebase")]
    pub branch_rebase: Option<Box<BranchRebase>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchMergeResponse {
    #[serde(rename = "BranchMerge")]
    pub branch_merge: Option<Box<BranchMerge>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchUpdateResponse {
    #[serde(rename = "BranchUpdate")]
    pub branch_update: Option<Box<BranchUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchValidateResponse {
    #[serde(rename = "BranchValidate")]
    pub branch_validate: Option<Box<BranchValidate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffUpdateResponse {
    #[serde(rename = "DiffUpdate")]
    pub diff_update: Option<Box<DiffUpdateMutation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubTaskRetryResponse {
    #[serde(rename = "InfrahubTaskRetry")]
    pub infrahub_task_retry: Option<Box<InfrahubTaskRetry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubTaskCancelResponse {
    #[serde(rename = "InfrahubTaskCancel")]
    pub infrahub_task_cancel: Option<Box<InfrahubTaskCancel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubReadOnlyRepositoryImportLastCommitResponse {
    #[serde(rename = "InfrahubReadOnlyRepositoryImportLastCommit")]
    pub infrahub_read_only_repository_import_last_commit: Option<Box<ReadOnlyRepositoryImportLastCommit>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubRepositoryProcessResponse {
    #[serde(rename = "InfrahubRepositoryProcess")]
    pub infrahub_repository_process: Option<Box<ProcessRepository>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubRepositoryConnectivityResponse {
    #[serde(rename = "InfrahubRepositoryConnectivity")]
    pub infrahub_repository_connectivity: Option<Box<ValidateRepositoryConnectivity>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubUpdateComputedAttributeResponse {
    #[serde(rename = "InfrahubUpdateComputedAttribute")]
    pub infrahub_update_computed_attribute: Option<Box<UpdateComputedAttribute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubUpdateDisplayLabelResponse {
    #[serde(rename = "InfrahubUpdateDisplayLabel")]
    pub infrahub_update_display_label: Option<Box<UpdateDisplayLabel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubUpdateHFIDResponse {
    #[serde(rename = "InfrahubUpdateHFID")]
    pub infrahub_update_hfid: Option<Box<UpdateHFID>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubRecomputeComputedAttributeResponse {
    #[serde(rename = "InfrahubRecomputeComputedAttribute")]
    pub infrahub_recompute_computed_attribute: Option<Box<RecomputeComputedAttribute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipAddResponse {
    #[serde(rename = "RelationshipAdd")]
    pub relationship_add: Option<Box<RelationshipAdd>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRemoveResponse {
    #[serde(rename = "RelationshipRemove")]
    pub relationship_remove: Option<Box<RelationshipRemove>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDropdownAddResponse {
    #[serde(rename = "SchemaDropdownAdd")]
    pub schema_dropdown_add: Option<Box<SchemaDropdownAdd>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDropdownRemoveResponse {
    #[serde(rename = "SchemaDropdownRemove")]
    pub schema_dropdown_remove: Option<Box<SchemaDropdownRemove>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaEnumAddResponse {
    #[serde(rename = "SchemaEnumAdd")]
    pub schema_enum_add: Option<Box<SchemaEnumAdd>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaEnumRemoveResponse {
    #[serde(rename = "SchemaEnumRemove")]
    pub schema_enum_remove: Option<Box<SchemaEnumRemove>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveDiffConflictResponse {
    #[serde(rename = "ResolveDiffConflict")]
    pub resolve_diff_conflict: Option<Box<ResolveDiffConflict>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertObjectTypeResponse {
    #[serde(rename = "ConvertObjectType")]
    pub convert_object_type: Option<Box<ConvertObjectType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeCheckForApprovalRevokeResponse {
    #[serde(rename = "CoreProposedChangeCheckForApprovalRevoke")]
    pub core_proposed_change_check_for_approval_revoke: Option<Box<ProposedChangeCheckForApprovalRevoke>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubProfilesRefreshResponse {
    #[serde(rename = "InfrahubProfilesRefresh")]
    pub infrahub_profiles_refresh: Option<Box<InfrahubProfilesRefresh>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubSetPreferencesResponse {
    #[serde(rename = "InfrahubSetPreferences")]
    pub infrahub_set_preferences: Option<Box<InfrahubSetPreferences>>,
}

