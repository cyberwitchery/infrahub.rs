//! generated input types

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branched_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_with_git: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchDeleteInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_from_git: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchEventTypeFilter {
    pub branches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchNameInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchUpdateInput {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPAddressUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPNamespaceUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPPrefixUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pool: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxAttributeCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxAttributeUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAccountInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextInput {
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreActionUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactTargetUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreBasePermissionUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCommentUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCredentialUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_certificates: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<JSONAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_branch: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_conflict_id: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_branch: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_conflict_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_branch: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_conflict_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<ListAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileObjectUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_query_response: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_in_proposed_change: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_after_merge: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_query_response: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_in_proposed_change: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_after_merge: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_query_response: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_in_proposed_change: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_after_merge: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericAccountUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericRepositoryUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_action: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_action: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_action: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_update: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_update: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_update: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_address_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix_length: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_address_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix_length: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_address_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix_length: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPoolUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix_length: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix_length: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix_length: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_prefix_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreKeyValueUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_weight: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permissions: Option<ListAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_weight: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permissions: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_weight: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permissions: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_weight: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permissions: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_previous: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_match: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_previous: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_match: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_previous: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_match: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerMatchUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_action: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_action: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_action: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_attribute: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_range: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_range: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_attribute: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_range: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_range: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_attribute: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_range: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_range: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectComponentTemplateUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectTemplateUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_path: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProfileUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_branch: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_branch: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_branch: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ref")]
    pub r#ref: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_status: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ref")]
    pub r#ref: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ref")]
    pub r#ref: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_status: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generators: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_objects: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreResourcePoolUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<JSONAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_conflict_id: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_conflict_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<JSONAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_conflict_id: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_certificates: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_key: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTaskTargetUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2CreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_path: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_definitions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2UpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_definitions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2UpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_definitions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_query_response: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_definitions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_query_response: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_definitions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_query_response: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_definitions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformationUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies_complete: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_definitions: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTriggerRuleUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_definition: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreValidatorUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_change: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWebhookUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_certificates: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWeightedPoolResourceUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_weight: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffTreeQueryFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IncExclFilterStatusOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<IncExclFilterOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<IncExclFilterOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffUpdateInput {
    pub branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_completion: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTypeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_merged: Option<BranchEventTypeFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_rebased: Option<BranchEventTypeFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_auto_create: Option<GroupAutoCreateEventTypeFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorDefinitionRequestRunInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericPoolInput {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAutoCreateEventTypeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPAddressPoolGetResourceInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPAddressPoolInput {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefixlen: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPPrefixPoolGetResourceInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPPrefixPoolInput {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncExclFilterOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncExclFilterStatusOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<DiffAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<DiffAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenDeleteInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountUpdateSelfInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubComputedAttributeRecomputeInput {
    pub kind: String,
    pub attribute: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<OrderDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<OrderDirection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONAttributeCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONAttributeUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAttributeCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAttributeUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataOrderInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_metadata: Option<InfrahubNodeMetadataOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberAttributeCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_pool: Option<GenericPoolInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberAttributeUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_pool: Option<GenericPoolInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderByItem {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<OrderDirection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_metadata: Option<InfrahubNodeMetadataOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<Vec<OrderByItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathTraversalInput {
    pub source_id: String,
    pub destination_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_paths: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortest_paths_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind_filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_namespaces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_kinds: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_kinds: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pool: Option<CheckboxAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pool: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pool: Option<CheckboxAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_namespace: Option<RelatedNodeInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceCreateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeCreate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceUpsertInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_priority: Option<NumberAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextAttributeUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_nodes: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefixes: Option<Vec<RelatedIPPrefixNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<RelatedIPAddressNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of_groups: Option<Vec<RelatedNodeInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_of_groups: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilesRefreshInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeCheckForApprovalRevokeInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeMergeInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeRequestRunCheckInput {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_paths: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortest_paths_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedIPAddressNodeInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_pool: Option<IPAddressPoolInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_relation__is_protected")]
    pub _relation_is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_relation__owner")]
    pub _relation_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_relation__source")]
    pub _relation_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedIPPrefixNodeInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_pool: Option<IPPrefixPoolInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_relation__is_protected")]
    pub _relation_is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_relation__owner")]
    pub _relation_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_relation__source")]
    pub _relation_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedNodeInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hfid: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_pool: Option<GenericPoolInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_relation__is_protected")]
    pub _relation_is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_relation__owner")]
    pub _relation_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_relation__source")]
    pub _relation_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipNodesInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<RelatedNodeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveDiffConflictInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_branch: Option<ConflictSelection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDropdownAddInput {
    pub kind: String,
    pub attribute: String,
    pub dropdown: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
pub struct TaskActionInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAttributeCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAttributeUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

