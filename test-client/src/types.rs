//! generated types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BranchRelativePermissionDecision {
    #[serde(rename = "DENY")]
    DENY,
    #[serde(rename = "ALLOW")]
    ALLOW,
    #[serde(rename = "ALLOW_DEFAULT")]
    ALLOWDEFAULT,
    #[serde(rename = "ALLOW_OTHER")]
    ALLOWOTHER,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BranchStatus {
    #[serde(rename = "OPEN")]
    OPEN,
    #[serde(rename = "NEED_REBASE")]
    NEEDREBASE,
    #[serde(rename = "NEED_UPGRADE_REBASE")]
    NEEDUPGRADEREBASE,
    #[serde(rename = "DELETING")]
    DELETING,
    #[serde(rename = "MERGING")]
    MERGING,
    #[serde(rename = "MERGED")]
    MERGED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CheckType {
    #[serde(rename = "ARTIFACT")]
    ARTIFACT,
    #[serde(rename = "DATA")]
    DATA,
    #[serde(rename = "GENERATOR")]
    GENERATOR,
    #[serde(rename = "REPOSITORY")]
    REPOSITORY,
    #[serde(rename = "SCHEMA")]
    SCHEMA,
    #[serde(rename = "TEST")]
    TEST,
    #[serde(rename = "USER")]
    USER,
    #[serde(rename = "ALL")]
    ALL,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictSelection {
    #[serde(rename = "BASE_BRANCH")]
    BASEBRANCH,
    #[serde(rename = "DIFF_BRANCH")]
    DIFFBRANCH,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiffAction {
    #[serde(rename = "ADDED")]
    ADDED,
    #[serde(rename = "REMOVED")]
    REMOVED,
    #[serde(rename = "UPDATED")]
    UPDATED,
    #[serde(rename = "UNCHANGED")]
    UNCHANGED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventSortOrder {
    #[serde(rename = "ASC")]
    ASC,
    #[serde(rename = "DESC")]
    DESC,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderDirection {
    #[serde(rename = "ASC")]
    ASC,
    #[serde(rename = "DESC")]
    DESC,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposedChangeApprovalDecision {
    #[serde(rename = "APPROVE")]
    APPROVE,
    #[serde(rename = "CANCEL_APPROVE")]
    CANCELAPPROVE,
    #[serde(rename = "REJECT")]
    REJECT,
    #[serde(rename = "CANCEL_REJECT")]
    CANCELREJECT,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipCardinality {
    #[serde(rename = "ONE")]
    ONE,
    #[serde(rename = "MANY")]
    MANY,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StateType {
    #[serde(rename = "SCHEDULED")]
    SCHEDULED,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "COMPLETED")]
    COMPLETED,
    #[serde(rename = "FAILED")]
    FAILED,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
    #[serde(rename = "CRASHED")]
    CRASHED,
    #[serde(rename = "PAUSED")]
    PAUSED,
    #[serde(rename = "CANCELLING")]
    CANCELLING,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountGlobalPermissionEdge {
    pub node: Box<AccountGlobalPermissionNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountGlobalPermissionEdges {
    pub count: i64,
    pub edges: Vec<AccountGlobalPermissionEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountGlobalPermissionNode {
    pub id: String,
    pub display_label: String,
    pub description: Option<String>,
    pub name: String,
    pub action: String,
    pub decision: String,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLoggedInEventType {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub kind: String,
    pub account_name: String,
    pub account_type: String,
    pub auth_method: String,
    pub session_id: String,
    pub groups: Vec<String>,
    pub roles: Vec<String>,
    pub identity_source: Option<String>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLoggedOutEventType {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub kind: String,
    pub account_name: String,
    pub session_id: String,
    pub logout_type: String,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountObjectPermissionEdge {
    pub node: Box<AccountObjectPermissionNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountObjectPermissionEdges {
    pub count: i64,
    pub edges: Vec<AccountObjectPermissionEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountObjectPermissionNode {
    pub id: String,
    pub display_label: String,
    pub description: Option<String>,
    pub namespace: String,
    pub name: String,
    pub action: String,
    pub decision: String,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountPermissionsEdges {
    pub global_permissions: Option<Box<AccountGlobalPermissionEdges>>,
    pub object_permissions: Option<Box<AccountObjectPermissionEdges>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTokenEdge {
    pub node: Box<AccountTokenNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTokenEdges {
    pub count: i64,
    pub edges: Vec<AccountTokenEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTokenNode {
    pub id: String,
    pub name: Option<String>,
    pub expiration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionAvailability {
    pub action: String,
    pub available: bool,
    pub unavailability_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionAvailabilityEdge {
    pub node: Box<ActionAvailability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyAttribute {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub value: Option<serde_json::Value>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub checksum: String,
    pub checksum_previous: Option<String>,
    pub storage_id: String,
    pub storage_id_previous: Option<String>,
    pub artifact_definition_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableActions {
    pub count: i64,
    pub edges: Vec<ActionAvailabilityEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub origin_branch: Option<String>,
    pub branched_from: Option<String>,
    pub status: BranchStatus,
    pub graph_version: Option<i64>,
    pub created_at: Option<String>,
    pub sync_with_git: Option<bool>,
    pub is_default: Option<bool>,
    pub has_schema_changes: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<Branch>>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCreatedEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub created_branch: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchDelete {
    pub ok: Option<bool>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchDeletedEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub deleted_branch: String,
    pub proposed_change_id: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchMerge {
    pub ok: Option<bool>,
    pub object: Option<Box<Branch>>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchMergedEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub source_branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchRebase {
    pub ok: Option<bool>,
    pub object: Option<Box<Branch>>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchRebasedEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub rebased_branch: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchUpdate {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchValidate {
    pub ok: Option<bool>,
    pub object: Option<Box<Branch>>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPAddressUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPNamespaceUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinIPPrefixUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTag {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub profiles: Box<NestedPaginatedCoreProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<BuiltinTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<BuiltinTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinTagUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<BuiltinTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxAttribute {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub value: Option<bool>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetails {
    pub uuid: String,
    pub base_branch_action: DiffAction,
    pub base_branch_value: Option<String>,
    pub base_branch_changed_at: String,
    pub base_branch_label: Option<String>,
    pub diff_branch_action: DiffAction,
    pub diff_branch_value: Option<String>,
    pub diff_branch_changed_at: String,
    pub diff_branch_label: Option<String>,
    pub selected_branch: Option<ConflictSelection>,
    pub resolvable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertObjectType {
    pub ok: Option<bool>,
    pub node: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccount {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub status: Option<Box<Dropdown>>,
    pub account_type: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub password: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreAccount>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroup {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub label: Option<Box<TextAttribute>>,
    pub group_type: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub roles: Box<NestedPaginatedCoreAccountRole>,
    pub members: Box<NestedPaginatedCoreNode>,
    pub subscribers: Box<NestedPaginatedCoreNode>,
    pub parent: Box<NestedEdgedCoreGroup>,
    pub children: Box<NestedPaginatedCoreGroup>,
    pub ancestors: Box<NestedPaginatedCoreGroup>,
    pub descendants: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreAccountGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreAccountGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountGroupUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreAccountGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRole {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub groups: Box<NestedPaginatedCoreAccountGroup>,
    pub permissions: Box<NestedPaginatedCoreBasePermission>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreAccountRole>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreAccountRole>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountRoleUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreAccountRole>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreAccount>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAccountUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreAccount>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreActionUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifact {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub parameters: Option<Box<JSONAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub status: Option<Box<TextAttribute>>,
    pub checksum: Option<Box<TextAttribute>>,
    pub content_type: Option<Box<TextAttribute>>,
    pub storage_id: Option<Box<TextAttribute>>,
    pub definition: Box<NestedEdgedCoreArtifactDefinition>,
    pub object: Box<NestedEdgedCoreArtifactTarget>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheck {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub severity: Option<Box<TextAttribute>>,
    pub kind: Option<Box<TextAttribute>>,
    pub origin: Option<Box<TextAttribute>>,
    pub created_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub message: Option<Box<TextAttribute>>,
    pub storage_id: Option<Box<TextAttribute>>,
    pub changed: Option<Box<CheckboxAttribute>>,
    pub checksum: Option<Box<TextAttribute>>,
    pub artifact_id: Option<Box<TextAttribute>>,
    pub line_number: Option<Box<NumberAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub validator: Box<NestedEdgedCoreValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCheckUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinition {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub artifact_name: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub parameters: Option<Box<JSONAttribute>>,
    pub content_type: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub transformation: Box<NestedEdgedCoreTransformation>,
    pub targets: Box<NestedEdgedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDefinitionUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactTargetUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThread {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub resolved: Option<Box<CheckboxAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub storage_id: Option<Box<TextAttribute>>,
    pub artifact_id: Option<Box<TextAttribute>>,
    pub line_number: Option<Box<NumberAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub change: Box<NestedEdgedCoreProposedChange>,
    pub comments: Box<NestedPaginatedCoreThreadComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactThreadUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidator {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub started_at: Option<Box<TextAttribute>>,
    pub state: Option<Box<TextAttribute>>,
    pub completed_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub definition: Box<NestedEdgedCoreArtifactDefinition>,
    pub proposed_change: Box<NestedEdgedCoreProposedChange>,
    pub checks: Box<NestedPaginatedCoreCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreArtifactValidatorUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreArtifactValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreBasePermissionUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeComment {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub text: Option<Box<TextAttribute>>,
    pub change: Box<NestedEdgedCoreProposedChange>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreChangeComment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreChangeComment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeCommentUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreChangeComment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThread {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub resolved: Option<Box<CheckboxAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub change: Box<NestedEdgedCoreProposedChange>,
    pub comments: Box<NestedPaginatedCoreThreadComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreChangeThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreChangeThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreChangeThreadUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreChangeThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinition {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub file_path: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub parameters: Option<Box<JSONAttribute>>,
    pub class_name: Option<Box<TextAttribute>>,
    pub timeout: Option<Box<NumberAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub tags: Box<NestedPaginatedBuiltinTag>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub query: Box<NestedEdgedCoreGraphQLQuery>,
    pub targets: Box<NestedEdgedCoreGroup>,
    pub repository: Box<NestedEdgedCoreGenericRepository>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreCheckDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreCheckDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckDefinitionUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreCheckDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCheckUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCommentUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCredentialUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhook {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub event_type: Option<Box<TextAttribute>>,
    pub validate_certificates: Option<Box<CheckboxAttribute>>,
    pub branch_scope: Option<Box<Dropdown>>,
    pub description: Option<Box<TextAttribute>>,
    pub url: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub node_kind: Option<Box<TextAttribute>>,
    pub active: Option<Box<CheckboxAttribute>>,
    pub shared_key: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub transformation: Box<NestedEdgedCoreTransformPython>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub headers: Box<NestedPaginatedCoreKeyValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreCustomWebhook>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreCustomWebhook>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreCustomWebhookUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreCustomWebhook>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheck {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub severity: Option<Box<TextAttribute>>,
    pub kind: Option<Box<TextAttribute>>,
    pub origin: Option<Box<TextAttribute>>,
    pub created_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub message: Option<Box<TextAttribute>>,
    pub conflicts: Option<Box<JSONAttribute>>,
    pub keep_branch: Option<Box<TextAttribute>>,
    pub enriched_conflict_id: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub validator: Box<NestedEdgedCoreValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreDataCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreDataCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataCheckUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreDataCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidator {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub started_at: Option<Box<TextAttribute>>,
    pub state: Option<Box<TextAttribute>>,
    pub completed_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub proposed_change: Box<NestedEdgedCoreProposedChange>,
    pub checks: Box<NestedPaginatedCoreCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreDataValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreDataValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDataValidatorUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreDataValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValue {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub value: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub key: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreEnvKeyValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreEnvKeyValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreEnvKeyValueUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreEnvKeyValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheck {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub severity: Option<Box<TextAttribute>>,
    pub kind: Option<Box<TextAttribute>>,
    pub origin: Option<Box<TextAttribute>>,
    pub created_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub message: Option<Box<TextAttribute>>,
    pub commit: Option<Box<TextAttribute>>,
    pub files: Option<Box<ListAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub validator: Box<NestedEdgedCoreValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreFileCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreFileCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileCheckUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreFileCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileObjectUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThread {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub resolved: Option<Box<CheckboxAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub line_number: Option<Box<NumberAttribute>>,
    pub commit: Option<Box<TextAttribute>>,
    pub file: Option<Box<TextAttribute>>,
    pub repository: Box<NestedEdgedCoreRepository>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub change: Box<NestedEdgedCoreProposedChange>,
    pub comments: Box<NestedPaginatedCoreThreadComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreFileThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreFileThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreFileThreadUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreFileThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAction {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub generator: Box<NestedEdgedCoreGeneratorDefinition>,
    pub triggers: Box<NestedPaginatedCoreTriggerRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorActionUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroup {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub label: Option<Box<TextAttribute>>,
    pub group_type: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub members: Box<NestedPaginatedCoreNode>,
    pub subscribers: Box<NestedPaginatedCoreNode>,
    pub parent: Box<NestedEdgedCoreGroup>,
    pub children: Box<NestedPaginatedCoreGroup>,
    pub ancestors: Box<NestedPaginatedCoreGroup>,
    pub descendants: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorAwareGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorAwareGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorAwareGroupUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorAwareGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheck {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub severity: Option<Box<TextAttribute>>,
    pub kind: Option<Box<TextAttribute>>,
    pub origin: Option<Box<TextAttribute>>,
    pub created_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub message: Option<Box<TextAttribute>>,
    pub instance: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub validator: Box<NestedEdgedCoreValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorCheckUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinition {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub execute_in_proposed_change: Option<Box<CheckboxAttribute>>,
    pub file_path: Option<Box<TextAttribute>>,
    pub class_name: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub parameters: Option<Box<JSONAttribute>>,
    pub convert_query_response: Option<Box<CheckboxAttribute>>,
    pub execute_after_merge: Option<Box<CheckboxAttribute>>,
    pub query: Box<NestedEdgedCoreGraphQLQuery>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub repository: Box<NestedEdgedCoreGenericRepository>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub targets: Box<NestedEdgedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorDefinitionUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroup {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub label: Option<Box<TextAttribute>>,
    pub group_type: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub members: Box<NestedPaginatedCoreNode>,
    pub subscribers: Box<NestedPaginatedCoreNode>,
    pub parent: Box<NestedEdgedCoreGroup>,
    pub children: Box<NestedPaginatedCoreGroup>,
    pub ancestors: Box<NestedPaginatedCoreGroup>,
    pub descendants: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorGroupUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstance {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub status: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub definition: Box<NestedEdgedCoreGeneratorDefinition>,
    pub object: Box<NestedEdgedCoreNode>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorInstance>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorInstance>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorInstanceUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorInstance>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidator {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub started_at: Option<Box<TextAttribute>>,
    pub state: Option<Box<TextAttribute>>,
    pub completed_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub definition: Box<NestedEdgedCoreGeneratorDefinition>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub proposed_change: Box<NestedEdgedCoreProposedChange>,
    pub checks: Box<NestedPaginatedCoreCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGeneratorValidatorUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGeneratorValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericAccountUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGenericRepositoryUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermission {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub identifier: Option<Box<TextAttribute>>,
    pub decision: Option<Box<NumberAttribute>>,
    pub action: Option<Box<Dropdown>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub roles: Box<NestedPaginatedCoreAccountRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGlobalPermission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGlobalPermission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGlobalPermissionUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGlobalPermission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQuery {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub models: Option<Box<ListAttribute>>,
    pub query: Option<Box<TextAttribute>>,
    pub height: Option<Box<NumberAttribute>>,
    pub operations: Option<Box<ListAttribute>>,
    pub depth: Option<Box<NumberAttribute>>,
    pub variables: Option<Box<JSONAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub repository: Box<NestedEdgedCoreGenericRepository>,
    pub tags: Box<NestedPaginatedBuiltinTag>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGraphQLQuery>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroup {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub label: Option<Box<TextAttribute>>,
    pub group_type: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub parameters: Option<Box<JSONAttribute>>,
    pub query: Box<NestedEdgedCoreGraphQLQuery>,
    pub members: Box<NestedPaginatedCoreNode>,
    pub subscribers: Box<NestedPaginatedCoreNode>,
    pub parent: Box<NestedEdgedCoreGroup>,
    pub children: Box<NestedPaginatedCoreGroup>,
    pub ancestors: Box<NestedPaginatedCoreGroup>,
    pub descendants: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGraphQLQueryGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGraphQLQueryGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryGroupUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGraphQLQueryGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGraphQLQuery>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGraphQLQueryUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGraphQLQuery>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupAction {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub member_action: Option<Box<Dropdown>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub group: Box<NestedEdgedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub triggers: Box<NestedPaginatedCoreTriggerRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGroupAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGroupAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupActionUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGroupAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRule {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub active: Option<Box<CheckboxAttribute>>,
    pub branch_scope: Option<Box<Dropdown>>,
    pub description: Option<Box<TextAttribute>>,
    pub member_update: Option<Box<Dropdown>>,
    pub group: Box<NestedEdgedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub action: Box<NestedEdgedCoreAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGroupTriggerRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGroupTriggerRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupTriggerRuleUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreGroupTriggerRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreGroupUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPool {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub default_prefix_length: Option<Box<NumberAttribute>>,
    pub default_address_type: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub ip_namespace: Box<NestedEdgedBuiltinIPNamespace>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub resources: Box<NestedPaginatedBuiltinIPPrefix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreIPAddressPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreIPAddressPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPAddressPoolUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreIPAddressPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPool {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub default_prefix_type: Option<Box<TextAttribute>>,
    pub default_prefix_length: Option<Box<NumberAttribute>>,
    pub default_member_type: Option<Box<TextAttribute>>,
    pub ip_namespace: Box<NestedEdgedBuiltinIPNamespace>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub resources: Box<NestedPaginatedBuiltinIPPrefix>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreIPPrefixPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreIPPrefixPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIPPrefixPoolUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreIPPrefixPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreKeyValueUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItem {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub icon: Option<Box<TextAttribute>>,
    pub required_permissions: Option<Box<ListAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub section: Option<Box<TextAttribute>>,
    pub namespace: Option<Box<TextAttribute>>,
    pub order_weight: Option<Box<NumberAttribute>>,
    pub path: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub protected: Option<Box<CheckboxAttribute>>,
    pub kind: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub parent: Box<NestedEdgedCoreMenu>,
    pub children: Box<NestedPaginatedCoreMenu>,
    pub ancestors: Box<NestedPaginatedCoreMenu>,
    pub descendants: Box<NestedPaginatedCoreMenu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreMenuItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreMenuItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuItemUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreMenuItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMenuUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatch {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub attribute_name: Option<Box<TextAttribute>>,
    pub value_previous: Option<Box<TextAttribute>>,
    pub value_match: Option<Box<Dropdown>>,
    pub value: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub trigger: Box<NestedEdgedCoreNodeTriggerRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNodeTriggerAttributeMatch>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNodeTriggerAttributeMatch>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerAttributeMatchUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNodeTriggerAttributeMatch>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerMatchUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatch {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub relationship_name: Option<Box<TextAttribute>>,
    pub peer: Option<Box<TextAttribute>>,
    pub modification_type: Option<Box<Dropdown>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub trigger: Box<NestedEdgedCoreNodeTriggerRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNodeTriggerRelationshipMatch>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNodeTriggerRelationshipMatch>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRelationshipMatchUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNodeTriggerRelationshipMatch>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRule {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub active: Option<Box<CheckboxAttribute>>,
    pub branch_scope: Option<Box<Dropdown>>,
    pub description: Option<Box<TextAttribute>>,
    pub mutation_action: Option<Box<TextAttribute>>,
    pub node_kind: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub matches: Box<NestedPaginatedCoreNodeTriggerMatch>,
    pub action: Box<NestedEdgedCoreAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNodeTriggerRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNodeTriggerRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeTriggerRuleUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNodeTriggerRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNodeUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPool {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub pool_type: Option<Box<TextAttribute>>,
    pub node: Option<Box<TextAttribute>>,
    pub end_range: Option<Box<NumberAttribute>>,
    pub node_attribute: Option<Box<TextAttribute>>,
    pub start_range: Option<Box<NumberAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNumberPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNumberPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreNumberPoolUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreNumberPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectComponentTemplateUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermission {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub identifier: Option<Box<TextAttribute>>,
    pub namespace: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub decision: Option<Box<NumberAttribute>>,
    pub action: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub roles: Box<NestedPaginatedCoreAccountRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreObjectPermission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreObjectPermission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectPermissionUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreObjectPermission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectTemplateUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThread {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub resolved: Option<Box<CheckboxAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub object_path: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub change: Box<NestedEdgedCoreProposedChange>,
    pub comments: Box<NestedPaginatedCoreThreadComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreObjectThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreObjectThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreObjectThreadUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreObjectThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredential {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub username: Option<Box<TextAttribute>>,
    pub password: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CorePasswordCredential>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CorePasswordCredential>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePasswordCredentialUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CorePasswordCredential>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProfileUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChange {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub source_branch: Option<Box<TextAttribute>>,
    pub is_draft: Option<Box<CheckboxAttribute>>,
    pub state: Option<Box<TextAttribute>>,
    pub total_comments: Option<Box<NumberAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub destination_branch: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub threads: Box<NestedPaginatedCoreThread>,
    pub validations: Box<NestedPaginatedCoreValidator>,
    pub reviewers: Box<NestedPaginatedCoreGenericAccount>,
    pub comments: Box<NestedPaginatedCoreChangeComment>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub approved_by: Box<NestedPaginatedCoreGenericAccount>,
    pub rejected_by: Box<NestedPaginatedCoreGenericAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreProposedChange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreProposedChange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreProposedChangeUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreProposedChange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepository {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub internal_status: Option<Box<Dropdown>>,
    pub location: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub sync_status: Option<Box<Dropdown>>,
    pub operational_status: Option<Box<Dropdown>>,
    #[serde(rename = "ref")]
    pub r#ref: Option<Box<TextAttribute>>,
    pub commit: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub groups_objects: Box<NestedPaginatedCoreRepositoryGroup>,
    pub checks: Box<NestedPaginatedCoreCheckDefinition>,
    pub credential: Box<NestedEdgedCoreCredential>,
    pub queries: Box<NestedPaginatedCoreGraphQLQuery>,
    pub generators: Box<NestedPaginatedCoreGeneratorDefinition>,
    pub tags: Box<NestedPaginatedBuiltinTag>,
    pub transformations: Box<NestedPaginatedCoreTransformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreReadOnlyRepository>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreReadOnlyRepository>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreReadOnlyRepositoryUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreReadOnlyRepository>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepository {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub internal_status: Option<Box<Dropdown>>,
    pub location: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub sync_status: Option<Box<Dropdown>>,
    pub operational_status: Option<Box<Dropdown>>,
    pub commit: Option<Box<TextAttribute>>,
    pub default_branch: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub groups_objects: Box<NestedPaginatedCoreRepositoryGroup>,
    pub checks: Box<NestedPaginatedCoreCheckDefinition>,
    pub credential: Box<NestedEdgedCoreCredential>,
    pub queries: Box<NestedPaginatedCoreGraphQLQuery>,
    pub generators: Box<NestedPaginatedCoreGeneratorDefinition>,
    pub tags: Box<NestedPaginatedBuiltinTag>,
    pub transformations: Box<NestedPaginatedCoreTransformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreRepository>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroup {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub label: Option<Box<TextAttribute>>,
    pub group_type: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub content: Option<Box<Dropdown>>,
    pub repository: Box<NestedEdgedCoreGenericRepository>,
    pub members: Box<NestedPaginatedCoreNode>,
    pub subscribers: Box<NestedPaginatedCoreNode>,
    pub parent: Box<NestedEdgedCoreGroup>,
    pub children: Box<NestedPaginatedCoreGroup>,
    pub ancestors: Box<NestedPaginatedCoreGroup>,
    pub descendants: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreRepositoryGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreRepositoryGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryGroupUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreRepositoryGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreRepository>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreRepository>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidator {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub started_at: Option<Box<TextAttribute>>,
    pub state: Option<Box<TextAttribute>>,
    pub completed_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub repository: Box<NestedEdgedCoreGenericRepository>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub proposed_change: Box<NestedEdgedCoreProposedChange>,
    pub checks: Box<NestedPaginatedCoreCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreRepositoryValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreRepositoryValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreRepositoryValidatorUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreRepositoryValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreResourcePoolUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheck {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub severity: Option<Box<TextAttribute>>,
    pub kind: Option<Box<TextAttribute>>,
    pub origin: Option<Box<TextAttribute>>,
    pub created_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub message: Option<Box<TextAttribute>>,
    pub conflicts: Option<Box<JSONAttribute>>,
    pub enriched_conflict_id: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub validator: Box<NestedEdgedCoreValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreSchemaCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreSchemaCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaCheckUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreSchemaCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidator {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub started_at: Option<Box<TextAttribute>>,
    pub state: Option<Box<TextAttribute>>,
    pub completed_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub proposed_change: Box<NestedEdgedCoreProposedChange>,
    pub checks: Box<NestedPaginatedCoreCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreSchemaValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreSchemaValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSchemaValidatorUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreSchemaValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheck {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub name: Option<Box<TextAttribute>>,
    pub severity: Option<Box<TextAttribute>>,
    pub kind: Option<Box<TextAttribute>>,
    pub origin: Option<Box<TextAttribute>>,
    pub created_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub message: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub validator: Box<NestedEdgedCoreValidator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStandardCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStandardCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardCheckUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStandardCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroup {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub label: Option<Box<TextAttribute>>,
    pub group_type: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub members: Box<NestedPaginatedCoreNode>,
    pub subscribers: Box<NestedPaginatedCoreNode>,
    pub parent: Box<NestedEdgedCoreGroup>,
    pub children: Box<NestedPaginatedCoreGroup>,
    pub ancestors: Box<NestedPaginatedCoreGroup>,
    pub descendants: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStandardGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStandardGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardGroupUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStandardGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhook {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub event_type: Option<Box<TextAttribute>>,
    pub validate_certificates: Option<Box<CheckboxAttribute>>,
    pub branch_scope: Option<Box<Dropdown>>,
    pub description: Option<Box<TextAttribute>>,
    pub url: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub node_kind: Option<Box<TextAttribute>>,
    pub active: Option<Box<CheckboxAttribute>>,
    pub shared_key: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub headers: Box<NestedPaginatedCoreKeyValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStandardWebhook>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStandardWebhook>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStandardWebhookUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStandardWebhook>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValue {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub value: Option<Box<TextAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub key: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStaticKeyValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStaticKeyValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStaticKeyValueUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreStaticKeyValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTaskTargetUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadComment {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub text: Option<Box<TextAttribute>>,
    pub thread: Box<NestedEdgedCoreThread>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreThreadComment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreThreadComment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadCommentUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreThreadComment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreThreadUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2 {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub timeout: Option<Box<NumberAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub template_path: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub repository: Box<NestedEdgedCoreGenericRepository>,
    pub tags: Box<NestedPaginatedBuiltinTag>,
    pub query: Box<NestedEdgedCoreGraphQLQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2Create {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreTransformJinja2>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2Delete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2Update {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreTransformJinja2>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformJinja2Upsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreTransformJinja2>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPython {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub timeout: Option<Box<NumberAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub class_name: Option<Box<TextAttribute>>,
    pub convert_query_response: Option<Box<CheckboxAttribute>>,
    pub file_path: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub repository: Box<NestedEdgedCoreGenericRepository>,
    pub tags: Box<NestedPaginatedBuiltinTag>,
    pub query: Box<NestedEdgedCoreGraphQLQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreTransformPython>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreTransformPython>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformPythonUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreTransformPython>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTransformationUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTriggerRuleUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidator {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub conclusion: Option<Box<TextAttribute>>,
    pub started_at: Option<Box<TextAttribute>>,
    pub state: Option<Box<TextAttribute>>,
    pub completed_at: Option<Box<TextAttribute>>,
    pub label: Option<Box<TextAttribute>>,
    pub check_definition: Box<NestedEdgedCoreCheckDefinition>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub repository: Box<NestedEdgedCoreGenericRepository>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub proposed_change: Box<NestedEdgedCoreProposedChange>,
    pub checks: Box<NestedPaginatedCoreCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreUserValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreUserValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreUserValidatorUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<CoreUserValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreValidatorUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWebhookUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWeightedPoolResourceUpdate {
    pub ok: Option<bool>,
    pub object: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffAttribute {
    pub num_added: i64,
    pub num_updated: i64,
    pub num_removed: i64,
    pub num_conflicts: i64,
    pub name: String,
    pub last_changed_at: String,
    pub status: DiffAction,
    pub path_identifier: String,
    pub properties: Option<Vec<DiffProperty>>,
    pub contains_conflict: bool,
    pub conflict: Option<Box<ConflictDetails>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffNode {
    pub num_added: i64,
    pub num_updated: i64,
    pub num_removed: i64,
    pub num_conflicts: i64,
    pub uuid: String,
    pub kind: String,
    pub label: String,
    pub status: DiffAction,
    pub path_identifier: String,
    pub conflict: Option<Box<ConflictDetails>>,
    pub contains_conflict: bool,
    pub last_changed_at: Option<String>,
    pub parent: Option<Box<DiffNodeParent>>,
    pub attributes: Vec<DiffAttribute>,
    pub relationships: Vec<DiffRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffNodeParent {
    pub uuid: String,
    pub kind: Option<String>,
    pub relationship_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffProperty {
    pub property_type: String,
    pub last_changed_at: String,
    pub previous_value: Option<String>,
    pub new_value: Option<String>,
    pub previous_label: Option<String>,
    pub new_label: Option<String>,
    pub status: DiffAction,
    pub path_identifier: String,
    pub conflict: Option<Box<ConflictDetails>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffRelationship {
    pub num_added: i64,
    pub num_updated: i64,
    pub num_removed: i64,
    pub num_conflicts: i64,
    pub name: String,
    pub label: Option<String>,
    pub last_changed_at: Option<String>,
    pub cardinality: RelationshipCardinality,
    pub status: DiffAction,
    pub path_identifier: String,
    pub elements: Vec<DiffSingleRelationship>,
    pub contains_conflict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSingleRelationship {
    pub num_added: i64,
    pub num_updated: i64,
    pub num_removed: i64,
    pub num_conflicts: i64,
    pub last_changed_at: Option<String>,
    pub status: DiffAction,
    pub peer_id: String,
    pub peer_label: Option<String>,
    pub path_identifier: String,
    pub contains_conflict: bool,
    pub conflict: Option<Box<ConflictDetails>>,
    pub properties: Option<Vec<DiffProperty>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffTree {
    pub num_added: i64,
    pub num_updated: i64,
    pub num_removed: i64,
    pub num_conflicts: i64,
    pub base_branch: String,
    pub diff_branch: String,
    pub from_time: String,
    pub to_time: String,
    pub num_untracked_base_changes: Option<i64>,
    pub num_untracked_diff_changes: Option<i64>,
    pub name: Option<String>,
    pub nodes: Option<Vec<DiffNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffTreeSummary {
    pub num_added: i64,
    pub num_updated: i64,
    pub num_removed: i64,
    pub num_conflicts: i64,
    pub base_branch: String,
    pub diff_branch: String,
    pub from_time: String,
    pub to_time: String,
    pub num_unchanged: i64,
    pub num_untracked_base_changes: Option<i64>,
    pub num_untracked_diff_changes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffUpdateMutation {
    pub ok: Option<bool>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dropdown {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownFields {
    pub value: Option<String>,
    pub label: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedBuiltinIPAddress {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedBuiltinIPNamespace {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedBuiltinIPPrefix {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedBuiltinTag {
    pub node: Option<Box<BuiltinTag>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreAccount {
    pub node: Option<Box<CoreAccount>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreAccountGroup {
    pub node: Option<Box<CoreAccountGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreAccountRole {
    pub node: Option<Box<CoreAccountRole>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreAction {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreArtifact {
    pub node: Option<Box<CoreArtifact>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreArtifactCheck {
    pub node: Option<Box<CoreArtifactCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreArtifactDefinition {
    pub node: Option<Box<CoreArtifactDefinition>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreArtifactTarget {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreArtifactThread {
    pub node: Option<Box<CoreArtifactThread>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreArtifactValidator {
    pub node: Option<Box<CoreArtifactValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreBasePermission {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreChangeComment {
    pub node: Option<Box<CoreChangeComment>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreChangeThread {
    pub node: Option<Box<CoreChangeThread>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreCheck {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreCheckDefinition {
    pub node: Option<Box<CoreCheckDefinition>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreComment {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreCredential {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreCustomWebhook {
    pub node: Option<Box<CoreCustomWebhook>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreDataCheck {
    pub node: Option<Box<CoreDataCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreDataValidator {
    pub node: Option<Box<CoreDataValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreEnvKeyValue {
    pub node: Option<Box<CoreEnvKeyValue>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreFileCheck {
    pub node: Option<Box<CoreFileCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreFileObject {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreFileThread {
    pub node: Option<Box<CoreFileThread>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGeneratorAction {
    pub node: Option<Box<CoreGeneratorAction>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGeneratorAwareGroup {
    pub node: Option<Box<CoreGeneratorAwareGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGeneratorCheck {
    pub node: Option<Box<CoreGeneratorCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGeneratorDefinition {
    pub node: Option<Box<CoreGeneratorDefinition>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGeneratorGroup {
    pub node: Option<Box<CoreGeneratorGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGeneratorInstance {
    pub node: Option<Box<CoreGeneratorInstance>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGeneratorValidator {
    pub node: Option<Box<CoreGeneratorValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGenericAccount {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGenericRepository {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGlobalPermission {
    pub node: Option<Box<CoreGlobalPermission>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGraphQLQuery {
    pub node: Option<Box<CoreGraphQLQuery>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGraphQLQueryGroup {
    pub node: Option<Box<CoreGraphQLQueryGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGroup {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGroupAction {
    pub node: Option<Box<CoreGroupAction>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreGroupTriggerRule {
    pub node: Option<Box<CoreGroupTriggerRule>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreIPAddressPool {
    pub node: Option<Box<CoreIPAddressPool>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreIPPrefixPool {
    pub node: Option<Box<CoreIPPrefixPool>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreKeyValue {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreMenu {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreMenuItem {
    pub node: Option<Box<CoreMenuItem>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreNode {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreNodeTriggerAttributeMatch {
    pub node: Option<Box<CoreNodeTriggerAttributeMatch>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreNodeTriggerMatch {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreNodeTriggerRelationshipMatch {
    pub node: Option<Box<CoreNodeTriggerRelationshipMatch>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreNodeTriggerRule {
    pub node: Option<Box<CoreNodeTriggerRule>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreNumberPool {
    pub node: Option<Box<CoreNumberPool>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreObjectComponentTemplate {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreObjectPermission {
    pub node: Option<Box<CoreObjectPermission>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreObjectTemplate {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreObjectThread {
    pub node: Option<Box<CoreObjectThread>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCorePasswordCredential {
    pub node: Option<Box<CorePasswordCredential>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreProfile {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreProposedChange {
    pub node: Option<Box<CoreProposedChange>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreReadOnlyRepository {
    pub node: Option<Box<CoreReadOnlyRepository>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreRepository {
    pub node: Option<Box<CoreRepository>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreRepositoryGroup {
    pub node: Option<Box<CoreRepositoryGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreRepositoryValidator {
    pub node: Option<Box<CoreRepositoryValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreResourcePool {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreSchemaCheck {
    pub node: Option<Box<CoreSchemaCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreSchemaValidator {
    pub node: Option<Box<CoreSchemaValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreStandardCheck {
    pub node: Option<Box<CoreStandardCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreStandardGroup {
    pub node: Option<Box<CoreStandardGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreStandardWebhook {
    pub node: Option<Box<CoreStandardWebhook>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreStaticKeyValue {
    pub node: Option<Box<CoreStaticKeyValue>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreTaskTarget {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreThread {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreThreadComment {
    pub node: Option<Box<CoreThreadComment>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreTransformJinja2 {
    pub node: Option<Box<CoreTransformJinja2>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreTransformPython {
    pub node: Option<Box<CoreTransformPython>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreTransformation {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreTriggerRule {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreUserValidator {
    pub node: Option<Box<CoreUserValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreValidator {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreWebhook {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedCoreWeightedPoolResource {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedInternalAccountToken {
    pub node: Option<Box<InternalAccountToken>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedInternalExternalIdentity {
    pub node: Option<Box<InternalExternalIdentity>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedInternalIPPrefixAvailable {
    pub node: Option<Box<InternalIPPrefixAvailable>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedInternalIPRangeAvailable {
    pub node: Option<Box<InternalIPRangeAvailable>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedInternalRefreshToken {
    pub node: Option<Box<InternalRefreshToken>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedIpamNamespace {
    pub node: Option<Box<IpamNamespace>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedLineageOwner {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedLineageSource {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedProfileBuiltinIPAddress {
    pub node: Option<Box<ProfileBuiltinIPAddress>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedProfileBuiltinIPPrefix {
    pub node: Option<Box<ProfileBuiltinIPPrefix>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedProfileBuiltinTag {
    pub node: Option<Box<ProfileBuiltinTag>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgedProfileIpamNamespace {
    pub node: Option<Box<ProfileIpamNamespace>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNodes {
    pub node: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Events {
    pub edges: Vec<EventNodes>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldsMapping {
    pub mapping: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorDefinitionRequestRun {
    pub ok: Option<bool>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLQueryReport {
    pub targets_unique_nodes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub members: Vec<RelatedNode>,
    pub ancestors: Vec<RelatedNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPAddressGetNextAvailable {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPAddressPoolGetResource {
    pub ok: Option<bool>,
    pub node: Option<Box<PoolAllocatedNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPHost {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub value: Option<String>,
    pub ip: Option<String>,
    pub hostmask: Option<String>,
    pub netmask: Option<String>,
    pub prefixlen: Option<i64>,
    pub version: Option<i64>,
    pub with_hostmask: Option<String>,
    pub with_netmask: Option<String>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPNetwork {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub value: Option<String>,
    pub broadcast_address: Option<String>,
    pub hostmask: Option<String>,
    pub netmask: Option<String>,
    pub prefixlen: Option<i64>,
    pub num_addresses: Option<i64>,
    pub version: Option<i64>,
    pub with_hostmask: Option<String>,
    pub with_netmask: Option<String>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPPoolUtilizationResource {
    pub id: String,
    pub display_label: String,
    pub kind: String,
    pub weight: i64,
    pub utilization: f64,
    pub utilization_branches: f64,
    pub utilization_default_branch: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPPrefixGetNextAvailable {
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPPrefixPoolGetResource {
    pub ok: Option<bool>,
    pub node: Option<Box<PoolAllocatedNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPPrefixUtilizationEdge {
    pub node: Box<IPPoolUtilizationResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub deployment_id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountSelfUpdate {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<InfrahubAccountTokenType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubAccountTokenType {
    pub id: String,
    pub token: Option<Box<ValueType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubBranch {
    pub id: String,
    pub name: Box<RequiredStringValueField>,
    pub description: Option<Box<NonRequiredStringValueField>>,
    pub origin_branch: Option<Box<NonRequiredStringValueField>>,
    pub branched_from: Option<Box<NonRequiredStringValueField>>,
    pub status: Box<StatusField>,
    pub graph_version: Option<Box<NonRequiredIntValueField>>,
    pub created_at: Option<String>,
    pub sync_with_git: Option<Box<NonRequiredBooleanValueField>>,
    pub is_default: Option<Box<NonRequiredBooleanValueField>>,
    pub has_schema_changes: Option<Box<NonRequiredBooleanValueField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubBranchEdge {
    pub node: Box<InfrahubBranch>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubBranchType {
    pub count: Option<i64>,
    pub edges: Vec<InfrahubBranchEdge>,
    pub default_branch: Box<InfrahubBranch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubMutatedAttribute {
    pub name: String,
    pub action: DiffAction,
    pub value: Option<String>,
    pub kind: String,
    pub value_previous: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubMutatedRelationship {
    pub name: String,
    pub action: DiffAction,
    pub peer: Box<RelatedNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubNodeMetadata {
    pub created_at: Option<String>,
    pub created_by: Option<serde_json::Value>,
    pub updated_at: Option<String>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubProfilesRefresh {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrahubRelationshipMetadata {
    pub created_at: Option<String>,
    pub created_by: Option<serde_json::Value>,
    pub updated_at: Option<String>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalAccountToken {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub token: Option<Box<TextAttribute>>,
    pub expiration: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub account: Box<NestedEdgedCoreGenericAccount>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalExternalIdentity {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub sub: Option<Box<TextAttribute>>,
    pub provider_name: Option<Box<TextAttribute>>,
    pub protocol: Option<Box<TextAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub account: Box<NestedEdgedCoreGenericAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalIPPrefixAvailable {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub netmask: Option<Box<TextAttribute>>,
    pub hostmask: Option<Box<TextAttribute>>,
    pub is_top_level: Option<Box<CheckboxAttribute>>,
    pub utilization: Option<Box<NumberAttribute>>,
    pub is_pool: Option<Box<CheckboxAttribute>>,
    pub broadcast_address: Option<Box<TextAttribute>>,
    pub member_type: Option<Box<Dropdown>>,
    pub network_address: Option<Box<TextAttribute>>,
    pub prefix: Option<Box<IPNetwork>>,
    pub description: Option<Box<TextAttribute>>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub ip_namespace: Box<NestedEdgedBuiltinIPNamespace>,
    pub ip_addresses: Box<NestedPaginatedBuiltinIPAddress>,
    pub resource_pool: Box<NestedPaginatedCoreIPAddressPool>,
    pub profiles: Box<NestedPaginatedCoreProfile>,
    pub parent: Box<NestedEdgedBuiltinIPPrefix>,
    pub children: Box<NestedPaginatedBuiltinIPPrefix>,
    pub ancestors: Box<NestedPaginatedBuiltinIPPrefix>,
    pub descendants: Box<NestedPaginatedBuiltinIPPrefix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalIPRangeAvailable {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub address: Option<Box<IPHost>>,
    pub description: Option<Box<TextAttribute>>,
    pub last_address: Option<Box<IPHost>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub ip_namespace: Box<NestedEdgedBuiltinIPNamespace>,
    pub ip_prefix: Box<NestedEdgedBuiltinIPPrefix>,
    pub profiles: Box<NestedPaginatedCoreProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalRefreshToken {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub expiration: Option<Box<TextAttribute>>,
    pub account: Box<NestedEdgedCoreGenericAccount>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespace {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub description: Option<Box<TextAttribute>>,
    pub name: Option<Box<TextAttribute>>,
    pub default: Option<Box<CheckboxAttribute>>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
    pub ip_prefixes: Box<NestedPaginatedBuiltinIPPrefix>,
    pub ip_addresses: Box<NestedPaginatedBuiltinIPAddress>,
    pub profiles: Box<NestedPaginatedCoreProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<IpamNamespace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<IpamNamespace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamNamespaceUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<IpamNamespace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONAttribute {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub value: Option<serde_json::Value>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAttribute {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub value: Option<serde_json::Value>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacAddress {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub value: Option<String>,
    pub oui: Option<String>,
    pub ei: Option<String>,
    pub version: Option<i64>,
    pub binary: Option<String>,
    pub eui48: Option<String>,
    pub eui64: Option<String>,
    pub bare: Option<String>,
    pub dot_notation: Option<String>,
    pub semicolon_notation: Option<String>,
    pub split_notation: Option<String>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mutation {
    #[serde(rename = "CoreMenuItemCreate")]
    pub core_menu_item_create: Option<Box<CoreMenuItemCreate>>,
    #[serde(rename = "CoreMenuItemUpdate")]
    pub core_menu_item_update: Option<Box<CoreMenuItemUpdate>>,
    #[serde(rename = "CoreMenuItemUpsert")]
    pub core_menu_item_upsert: Option<Box<CoreMenuItemUpsert>>,
    #[serde(rename = "CoreMenuItemDelete")]
    pub core_menu_item_delete: Option<Box<CoreMenuItemDelete>>,
    #[serde(rename = "CoreGroupActionCreate")]
    pub core_group_action_create: Option<Box<CoreGroupActionCreate>>,
    #[serde(rename = "CoreGroupActionUpdate")]
    pub core_group_action_update: Option<Box<CoreGroupActionUpdate>>,
    #[serde(rename = "CoreGroupActionUpsert")]
    pub core_group_action_upsert: Option<Box<CoreGroupActionUpsert>>,
    #[serde(rename = "CoreGroupActionDelete")]
    pub core_group_action_delete: Option<Box<CoreGroupActionDelete>>,
    #[serde(rename = "CoreStandardGroupCreate")]
    pub core_standard_group_create: Option<Box<CoreStandardGroupCreate>>,
    #[serde(rename = "CoreStandardGroupUpdate")]
    pub core_standard_group_update: Option<Box<CoreStandardGroupUpdate>>,
    #[serde(rename = "CoreStandardGroupUpsert")]
    pub core_standard_group_upsert: Option<Box<CoreStandardGroupUpsert>>,
    #[serde(rename = "CoreStandardGroupDelete")]
    pub core_standard_group_delete: Option<Box<CoreStandardGroupDelete>>,
    #[serde(rename = "CoreGeneratorGroupCreate")]
    pub core_generator_group_create: Option<Box<CoreGeneratorGroupCreate>>,
    #[serde(rename = "CoreGeneratorGroupUpdate")]
    pub core_generator_group_update: Option<Box<CoreGeneratorGroupUpdate>>,
    #[serde(rename = "CoreGeneratorGroupUpsert")]
    pub core_generator_group_upsert: Option<Box<CoreGeneratorGroupUpsert>>,
    #[serde(rename = "CoreGeneratorGroupDelete")]
    pub core_generator_group_delete: Option<Box<CoreGeneratorGroupDelete>>,
    #[serde(rename = "CoreGeneratorAwareGroupCreate")]
    pub core_generator_aware_group_create: Option<Box<CoreGeneratorAwareGroupCreate>>,
    #[serde(rename = "CoreGeneratorAwareGroupUpdate")]
    pub core_generator_aware_group_update: Option<Box<CoreGeneratorAwareGroupUpdate>>,
    #[serde(rename = "CoreGeneratorAwareGroupUpsert")]
    pub core_generator_aware_group_upsert: Option<Box<CoreGeneratorAwareGroupUpsert>>,
    #[serde(rename = "CoreGeneratorAwareGroupDelete")]
    pub core_generator_aware_group_delete: Option<Box<CoreGeneratorAwareGroupDelete>>,
    #[serde(rename = "CoreGraphQLQueryGroupCreate")]
    pub core_graph_q_l_query_group_create: Option<Box<CoreGraphQLQueryGroupCreate>>,
    #[serde(rename = "CoreGraphQLQueryGroupUpdate")]
    pub core_graph_q_l_query_group_update: Option<Box<CoreGraphQLQueryGroupUpdate>>,
    #[serde(rename = "CoreGraphQLQueryGroupUpsert")]
    pub core_graph_q_l_query_group_upsert: Option<Box<CoreGraphQLQueryGroupUpsert>>,
    #[serde(rename = "CoreGraphQLQueryGroupDelete")]
    pub core_graph_q_l_query_group_delete: Option<Box<CoreGraphQLQueryGroupDelete>>,
    #[serde(rename = "CoreRepositoryGroupCreate")]
    pub core_repository_group_create: Option<Box<CoreRepositoryGroupCreate>>,
    #[serde(rename = "CoreRepositoryGroupUpdate")]
    pub core_repository_group_update: Option<Box<CoreRepositoryGroupUpdate>>,
    #[serde(rename = "CoreRepositoryGroupUpsert")]
    pub core_repository_group_upsert: Option<Box<CoreRepositoryGroupUpsert>>,
    #[serde(rename = "CoreRepositoryGroupDelete")]
    pub core_repository_group_delete: Option<Box<CoreRepositoryGroupDelete>>,
    #[serde(rename = "BuiltinTagCreate")]
    pub builtin_tag_create: Option<Box<BuiltinTagCreate>>,
    #[serde(rename = "BuiltinTagUpdate")]
    pub builtin_tag_update: Option<Box<BuiltinTagUpdate>>,
    #[serde(rename = "BuiltinTagUpsert")]
    pub builtin_tag_upsert: Option<Box<BuiltinTagUpsert>>,
    #[serde(rename = "BuiltinTagDelete")]
    pub builtin_tag_delete: Option<Box<BuiltinTagDelete>>,
    #[serde(rename = "CoreAccountCreate")]
    pub core_account_create: Option<Box<CoreAccountCreate>>,
    #[serde(rename = "CoreAccountUpdate")]
    pub core_account_update: Option<Box<CoreAccountUpdate>>,
    #[serde(rename = "CoreAccountUpsert")]
    pub core_account_upsert: Option<Box<CoreAccountUpsert>>,
    #[serde(rename = "CoreAccountDelete")]
    pub core_account_delete: Option<Box<CoreAccountDelete>>,
    #[serde(rename = "CoreGeneratorActionCreate")]
    pub core_generator_action_create: Option<Box<CoreGeneratorActionCreate>>,
    #[serde(rename = "CoreGeneratorActionUpdate")]
    pub core_generator_action_update: Option<Box<CoreGeneratorActionUpdate>>,
    #[serde(rename = "CoreGeneratorActionUpsert")]
    pub core_generator_action_upsert: Option<Box<CoreGeneratorActionUpsert>>,
    #[serde(rename = "CoreGeneratorActionDelete")]
    pub core_generator_action_delete: Option<Box<CoreGeneratorActionDelete>>,
    #[serde(rename = "CoreGroupTriggerRuleCreate")]
    pub core_group_trigger_rule_create: Option<Box<CoreGroupTriggerRuleCreate>>,
    #[serde(rename = "CoreGroupTriggerRuleUpdate")]
    pub core_group_trigger_rule_update: Option<Box<CoreGroupTriggerRuleUpdate>>,
    #[serde(rename = "CoreGroupTriggerRuleUpsert")]
    pub core_group_trigger_rule_upsert: Option<Box<CoreGroupTriggerRuleUpsert>>,
    #[serde(rename = "CoreGroupTriggerRuleDelete")]
    pub core_group_trigger_rule_delete: Option<Box<CoreGroupTriggerRuleDelete>>,
    #[serde(rename = "CoreNodeTriggerRuleCreate")]
    pub core_node_trigger_rule_create: Option<Box<CoreNodeTriggerRuleCreate>>,
    #[serde(rename = "CoreNodeTriggerRuleUpdate")]
    pub core_node_trigger_rule_update: Option<Box<CoreNodeTriggerRuleUpdate>>,
    #[serde(rename = "CoreNodeTriggerRuleUpsert")]
    pub core_node_trigger_rule_upsert: Option<Box<CoreNodeTriggerRuleUpsert>>,
    #[serde(rename = "CoreNodeTriggerRuleDelete")]
    pub core_node_trigger_rule_delete: Option<Box<CoreNodeTriggerRuleDelete>>,
    #[serde(rename = "CoreNodeTriggerAttributeMatchCreate")]
    pub core_node_trigger_attribute_match_create: Option<Box<CoreNodeTriggerAttributeMatchCreate>>,
    #[serde(rename = "CoreNodeTriggerAttributeMatchUpdate")]
    pub core_node_trigger_attribute_match_update: Option<Box<CoreNodeTriggerAttributeMatchUpdate>>,
    #[serde(rename = "CoreNodeTriggerAttributeMatchUpsert")]
    pub core_node_trigger_attribute_match_upsert: Option<Box<CoreNodeTriggerAttributeMatchUpsert>>,
    #[serde(rename = "CoreNodeTriggerAttributeMatchDelete")]
    pub core_node_trigger_attribute_match_delete: Option<Box<CoreNodeTriggerAttributeMatchDelete>>,
    #[serde(rename = "CoreNodeTriggerRelationshipMatchCreate")]
    pub core_node_trigger_relationship_match_create: Option<Box<CoreNodeTriggerRelationshipMatchCreate>>,
    #[serde(rename = "CoreNodeTriggerRelationshipMatchUpdate")]
    pub core_node_trigger_relationship_match_update: Option<Box<CoreNodeTriggerRelationshipMatchUpdate>>,
    #[serde(rename = "CoreNodeTriggerRelationshipMatchUpsert")]
    pub core_node_trigger_relationship_match_upsert: Option<Box<CoreNodeTriggerRelationshipMatchUpsert>>,
    #[serde(rename = "CoreNodeTriggerRelationshipMatchDelete")]
    pub core_node_trigger_relationship_match_delete: Option<Box<CoreNodeTriggerRelationshipMatchDelete>>,
    #[serde(rename = "CorePasswordCredentialCreate")]
    pub core_password_credential_create: Option<Box<CorePasswordCredentialCreate>>,
    #[serde(rename = "CorePasswordCredentialUpdate")]
    pub core_password_credential_update: Option<Box<CorePasswordCredentialUpdate>>,
    #[serde(rename = "CorePasswordCredentialUpsert")]
    pub core_password_credential_upsert: Option<Box<CorePasswordCredentialUpsert>>,
    #[serde(rename = "CorePasswordCredentialDelete")]
    pub core_password_credential_delete: Option<Box<CorePasswordCredentialDelete>>,
    #[serde(rename = "CoreProposedChangeCreate")]
    pub core_proposed_change_create: Option<Box<CoreProposedChangeCreate>>,
    #[serde(rename = "CoreProposedChangeUpdate")]
    pub core_proposed_change_update: Option<Box<CoreProposedChangeUpdate>>,
    #[serde(rename = "CoreProposedChangeUpsert")]
    pub core_proposed_change_upsert: Option<Box<CoreProposedChangeUpsert>>,
    #[serde(rename = "CoreProposedChangeDelete")]
    pub core_proposed_change_delete: Option<Box<CoreProposedChangeDelete>>,
    #[serde(rename = "CoreChangeThreadCreate")]
    pub core_change_thread_create: Option<Box<CoreChangeThreadCreate>>,
    #[serde(rename = "CoreChangeThreadUpdate")]
    pub core_change_thread_update: Option<Box<CoreChangeThreadUpdate>>,
    #[serde(rename = "CoreChangeThreadUpsert")]
    pub core_change_thread_upsert: Option<Box<CoreChangeThreadUpsert>>,
    #[serde(rename = "CoreChangeThreadDelete")]
    pub core_change_thread_delete: Option<Box<CoreChangeThreadDelete>>,
    #[serde(rename = "CoreFileThreadCreate")]
    pub core_file_thread_create: Option<Box<CoreFileThreadCreate>>,
    #[serde(rename = "CoreFileThreadUpdate")]
    pub core_file_thread_update: Option<Box<CoreFileThreadUpdate>>,
    #[serde(rename = "CoreFileThreadUpsert")]
    pub core_file_thread_upsert: Option<Box<CoreFileThreadUpsert>>,
    #[serde(rename = "CoreFileThreadDelete")]
    pub core_file_thread_delete: Option<Box<CoreFileThreadDelete>>,
    #[serde(rename = "CoreArtifactThreadCreate")]
    pub core_artifact_thread_create: Option<Box<CoreArtifactThreadCreate>>,
    #[serde(rename = "CoreArtifactThreadUpdate")]
    pub core_artifact_thread_update: Option<Box<CoreArtifactThreadUpdate>>,
    #[serde(rename = "CoreArtifactThreadUpsert")]
    pub core_artifact_thread_upsert: Option<Box<CoreArtifactThreadUpsert>>,
    #[serde(rename = "CoreArtifactThreadDelete")]
    pub core_artifact_thread_delete: Option<Box<CoreArtifactThreadDelete>>,
    #[serde(rename = "CoreObjectThreadCreate")]
    pub core_object_thread_create: Option<Box<CoreObjectThreadCreate>>,
    #[serde(rename = "CoreObjectThreadUpdate")]
    pub core_object_thread_update: Option<Box<CoreObjectThreadUpdate>>,
    #[serde(rename = "CoreObjectThreadUpsert")]
    pub core_object_thread_upsert: Option<Box<CoreObjectThreadUpsert>>,
    #[serde(rename = "CoreObjectThreadDelete")]
    pub core_object_thread_delete: Option<Box<CoreObjectThreadDelete>>,
    #[serde(rename = "CoreChangeCommentCreate")]
    pub core_change_comment_create: Option<Box<CoreChangeCommentCreate>>,
    #[serde(rename = "CoreChangeCommentUpdate")]
    pub core_change_comment_update: Option<Box<CoreChangeCommentUpdate>>,
    #[serde(rename = "CoreChangeCommentUpsert")]
    pub core_change_comment_upsert: Option<Box<CoreChangeCommentUpsert>>,
    #[serde(rename = "CoreChangeCommentDelete")]
    pub core_change_comment_delete: Option<Box<CoreChangeCommentDelete>>,
    #[serde(rename = "CoreThreadCommentCreate")]
    pub core_thread_comment_create: Option<Box<CoreThreadCommentCreate>>,
    #[serde(rename = "CoreThreadCommentUpdate")]
    pub core_thread_comment_update: Option<Box<CoreThreadCommentUpdate>>,
    #[serde(rename = "CoreThreadCommentUpsert")]
    pub core_thread_comment_upsert: Option<Box<CoreThreadCommentUpsert>>,
    #[serde(rename = "CoreThreadCommentDelete")]
    pub core_thread_comment_delete: Option<Box<CoreThreadCommentDelete>>,
    #[serde(rename = "CoreRepositoryCreate")]
    pub core_repository_create: Option<Box<CoreRepositoryCreate>>,
    #[serde(rename = "CoreRepositoryUpdate")]
    pub core_repository_update: Option<Box<CoreRepositoryUpdate>>,
    #[serde(rename = "CoreRepositoryUpsert")]
    pub core_repository_upsert: Option<Box<CoreRepositoryUpsert>>,
    #[serde(rename = "CoreRepositoryDelete")]
    pub core_repository_delete: Option<Box<CoreRepositoryDelete>>,
    #[serde(rename = "CoreReadOnlyRepositoryCreate")]
    pub core_read_only_repository_create: Option<Box<CoreReadOnlyRepositoryCreate>>,
    #[serde(rename = "CoreReadOnlyRepositoryUpdate")]
    pub core_read_only_repository_update: Option<Box<CoreReadOnlyRepositoryUpdate>>,
    #[serde(rename = "CoreReadOnlyRepositoryUpsert")]
    pub core_read_only_repository_upsert: Option<Box<CoreReadOnlyRepositoryUpsert>>,
    #[serde(rename = "CoreReadOnlyRepositoryDelete")]
    pub core_read_only_repository_delete: Option<Box<CoreReadOnlyRepositoryDelete>>,
    #[serde(rename = "CoreTransformJinja2Create")]
    pub core_transform_jinja2_create: Option<Box<CoreTransformJinja2Create>>,
    #[serde(rename = "CoreTransformJinja2Update")]
    pub core_transform_jinja2_update: Option<Box<CoreTransformJinja2Update>>,
    #[serde(rename = "CoreTransformJinja2Upsert")]
    pub core_transform_jinja2_upsert: Option<Box<CoreTransformJinja2Upsert>>,
    #[serde(rename = "CoreTransformJinja2Delete")]
    pub core_transform_jinja2_delete: Option<Box<CoreTransformJinja2Delete>>,
    #[serde(rename = "CoreDataCheckCreate")]
    pub core_data_check_create: Option<Box<CoreDataCheckCreate>>,
    #[serde(rename = "CoreDataCheckUpdate")]
    pub core_data_check_update: Option<Box<CoreDataCheckUpdate>>,
    #[serde(rename = "CoreDataCheckUpsert")]
    pub core_data_check_upsert: Option<Box<CoreDataCheckUpsert>>,
    #[serde(rename = "CoreDataCheckDelete")]
    pub core_data_check_delete: Option<Box<CoreDataCheckDelete>>,
    #[serde(rename = "CoreStandardCheckCreate")]
    pub core_standard_check_create: Option<Box<CoreStandardCheckCreate>>,
    #[serde(rename = "CoreStandardCheckUpdate")]
    pub core_standard_check_update: Option<Box<CoreStandardCheckUpdate>>,
    #[serde(rename = "CoreStandardCheckUpsert")]
    pub core_standard_check_upsert: Option<Box<CoreStandardCheckUpsert>>,
    #[serde(rename = "CoreStandardCheckDelete")]
    pub core_standard_check_delete: Option<Box<CoreStandardCheckDelete>>,
    #[serde(rename = "CoreSchemaCheckCreate")]
    pub core_schema_check_create: Option<Box<CoreSchemaCheckCreate>>,
    #[serde(rename = "CoreSchemaCheckUpdate")]
    pub core_schema_check_update: Option<Box<CoreSchemaCheckUpdate>>,
    #[serde(rename = "CoreSchemaCheckUpsert")]
    pub core_schema_check_upsert: Option<Box<CoreSchemaCheckUpsert>>,
    #[serde(rename = "CoreSchemaCheckDelete")]
    pub core_schema_check_delete: Option<Box<CoreSchemaCheckDelete>>,
    #[serde(rename = "CoreFileCheckCreate")]
    pub core_file_check_create: Option<Box<CoreFileCheckCreate>>,
    #[serde(rename = "CoreFileCheckUpdate")]
    pub core_file_check_update: Option<Box<CoreFileCheckUpdate>>,
    #[serde(rename = "CoreFileCheckUpsert")]
    pub core_file_check_upsert: Option<Box<CoreFileCheckUpsert>>,
    #[serde(rename = "CoreFileCheckDelete")]
    pub core_file_check_delete: Option<Box<CoreFileCheckDelete>>,
    #[serde(rename = "CoreArtifactCheckCreate")]
    pub core_artifact_check_create: Option<Box<CoreArtifactCheckCreate>>,
    #[serde(rename = "CoreArtifactCheckUpdate")]
    pub core_artifact_check_update: Option<Box<CoreArtifactCheckUpdate>>,
    #[serde(rename = "CoreArtifactCheckUpsert")]
    pub core_artifact_check_upsert: Option<Box<CoreArtifactCheckUpsert>>,
    #[serde(rename = "CoreArtifactCheckDelete")]
    pub core_artifact_check_delete: Option<Box<CoreArtifactCheckDelete>>,
    #[serde(rename = "CoreGeneratorCheckCreate")]
    pub core_generator_check_create: Option<Box<CoreGeneratorCheckCreate>>,
    #[serde(rename = "CoreGeneratorCheckUpdate")]
    pub core_generator_check_update: Option<Box<CoreGeneratorCheckUpdate>>,
    #[serde(rename = "CoreGeneratorCheckUpsert")]
    pub core_generator_check_upsert: Option<Box<CoreGeneratorCheckUpsert>>,
    #[serde(rename = "CoreGeneratorCheckDelete")]
    pub core_generator_check_delete: Option<Box<CoreGeneratorCheckDelete>>,
    #[serde(rename = "CoreDataValidatorCreate")]
    pub core_data_validator_create: Option<Box<CoreDataValidatorCreate>>,
    #[serde(rename = "CoreDataValidatorUpdate")]
    pub core_data_validator_update: Option<Box<CoreDataValidatorUpdate>>,
    #[serde(rename = "CoreDataValidatorUpsert")]
    pub core_data_validator_upsert: Option<Box<CoreDataValidatorUpsert>>,
    #[serde(rename = "CoreDataValidatorDelete")]
    pub core_data_validator_delete: Option<Box<CoreDataValidatorDelete>>,
    #[serde(rename = "CoreRepositoryValidatorCreate")]
    pub core_repository_validator_create: Option<Box<CoreRepositoryValidatorCreate>>,
    #[serde(rename = "CoreRepositoryValidatorUpdate")]
    pub core_repository_validator_update: Option<Box<CoreRepositoryValidatorUpdate>>,
    #[serde(rename = "CoreRepositoryValidatorUpsert")]
    pub core_repository_validator_upsert: Option<Box<CoreRepositoryValidatorUpsert>>,
    #[serde(rename = "CoreRepositoryValidatorDelete")]
    pub core_repository_validator_delete: Option<Box<CoreRepositoryValidatorDelete>>,
    #[serde(rename = "CoreUserValidatorCreate")]
    pub core_user_validator_create: Option<Box<CoreUserValidatorCreate>>,
    #[serde(rename = "CoreUserValidatorUpdate")]
    pub core_user_validator_update: Option<Box<CoreUserValidatorUpdate>>,
    #[serde(rename = "CoreUserValidatorUpsert")]
    pub core_user_validator_upsert: Option<Box<CoreUserValidatorUpsert>>,
    #[serde(rename = "CoreUserValidatorDelete")]
    pub core_user_validator_delete: Option<Box<CoreUserValidatorDelete>>,
    #[serde(rename = "CoreSchemaValidatorCreate")]
    pub core_schema_validator_create: Option<Box<CoreSchemaValidatorCreate>>,
    #[serde(rename = "CoreSchemaValidatorUpdate")]
    pub core_schema_validator_update: Option<Box<CoreSchemaValidatorUpdate>>,
    #[serde(rename = "CoreSchemaValidatorUpsert")]
    pub core_schema_validator_upsert: Option<Box<CoreSchemaValidatorUpsert>>,
    #[serde(rename = "CoreSchemaValidatorDelete")]
    pub core_schema_validator_delete: Option<Box<CoreSchemaValidatorDelete>>,
    #[serde(rename = "CoreArtifactValidatorCreate")]
    pub core_artifact_validator_create: Option<Box<CoreArtifactValidatorCreate>>,
    #[serde(rename = "CoreArtifactValidatorUpdate")]
    pub core_artifact_validator_update: Option<Box<CoreArtifactValidatorUpdate>>,
    #[serde(rename = "CoreArtifactValidatorUpsert")]
    pub core_artifact_validator_upsert: Option<Box<CoreArtifactValidatorUpsert>>,
    #[serde(rename = "CoreArtifactValidatorDelete")]
    pub core_artifact_validator_delete: Option<Box<CoreArtifactValidatorDelete>>,
    #[serde(rename = "CoreGeneratorValidatorCreate")]
    pub core_generator_validator_create: Option<Box<CoreGeneratorValidatorCreate>>,
    #[serde(rename = "CoreGeneratorValidatorUpdate")]
    pub core_generator_validator_update: Option<Box<CoreGeneratorValidatorUpdate>>,
    #[serde(rename = "CoreGeneratorValidatorUpsert")]
    pub core_generator_validator_upsert: Option<Box<CoreGeneratorValidatorUpsert>>,
    #[serde(rename = "CoreGeneratorValidatorDelete")]
    pub core_generator_validator_delete: Option<Box<CoreGeneratorValidatorDelete>>,
    #[serde(rename = "CoreCheckDefinitionCreate")]
    pub core_check_definition_create: Option<Box<CoreCheckDefinitionCreate>>,
    #[serde(rename = "CoreCheckDefinitionUpdate")]
    pub core_check_definition_update: Option<Box<CoreCheckDefinitionUpdate>>,
    #[serde(rename = "CoreCheckDefinitionUpsert")]
    pub core_check_definition_upsert: Option<Box<CoreCheckDefinitionUpsert>>,
    #[serde(rename = "CoreCheckDefinitionDelete")]
    pub core_check_definition_delete: Option<Box<CoreCheckDefinitionDelete>>,
    #[serde(rename = "CoreTransformPythonCreate")]
    pub core_transform_python_create: Option<Box<CoreTransformPythonCreate>>,
    #[serde(rename = "CoreTransformPythonUpdate")]
    pub core_transform_python_update: Option<Box<CoreTransformPythonUpdate>>,
    #[serde(rename = "CoreTransformPythonUpsert")]
    pub core_transform_python_upsert: Option<Box<CoreTransformPythonUpsert>>,
    #[serde(rename = "CoreTransformPythonDelete")]
    pub core_transform_python_delete: Option<Box<CoreTransformPythonDelete>>,
    #[serde(rename = "CoreGraphQLQueryCreate")]
    pub core_graph_q_l_query_create: Option<Box<CoreGraphQLQueryCreate>>,
    #[serde(rename = "CoreGraphQLQueryUpdate")]
    pub core_graph_q_l_query_update: Option<Box<CoreGraphQLQueryUpdate>>,
    #[serde(rename = "CoreGraphQLQueryUpsert")]
    pub core_graph_q_l_query_upsert: Option<Box<CoreGraphQLQueryUpsert>>,
    #[serde(rename = "CoreGraphQLQueryDelete")]
    pub core_graph_q_l_query_delete: Option<Box<CoreGraphQLQueryDelete>>,
    #[serde(rename = "CoreArtifactCreate")]
    pub core_artifact_create: Option<Box<CoreArtifactCreate>>,
    #[serde(rename = "CoreArtifactUpdate")]
    pub core_artifact_update: Option<Box<CoreArtifactUpdate>>,
    #[serde(rename = "CoreArtifactUpsert")]
    pub core_artifact_upsert: Option<Box<CoreArtifactUpsert>>,
    #[serde(rename = "CoreArtifactDelete")]
    pub core_artifact_delete: Option<Box<CoreArtifactDelete>>,
    #[serde(rename = "CoreArtifactDefinitionCreate")]
    pub core_artifact_definition_create: Option<Box<CoreArtifactDefinitionCreate>>,
    #[serde(rename = "CoreArtifactDefinitionUpdate")]
    pub core_artifact_definition_update: Option<Box<CoreArtifactDefinitionUpdate>>,
    #[serde(rename = "CoreArtifactDefinitionUpsert")]
    pub core_artifact_definition_upsert: Option<Box<CoreArtifactDefinitionUpsert>>,
    #[serde(rename = "CoreArtifactDefinitionDelete")]
    pub core_artifact_definition_delete: Option<Box<CoreArtifactDefinitionDelete>>,
    #[serde(rename = "CoreGeneratorDefinitionCreate")]
    pub core_generator_definition_create: Option<Box<CoreGeneratorDefinitionCreate>>,
    #[serde(rename = "CoreGeneratorDefinitionUpdate")]
    pub core_generator_definition_update: Option<Box<CoreGeneratorDefinitionUpdate>>,
    #[serde(rename = "CoreGeneratorDefinitionUpsert")]
    pub core_generator_definition_upsert: Option<Box<CoreGeneratorDefinitionUpsert>>,
    #[serde(rename = "CoreGeneratorDefinitionDelete")]
    pub core_generator_definition_delete: Option<Box<CoreGeneratorDefinitionDelete>>,
    #[serde(rename = "CoreGeneratorInstanceCreate")]
    pub core_generator_instance_create: Option<Box<CoreGeneratorInstanceCreate>>,
    #[serde(rename = "CoreGeneratorInstanceUpdate")]
    pub core_generator_instance_update: Option<Box<CoreGeneratorInstanceUpdate>>,
    #[serde(rename = "CoreGeneratorInstanceUpsert")]
    pub core_generator_instance_upsert: Option<Box<CoreGeneratorInstanceUpsert>>,
    #[serde(rename = "CoreGeneratorInstanceDelete")]
    pub core_generator_instance_delete: Option<Box<CoreGeneratorInstanceDelete>>,
    #[serde(rename = "CoreStaticKeyValueCreate")]
    pub core_static_key_value_create: Option<Box<CoreStaticKeyValueCreate>>,
    #[serde(rename = "CoreStaticKeyValueUpdate")]
    pub core_static_key_value_update: Option<Box<CoreStaticKeyValueUpdate>>,
    #[serde(rename = "CoreStaticKeyValueUpsert")]
    pub core_static_key_value_upsert: Option<Box<CoreStaticKeyValueUpsert>>,
    #[serde(rename = "CoreStaticKeyValueDelete")]
    pub core_static_key_value_delete: Option<Box<CoreStaticKeyValueDelete>>,
    #[serde(rename = "CoreEnvKeyValueCreate")]
    pub core_env_key_value_create: Option<Box<CoreEnvKeyValueCreate>>,
    #[serde(rename = "CoreEnvKeyValueUpdate")]
    pub core_env_key_value_update: Option<Box<CoreEnvKeyValueUpdate>>,
    #[serde(rename = "CoreEnvKeyValueUpsert")]
    pub core_env_key_value_upsert: Option<Box<CoreEnvKeyValueUpsert>>,
    #[serde(rename = "CoreEnvKeyValueDelete")]
    pub core_env_key_value_delete: Option<Box<CoreEnvKeyValueDelete>>,
    #[serde(rename = "CoreStandardWebhookCreate")]
    pub core_standard_webhook_create: Option<Box<CoreStandardWebhookCreate>>,
    #[serde(rename = "CoreStandardWebhookUpdate")]
    pub core_standard_webhook_update: Option<Box<CoreStandardWebhookUpdate>>,
    #[serde(rename = "CoreStandardWebhookUpsert")]
    pub core_standard_webhook_upsert: Option<Box<CoreStandardWebhookUpsert>>,
    #[serde(rename = "CoreStandardWebhookDelete")]
    pub core_standard_webhook_delete: Option<Box<CoreStandardWebhookDelete>>,
    #[serde(rename = "CoreCustomWebhookCreate")]
    pub core_custom_webhook_create: Option<Box<CoreCustomWebhookCreate>>,
    #[serde(rename = "CoreCustomWebhookUpdate")]
    pub core_custom_webhook_update: Option<Box<CoreCustomWebhookUpdate>>,
    #[serde(rename = "CoreCustomWebhookUpsert")]
    pub core_custom_webhook_upsert: Option<Box<CoreCustomWebhookUpsert>>,
    #[serde(rename = "CoreCustomWebhookDelete")]
    pub core_custom_webhook_delete: Option<Box<CoreCustomWebhookDelete>>,
    #[serde(rename = "IpamNamespaceCreate")]
    pub ipam_namespace_create: Option<Box<IpamNamespaceCreate>>,
    #[serde(rename = "IpamNamespaceUpdate")]
    pub ipam_namespace_update: Option<Box<IpamNamespaceUpdate>>,
    #[serde(rename = "IpamNamespaceUpsert")]
    pub ipam_namespace_upsert: Option<Box<IpamNamespaceUpsert>>,
    #[serde(rename = "IpamNamespaceDelete")]
    pub ipam_namespace_delete: Option<Box<IpamNamespaceDelete>>,
    #[serde(rename = "CoreIPPrefixPoolCreate")]
    pub core_i_p_prefix_pool_create: Option<Box<CoreIPPrefixPoolCreate>>,
    #[serde(rename = "CoreIPPrefixPoolUpdate")]
    pub core_i_p_prefix_pool_update: Option<Box<CoreIPPrefixPoolUpdate>>,
    #[serde(rename = "CoreIPPrefixPoolUpsert")]
    pub core_i_p_prefix_pool_upsert: Option<Box<CoreIPPrefixPoolUpsert>>,
    #[serde(rename = "CoreIPPrefixPoolDelete")]
    pub core_i_p_prefix_pool_delete: Option<Box<CoreIPPrefixPoolDelete>>,
    #[serde(rename = "CoreIPAddressPoolCreate")]
    pub core_i_p_address_pool_create: Option<Box<CoreIPAddressPoolCreate>>,
    #[serde(rename = "CoreIPAddressPoolUpdate")]
    pub core_i_p_address_pool_update: Option<Box<CoreIPAddressPoolUpdate>>,
    #[serde(rename = "CoreIPAddressPoolUpsert")]
    pub core_i_p_address_pool_upsert: Option<Box<CoreIPAddressPoolUpsert>>,
    #[serde(rename = "CoreIPAddressPoolDelete")]
    pub core_i_p_address_pool_delete: Option<Box<CoreIPAddressPoolDelete>>,
    #[serde(rename = "CoreNumberPoolCreate")]
    pub core_number_pool_create: Option<Box<CoreNumberPoolCreate>>,
    #[serde(rename = "CoreNumberPoolUpdate")]
    pub core_number_pool_update: Option<Box<CoreNumberPoolUpdate>>,
    #[serde(rename = "CoreNumberPoolUpsert")]
    pub core_number_pool_upsert: Option<Box<CoreNumberPoolUpsert>>,
    #[serde(rename = "CoreNumberPoolDelete")]
    pub core_number_pool_delete: Option<Box<CoreNumberPoolDelete>>,
    #[serde(rename = "CoreGlobalPermissionCreate")]
    pub core_global_permission_create: Option<Box<CoreGlobalPermissionCreate>>,
    #[serde(rename = "CoreGlobalPermissionUpdate")]
    pub core_global_permission_update: Option<Box<CoreGlobalPermissionUpdate>>,
    #[serde(rename = "CoreGlobalPermissionUpsert")]
    pub core_global_permission_upsert: Option<Box<CoreGlobalPermissionUpsert>>,
    #[serde(rename = "CoreGlobalPermissionDelete")]
    pub core_global_permission_delete: Option<Box<CoreGlobalPermissionDelete>>,
    #[serde(rename = "CoreObjectPermissionCreate")]
    pub core_object_permission_create: Option<Box<CoreObjectPermissionCreate>>,
    #[serde(rename = "CoreObjectPermissionUpdate")]
    pub core_object_permission_update: Option<Box<CoreObjectPermissionUpdate>>,
    #[serde(rename = "CoreObjectPermissionUpsert")]
    pub core_object_permission_upsert: Option<Box<CoreObjectPermissionUpsert>>,
    #[serde(rename = "CoreObjectPermissionDelete")]
    pub core_object_permission_delete: Option<Box<CoreObjectPermissionDelete>>,
    #[serde(rename = "CoreAccountRoleCreate")]
    pub core_account_role_create: Option<Box<CoreAccountRoleCreate>>,
    #[serde(rename = "CoreAccountRoleUpdate")]
    pub core_account_role_update: Option<Box<CoreAccountRoleUpdate>>,
    #[serde(rename = "CoreAccountRoleUpsert")]
    pub core_account_role_upsert: Option<Box<CoreAccountRoleUpsert>>,
    #[serde(rename = "CoreAccountRoleDelete")]
    pub core_account_role_delete: Option<Box<CoreAccountRoleDelete>>,
    #[serde(rename = "CoreAccountGroupCreate")]
    pub core_account_group_create: Option<Box<CoreAccountGroupCreate>>,
    #[serde(rename = "CoreAccountGroupUpdate")]
    pub core_account_group_update: Option<Box<CoreAccountGroupUpdate>>,
    #[serde(rename = "CoreAccountGroupUpsert")]
    pub core_account_group_upsert: Option<Box<CoreAccountGroupUpsert>>,
    #[serde(rename = "CoreAccountGroupDelete")]
    pub core_account_group_delete: Option<Box<CoreAccountGroupDelete>>,
    #[serde(rename = "CoreProfileUpdate")]
    pub core_profile_update: Option<Box<CoreProfileUpdate>>,
    #[serde(rename = "CoreActionUpdate")]
    pub core_action_update: Option<Box<CoreActionUpdate>>,
    #[serde(rename = "CoreTriggerRuleUpdate")]
    pub core_trigger_rule_update: Option<Box<CoreTriggerRuleUpdate>>,
    #[serde(rename = "CoreNodeTriggerMatchUpdate")]
    pub core_node_trigger_match_update: Option<Box<CoreNodeTriggerMatchUpdate>>,
    #[serde(rename = "CoreNodeUpdate")]
    pub core_node_update: Option<Box<CoreNodeUpdate>>,
    #[serde(rename = "CoreCommentUpdate")]
    pub core_comment_update: Option<Box<CoreCommentUpdate>>,
    #[serde(rename = "CoreThreadUpdate")]
    pub core_thread_update: Option<Box<CoreThreadUpdate>>,
    #[serde(rename = "CoreGroupUpdate")]
    pub core_group_update: Option<Box<CoreGroupUpdate>>,
    #[serde(rename = "CoreValidatorUpdate")]
    pub core_validator_update: Option<Box<CoreValidatorUpdate>>,
    #[serde(rename = "CoreCheckUpdate")]
    pub core_check_update: Option<Box<CoreCheckUpdate>>,
    #[serde(rename = "CoreTransformationUpdate")]
    pub core_transformation_update: Option<Box<CoreTransformationUpdate>>,
    #[serde(rename = "CoreArtifactTargetUpdate")]
    pub core_artifact_target_update: Option<Box<CoreArtifactTargetUpdate>>,
    #[serde(rename = "CoreFileObjectUpdate")]
    pub core_file_object_update: Option<Box<CoreFileObjectUpdate>>,
    #[serde(rename = "CoreTaskTargetUpdate")]
    pub core_task_target_update: Option<Box<CoreTaskTargetUpdate>>,
    #[serde(rename = "CoreKeyValueUpdate")]
    pub core_key_value_update: Option<Box<CoreKeyValueUpdate>>,
    #[serde(rename = "CoreWebhookUpdate")]
    pub core_webhook_update: Option<Box<CoreWebhookUpdate>>,
    #[serde(rename = "CoreGenericRepositoryUpdate")]
    pub core_generic_repository_update: Option<Box<CoreGenericRepositoryUpdate>>,
    #[serde(rename = "BuiltinIPNamespaceUpdate")]
    pub builtin_i_p_namespace_update: Option<Box<BuiltinIPNamespaceUpdate>>,
    #[serde(rename = "BuiltinIPPrefixUpdate")]
    pub builtin_i_p_prefix_update: Option<Box<BuiltinIPPrefixUpdate>>,
    #[serde(rename = "BuiltinIPAddressUpdate")]
    pub builtin_i_p_address_update: Option<Box<BuiltinIPAddressUpdate>>,
    #[serde(rename = "CoreResourcePoolUpdate")]
    pub core_resource_pool_update: Option<Box<CoreResourcePoolUpdate>>,
    #[serde(rename = "CoreWeightedPoolResourceUpdate")]
    pub core_weighted_pool_resource_update: Option<Box<CoreWeightedPoolResourceUpdate>>,
    #[serde(rename = "CoreGenericAccountUpdate")]
    pub core_generic_account_update: Option<Box<CoreGenericAccountUpdate>>,
    #[serde(rename = "CoreBasePermissionUpdate")]
    pub core_base_permission_update: Option<Box<CoreBasePermissionUpdate>>,
    #[serde(rename = "CoreCredentialUpdate")]
    pub core_credential_update: Option<Box<CoreCredentialUpdate>>,
    #[serde(rename = "CoreObjectTemplateUpdate")]
    pub core_object_template_update: Option<Box<CoreObjectTemplateUpdate>>,
    #[serde(rename = "CoreObjectComponentTemplateUpdate")]
    pub core_object_component_template_update: Option<Box<CoreObjectComponentTemplateUpdate>>,
    #[serde(rename = "CoreMenuUpdate")]
    pub core_menu_update: Option<Box<CoreMenuUpdate>>,
    #[serde(rename = "ProfileBuiltinTagCreate")]
    pub profile_builtin_tag_create: Option<Box<ProfileBuiltinTagCreate>>,
    #[serde(rename = "ProfileBuiltinTagUpdate")]
    pub profile_builtin_tag_update: Option<Box<ProfileBuiltinTagUpdate>>,
    #[serde(rename = "ProfileBuiltinTagUpsert")]
    pub profile_builtin_tag_upsert: Option<Box<ProfileBuiltinTagUpsert>>,
    #[serde(rename = "ProfileBuiltinTagDelete")]
    pub profile_builtin_tag_delete: Option<Box<ProfileBuiltinTagDelete>>,
    #[serde(rename = "ProfileIpamNamespaceCreate")]
    pub profile_ipam_namespace_create: Option<Box<ProfileIpamNamespaceCreate>>,
    #[serde(rename = "ProfileIpamNamespaceUpdate")]
    pub profile_ipam_namespace_update: Option<Box<ProfileIpamNamespaceUpdate>>,
    #[serde(rename = "ProfileIpamNamespaceUpsert")]
    pub profile_ipam_namespace_upsert: Option<Box<ProfileIpamNamespaceUpsert>>,
    #[serde(rename = "ProfileIpamNamespaceDelete")]
    pub profile_ipam_namespace_delete: Option<Box<ProfileIpamNamespaceDelete>>,
    #[serde(rename = "ProfileBuiltinIPPrefixCreate")]
    pub profile_builtin_i_p_prefix_create: Option<Box<ProfileBuiltinIPPrefixCreate>>,
    #[serde(rename = "ProfileBuiltinIPPrefixUpdate")]
    pub profile_builtin_i_p_prefix_update: Option<Box<ProfileBuiltinIPPrefixUpdate>>,
    #[serde(rename = "ProfileBuiltinIPPrefixUpsert")]
    pub profile_builtin_i_p_prefix_upsert: Option<Box<ProfileBuiltinIPPrefixUpsert>>,
    #[serde(rename = "ProfileBuiltinIPPrefixDelete")]
    pub profile_builtin_i_p_prefix_delete: Option<Box<ProfileBuiltinIPPrefixDelete>>,
    #[serde(rename = "ProfileBuiltinIPAddressCreate")]
    pub profile_builtin_i_p_address_create: Option<Box<ProfileBuiltinIPAddressCreate>>,
    #[serde(rename = "ProfileBuiltinIPAddressUpdate")]
    pub profile_builtin_i_p_address_update: Option<Box<ProfileBuiltinIPAddressUpdate>>,
    #[serde(rename = "ProfileBuiltinIPAddressUpsert")]
    pub profile_builtin_i_p_address_upsert: Option<Box<ProfileBuiltinIPAddressUpsert>>,
    #[serde(rename = "ProfileBuiltinIPAddressDelete")]
    pub profile_builtin_i_p_address_delete: Option<Box<ProfileBuiltinIPAddressDelete>>,
    #[serde(rename = "InfrahubAccountTokenCreate")]
    pub infrahub_account_token_create: Option<Box<InfrahubAccountTokenCreate>>,
    #[serde(rename = "InfrahubAccountSelfUpdate")]
    pub infrahub_account_self_update: Option<Box<InfrahubAccountSelfUpdate>>,
    #[serde(rename = "InfrahubAccountTokenDelete")]
    pub infrahub_account_token_delete: Option<Box<InfrahubAccountTokenDelete>>,
    #[serde(rename = "CoreProposedChangeRunCheck")]
    pub core_proposed_change_run_check: Option<Box<ProposedChangeRequestRunCheck>>,
    #[serde(rename = "CoreProposedChangeMerge")]
    pub core_proposed_change_merge: Option<Box<ProposedChangeMerge>>,
    #[serde(rename = "CoreProposedChangeReview")]
    pub core_proposed_change_review: Option<Box<ProposedChangeReview>>,
    #[serde(rename = "CoreGeneratorDefinitionRun")]
    pub core_generator_definition_run: Option<Box<GeneratorDefinitionRequestRun>>,
    #[serde(rename = "InfrahubIPPrefixPoolGetResource")]
    pub infrahub_i_p_prefix_pool_get_resource: Option<Box<IPPrefixPoolGetResource>>,
    #[serde(rename = "InfrahubIPAddressPoolGetResource")]
    pub infrahub_i_p_address_pool_get_resource: Option<Box<IPAddressPoolGetResource>>,
    #[serde(rename = "BranchCreate")]
    pub branch_create: Option<Box<BranchCreate>>,
    #[serde(rename = "BranchDelete")]
    pub branch_delete: Option<Box<BranchDelete>>,
    #[serde(rename = "BranchRebase")]
    pub branch_rebase: Option<Box<BranchRebase>>,
    #[serde(rename = "BranchMerge")]
    pub branch_merge: Option<Box<BranchMerge>>,
    #[serde(rename = "BranchUpdate")]
    pub branch_update: Option<Box<BranchUpdate>>,
    #[serde(rename = "BranchValidate")]
    pub branch_validate: Option<Box<BranchValidate>>,
    #[serde(rename = "DiffUpdate")]
    pub diff_update: Option<Box<DiffUpdateMutation>>,
    #[serde(rename = "InfrahubReadOnlyRepositoryImportLastCommit")]
    pub infrahub_read_only_repository_import_last_commit: Option<Box<ReadOnlyRepositoryImportLastCommit>>,
    #[serde(rename = "InfrahubRepositoryProcess")]
    pub infrahub_repository_process: Option<Box<ProcessRepository>>,
    #[serde(rename = "InfrahubRepositoryConnectivity")]
    pub infrahub_repository_connectivity: Option<Box<ValidateRepositoryConnectivity>>,
    #[serde(rename = "InfrahubUpdateComputedAttribute")]
    pub infrahub_update_computed_attribute: Option<Box<UpdateComputedAttribute>>,
    #[serde(rename = "InfrahubUpdateDisplayLabel")]
    pub infrahub_update_display_label: Option<Box<UpdateDisplayLabel>>,
    #[serde(rename = "InfrahubUpdateHFID")]
    pub infrahub_update_h_f_i_d: Option<Box<UpdateHFID>>,
    #[serde(rename = "InfrahubRecomputeComputedAttribute")]
    pub infrahub_recompute_computed_attribute: Option<Box<RecomputeComputedAttribute>>,
    #[serde(rename = "RelationshipAdd")]
    pub relationship_add: Option<Box<RelationshipAdd>>,
    #[serde(rename = "RelationshipRemove")]
    pub relationship_remove: Option<Box<RelationshipRemove>>,
    #[serde(rename = "SchemaDropdownAdd")]
    pub schema_dropdown_add: Option<Box<SchemaDropdownAdd>>,
    #[serde(rename = "SchemaDropdownRemove")]
    pub schema_dropdown_remove: Option<Box<SchemaDropdownRemove>>,
    #[serde(rename = "SchemaEnumAdd")]
    pub schema_enum_add: Option<Box<SchemaEnumAdd>>,
    #[serde(rename = "SchemaEnumRemove")]
    pub schema_enum_remove: Option<Box<SchemaEnumRemove>>,
    #[serde(rename = "ResolveDiffConflict")]
    pub resolve_diff_conflict: Option<Box<ResolveDiffConflict>>,
    #[serde(rename = "ConvertObjectType")]
    pub convert_object_type: Option<Box<ConvertObjectType>>,
    #[serde(rename = "CoreProposedChangeCheckForApprovalRevoke")]
    pub core_proposed_change_check_for_approval_revoke: Option<Box<ProposedChangeCheckForApprovalRevoke>>,
    #[serde(rename = "InfrahubProfilesRefresh")]
    pub infrahub_profiles_refresh: Option<Box<InfrahubProfilesRefresh>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedBuiltinIPAddress {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedBuiltinIPNamespace {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedBuiltinIPPrefix {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedBuiltinTag {
    pub node: Option<Box<BuiltinTag>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreAccount {
    pub node: Option<Box<CoreAccount>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreAccountGroup {
    pub node: Option<Box<CoreAccountGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreAccountRole {
    pub node: Option<Box<CoreAccountRole>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreAction {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreArtifact {
    pub node: Option<Box<CoreArtifact>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreArtifactCheck {
    pub node: Option<Box<CoreArtifactCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreArtifactDefinition {
    pub node: Option<Box<CoreArtifactDefinition>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreArtifactTarget {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreArtifactThread {
    pub node: Option<Box<CoreArtifactThread>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreArtifactValidator {
    pub node: Option<Box<CoreArtifactValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreBasePermission {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreChangeComment {
    pub node: Option<Box<CoreChangeComment>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreChangeThread {
    pub node: Option<Box<CoreChangeThread>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreCheck {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreCheckDefinition {
    pub node: Option<Box<CoreCheckDefinition>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreComment {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreCredential {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreCustomWebhook {
    pub node: Option<Box<CoreCustomWebhook>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreDataCheck {
    pub node: Option<Box<CoreDataCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreDataValidator {
    pub node: Option<Box<CoreDataValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreEnvKeyValue {
    pub node: Option<Box<CoreEnvKeyValue>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreFileCheck {
    pub node: Option<Box<CoreFileCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreFileObject {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreFileThread {
    pub node: Option<Box<CoreFileThread>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGeneratorAction {
    pub node: Option<Box<CoreGeneratorAction>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGeneratorAwareGroup {
    pub node: Option<Box<CoreGeneratorAwareGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGeneratorCheck {
    pub node: Option<Box<CoreGeneratorCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGeneratorDefinition {
    pub node: Option<Box<CoreGeneratorDefinition>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGeneratorGroup {
    pub node: Option<Box<CoreGeneratorGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGeneratorInstance {
    pub node: Option<Box<CoreGeneratorInstance>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGeneratorValidator {
    pub node: Option<Box<CoreGeneratorValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGenericAccount {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGenericRepository {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGlobalPermission {
    pub node: Option<Box<CoreGlobalPermission>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGraphQLQuery {
    pub node: Option<Box<CoreGraphQLQuery>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGraphQLQueryGroup {
    pub node: Option<Box<CoreGraphQLQueryGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGroup {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGroupAction {
    pub node: Option<Box<CoreGroupAction>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreGroupTriggerRule {
    pub node: Option<Box<CoreGroupTriggerRule>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreIPAddressPool {
    pub node: Option<Box<CoreIPAddressPool>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreIPPrefixPool {
    pub node: Option<Box<CoreIPPrefixPool>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreKeyValue {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreMenu {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreMenuItem {
    pub node: Option<Box<CoreMenuItem>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreNode {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreNodeTriggerAttributeMatch {
    pub node: Option<Box<CoreNodeTriggerAttributeMatch>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreNodeTriggerMatch {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreNodeTriggerRelationshipMatch {
    pub node: Option<Box<CoreNodeTriggerRelationshipMatch>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreNodeTriggerRule {
    pub node: Option<Box<CoreNodeTriggerRule>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreNumberPool {
    pub node: Option<Box<CoreNumberPool>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreObjectComponentTemplate {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreObjectPermission {
    pub node: Option<Box<CoreObjectPermission>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreObjectTemplate {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreObjectThread {
    pub node: Option<Box<CoreObjectThread>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCorePasswordCredential {
    pub node: Option<Box<CorePasswordCredential>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreProfile {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreProposedChange {
    pub node: Option<Box<CoreProposedChange>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreReadOnlyRepository {
    pub node: Option<Box<CoreReadOnlyRepository>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreRepository {
    pub node: Option<Box<CoreRepository>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreRepositoryGroup {
    pub node: Option<Box<CoreRepositoryGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreRepositoryValidator {
    pub node: Option<Box<CoreRepositoryValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreResourcePool {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreSchemaCheck {
    pub node: Option<Box<CoreSchemaCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreSchemaValidator {
    pub node: Option<Box<CoreSchemaValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreStandardCheck {
    pub node: Option<Box<CoreStandardCheck>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreStandardGroup {
    pub node: Option<Box<CoreStandardGroup>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreStandardWebhook {
    pub node: Option<Box<CoreStandardWebhook>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreStaticKeyValue {
    pub node: Option<Box<CoreStaticKeyValue>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreTaskTarget {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreThread {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreThreadComment {
    pub node: Option<Box<CoreThreadComment>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreTransformJinja2 {
    pub node: Option<Box<CoreTransformJinja2>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreTransformPython {
    pub node: Option<Box<CoreTransformPython>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreTransformation {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreTriggerRule {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreUserValidator {
    pub node: Option<Box<CoreUserValidator>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreValidator {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreWebhook {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedCoreWeightedPoolResource {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedInternalAccountToken {
    pub node: Option<Box<InternalAccountToken>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedInternalExternalIdentity {
    pub node: Option<Box<InternalExternalIdentity>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedInternalIPPrefixAvailable {
    pub node: Option<Box<InternalIPPrefixAvailable>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedInternalIPRangeAvailable {
    pub node: Option<Box<InternalIPRangeAvailable>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedInternalRefreshToken {
    pub node: Option<Box<InternalRefreshToken>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedIpamNamespace {
    pub node: Option<Box<IpamNamespace>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedLineageOwner {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedLineageSource {
    pub node: Option<serde_json::Value>,
    pub node_metadata: Box<InfrahubNodeMetadata>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedProfileBuiltinIPAddress {
    pub node: Option<Box<ProfileBuiltinIPAddress>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedProfileBuiltinIPPrefix {
    pub node: Option<Box<ProfileBuiltinIPPrefix>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedProfileBuiltinTag {
    pub node: Option<Box<ProfileBuiltinTag>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedEdgedProfileIpamNamespace {
    pub node: Option<Box<ProfileIpamNamespace>>,
    pub node_metadata: Option<Box<InfrahubNodeMetadata>>,
    pub properties: Option<Box<RelationshipProperty>>,
    pub relationship_metadata: Option<Box<InfrahubRelationshipMetadata>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedBuiltinIPAddress {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedBuiltinIPAddress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedBuiltinIPNamespace {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedBuiltinIPNamespace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedBuiltinIPPrefix {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedBuiltinIPPrefix>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedBuiltinTag {
    pub count: i64,
    pub edges: Vec<NestedEdgedBuiltinTag>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreAccount {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreAccount>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreAccountGroup {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreAccountGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreAccountRole {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreAccountRole>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreAction {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreArtifact {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreArtifact>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreArtifactCheck {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreArtifactCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreArtifactDefinition {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreArtifactDefinition>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreArtifactTarget {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreArtifactTarget>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreArtifactThread {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreArtifactThread>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreArtifactValidator {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreArtifactValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreBasePermission {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreBasePermission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreChangeComment {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreChangeComment>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreChangeThread {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreChangeThread>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreCheck {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreCheck>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreCheckDefinition {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreCheckDefinition>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreComment {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreComment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreCredential {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreCredential>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreCustomWebhook {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreCustomWebhook>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreDataCheck {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreDataCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreDataValidator {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreDataValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreEnvKeyValue {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreEnvKeyValue>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreFileCheck {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreFileCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreFileObject {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreFileObject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreFileThread {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreFileThread>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGeneratorAction {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGeneratorAction>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGeneratorAwareGroup {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGeneratorAwareGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGeneratorCheck {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGeneratorCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGeneratorDefinition {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGeneratorDefinition>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGeneratorGroup {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGeneratorGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGeneratorInstance {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGeneratorInstance>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGeneratorValidator {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGeneratorValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGenericAccount {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreGenericAccount>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGenericRepository {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreGenericRepository>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGlobalPermission {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGlobalPermission>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGraphQLQuery {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGraphQLQuery>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGraphQLQueryGroup {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGraphQLQueryGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGroup {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGroupAction {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGroupAction>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreGroupTriggerRule {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreGroupTriggerRule>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreIPAddressPool {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreIPAddressPool>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreIPPrefixPool {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreIPPrefixPool>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreKeyValue {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreKeyValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreMenu {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreMenu>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreMenuItem {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreMenuItem>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreNode {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreNodeTriggerAttributeMatch {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreNodeTriggerAttributeMatch>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreNodeTriggerMatch {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreNodeTriggerMatch>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreNodeTriggerRelationshipMatch {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreNodeTriggerRelationshipMatch>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreNodeTriggerRule {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreNodeTriggerRule>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreNumberPool {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreNumberPool>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreObjectComponentTemplate {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreObjectComponentTemplate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreObjectPermission {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreObjectPermission>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreObjectTemplate {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreObjectTemplate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreObjectThread {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreObjectThread>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCorePasswordCredential {
    pub count: i64,
    pub edges: Vec<NestedEdgedCorePasswordCredential>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreProfile {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreProfile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreProposedChange {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreProposedChange>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreReadOnlyRepository {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreReadOnlyRepository>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreRepository {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreRepository>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreRepositoryGroup {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreRepositoryGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreRepositoryValidator {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreRepositoryValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreResourcePool {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreResourcePool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreSchemaCheck {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreSchemaCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreSchemaValidator {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreSchemaValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreStandardCheck {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreStandardCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreStandardGroup {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreStandardGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreStandardWebhook {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreStandardWebhook>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreStaticKeyValue {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreStaticKeyValue>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreTaskTarget {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreTaskTarget>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreThread {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreThread>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreThreadComment {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreThreadComment>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreTransformJinja2 {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreTransformJinja2>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreTransformPython {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreTransformPython>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreTransformation {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreTransformation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreTriggerRule {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreTriggerRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreUserValidator {
    pub count: i64,
    pub edges: Vec<NestedEdgedCoreUserValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreValidator {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreWebhook {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreWebhook>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedCoreWeightedPoolResource {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedCoreWeightedPoolResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedInternalAccountToken {
    pub count: i64,
    pub edges: Vec<NestedEdgedInternalAccountToken>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedInternalExternalIdentity {
    pub count: i64,
    pub edges: Vec<NestedEdgedInternalExternalIdentity>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedInternalIPPrefixAvailable {
    pub count: i64,
    pub edges: Vec<NestedEdgedInternalIPPrefixAvailable>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedInternalIPRangeAvailable {
    pub count: i64,
    pub edges: Vec<NestedEdgedInternalIPRangeAvailable>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedInternalRefreshToken {
    pub count: i64,
    pub edges: Vec<NestedEdgedInternalRefreshToken>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedIpamNamespace {
    pub count: i64,
    pub edges: Vec<NestedEdgedIpamNamespace>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedLineageOwner {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedLineageOwner>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedLineageSource {
    pub count: i64,
    pub edges: Option<Vec<NestedEdgedLineageSource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedProfileBuiltinIPAddress {
    pub count: i64,
    pub edges: Vec<NestedEdgedProfileBuiltinIPAddress>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedProfileBuiltinIPPrefix {
    pub count: i64,
    pub edges: Vec<NestedEdgedProfileBuiltinIPPrefix>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedProfileBuiltinTag {
    pub count: i64,
    pub edges: Vec<NestedEdgedProfileBuiltinTag>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedPaginatedProfileIpamNamespace {
    pub count: i64,
    pub edges: Vec<NestedEdgedProfileIpamNamespace>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEdge {
    pub node: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEdges {
    pub count: i64,
    pub edges: Vec<NodeEdge>,
    pub parent_prefixes: Option<Vec<NodeEdge>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMutatedEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub payload: serde_json::Value,
    pub attributes: Vec<InfrahubMutatedAttribute>,
    pub relationships: Vec<InfrahubMutatedRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonRequiredBooleanValueField {
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonRequiredIntValueField {
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonRequiredStringValueField {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberAttribute {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub value: Option<i64>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPermission {
    pub kind: String,
    pub view: BranchRelativePermissionDecision,
    pub create: BranchRelativePermissionDecision,
    pub update: BranchRelativePermissionDecision,
    pub delete: BranchRelativePermissionDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPermissionNode {
    pub node: Box<ObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedBuiltinIPAddress {
    pub count: i64,
    pub edges: Vec<EdgedBuiltinIPAddress>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedBuiltinIPNamespace {
    pub count: i64,
    pub edges: Vec<EdgedBuiltinIPNamespace>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedBuiltinIPPrefix {
    pub count: i64,
    pub edges: Vec<EdgedBuiltinIPPrefix>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedBuiltinTag {
    pub count: i64,
    pub edges: Vec<EdgedBuiltinTag>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreAccount {
    pub count: i64,
    pub edges: Vec<EdgedCoreAccount>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreAccountGroup {
    pub count: i64,
    pub edges: Vec<EdgedCoreAccountGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreAccountRole {
    pub count: i64,
    pub edges: Vec<EdgedCoreAccountRole>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreAction {
    pub count: i64,
    pub edges: Vec<EdgedCoreAction>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreArtifact {
    pub count: i64,
    pub edges: Vec<EdgedCoreArtifact>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreArtifactCheck {
    pub count: i64,
    pub edges: Vec<EdgedCoreArtifactCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreArtifactDefinition {
    pub count: i64,
    pub edges: Vec<EdgedCoreArtifactDefinition>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreArtifactTarget {
    pub count: i64,
    pub edges: Vec<EdgedCoreArtifactTarget>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreArtifactThread {
    pub count: i64,
    pub edges: Vec<EdgedCoreArtifactThread>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreArtifactValidator {
    pub count: i64,
    pub edges: Vec<EdgedCoreArtifactValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreBasePermission {
    pub count: i64,
    pub edges: Vec<EdgedCoreBasePermission>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreChangeComment {
    pub count: i64,
    pub edges: Vec<EdgedCoreChangeComment>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreChangeThread {
    pub count: i64,
    pub edges: Vec<EdgedCoreChangeThread>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreCheck {
    pub count: i64,
    pub edges: Vec<EdgedCoreCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreCheckDefinition {
    pub count: i64,
    pub edges: Vec<EdgedCoreCheckDefinition>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreComment {
    pub count: i64,
    pub edges: Vec<EdgedCoreComment>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreCredential {
    pub count: i64,
    pub edges: Vec<EdgedCoreCredential>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreCustomWebhook {
    pub count: i64,
    pub edges: Vec<EdgedCoreCustomWebhook>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreDataCheck {
    pub count: i64,
    pub edges: Vec<EdgedCoreDataCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreDataValidator {
    pub count: i64,
    pub edges: Vec<EdgedCoreDataValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreEnvKeyValue {
    pub count: i64,
    pub edges: Vec<EdgedCoreEnvKeyValue>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreFileCheck {
    pub count: i64,
    pub edges: Vec<EdgedCoreFileCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreFileObject {
    pub count: i64,
    pub edges: Vec<EdgedCoreFileObject>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreFileThread {
    pub count: i64,
    pub edges: Vec<EdgedCoreFileThread>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGeneratorAction {
    pub count: i64,
    pub edges: Vec<EdgedCoreGeneratorAction>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGeneratorAwareGroup {
    pub count: i64,
    pub edges: Vec<EdgedCoreGeneratorAwareGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGeneratorCheck {
    pub count: i64,
    pub edges: Vec<EdgedCoreGeneratorCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGeneratorDefinition {
    pub count: i64,
    pub edges: Vec<EdgedCoreGeneratorDefinition>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGeneratorGroup {
    pub count: i64,
    pub edges: Vec<EdgedCoreGeneratorGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGeneratorInstance {
    pub count: i64,
    pub edges: Vec<EdgedCoreGeneratorInstance>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGeneratorValidator {
    pub count: i64,
    pub edges: Vec<EdgedCoreGeneratorValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGenericAccount {
    pub count: i64,
    pub edges: Vec<EdgedCoreGenericAccount>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGenericRepository {
    pub count: i64,
    pub edges: Vec<EdgedCoreGenericRepository>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGlobalPermission {
    pub count: i64,
    pub edges: Vec<EdgedCoreGlobalPermission>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGraphQLQuery {
    pub count: i64,
    pub edges: Vec<EdgedCoreGraphQLQuery>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGraphQLQueryGroup {
    pub count: i64,
    pub edges: Vec<EdgedCoreGraphQLQueryGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGroup {
    pub count: i64,
    pub edges: Vec<EdgedCoreGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGroupAction {
    pub count: i64,
    pub edges: Vec<EdgedCoreGroupAction>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreGroupTriggerRule {
    pub count: i64,
    pub edges: Vec<EdgedCoreGroupTriggerRule>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreIPAddressPool {
    pub count: i64,
    pub edges: Vec<EdgedCoreIPAddressPool>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreIPPrefixPool {
    pub count: i64,
    pub edges: Vec<EdgedCoreIPPrefixPool>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreKeyValue {
    pub count: i64,
    pub edges: Vec<EdgedCoreKeyValue>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreMenu {
    pub count: i64,
    pub edges: Vec<EdgedCoreMenu>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreMenuItem {
    pub count: i64,
    pub edges: Vec<EdgedCoreMenuItem>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreNode {
    pub count: i64,
    pub edges: Vec<EdgedCoreNode>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreNodeTriggerAttributeMatch {
    pub count: i64,
    pub edges: Vec<EdgedCoreNodeTriggerAttributeMatch>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreNodeTriggerMatch {
    pub count: i64,
    pub edges: Vec<EdgedCoreNodeTriggerMatch>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreNodeTriggerRelationshipMatch {
    pub count: i64,
    pub edges: Vec<EdgedCoreNodeTriggerRelationshipMatch>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreNodeTriggerRule {
    pub count: i64,
    pub edges: Vec<EdgedCoreNodeTriggerRule>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreNumberPool {
    pub count: i64,
    pub edges: Vec<EdgedCoreNumberPool>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreObjectComponentTemplate {
    pub count: i64,
    pub edges: Vec<EdgedCoreObjectComponentTemplate>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreObjectPermission {
    pub count: i64,
    pub edges: Vec<EdgedCoreObjectPermission>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreObjectTemplate {
    pub count: i64,
    pub edges: Vec<EdgedCoreObjectTemplate>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreObjectThread {
    pub count: i64,
    pub edges: Vec<EdgedCoreObjectThread>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCorePasswordCredential {
    pub count: i64,
    pub edges: Vec<EdgedCorePasswordCredential>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreProfile {
    pub count: i64,
    pub edges: Vec<EdgedCoreProfile>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreProposedChange {
    pub count: i64,
    pub edges: Vec<EdgedCoreProposedChange>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreReadOnlyRepository {
    pub count: i64,
    pub edges: Vec<EdgedCoreReadOnlyRepository>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreRepository {
    pub count: i64,
    pub edges: Vec<EdgedCoreRepository>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreRepositoryGroup {
    pub count: i64,
    pub edges: Vec<EdgedCoreRepositoryGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreRepositoryValidator {
    pub count: i64,
    pub edges: Vec<EdgedCoreRepositoryValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreResourcePool {
    pub count: i64,
    pub edges: Vec<EdgedCoreResourcePool>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreSchemaCheck {
    pub count: i64,
    pub edges: Vec<EdgedCoreSchemaCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreSchemaValidator {
    pub count: i64,
    pub edges: Vec<EdgedCoreSchemaValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreStandardCheck {
    pub count: i64,
    pub edges: Vec<EdgedCoreStandardCheck>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreStandardGroup {
    pub count: i64,
    pub edges: Vec<EdgedCoreStandardGroup>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreStandardWebhook {
    pub count: i64,
    pub edges: Vec<EdgedCoreStandardWebhook>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreStaticKeyValue {
    pub count: i64,
    pub edges: Vec<EdgedCoreStaticKeyValue>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreTaskTarget {
    pub count: i64,
    pub edges: Vec<EdgedCoreTaskTarget>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreThread {
    pub count: i64,
    pub edges: Vec<EdgedCoreThread>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreThreadComment {
    pub count: i64,
    pub edges: Vec<EdgedCoreThreadComment>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreTransformJinja2 {
    pub count: i64,
    pub edges: Vec<EdgedCoreTransformJinja2>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreTransformPython {
    pub count: i64,
    pub edges: Vec<EdgedCoreTransformPython>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreTransformation {
    pub count: i64,
    pub edges: Vec<EdgedCoreTransformation>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreTriggerRule {
    pub count: i64,
    pub edges: Vec<EdgedCoreTriggerRule>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreUserValidator {
    pub count: i64,
    pub edges: Vec<EdgedCoreUserValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreValidator {
    pub count: i64,
    pub edges: Vec<EdgedCoreValidator>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreWebhook {
    pub count: i64,
    pub edges: Vec<EdgedCoreWebhook>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCoreWeightedPoolResource {
    pub count: i64,
    pub edges: Vec<EdgedCoreWeightedPoolResource>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedInternalAccountToken {
    pub count: i64,
    pub edges: Vec<EdgedInternalAccountToken>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedInternalExternalIdentity {
    pub count: i64,
    pub edges: Vec<EdgedInternalExternalIdentity>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedInternalIPPrefixAvailable {
    pub count: i64,
    pub edges: Vec<EdgedInternalIPPrefixAvailable>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedInternalIPRangeAvailable {
    pub count: i64,
    pub edges: Vec<EdgedInternalIPRangeAvailable>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedInternalRefreshToken {
    pub count: i64,
    pub edges: Vec<EdgedInternalRefreshToken>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedIpamNamespace {
    pub count: i64,
    pub edges: Vec<EdgedIpamNamespace>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedLineageOwner {
    pub count: i64,
    pub edges: Vec<EdgedLineageOwner>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedLineageSource {
    pub count: i64,
    pub edges: Vec<EdgedLineageSource>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedObjectPermission {
    pub count: i64,
    pub edges: Vec<ObjectPermissionNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedProfileBuiltinIPAddress {
    pub count: i64,
    pub edges: Vec<EdgedProfileBuiltinIPAddress>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedProfileBuiltinIPPrefix {
    pub count: i64,
    pub edges: Vec<EdgedProfileBuiltinIPPrefix>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedProfileBuiltinTag {
    pub count: i64,
    pub edges: Vec<EdgedProfileBuiltinTag>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedProfileIpamNamespace {
    pub count: i64,
    pub edges: Vec<EdgedProfileIpamNamespace>,
    pub permissions: Box<PaginatedObjectPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionType {
    pub update_value: Option<BranchRelativePermissionDecision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAllocated {
    pub count: i64,
    pub edges: Vec<PoolAllocatedEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAllocatedEdge {
    pub node: Box<PoolAllocatedNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAllocatedNode {
    pub id: String,
    pub display_label: String,
    pub kind: String,
    pub branch: String,
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolUtilization {
    pub count: i64,
    pub utilization: f64,
    pub utilization_branches: f64,
    pub utilization_default_branch: f64,
    pub edges: Vec<IPPrefixUtilizationEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRepository {
    pub ok: Option<bool>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddress {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub profile_name: Option<Box<TextAttribute>>,
    pub profile_priority: Option<Box<NumberAttribute>>,
    pub address: Option<Box<IPHost>>,
    pub description: Option<Box<TextAttribute>>,
    pub related_nodes: Box<NestedPaginatedBuiltinIPAddress>,
    pub ip_namespace: Box<NestedEdgedBuiltinIPNamespace>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileBuiltinIPAddress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileBuiltinIPAddress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPAddressUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileBuiltinIPAddress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefix {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub profile_name: Option<Box<TextAttribute>>,
    pub profile_priority: Option<Box<NumberAttribute>>,
    pub is_pool: Option<Box<CheckboxAttribute>>,
    pub member_type: Option<Box<Dropdown>>,
    pub prefix: Option<Box<IPNetwork>>,
    pub description: Option<Box<TextAttribute>>,
    pub related_nodes: Box<NestedPaginatedBuiltinIPPrefix>,
    pub ip_namespace: Box<NestedEdgedBuiltinIPNamespace>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileBuiltinIPPrefix>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileBuiltinIPPrefix>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinIPPrefixUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileBuiltinIPPrefix>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTag {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub profile_name: Option<Box<TextAttribute>>,
    pub profile_priority: Option<Box<NumberAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub related_nodes: Box<NestedPaginatedBuiltinTag>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileBuiltinTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileBuiltinTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBuiltinTagUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileBuiltinTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespace {
    pub id: String,
    pub hfid: Option<Vec<String>>,
    pub display_label: Option<String>,
    pub profile_name: Option<Box<TextAttribute>>,
    pub profile_priority: Option<Box<NumberAttribute>>,
    pub description: Option<Box<TextAttribute>>,
    pub related_nodes: Box<NestedPaginatedIpamNamespace>,
    pub ip_prefixes: Box<NestedPaginatedBuiltinIPPrefix>,
    pub ip_addresses: Box<NestedPaginatedBuiltinIPAddress>,
    pub member_of_groups: Box<NestedPaginatedCoreGroup>,
    pub subscriber_of_groups: Box<NestedPaginatedCoreGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceCreate {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileIpamNamespace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceDelete {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceUpdate {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileIpamNamespace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIpamNamespaceUpsert {
    pub ok: Option<bool>,
    pub object: Option<Box<ProfileIpamNamespace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeApprovalsRevokedEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeCheckForApprovalRevoke {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeMerge {
    pub ok: Option<bool>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeMergedEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub merged_by_account_id: String,
    pub merged_by_account_name: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeRequestRunCheck {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeReview {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeReviewEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub reviewer_account_id: String,
    pub reviewer_account_name: String,
    pub reviewer_decision: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeReviewRequestedEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub requested_by_account_id: String,
    pub requested_by_account_name: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeReviewRevokedEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub reviewer_account_id: String,
    pub reviewer_account_name: String,
    pub reviewer_former_decision: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChangeThreadEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadOnlyRepositoryImportLastCommit {
    pub ok: Option<bool>,
    pub task: Option<Box<TaskInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecomputeComputedAttribute {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedNode {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub updated_by: Option<String>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub identifier: Option<String>,
    pub peers: Option<Vec<RelationshipPeer>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipAdd {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipNode {
    pub node: Box<Relationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipPeer {
    pub id: Option<String>,
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipProperty {
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRemove {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationships {
    pub edges: Vec<RelationshipNode>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredStringValueField {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveDiffConflict {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDropdownAdd {
    pub ok: Option<bool>,
    pub object: Option<Box<DropdownFields>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDropdownRemove {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaEnumAdd {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaEnumRemove {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardEvent {
    pub id: String,
    pub event: String,
    pub branch: Option<String>,
    pub account_id: Option<String>,
    pub occurred_at: String,
    pub level: i64,
    pub primary_node: Option<Box<RelatedNode>>,
    pub related_nodes: Vec<RelatedNode>,
    pub has_children: bool,
    pub parent_id: Option<String>,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub summary: Box<StatusSummary>,
    pub workers: Box<StatusWorkerEdges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusField {
    pub value: BranchStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusSummary {
    pub schema_hash_synced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusWorker {
    pub id: String,
    pub active: bool,
    pub schema_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusWorkerEdge {
    pub node: Box<StatusWorker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusWorkerEdges {
    pub edges: Vec<StatusWorkerEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub query: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLog {
    pub message: String,
    pub severity: String,
    pub task_id: String,
    pub timestamp: String,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLogEdge {
    pub edges: Vec<TaskLogNodes>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLogNodes {
    pub node: Option<Box<TaskLog>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNode {
    pub id: String,
    pub title: String,
    pub conclusion: String,
    pub state: Option<StateType>,
    pub progress: Option<f64>,
    pub workflow: Option<String>,
    pub branch: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub parameters: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub start_time: Option<String>,
    pub related_nodes: Option<Vec<TaskRelatedNode>>,
    pub logs: Option<Box<TaskLogEdge>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNodes {
    pub node: Option<Box<TaskNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRelatedNode {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tasks {
    pub edges: Vec<TaskNodes>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAttribute {
    pub is_default: Option<bool>,
    pub is_protected: Option<bool>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub is_from_profile: Option<bool>,
    pub permissions: Option<Box<PermissionType>>,
    pub value: Option<String>,
    pub source: Option<serde_json::Value>,
    pub owner: Option<serde_json::Value>,
    pub updated_by: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateComputedAttribute {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDisplayLabel {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHFID {
    pub ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateRepositoryConnectivity {
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueType {
    pub value: String,
}

