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
    pub core_graph_q_l_query_group: Box<PaginatedCoreGraphQLQueryGroup>,
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
    pub core_graph_q_l_query: Box<PaginatedCoreGraphQLQuery>,
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
    pub core_i_p_prefix_pool: Box<PaginatedCoreIPPrefixPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolResponse {
    #[serde(rename = "CoreIPAddressPool")]
    pub core_i_p_address_pool: Box<PaginatedCoreIPAddressPool>,
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
    pub builtin_i_p_namespace: Box<PaginatedBuiltinIPNamespace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPPrefixResponse {
    #[serde(rename = "BuiltinIPPrefix")]
    pub builtin_i_p_prefix: Box<PaginatedBuiltinIPPrefix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPAddressResponse {
    #[serde(rename = "BuiltinIPAddress")]
    pub builtin_i_p_address: Box<PaginatedBuiltinIPAddress>,
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
    pub profile_builtin_i_p_prefix: Box<PaginatedProfileBuiltinIPPrefix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressResponse {
    #[serde(rename = "ProfileBuiltinIPAddress")]
    pub profile_builtin_i_p_address: Box<PaginatedProfileBuiltinIPAddress>,
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
    pub infrahub_graph_q_l_query_report: Box<GraphQLQueryReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubInfoResponse {
    #[serde(rename = "InfrahubInfo")]
    pub infrahub_info: Box<Info>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubIPAddressGetNextAvailableResponse {
    #[serde(rename = "InfrahubIPAddressGetNextAvailable")]
    pub infrahub_i_p_address_get_next_available: Box<IPAddressGetNextAvailable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubIPPrefixGetNextAvailableResponse {
    #[serde(rename = "InfrahubIPPrefixGetNextAvailable")]
    pub infrahub_i_p_prefix_get_next_available: Box<IPPrefixGetNextAvailable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeAvailableActionsResponse {
    #[serde(rename = "CoreProposedChangeAvailableActions")]
    pub core_proposed_change_available_actions: Box<AvailableActions>,
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

