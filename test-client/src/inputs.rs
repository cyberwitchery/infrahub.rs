//! generated input types

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCreateInput {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub origin_branch: Option<String>,
    pub branched_from: Option<String>,
    pub sync_with_git: Option<bool>,
    pub is_isolated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchDeleteInput {
    pub name: Option<String>,
    pub delete_from_git: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchEventTypeFilter {
    pub branches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchNameInput {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchUpdateInput {
    pub name: String,
    pub description: Option<String>,
    pub is_isolated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPAddressUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub address: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPNamespaceUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPPrefixUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub member_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub is_pool: Option<CheckboxAttributeUpdate>,
    pub prefix: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxAttributeCreate {
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxAttributeUpdate {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAccountInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextInput {
    pub account: Option<ContextAccountInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertObjectTypeInput {
    pub node_id: String,
    pub target_kind: String,
    pub fields_mapping: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountCreateInput {
    pub id: Option<String>,
    pub password: Option<TextAttributeCreate>,
    pub status: Option<TextAttributeCreate>,
    pub account_type: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub group_type: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub groups: Option<Vec<RelatedNodeInput>>,
    pub permissions: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub groups: Option<Vec<RelatedNodeInput>>,
    pub permissions: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub groups: Option<Vec<RelatedNodeInput>>,
    pub permissions: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub password: Option<TextAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub account_type: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub password: Option<TextAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub account_type: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreActionUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub triggers: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckCreateInput {
    pub id: Option<String>,
    pub storage_id: Option<TextAttributeCreate>,
    pub checksum: Option<TextAttributeCreate>,
    pub changed: Option<CheckboxAttributeCreate>,
    pub artifact_id: Option<TextAttributeCreate>,
    pub line_number: Option<NumberAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub created_at: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub origin: Option<TextAttributeCreate>,
    pub severity: Option<TextAttributeCreate>,
    pub kind: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub message: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub storage_id: Option<TextAttributeUpdate>,
    pub checksum: Option<TextAttributeUpdate>,
    pub changed: Option<CheckboxAttributeUpdate>,
    pub artifact_id: Option<TextAttributeUpdate>,
    pub line_number: Option<NumberAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub storage_id: Option<TextAttributeUpdate>,
    pub checksum: Option<TextAttributeUpdate>,
    pub changed: Option<CheckboxAttributeUpdate>,
    pub artifact_id: Option<TextAttributeUpdate>,
    pub line_number: Option<NumberAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCreateInput {
    pub id: Option<String>,
    pub status: Option<TextAttributeCreate>,
    pub content_type: Option<TextAttributeCreate>,
    pub checksum: Option<TextAttributeCreate>,
    pub storage_id: Option<TextAttributeCreate>,
    pub parameters: Option<JSONAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub object: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub definition: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionCreateInput {
    pub id: Option<String>,
    pub content_type: Option<TextAttributeCreate>,
    pub parameters: Option<JSONAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub artifact_name: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub targets: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub transformation: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub content_type: Option<TextAttributeUpdate>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub artifact_name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub targets: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub transformation: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub content_type: Option<TextAttributeUpdate>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub artifact_name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub targets: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub transformation: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactTargetUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub artifacts: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadCreateInput {
    pub id: Option<String>,
    pub line_number: Option<NumberAttributeCreate>,
    pub storage_id: Option<TextAttributeCreate>,
    pub artifact_id: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub resolved: Option<CheckboxAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub line_number: Option<NumberAttributeUpdate>,
    pub storage_id: Option<TextAttributeUpdate>,
    pub artifact_id: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub resolved: Option<CheckboxAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub line_number: Option<NumberAttributeUpdate>,
    pub storage_id: Option<TextAttributeUpdate>,
    pub artifact_id: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub resolved: Option<CheckboxAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub status: Option<TextAttributeUpdate>,
    pub content_type: Option<TextAttributeUpdate>,
    pub checksum: Option<TextAttributeUpdate>,
    pub storage_id: Option<TextAttributeUpdate>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub object: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub definition: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub status: Option<TextAttributeUpdate>,
    pub content_type: Option<TextAttributeUpdate>,
    pub checksum: Option<TextAttributeUpdate>,
    pub storage_id: Option<TextAttributeUpdate>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub object: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub definition: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorCreateInput {
    pub id: Option<String>,
    pub started_at: Option<TextAttributeCreate>,
    pub state: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub completed_at: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub definition: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub definition: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub definition: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreBasePermissionUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentCreateInput {
    pub id: Option<String>,
    pub text: Option<TextAttributeCreate>,
    pub change: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub text: Option<TextAttributeUpdate>,
    pub change: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub text: Option<TextAttributeUpdate>,
    pub change: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadCreateInput {
    pub id: Option<String>,
    pub label: Option<TextAttributeCreate>,
    pub resolved: Option<CheckboxAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub resolved: Option<CheckboxAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub resolved: Option<CheckboxAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub class_name: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub timeout: Option<NumberAttributeCreate>,
    pub parameters: Option<JSONAttributeCreate>,
    pub file_path: Option<TextAttributeCreate>,
    pub targets: Option<RelatedNodeInput>,
    pub query: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub class_name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub timeout: Option<NumberAttributeUpdate>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub file_path: Option<TextAttributeUpdate>,
    pub targets: Option<RelatedNodeInput>,
    pub query: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub class_name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub timeout: Option<NumberAttributeUpdate>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub file_path: Option<TextAttributeUpdate>,
    pub targets: Option<RelatedNodeInput>,
    pub query: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCommentUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub text: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCredentialUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookCreateInput {
    pub id: Option<String>,
    pub shared_key: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub active: Option<CheckboxAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub validate_certificates: Option<CheckboxAttributeCreate>,
    pub event_type: Option<TextAttributeCreate>,
    pub node_kind: Option<TextAttributeCreate>,
    pub branch_scope: Option<TextAttributeCreate>,
    pub url: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub transformation: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub headers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub shared_key: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    pub event_type: Option<TextAttributeUpdate>,
    pub node_kind: Option<TextAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub url: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub transformation: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub headers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub shared_key: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    pub event_type: Option<TextAttributeUpdate>,
    pub node_kind: Option<TextAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub url: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub transformation: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub headers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckCreateInput {
    pub id: Option<String>,
    pub enriched_conflict_id: Option<TextAttributeCreate>,
    pub keep_branch: Option<TextAttributeCreate>,
    pub conflicts: Option<JSONAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub created_at: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub origin: Option<TextAttributeCreate>,
    pub severity: Option<TextAttributeCreate>,
    pub kind: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub message: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub enriched_conflict_id: Option<TextAttributeUpdate>,
    pub keep_branch: Option<TextAttributeUpdate>,
    pub conflicts: Option<JSONAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub enriched_conflict_id: Option<TextAttributeUpdate>,
    pub keep_branch: Option<TextAttributeUpdate>,
    pub conflicts: Option<JSONAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorCreateInput {
    pub id: Option<String>,
    pub started_at: Option<TextAttributeCreate>,
    pub state: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub completed_at: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueCreateInput {
    pub id: Option<String>,
    pub description: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub value: Option<TextAttributeCreate>,
    pub key: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub value: Option<TextAttributeUpdate>,
    pub key: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub value: Option<TextAttributeUpdate>,
    pub key: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckCreateInput {
    pub id: Option<String>,
    pub commit: Option<TextAttributeCreate>,
    pub files: Option<ListAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub created_at: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub origin: Option<TextAttributeCreate>,
    pub severity: Option<TextAttributeCreate>,
    pub kind: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub message: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub commit: Option<TextAttributeUpdate>,
    pub files: Option<ListAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub commit: Option<TextAttributeUpdate>,
    pub files: Option<ListAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileObjectUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadCreateInput {
    pub id: Option<String>,
    pub commit: Option<TextAttributeCreate>,
    pub line_number: Option<NumberAttributeCreate>,
    pub file: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub resolved: Option<CheckboxAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub commit: Option<TextAttributeUpdate>,
    pub line_number: Option<NumberAttributeUpdate>,
    pub file: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub resolved: Option<CheckboxAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub commit: Option<TextAttributeUpdate>,
    pub line_number: Option<NumberAttributeUpdate>,
    pub file: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub resolved: Option<CheckboxAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionCreateInput {
    pub id: Option<String>,
    pub description: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub generator: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub triggers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub generator: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub triggers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub generator: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub triggers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub group_type: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckCreateInput {
    pub id: Option<String>,
    pub instance: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub created_at: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub origin: Option<TextAttributeCreate>,
    pub severity: Option<TextAttributeCreate>,
    pub kind: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub message: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub instance: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub instance: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionCreateInput {
    pub id: Option<String>,
    pub parameters: Option<JSONAttributeCreate>,
    pub convert_query_response: Option<CheckboxAttributeCreate>,
    pub execute_after_merge: Option<CheckboxAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub file_path: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub class_name: Option<TextAttributeCreate>,
    pub execute_in_proposed_change: Option<CheckboxAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub targets: Option<RelatedNodeInput>,
    pub query: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub convert_query_response: Option<CheckboxAttributeUpdate>,
    pub execute_after_merge: Option<CheckboxAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub file_path: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub class_name: Option<TextAttributeUpdate>,
    pub execute_in_proposed_change: Option<CheckboxAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub targets: Option<RelatedNodeInput>,
    pub query: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub convert_query_response: Option<CheckboxAttributeUpdate>,
    pub execute_after_merge: Option<CheckboxAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub file_path: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub class_name: Option<TextAttributeUpdate>,
    pub execute_in_proposed_change: Option<CheckboxAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub targets: Option<RelatedNodeInput>,
    pub query: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub group_type: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceCreateInput {
    pub id: Option<String>,
    pub status: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub definition: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub object: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub status: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub definition: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub object: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub status: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub definition: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub object: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorCreateInput {
    pub id: Option<String>,
    pub started_at: Option<TextAttributeCreate>,
    pub state: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub completed_at: Option<TextAttributeCreate>,
    pub definition: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub definition: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub definition: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericAccountUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub password: Option<TextAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub account_type: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericRepositoryUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub sync_status: Option<TextAttributeUpdate>,
    pub internal_status: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub location: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub operational_status: Option<TextAttributeUpdate>,
    pub generators: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub credential: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub transformations: Option<Vec<RelatedNodeInput>>,
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub queries: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionCreateInput {
    pub id: Option<String>,
    pub decision: Option<NumberAttributeCreate>,
    pub action: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub decision: Option<NumberAttributeUpdate>,
    pub action: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub decision: Option<NumberAttributeUpdate>,
    pub action: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub query: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub tags: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupCreateInput {
    pub id: Option<String>,
    pub parameters: Option<JSONAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub group_type: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub query: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub query: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub parameters: Option<JSONAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub query: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub query: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub tags: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub query: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub tags: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionCreateInput {
    pub id: Option<String>,
    pub member_action: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub group: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub triggers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub member_action: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub group: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub triggers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub member_action: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub group: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub triggers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleCreateInput {
    pub id: Option<String>,
    pub member_update: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub active: Option<CheckboxAttributeCreate>,
    pub branch_scope: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub group: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub action: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub member_update: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub group: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub action: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub member_update: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub group: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub action: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolCreateInput {
    pub id: Option<String>,
    pub default_prefix_length: Option<NumberAttributeCreate>,
    pub default_address_type: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub default_prefix_length: Option<NumberAttributeUpdate>,
    pub default_address_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub default_prefix_length: Option<NumberAttributeUpdate>,
    pub default_address_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolCreateInput {
    pub id: Option<String>,
    pub default_member_type: Option<TextAttributeCreate>,
    pub default_prefix_type: Option<TextAttributeCreate>,
    pub default_prefix_length: Option<NumberAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub default_member_type: Option<TextAttributeUpdate>,
    pub default_prefix_type: Option<TextAttributeUpdate>,
    pub default_prefix_length: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub default_member_type: Option<TextAttributeUpdate>,
    pub default_prefix_type: Option<TextAttributeUpdate>,
    pub default_prefix_length: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreKeyValueUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub value: Option<TextAttributeUpdate>,
    pub key: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemCreateInput {
    pub id: Option<String>,
    pub icon: Option<TextAttributeCreate>,
    pub namespace: Option<TextAttributeCreate>,
    pub kind: Option<TextAttributeCreate>,
    pub required_permissions: Option<ListAttributeCreate>,
    pub section: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub path: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub order_weight: Option<NumberAttributeCreate>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub icon: Option<TextAttributeUpdate>,
    pub namespace: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub required_permissions: Option<ListAttributeUpdate>,
    pub section: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub path: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub order_weight: Option<NumberAttributeUpdate>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub icon: Option<TextAttributeUpdate>,
    pub namespace: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub required_permissions: Option<ListAttributeUpdate>,
    pub section: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub path: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub order_weight: Option<NumberAttributeUpdate>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub icon: Option<TextAttributeUpdate>,
    pub namespace: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub required_permissions: Option<ListAttributeUpdate>,
    pub section: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub path: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub order_weight: Option<NumberAttributeUpdate>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchCreateInput {
    pub id: Option<String>,
    pub value_match: Option<TextAttributeCreate>,
    pub attribute_name: Option<TextAttributeCreate>,
    pub value: Option<TextAttributeCreate>,
    pub value_previous: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub trigger: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub value_match: Option<TextAttributeUpdate>,
    pub attribute_name: Option<TextAttributeUpdate>,
    pub value: Option<TextAttributeUpdate>,
    pub value_previous: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub trigger: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub value_match: Option<TextAttributeUpdate>,
    pub attribute_name: Option<TextAttributeUpdate>,
    pub value: Option<TextAttributeUpdate>,
    pub value_previous: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub trigger: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerMatchUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub trigger: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchCreateInput {
    pub id: Option<String>,
    pub peer: Option<TextAttributeCreate>,
    pub relationship_name: Option<TextAttributeCreate>,
    pub modification_type: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub trigger: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub peer: Option<TextAttributeUpdate>,
    pub relationship_name: Option<TextAttributeUpdate>,
    pub modification_type: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub trigger: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub peer: Option<TextAttributeUpdate>,
    pub relationship_name: Option<TextAttributeUpdate>,
    pub modification_type: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub trigger: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleCreateInput {
    pub id: Option<String>,
    pub mutation_action: Option<TextAttributeCreate>,
    pub node_kind: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub active: Option<CheckboxAttributeCreate>,
    pub branch_scope: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub matches: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub action: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub mutation_action: Option<TextAttributeUpdate>,
    pub node_kind: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub matches: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub action: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub mutation_action: Option<TextAttributeUpdate>,
    pub node_kind: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub matches: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub action: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolCreateInput {
    pub id: Option<String>,
    pub node: Option<TextAttributeCreate>,
    pub start_range: Option<NumberAttributeCreate>,
    pub node_attribute: Option<TextAttributeCreate>,
    pub end_range: Option<NumberAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub node: Option<TextAttributeUpdate>,
    pub start_range: Option<NumberAttributeUpdate>,
    pub node_attribute: Option<TextAttributeUpdate>,
    pub end_range: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub node: Option<TextAttributeUpdate>,
    pub start_range: Option<NumberAttributeUpdate>,
    pub node_attribute: Option<TextAttributeUpdate>,
    pub end_range: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectComponentTemplateUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub template_name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionCreateInput {
    pub id: Option<String>,
    pub action: Option<TextAttributeCreate>,
    pub decision: Option<NumberAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub namespace: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub action: Option<TextAttributeUpdate>,
    pub decision: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub namespace: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub action: Option<TextAttributeUpdate>,
    pub decision: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub namespace: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub roles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectTemplateUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub template_name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadCreateInput {
    pub id: Option<String>,
    pub object_path: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub resolved: Option<CheckboxAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub object_path: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub resolved: Option<CheckboxAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub object_path: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub resolved: Option<CheckboxAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialCreateInput {
    pub id: Option<String>,
    pub username: Option<TextAttributeCreate>,
    pub password: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub username: Option<TextAttributeUpdate>,
    pub password: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub username: Option<TextAttributeUpdate>,
    pub password: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProfileUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeCreateInput {
    pub id: Option<String>,
    pub destination_branch: Option<TextAttributeCreate>,
    pub is_draft: Option<CheckboxAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub source_branch: Option<TextAttributeCreate>,
    pub state: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub threads: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub reviewers: Option<Vec<RelatedNodeInput>>,
    pub validations: Option<Vec<RelatedNodeInput>>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub destination_branch: Option<TextAttributeUpdate>,
    pub is_draft: Option<CheckboxAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub source_branch: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub threads: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub reviewers: Option<Vec<RelatedNodeInput>>,
    pub validations: Option<Vec<RelatedNodeInput>>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub destination_branch: Option<TextAttributeUpdate>,
    pub is_draft: Option<CheckboxAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub source_branch: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub threads: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub reviewers: Option<Vec<RelatedNodeInput>>,
    pub validations: Option<Vec<RelatedNodeInput>>,
    pub comments: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryCreateInput {
    pub id: Option<String>,
    pub commit: Option<TextAttributeCreate>,
    #[serde(rename = "ref")]
    pub r#ref: Option<TextAttributeCreate>,
    pub sync_status: Option<TextAttributeCreate>,
    pub internal_status: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub location: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub operational_status: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub generators: Option<Vec<RelatedNodeInput>>,
    pub credential: Option<RelatedNodeInput>,
    pub transformations: Option<Vec<RelatedNodeInput>>,
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub queries: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub commit: Option<TextAttributeUpdate>,
    #[serde(rename = "ref")]
    pub r#ref: Option<TextAttributeUpdate>,
    pub sync_status: Option<TextAttributeUpdate>,
    pub internal_status: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub location: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub operational_status: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub generators: Option<Vec<RelatedNodeInput>>,
    pub credential: Option<RelatedNodeInput>,
    pub transformations: Option<Vec<RelatedNodeInput>>,
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub queries: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub commit: Option<TextAttributeUpdate>,
    #[serde(rename = "ref")]
    pub r#ref: Option<TextAttributeUpdate>,
    pub sync_status: Option<TextAttributeUpdate>,
    pub internal_status: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub location: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub operational_status: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub generators: Option<Vec<RelatedNodeInput>>,
    pub credential: Option<RelatedNodeInput>,
    pub transformations: Option<Vec<RelatedNodeInput>>,
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub queries: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryCreateInput {
    pub id: Option<String>,
    pub commit: Option<TextAttributeCreate>,
    pub default_branch: Option<TextAttributeCreate>,
    pub sync_status: Option<TextAttributeCreate>,
    pub internal_status: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub location: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub operational_status: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub generators: Option<Vec<RelatedNodeInput>>,
    pub credential: Option<RelatedNodeInput>,
    pub transformations: Option<Vec<RelatedNodeInput>>,
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub queries: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupCreateInput {
    pub id: Option<String>,
    pub content: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub group_type: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub repository: Option<RelatedNodeInput>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub content: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub repository: Option<RelatedNodeInput>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub content: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub repository: Option<RelatedNodeInput>,
    pub parent: Option<RelatedNodeInput>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub commit: Option<TextAttributeUpdate>,
    pub default_branch: Option<TextAttributeUpdate>,
    pub sync_status: Option<TextAttributeUpdate>,
    pub internal_status: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub location: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub operational_status: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub generators: Option<Vec<RelatedNodeInput>>,
    pub credential: Option<RelatedNodeInput>,
    pub transformations: Option<Vec<RelatedNodeInput>>,
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub queries: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub commit: Option<TextAttributeUpdate>,
    pub default_branch: Option<TextAttributeUpdate>,
    pub sync_status: Option<TextAttributeUpdate>,
    pub internal_status: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub location: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub operational_status: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub generators: Option<Vec<RelatedNodeInput>>,
    pub credential: Option<RelatedNodeInput>,
    pub transformations: Option<Vec<RelatedNodeInput>>,
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub queries: Option<Vec<RelatedNodeInput>>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorCreateInput {
    pub id: Option<String>,
    pub started_at: Option<TextAttributeCreate>,
    pub state: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub completed_at: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub repository: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreResourcePoolUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckCreateInput {
    pub id: Option<String>,
    pub enriched_conflict_id: Option<TextAttributeCreate>,
    pub conflicts: Option<JSONAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub created_at: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub origin: Option<TextAttributeCreate>,
    pub severity: Option<TextAttributeCreate>,
    pub kind: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub message: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub enriched_conflict_id: Option<TextAttributeUpdate>,
    pub conflicts: Option<JSONAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub enriched_conflict_id: Option<TextAttributeUpdate>,
    pub conflicts: Option<JSONAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorCreateInput {
    pub id: Option<String>,
    pub started_at: Option<TextAttributeCreate>,
    pub state: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub completed_at: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckCreateInput {
    pub id: Option<String>,
    pub label: Option<TextAttributeCreate>,
    pub created_at: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub origin: Option<TextAttributeCreate>,
    pub severity: Option<TextAttributeCreate>,
    pub kind: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub message: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub created_at: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub origin: Option<TextAttributeUpdate>,
    pub severity: Option<TextAttributeUpdate>,
    pub kind: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub message: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub validator: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub group_type: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub group_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub children: Option<Vec<RelatedNodeInput>>,
    pub parent: Option<RelatedNodeInput>,
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    pub members: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookCreateInput {
    pub id: Option<String>,
    pub shared_key: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub active: Option<CheckboxAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub validate_certificates: Option<CheckboxAttributeCreate>,
    pub event_type: Option<TextAttributeCreate>,
    pub node_kind: Option<TextAttributeCreate>,
    pub branch_scope: Option<TextAttributeCreate>,
    pub url: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub headers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub shared_key: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    pub event_type: Option<TextAttributeUpdate>,
    pub node_kind: Option<TextAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub url: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub headers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub shared_key: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    pub event_type: Option<TextAttributeUpdate>,
    pub node_kind: Option<TextAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub url: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub headers: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueCreateInput {
    pub id: Option<String>,
    pub description: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub value: Option<TextAttributeCreate>,
    pub key: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub value: Option<TextAttributeUpdate>,
    pub key: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub value: Option<TextAttributeUpdate>,
    pub key: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTaskTargetUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentCreateInput {
    pub id: Option<String>,
    pub text: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub thread: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub text: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub thread: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub text: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub thread: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub resolved: Option<CheckboxAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub change: Option<RelatedNodeInput>,
    pub comments: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2CreateInput {
    pub id: Option<String>,
    pub template_path: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub timeout: Option<NumberAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub query: Option<RelatedNodeInput>,
    pub repository: Option<RelatedNodeInput>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2UpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub template_path: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub timeout: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub query: Option<RelatedNodeInput>,
    pub repository: Option<RelatedNodeInput>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2UpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub template_path: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub timeout: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub query: Option<RelatedNodeInput>,
    pub repository: Option<RelatedNodeInput>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonCreateInput {
    pub id: Option<String>,
    pub convert_query_response: Option<CheckboxAttributeCreate>,
    pub file_path: Option<TextAttributeCreate>,
    pub class_name: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub timeout: Option<NumberAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub query: Option<RelatedNodeInput>,
    pub repository: Option<RelatedNodeInput>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub convert_query_response: Option<CheckboxAttributeUpdate>,
    pub file_path: Option<TextAttributeUpdate>,
    pub class_name: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub timeout: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub query: Option<RelatedNodeInput>,
    pub repository: Option<RelatedNodeInput>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub convert_query_response: Option<CheckboxAttributeUpdate>,
    pub file_path: Option<TextAttributeUpdate>,
    pub class_name: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub timeout: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub query: Option<RelatedNodeInput>,
    pub repository: Option<RelatedNodeInput>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformationUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub timeout: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub query: Option<RelatedNodeInput>,
    pub repository: Option<RelatedNodeInput>,
    pub tags: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTriggerRuleUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub description: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub action: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorCreateInput {
    pub id: Option<String>,
    pub started_at: Option<TextAttributeCreate>,
    pub state: Option<TextAttributeCreate>,
    pub conclusion: Option<TextAttributeCreate>,
    pub label: Option<TextAttributeCreate>,
    pub completed_at: Option<TextAttributeCreate>,
    pub repository: Option<RelatedNodeInput>,
    pub check_definition: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub repository: Option<RelatedNodeInput>,
    pub check_definition: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub repository: Option<RelatedNodeInput>,
    pub check_definition: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreValidatorUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub started_at: Option<TextAttributeUpdate>,
    pub state: Option<TextAttributeUpdate>,
    pub conclusion: Option<TextAttributeUpdate>,
    pub label: Option<TextAttributeUpdate>,
    pub completed_at: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub checks: Option<Vec<RelatedNodeInput>>,
    pub proposed_change: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWebhookUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub active: Option<CheckboxAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    pub event_type: Option<TextAttributeUpdate>,
    pub node_kind: Option<TextAttributeUpdate>,
    pub branch_scope: Option<TextAttributeUpdate>,
    pub url: Option<TextAttributeUpdate>,
    pub headers: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWeightedPoolResourceUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub allocation_weight: Option<NumberAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimCableCreateInput {
    pub id: Option<String>,
    pub label: Option<TextAttributeCreate>,
    pub b_terminations: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub a_terminations: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimCableUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub b_terminations: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub a_terminations: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimCableUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub label: Option<TextAttributeUpdate>,
    pub b_terminations: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub a_terminations: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimDeviceCreateInput {
    pub id: Option<String>,
    pub status: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub role: Option<RelatedNodeInput>,
    pub platform: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub site: Option<RelatedNodeInput>,
    pub primary_ip4: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub device_type: Option<RelatedNodeInput>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimDeviceRoleCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub slug: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimDeviceRoleUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub slug: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimDeviceRoleUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub slug: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimDeviceTypeCreateInput {
    pub id: Option<String>,
    pub model: Option<TextAttributeCreate>,
    pub slug: Option<TextAttributeCreate>,
    pub manufacturer: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimDeviceTypeUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub model: Option<TextAttributeUpdate>,
    pub slug: Option<TextAttributeUpdate>,
    pub manufacturer: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimDeviceTypeUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub model: Option<TextAttributeUpdate>,
    pub slug: Option<TextAttributeUpdate>,
    pub manufacturer: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimDeviceUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub status: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub role: Option<RelatedNodeInput>,
    pub platform: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub site: Option<RelatedNodeInput>,
    pub primary_ip4: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub device_type: Option<RelatedNodeInput>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimDeviceUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub status: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub role: Option<RelatedNodeInput>,
    pub platform: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub site: Option<RelatedNodeInput>,
    pub primary_ip4: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub device_type: Option<RelatedNodeInput>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimInterfaceCreateInput {
    pub id: Option<String>,
    pub if_type: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub enabled: Option<CheckboxAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub device: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimInterfaceUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub if_type: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub enabled: Option<CheckboxAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub device: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimInterfaceUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub if_type: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub enabled: Option<CheckboxAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub device: Option<RelatedNodeInput>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimManufacturerCreateInput {
    pub id: Option<String>,
    pub slug: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimManufacturerUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub slug: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimManufacturerUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub slug: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimPlatformCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub slug: Option<TextAttributeCreate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimPlatformUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub slug: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimPlatformUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub slug: Option<TextAttributeUpdate>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimSiteCreateInput {
    pub id: Option<String>,
    pub slug: Option<TextAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub status: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimSiteUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub slug: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcimSiteUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub slug: Option<TextAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffTreeQueryFilters {
    pub ids: Option<Vec<String>>,
    pub status: Option<IncExclFilterStatusOptions>,
    pub kind: Option<IncExclFilterOptions>,
    pub namespace: Option<IncExclFilterOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffUpdateInput {
    pub branch: String,
    pub name: Option<String>,
    pub from_time: Option<String>,
    pub to_time: Option<String>,
    pub wait_for_completion: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTypeFilter {
    pub branch_merged: Option<BranchEventTypeFilter>,
    pub branch_rebased: Option<BranchEventTypeFilter>,
    pub group_auto_create: Option<GroupAutoCreateEventTypeFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorDefinitionRequestRunInput {
    pub id: Option<String>,
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericPoolInput {
    pub id: String,
    pub identifier: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAutoCreateEventTypeFilter {
    pub idp: Option<Vec<String>>,
    pub protocol: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPAddressPoolGetResourceInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub identifier: Option<String>,
    pub prefix_length: Option<i64>,
    pub address_type: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPAddressPoolInput {
    pub id: String,
    pub identifier: Option<String>,
    pub data: Option<serde_json::Value>,
    pub prefixlen: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPPrefixPoolGetResourceInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub identifier: Option<String>,
    pub prefix_length: Option<i64>,
    pub member_type: Option<String>,
    pub prefix_type: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPPrefixPoolInput {
    pub id: String,
    pub identifier: Option<String>,
    pub data: Option<serde_json::Value>,
    pub size: Option<i64>,
    pub member_type: Option<String>,
    pub prefix_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncExclFilterOptions {
    pub includes: Option<Vec<String>>,
    pub excludes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncExclFilterStatusOptions {
    pub includes: Option<Vec<DiffAction>>,
    pub excludes: Option<Vec<DiffAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenCreateInput {
    pub name: Option<String>,
    pub expiration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenDeleteInput {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountUpdateSelfInput {
    pub password: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubComputedAttributeRecomputeInput {
    pub kind: String,
    pub attribute: String,
    pub node_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubComputedAttributeUpdateInput {
    pub id: String,
    pub kind: String,
    pub attribute: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubDisplayLabelUpdateInput {
    pub id: String,
    pub kind: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubHFIDUpdateInput {
    pub id: String,
    pub kind: String,
    pub value: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubNodeMetadataOrder {
    pub created_at: Option<OrderDirection>,
    pub updated_at: Option<OrderDirection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamIpAddressCreateInput {
    pub id: Option<String>,
    pub address: Option<TextAttributeCreate>,
    pub status: Option<TextAttributeCreate>,
    pub assigned_object: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamIpAddressUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub address: Option<TextAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub assigned_object: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamIpAddressUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub address: Option<TextAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub assigned_object: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceCreateInput {
    pub id: Option<String>,
    pub name: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub name: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub profiles: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONAttributeCreate {
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONAttributeUpdate {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAttributeCreate {
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAttributeUpdate {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataOrderInput {
    pub node_metadata: Option<InfrahubNodeMetadataOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberAttributeCreate {
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<i64>,
    pub from_pool: Option<GenericPoolInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberAttributeUpdate {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<i64>,
    pub from_pool: Option<GenericPoolInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderByItem {
    pub field: String,
    pub direction: Option<OrderDirection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInput {
    pub disable: Option<bool>,
    pub node_metadata: Option<InfrahubNodeMetadataOrder>,
    pub by: Option<Vec<OrderByItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathTraversalInput {
    pub source_id: String,
    pub destination_id: String,
    pub max_depth: Option<i64>,
    pub max_paths: Option<i64>,
    pub kind_filter: Option<Vec<String>>,
    pub relationship_filter: Option<Vec<String>>,
    pub excluded_namespaces: Option<Vec<String>>,
    pub excluded_kinds: Option<Vec<String>>,
    pub included_kinds: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub address: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub address: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub address: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub member_type: Option<TextAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub is_pool: Option<CheckboxAttributeCreate>,
    pub prefix: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub member_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub is_pool: Option<CheckboxAttributeUpdate>,
    pub prefix: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub member_type: Option<TextAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub is_pool: Option<CheckboxAttributeUpdate>,
    pub prefix: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub ip_namespace: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimCableCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub b_terminations: Option<Vec<RelatedNodeInput>>,
    pub a_terminations: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimCableUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub b_terminations: Option<Vec<RelatedNodeInput>>,
    pub a_terminations: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimCableUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub b_terminations: Option<Vec<RelatedNodeInput>>,
    pub a_terminations: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimDeviceCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub status: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub role: Option<RelatedNodeInput>,
    pub platform: Option<RelatedNodeInput>,
    pub site: Option<RelatedNodeInput>,
    pub primary_ip4: Option<RelatedNodeInput>,
    pub device_type: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimDeviceRoleCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimDeviceRoleUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimDeviceRoleUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimDeviceTypeCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub model: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub manufacturer: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimDeviceTypeUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub model: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub manufacturer: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimDeviceTypeUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub model: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub manufacturer: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimDeviceUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub role: Option<RelatedNodeInput>,
    pub platform: Option<RelatedNodeInput>,
    pub site: Option<RelatedNodeInput>,
    pub primary_ip4: Option<RelatedNodeInput>,
    pub device_type: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimDeviceUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub role: Option<RelatedNodeInput>,
    pub platform: Option<RelatedNodeInput>,
    pub site: Option<RelatedNodeInput>,
    pub primary_ip4: Option<RelatedNodeInput>,
    pub device_type: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimInterfaceCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub if_type: Option<TextAttributeCreate>,
    pub enabled: Option<CheckboxAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimInterfaceUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub if_type: Option<TextAttributeUpdate>,
    pub enabled: Option<CheckboxAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimInterfaceUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub if_type: Option<TextAttributeUpdate>,
    pub enabled: Option<CheckboxAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimManufacturerCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimManufacturerUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimManufacturerUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimPlatformCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimPlatformUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimPlatformUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimSiteCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub name: Option<TextAttributeCreate>,
    pub status: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimSiteUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDcimSiteUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub name: Option<TextAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamIpAddressCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub status: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub assigned_object: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamIpAddressUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub assigned_object: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamIpAddressUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub status: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub assigned_object: Option<RelatedNodeInput>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceCreateInput {
    pub id: Option<String>,
    pub profile_name: Option<TextAttributeCreate>,
    pub profile_priority: Option<NumberAttributeCreate>,
    pub description: Option<TextAttributeCreate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceUpdateInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceUpsertInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub profile_name: Option<TextAttributeUpdate>,
    pub profile_priority: Option<NumberAttributeUpdate>,
    pub description: Option<TextAttributeUpdate>,
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilesRefreshInput {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeCheckForApprovalRevokeInput {
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeMergeInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeRequestRunCheckInput {
    pub id: String,
    pub check_type: Option<CheckType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeReviewInput {
    pub id: String,
    pub decision: ProposedChangeApprovalDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReachableNodesInput {
    pub source_id: String,
    pub target_kinds: Vec<String>,
    pub max_depth: Option<i64>,
    pub max_results: Option<i64>,
    pub max_paths: Option<i64>,
    pub shortest_paths_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedIPAddressNodeInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub from_pool: Option<IPAddressPoolInput>,
    #[serde(rename = "_relation__is_protected")]
    pub _relation_is_protected: Option<bool>,
    #[serde(rename = "_relation__owner")]
    pub _relation_owner: Option<String>,
    #[serde(rename = "_relation__source")]
    pub _relation_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedIPPrefixNodeInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub from_pool: Option<IPPrefixPoolInput>,
    #[serde(rename = "_relation__is_protected")]
    pub _relation_is_protected: Option<bool>,
    #[serde(rename = "_relation__owner")]
    pub _relation_owner: Option<String>,
    #[serde(rename = "_relation__source")]
    pub _relation_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedNodeInput {
    pub id: Option<String>,
    pub hfid: Option<Vec<String>>,
    pub kind: Option<String>,
    pub from_pool: Option<GenericPoolInput>,
    #[serde(rename = "_relation__is_protected")]
    pub _relation_is_protected: Option<bool>,
    #[serde(rename = "_relation__owner")]
    pub _relation_owner: Option<String>,
    #[serde(rename = "_relation__source")]
    pub _relation_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipNodesInput {
    pub id: Option<String>,
    pub name: Option<String>,
    pub nodes: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveDiffConflictInput {
    pub conflict_id: Option<String>,
    pub selected_branch: Option<ConflictSelection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDropdownAddInput {
    pub kind: String,
    pub attribute: String,
    pub dropdown: String,
    pub color: Option<String>,
    pub description: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDropdownRemoveInput {
    pub kind: String,
    pub attribute: String,
    pub dropdown: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaEnumInput {
    pub kind: String,
    pub attribute: String,
    #[serde(rename = "enum")]
    pub r#enum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAttributeCreate {
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAttributeUpdate {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub source: Option<String>,
    pub owner: Option<String>,
    pub value: Option<String>,
}
