//! generated api module

#![allow(non_snake_case, unused_imports, unused_assignments, clippy::field_reassign_with_default)]

use infrahub::{BoxExtract, BoxFetch, BoxFutureResult, Client, DynPaginator, EdgePage, Error, Result};
use serde_json::Value;

use crate::inputs::*;
use crate::responses::*;
use crate::types::*;

pub struct BuiltinApi<'a> {
    client: &'a Client,
}

impl<'a> BuiltinApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn ip_address(&self) -> BuiltinIPAddressClient<'a> {
        BuiltinIPAddressClient::new(self.client)
    }
    pub fn ip_namespace(&self) -> BuiltinIPNamespaceClient<'a> {
        BuiltinIPNamespaceClient::new(self.client)
    }
    pub fn ip_prefix(&self) -> BuiltinIPPrefixClient<'a> {
        BuiltinIPPrefixClient::new(self.client)
    }
    pub fn tag(&self) -> BuiltinTagClient<'a> {
        BuiltinTagClient::new(self.client)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BuiltinIPAddressFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub address_value: Option<String>,
    pub address_values: Option<Vec<String>>,
    pub address_isnull: Option<bool>,
    pub address_source_id: Option<String>,
    pub address_owner_id: Option<String>,
    pub address_is_protected: Option<bool>,
    pub description_value: Option<String>,
    pub description_values: Option<Vec<String>>,
    pub description_isnull: Option<bool>,
    pub description_source_id: Option<String>,
    pub description_owner_id: Option<String>,
    pub description_is_protected: Option<bool>,
    pub any_value: Option<String>,
    pub any_values: Option<Vec<String>>,
    pub any_source_id: Option<String>,
    pub any_owner_id: Option<String>,
    pub any_is_protected: Option<bool>,
    pub partial_match: Option<bool>,
    pub node_metadata_created_by_id: Option<String>,
    pub node_metadata_created_by_ids: Option<Vec<String>>,
    pub node_metadata_updated_by_id: Option<String>,
    pub node_metadata_updated_by_ids: Option<Vec<String>>,
    pub node_metadata_created_at: Option<String>,
    pub node_metadata_created_at_before: Option<String>,
    pub node_metadata_created_at_after: Option<String>,
    pub node_metadata_updated_at: Option<String>,
    pub node_metadata_updated_at_before: Option<String>,
    pub node_metadata_updated_at_after: Option<String>,
    pub include_available: Option<bool>,
    pub kinds: Option<Vec<String>>,
    pub ip_namespace_ids: Option<Vec<String>>,
    pub ip_namespace_isnull: Option<bool>,
    pub ip_namespace_display_label_value: Option<String>,
    pub ip_namespace_display_label_values: Option<Vec<String>>,
    pub ip_namespace_display_label_isnull: Option<bool>,
    pub ip_namespace_description_value: Option<String>,
    pub ip_namespace_description_values: Option<Vec<String>>,
    pub ip_namespace_description_source_id: Option<String>,
    pub ip_namespace_description_owner_id: Option<String>,
    pub ip_namespace_description_is_protected: Option<bool>,
    pub ip_namespace_name_value: Option<String>,
    pub ip_namespace_name_values: Option<Vec<String>>,
    pub ip_namespace_name_source_id: Option<String>,
    pub ip_namespace_name_owner_id: Option<String>,
    pub ip_namespace_name_is_protected: Option<bool>,
    pub ip_prefix_ids: Option<Vec<String>>,
    pub ip_prefix_isnull: Option<bool>,
    pub ip_prefix_display_label_value: Option<String>,
    pub ip_prefix_display_label_values: Option<Vec<String>>,
    pub ip_prefix_display_label_isnull: Option<bool>,
    pub ip_prefix_netmask_value: Option<String>,
    pub ip_prefix_netmask_values: Option<Vec<String>>,
    pub ip_prefix_netmask_source_id: Option<String>,
    pub ip_prefix_netmask_owner_id: Option<String>,
    pub ip_prefix_netmask_is_protected: Option<bool>,
    pub ip_prefix_hostmask_value: Option<String>,
    pub ip_prefix_hostmask_values: Option<Vec<String>>,
    pub ip_prefix_hostmask_source_id: Option<String>,
    pub ip_prefix_hostmask_owner_id: Option<String>,
    pub ip_prefix_hostmask_is_protected: Option<bool>,
    pub ip_prefix_is_top_level_value: Option<bool>,
    pub ip_prefix_is_top_level_values: Option<Vec<bool>>,
    pub ip_prefix_is_top_level_source_id: Option<String>,
    pub ip_prefix_is_top_level_owner_id: Option<String>,
    pub ip_prefix_is_top_level_is_protected: Option<bool>,
    pub ip_prefix_utilization_value: Option<i64>,
    pub ip_prefix_utilization_values: Option<Vec<i64>>,
    pub ip_prefix_utilization_source_id: Option<String>,
    pub ip_prefix_utilization_owner_id: Option<String>,
    pub ip_prefix_utilization_is_protected: Option<bool>,
    pub ip_prefix_is_pool_value: Option<bool>,
    pub ip_prefix_is_pool_values: Option<Vec<bool>>,
    pub ip_prefix_is_pool_source_id: Option<String>,
    pub ip_prefix_is_pool_owner_id: Option<String>,
    pub ip_prefix_is_pool_is_protected: Option<bool>,
    pub ip_prefix_broadcast_address_value: Option<String>,
    pub ip_prefix_broadcast_address_values: Option<Vec<String>>,
    pub ip_prefix_broadcast_address_source_id: Option<String>,
    pub ip_prefix_broadcast_address_owner_id: Option<String>,
    pub ip_prefix_broadcast_address_is_protected: Option<bool>,
    pub ip_prefix_member_type_value: Option<String>,
    pub ip_prefix_member_type_values: Option<Vec<String>>,
    pub ip_prefix_member_type_source_id: Option<String>,
    pub ip_prefix_member_type_owner_id: Option<String>,
    pub ip_prefix_member_type_is_protected: Option<bool>,
    pub ip_prefix_network_address_value: Option<String>,
    pub ip_prefix_network_address_values: Option<Vec<String>>,
    pub ip_prefix_network_address_source_id: Option<String>,
    pub ip_prefix_network_address_owner_id: Option<String>,
    pub ip_prefix_network_address_is_protected: Option<bool>,
    pub ip_prefix_prefix_value: Option<String>,
    pub ip_prefix_prefix_values: Option<Vec<String>>,
    pub ip_prefix_prefix_source_id: Option<String>,
    pub ip_prefix_prefix_owner_id: Option<String>,
    pub ip_prefix_prefix_is_protected: Option<bool>,
    pub ip_prefix_description_value: Option<String>,
    pub ip_prefix_description_values: Option<Vec<String>>,
    pub ip_prefix_description_source_id: Option<String>,
    pub ip_prefix_description_owner_id: Option<String>,
    pub ip_prefix_description_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub profiles_ids: Option<Vec<String>>,
    pub profiles_isnull: Option<bool>,
    pub profiles_display_label_value: Option<String>,
    pub profiles_display_label_values: Option<Vec<String>>,
    pub profiles_display_label_isnull: Option<bool>,
    pub profiles_profile_name_value: Option<String>,
    pub profiles_profile_name_values: Option<Vec<String>>,
    pub profiles_profile_name_source_id: Option<String>,
    pub profiles_profile_name_owner_id: Option<String>,
    pub profiles_profile_name_is_protected: Option<bool>,
    pub profiles_profile_priority_value: Option<i64>,
    pub profiles_profile_priority_values: Option<Vec<i64>>,
    pub profiles_profile_priority_source_id: Option<String>,
    pub profiles_profile_priority_owner_id: Option<String>,
    pub profiles_profile_priority_is_protected: Option<bool>,
}

impl BuiltinIPAddressFilters {
    fn to_vars(&self) -> Result<Value> {
        let mut vars = serde_json::Map::new();
        if let Some(value) = &self.offset {
            vars.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.limit {
            vars.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.order {
            vars.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ids {
            vars.insert("ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_value {
            vars.insert("display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_values {
            vars.insert("display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert("display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address_value {
            vars.insert("address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address_values {
            vars.insert("address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address_isnull {
            vars.insert("address__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address_source_id {
            vars.insert("address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address_owner_id {
            vars.insert("address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address_is_protected {
            vars.insert("address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_value {
            vars.insert("description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_values {
            vars.insert("description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_isnull {
            vars.insert("description__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_source_id {
            vars.insert("description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_owner_id {
            vars.insert("description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_is_protected {
            vars.insert("description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_value {
            vars.insert("any__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_values {
            vars.insert("any__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_source_id {
            vars.insert("any__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_owner_id {
            vars.insert("any__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_is_protected {
            vars.insert("any__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert("node_metadata__created_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert("node_metadata__created_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert("node_metadata__updated_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert("node_metadata__updated_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert("node_metadata__created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert("node_metadata__created_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert("node_metadata__created_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert("node_metadata__updated_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert("node_metadata__updated_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert("node_metadata__updated_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.include_available {
            vars.insert("include_available".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.kinds {
            vars.insert("kinds".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_ids {
            vars.insert("ip_namespace__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_isnull {
            vars.insert("ip_namespace__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_display_label_value {
            vars.insert("ip_namespace__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_display_label_values {
            vars.insert("ip_namespace__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_display_label_isnull {
            vars.insert("ip_namespace__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_value {
            vars.insert("ip_namespace__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_values {
            vars.insert("ip_namespace__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_source_id {
            vars.insert("ip_namespace__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_owner_id {
            vars.insert("ip_namespace__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_is_protected {
            vars.insert("ip_namespace__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_value {
            vars.insert("ip_namespace__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_values {
            vars.insert("ip_namespace__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_source_id {
            vars.insert("ip_namespace__name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_owner_id {
            vars.insert("ip_namespace__name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_is_protected {
            vars.insert("ip_namespace__name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_ids {
            vars.insert("ip_prefix__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_isnull {
            vars.insert("ip_prefix__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_display_label_value {
            vars.insert("ip_prefix__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_display_label_values {
            vars.insert("ip_prefix__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_display_label_isnull {
            vars.insert("ip_prefix__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_netmask_value {
            vars.insert("ip_prefix__netmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_netmask_values {
            vars.insert("ip_prefix__netmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_netmask_source_id {
            vars.insert("ip_prefix__netmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_netmask_owner_id {
            vars.insert("ip_prefix__netmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_netmask_is_protected {
            vars.insert("ip_prefix__netmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_hostmask_value {
            vars.insert("ip_prefix__hostmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_hostmask_values {
            vars.insert("ip_prefix__hostmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_hostmask_source_id {
            vars.insert("ip_prefix__hostmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_hostmask_owner_id {
            vars.insert("ip_prefix__hostmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_hostmask_is_protected {
            vars.insert("ip_prefix__hostmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_top_level_value {
            vars.insert("ip_prefix__is_top_level__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_top_level_values {
            vars.insert("ip_prefix__is_top_level__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_top_level_source_id {
            vars.insert("ip_prefix__is_top_level__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_top_level_owner_id {
            vars.insert("ip_prefix__is_top_level__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_top_level_is_protected {
            vars.insert("ip_prefix__is_top_level__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_utilization_value {
            vars.insert("ip_prefix__utilization__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_utilization_values {
            vars.insert("ip_prefix__utilization__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_utilization_source_id {
            vars.insert("ip_prefix__utilization__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_utilization_owner_id {
            vars.insert("ip_prefix__utilization__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_utilization_is_protected {
            vars.insert("ip_prefix__utilization__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_pool_value {
            vars.insert("ip_prefix__is_pool__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_pool_values {
            vars.insert("ip_prefix__is_pool__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_pool_source_id {
            vars.insert("ip_prefix__is_pool__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_pool_owner_id {
            vars.insert("ip_prefix__is_pool__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_is_pool_is_protected {
            vars.insert("ip_prefix__is_pool__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_broadcast_address_value {
            vars.insert("ip_prefix__broadcast_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_broadcast_address_values {
            vars.insert("ip_prefix__broadcast_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_broadcast_address_source_id {
            vars.insert("ip_prefix__broadcast_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_broadcast_address_owner_id {
            vars.insert("ip_prefix__broadcast_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_broadcast_address_is_protected {
            vars.insert("ip_prefix__broadcast_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_member_type_value {
            vars.insert("ip_prefix__member_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_member_type_values {
            vars.insert("ip_prefix__member_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_member_type_source_id {
            vars.insert("ip_prefix__member_type__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_member_type_owner_id {
            vars.insert("ip_prefix__member_type__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_member_type_is_protected {
            vars.insert("ip_prefix__member_type__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_network_address_value {
            vars.insert("ip_prefix__network_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_network_address_values {
            vars.insert("ip_prefix__network_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_network_address_source_id {
            vars.insert("ip_prefix__network_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_network_address_owner_id {
            vars.insert("ip_prefix__network_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_network_address_is_protected {
            vars.insert("ip_prefix__network_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_prefix_value {
            vars.insert("ip_prefix__prefix__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_prefix_values {
            vars.insert("ip_prefix__prefix__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_prefix_source_id {
            vars.insert("ip_prefix__prefix__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_prefix_owner_id {
            vars.insert("ip_prefix__prefix__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_prefix_is_protected {
            vars.insert("ip_prefix__prefix__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_description_value {
            vars.insert("ip_prefix__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_description_values {
            vars.insert("ip_prefix__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_description_source_id {
            vars.insert("ip_prefix__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_description_owner_id {
            vars.insert("ip_prefix__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefix_description_is_protected {
            vars.insert("ip_prefix__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert("member_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert("member_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert("member_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert("member_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert("member_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert("member_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert("member_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert("member_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert("member_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert("member_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert("member_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert("member_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert("member_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert("subscriber_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert("subscriber_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert("subscriber_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert("subscriber_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert("subscriber_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert("subscriber_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert("subscriber_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert("subscriber_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert("subscriber_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert("subscriber_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert("subscriber_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert("subscriber_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert("subscriber_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert("profiles__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert("profiles__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert("profiles__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert("profiles__profile_name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert("profiles__profile_name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert("profiles__profile_name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert("profiles__profile_name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert("profiles__profile_name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert("profiles__profile_priority__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert("profiles__profile_priority__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert("profiles__profile_priority__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert("profiles__profile_priority__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert("profiles__profile_priority__is_protected".to_string(), serde_json::to_value(value)?);
        }
        Ok(Value::Object(vars))
    }
}

pub struct BuiltinIPAddressClient<'a> {
    client: &'a Client,
}

impl<'a> BuiltinIPAddressClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, filters: Option<BuiltinIPAddressFilters>, request_branch: Option<&str>) -> Result<Vec<serde_json::Value>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query BuiltinIPAddress($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $address__value: String, $address__values: [String], $address__isnull: Boolean, $address__source__id: ID, $address__owner__id: ID, $address__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $include_available: Boolean, $kinds: [String!], $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $ip_prefix__ids: [ID], $ip_prefix__isnull: Boolean, $ip_prefix__display_label__value: String, $ip_prefix__display_label__values: [String], $ip_prefix__display_label__isnull: Boolean, $ip_prefix__netmask__value: String, $ip_prefix__netmask__values: [String], $ip_prefix__netmask__source__id: ID, $ip_prefix__netmask__owner__id: ID, $ip_prefix__netmask__is_protected: Boolean, $ip_prefix__hostmask__value: String, $ip_prefix__hostmask__values: [String], $ip_prefix__hostmask__source__id: ID, $ip_prefix__hostmask__owner__id: ID, $ip_prefix__hostmask__is_protected: Boolean, $ip_prefix__is_top_level__value: Boolean, $ip_prefix__is_top_level__values: [Boolean], $ip_prefix__is_top_level__source__id: ID, $ip_prefix__is_top_level__owner__id: ID, $ip_prefix__is_top_level__is_protected: Boolean, $ip_prefix__utilization__value: BigInt, $ip_prefix__utilization__values: [BigInt], $ip_prefix__utilization__source__id: ID, $ip_prefix__utilization__owner__id: ID, $ip_prefix__utilization__is_protected: Boolean, $ip_prefix__is_pool__value: Boolean, $ip_prefix__is_pool__values: [Boolean], $ip_prefix__is_pool__source__id: ID, $ip_prefix__is_pool__owner__id: ID, $ip_prefix__is_pool__is_protected: Boolean, $ip_prefix__broadcast_address__value: String, $ip_prefix__broadcast_address__values: [String], $ip_prefix__broadcast_address__source__id: ID, $ip_prefix__broadcast_address__owner__id: ID, $ip_prefix__broadcast_address__is_protected: Boolean, $ip_prefix__member_type__value: String, $ip_prefix__member_type__values: [String], $ip_prefix__member_type__source__id: ID, $ip_prefix__member_type__owner__id: ID, $ip_prefix__member_type__is_protected: Boolean, $ip_prefix__network_address__value: String, $ip_prefix__network_address__values: [String], $ip_prefix__network_address__source__id: ID, $ip_prefix__network_address__owner__id: ID, $ip_prefix__network_address__is_protected: Boolean, $ip_prefix__prefix__value: String, $ip_prefix__prefix__values: [String], $ip_prefix__prefix__source__id: ID, $ip_prefix__prefix__owner__id: ID, $ip_prefix__prefix__is_protected: Boolean, $ip_prefix__description__value: String, $ip_prefix__description__values: [String], $ip_prefix__description__source__id: ID, $ip_prefix__description__owner__id: ID, $ip_prefix__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { BuiltinIPAddress(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, address__value: $address__value, address__values: $address__values, address__isnull: $address__isnull, address__source__id: $address__source__id, address__owner__id: $address__owner__id, address__is_protected: $address__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, include_available: $include_available, kinds: $kinds, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, ip_prefix__ids: $ip_prefix__ids, ip_prefix__isnull: $ip_prefix__isnull, ip_prefix__display_label__value: $ip_prefix__display_label__value, ip_prefix__display_label__values: $ip_prefix__display_label__values, ip_prefix__display_label__isnull: $ip_prefix__display_label__isnull, ip_prefix__netmask__value: $ip_prefix__netmask__value, ip_prefix__netmask__values: $ip_prefix__netmask__values, ip_prefix__netmask__source__id: $ip_prefix__netmask__source__id, ip_prefix__netmask__owner__id: $ip_prefix__netmask__owner__id, ip_prefix__netmask__is_protected: $ip_prefix__netmask__is_protected, ip_prefix__hostmask__value: $ip_prefix__hostmask__value, ip_prefix__hostmask__values: $ip_prefix__hostmask__values, ip_prefix__hostmask__source__id: $ip_prefix__hostmask__source__id, ip_prefix__hostmask__owner__id: $ip_prefix__hostmask__owner__id, ip_prefix__hostmask__is_protected: $ip_prefix__hostmask__is_protected, ip_prefix__is_top_level__value: $ip_prefix__is_top_level__value, ip_prefix__is_top_level__values: $ip_prefix__is_top_level__values, ip_prefix__is_top_level__source__id: $ip_prefix__is_top_level__source__id, ip_prefix__is_top_level__owner__id: $ip_prefix__is_top_level__owner__id, ip_prefix__is_top_level__is_protected: $ip_prefix__is_top_level__is_protected, ip_prefix__utilization__value: $ip_prefix__utilization__value, ip_prefix__utilization__values: $ip_prefix__utilization__values, ip_prefix__utilization__source__id: $ip_prefix__utilization__source__id, ip_prefix__utilization__owner__id: $ip_prefix__utilization__owner__id, ip_prefix__utilization__is_protected: $ip_prefix__utilization__is_protected, ip_prefix__is_pool__value: $ip_prefix__is_pool__value, ip_prefix__is_pool__values: $ip_prefix__is_pool__values, ip_prefix__is_pool__source__id: $ip_prefix__is_pool__source__id, ip_prefix__is_pool__owner__id: $ip_prefix__is_pool__owner__id, ip_prefix__is_pool__is_protected: $ip_prefix__is_pool__is_protected, ip_prefix__broadcast_address__value: $ip_prefix__broadcast_address__value, ip_prefix__broadcast_address__values: $ip_prefix__broadcast_address__values, ip_prefix__broadcast_address__source__id: $ip_prefix__broadcast_address__source__id, ip_prefix__broadcast_address__owner__id: $ip_prefix__broadcast_address__owner__id, ip_prefix__broadcast_address__is_protected: $ip_prefix__broadcast_address__is_protected, ip_prefix__member_type__value: $ip_prefix__member_type__value, ip_prefix__member_type__values: $ip_prefix__member_type__values, ip_prefix__member_type__source__id: $ip_prefix__member_type__source__id, ip_prefix__member_type__owner__id: $ip_prefix__member_type__owner__id, ip_prefix__member_type__is_protected: $ip_prefix__member_type__is_protected, ip_prefix__network_address__value: $ip_prefix__network_address__value, ip_prefix__network_address__values: $ip_prefix__network_address__values, ip_prefix__network_address__source__id: $ip_prefix__network_address__source__id, ip_prefix__network_address__owner__id: $ip_prefix__network_address__owner__id, ip_prefix__network_address__is_protected: $ip_prefix__network_address__is_protected, ip_prefix__prefix__value: $ip_prefix__prefix__value, ip_prefix__prefix__values: $ip_prefix__prefix__values, ip_prefix__prefix__source__id: $ip_prefix__prefix__source__id, ip_prefix__prefix__owner__id: $ip_prefix__prefix__owner__id, ip_prefix__prefix__is_protected: $ip_prefix__prefix__is_protected, ip_prefix__description__value: $ip_prefix__description__value, ip_prefix__description__values: $ip_prefix__description__values, ip_prefix__description__source__id: $ip_prefix__description__source__id, ip_prefix__description__owner__id: $ip_prefix__description__owner__id, ip_prefix__description__is_protected: $ip_prefix__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<BuiltinIPAddressResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.builtin_ip_address.edges {
            if let Some(node) = edge.node {
                items.push(node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<BuiltinIPAddressFilters>, request_branch: Option<&str>) -> DynPaginator<'a, serde_json::Value, String, (BuiltinIPAddressResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query BuiltinIPAddress($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $address__value: String, $address__values: [String], $address__isnull: Boolean, $address__source__id: ID, $address__owner__id: ID, $address__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $include_available: Boolean, $kinds: [String!], $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $ip_prefix__ids: [ID], $ip_prefix__isnull: Boolean, $ip_prefix__display_label__value: String, $ip_prefix__display_label__values: [String], $ip_prefix__display_label__isnull: Boolean, $ip_prefix__netmask__value: String, $ip_prefix__netmask__values: [String], $ip_prefix__netmask__source__id: ID, $ip_prefix__netmask__owner__id: ID, $ip_prefix__netmask__is_protected: Boolean, $ip_prefix__hostmask__value: String, $ip_prefix__hostmask__values: [String], $ip_prefix__hostmask__source__id: ID, $ip_prefix__hostmask__owner__id: ID, $ip_prefix__hostmask__is_protected: Boolean, $ip_prefix__is_top_level__value: Boolean, $ip_prefix__is_top_level__values: [Boolean], $ip_prefix__is_top_level__source__id: ID, $ip_prefix__is_top_level__owner__id: ID, $ip_prefix__is_top_level__is_protected: Boolean, $ip_prefix__utilization__value: BigInt, $ip_prefix__utilization__values: [BigInt], $ip_prefix__utilization__source__id: ID, $ip_prefix__utilization__owner__id: ID, $ip_prefix__utilization__is_protected: Boolean, $ip_prefix__is_pool__value: Boolean, $ip_prefix__is_pool__values: [Boolean], $ip_prefix__is_pool__source__id: ID, $ip_prefix__is_pool__owner__id: ID, $ip_prefix__is_pool__is_protected: Boolean, $ip_prefix__broadcast_address__value: String, $ip_prefix__broadcast_address__values: [String], $ip_prefix__broadcast_address__source__id: ID, $ip_prefix__broadcast_address__owner__id: ID, $ip_prefix__broadcast_address__is_protected: Boolean, $ip_prefix__member_type__value: String, $ip_prefix__member_type__values: [String], $ip_prefix__member_type__source__id: ID, $ip_prefix__member_type__owner__id: ID, $ip_prefix__member_type__is_protected: Boolean, $ip_prefix__network_address__value: String, $ip_prefix__network_address__values: [String], $ip_prefix__network_address__source__id: ID, $ip_prefix__network_address__owner__id: ID, $ip_prefix__network_address__is_protected: Boolean, $ip_prefix__prefix__value: String, $ip_prefix__prefix__values: [String], $ip_prefix__prefix__source__id: ID, $ip_prefix__prefix__owner__id: ID, $ip_prefix__prefix__is_protected: Boolean, $ip_prefix__description__value: String, $ip_prefix__description__values: [String], $ip_prefix__description__source__id: ID, $ip_prefix__description__owner__id: ID, $ip_prefix__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { BuiltinIPAddress(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, address__value: $address__value, address__values: $address__values, address__isnull: $address__isnull, address__source__id: $address__source__id, address__owner__id: $address__owner__id, address__is_protected: $address__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, include_available: $include_available, kinds: $kinds, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, ip_prefix__ids: $ip_prefix__ids, ip_prefix__isnull: $ip_prefix__isnull, ip_prefix__display_label__value: $ip_prefix__display_label__value, ip_prefix__display_label__values: $ip_prefix__display_label__values, ip_prefix__display_label__isnull: $ip_prefix__display_label__isnull, ip_prefix__netmask__value: $ip_prefix__netmask__value, ip_prefix__netmask__values: $ip_prefix__netmask__values, ip_prefix__netmask__source__id: $ip_prefix__netmask__source__id, ip_prefix__netmask__owner__id: $ip_prefix__netmask__owner__id, ip_prefix__netmask__is_protected: $ip_prefix__netmask__is_protected, ip_prefix__hostmask__value: $ip_prefix__hostmask__value, ip_prefix__hostmask__values: $ip_prefix__hostmask__values, ip_prefix__hostmask__source__id: $ip_prefix__hostmask__source__id, ip_prefix__hostmask__owner__id: $ip_prefix__hostmask__owner__id, ip_prefix__hostmask__is_protected: $ip_prefix__hostmask__is_protected, ip_prefix__is_top_level__value: $ip_prefix__is_top_level__value, ip_prefix__is_top_level__values: $ip_prefix__is_top_level__values, ip_prefix__is_top_level__source__id: $ip_prefix__is_top_level__source__id, ip_prefix__is_top_level__owner__id: $ip_prefix__is_top_level__owner__id, ip_prefix__is_top_level__is_protected: $ip_prefix__is_top_level__is_protected, ip_prefix__utilization__value: $ip_prefix__utilization__value, ip_prefix__utilization__values: $ip_prefix__utilization__values, ip_prefix__utilization__source__id: $ip_prefix__utilization__source__id, ip_prefix__utilization__owner__id: $ip_prefix__utilization__owner__id, ip_prefix__utilization__is_protected: $ip_prefix__utilization__is_protected, ip_prefix__is_pool__value: $ip_prefix__is_pool__value, ip_prefix__is_pool__values: $ip_prefix__is_pool__values, ip_prefix__is_pool__source__id: $ip_prefix__is_pool__source__id, ip_prefix__is_pool__owner__id: $ip_prefix__is_pool__owner__id, ip_prefix__is_pool__is_protected: $ip_prefix__is_pool__is_protected, ip_prefix__broadcast_address__value: $ip_prefix__broadcast_address__value, ip_prefix__broadcast_address__values: $ip_prefix__broadcast_address__values, ip_prefix__broadcast_address__source__id: $ip_prefix__broadcast_address__source__id, ip_prefix__broadcast_address__owner__id: $ip_prefix__broadcast_address__owner__id, ip_prefix__broadcast_address__is_protected: $ip_prefix__broadcast_address__is_protected, ip_prefix__member_type__value: $ip_prefix__member_type__value, ip_prefix__member_type__values: $ip_prefix__member_type__values, ip_prefix__member_type__source__id: $ip_prefix__member_type__source__id, ip_prefix__member_type__owner__id: $ip_prefix__member_type__owner__id, ip_prefix__member_type__is_protected: $ip_prefix__member_type__is_protected, ip_prefix__network_address__value: $ip_prefix__network_address__value, ip_prefix__network_address__values: $ip_prefix__network_address__values, ip_prefix__network_address__source__id: $ip_prefix__network_address__source__id, ip_prefix__network_address__owner__id: $ip_prefix__network_address__owner__id, ip_prefix__network_address__is_protected: $ip_prefix__network_address__is_protected, ip_prefix__prefix__value: $ip_prefix__prefix__value, ip_prefix__prefix__values: $ip_prefix__prefix__values, ip_prefix__prefix__source__id: $ip_prefix__prefix__source__id, ip_prefix__prefix__owner__id: $ip_prefix__prefix__owner__id, ip_prefix__prefix__is_protected: $ip_prefix__prefix__is_protected, ip_prefix__description__value: $ip_prefix__description__value, ip_prefix__description__values: $ip_prefix__description__values, ip_prefix__description__source__id: $ip_prefix__description__source__id, ip_prefix__description__owner__id: $ip_prefix__description__owner__id, ip_prefix__description__is_protected: $ip_prefix__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (BuiltinIPAddressResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (BuiltinIPAddressResponse, i64)> {
            let mut page_filters = base_filters.clone();
            let branch = request_branch.clone();
            let mut current_offset: i64 = 0;
            let base_offset = page_filters.offset.unwrap_or(0);
            current_offset = cursor
                .as_deref()
                .and_then(|c| c.parse::<i64>().ok())
                .unwrap_or(base_offset);
            page_filters.offset = Some(current_offset);
            Box::pin(async move {
            let vars = page_filters.to_vars()?;
                let response = client.execute::<BuiltinIPAddressResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, serde_json::Value, String, (BuiltinIPAddressResponse, i64)> = Box::new(move |(data, current_offset): (BuiltinIPAddressResponse, i64)| -> Result<EdgePage<serde_json::Value, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.builtin_ip_address.edges {
                if let Some(node) = edge.node {
                    items.push(node);
                }
            }
            if !items.is_empty() {
                next = Some((current_offset + items.len() as i64).to_string());
            }
            Ok(EdgePage { nodes: items, next_cursor: next })
        });
        infrahub::Paginator::new(fetch, extract)
    }

    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<serde_json::Value>> {
        let mut filters = BuiltinIPAddressFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }

}

#[derive(Debug, Clone, Default)]
pub struct BuiltinIPNamespaceFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub description_value: Option<String>,
    pub description_values: Option<Vec<String>>,
    pub description_isnull: Option<bool>,
    pub description_source_id: Option<String>,
    pub description_owner_id: Option<String>,
    pub description_is_protected: Option<bool>,
    pub name_value: Option<String>,
    pub name_values: Option<Vec<String>>,
    pub name_isnull: Option<bool>,
    pub name_source_id: Option<String>,
    pub name_owner_id: Option<String>,
    pub name_is_protected: Option<bool>,
    pub any_value: Option<String>,
    pub any_values: Option<Vec<String>>,
    pub any_source_id: Option<String>,
    pub any_owner_id: Option<String>,
    pub any_is_protected: Option<bool>,
    pub partial_match: Option<bool>,
    pub node_metadata_created_by_id: Option<String>,
    pub node_metadata_created_by_ids: Option<Vec<String>>,
    pub node_metadata_updated_by_id: Option<String>,
    pub node_metadata_updated_by_ids: Option<Vec<String>>,
    pub node_metadata_created_at: Option<String>,
    pub node_metadata_created_at_before: Option<String>,
    pub node_metadata_created_at_after: Option<String>,
    pub node_metadata_updated_at: Option<String>,
    pub node_metadata_updated_at_before: Option<String>,
    pub node_metadata_updated_at_after: Option<String>,
    pub ip_prefixes_ids: Option<Vec<String>>,
    pub ip_prefixes_isnull: Option<bool>,
    pub ip_prefixes_display_label_value: Option<String>,
    pub ip_prefixes_display_label_values: Option<Vec<String>>,
    pub ip_prefixes_display_label_isnull: Option<bool>,
    pub ip_prefixes_netmask_value: Option<String>,
    pub ip_prefixes_netmask_values: Option<Vec<String>>,
    pub ip_prefixes_netmask_source_id: Option<String>,
    pub ip_prefixes_netmask_owner_id: Option<String>,
    pub ip_prefixes_netmask_is_protected: Option<bool>,
    pub ip_prefixes_hostmask_value: Option<String>,
    pub ip_prefixes_hostmask_values: Option<Vec<String>>,
    pub ip_prefixes_hostmask_source_id: Option<String>,
    pub ip_prefixes_hostmask_owner_id: Option<String>,
    pub ip_prefixes_hostmask_is_protected: Option<bool>,
    pub ip_prefixes_is_top_level_value: Option<bool>,
    pub ip_prefixes_is_top_level_values: Option<Vec<bool>>,
    pub ip_prefixes_is_top_level_source_id: Option<String>,
    pub ip_prefixes_is_top_level_owner_id: Option<String>,
    pub ip_prefixes_is_top_level_is_protected: Option<bool>,
    pub ip_prefixes_utilization_value: Option<i64>,
    pub ip_prefixes_utilization_values: Option<Vec<i64>>,
    pub ip_prefixes_utilization_source_id: Option<String>,
    pub ip_prefixes_utilization_owner_id: Option<String>,
    pub ip_prefixes_utilization_is_protected: Option<bool>,
    pub ip_prefixes_is_pool_value: Option<bool>,
    pub ip_prefixes_is_pool_values: Option<Vec<bool>>,
    pub ip_prefixes_is_pool_source_id: Option<String>,
    pub ip_prefixes_is_pool_owner_id: Option<String>,
    pub ip_prefixes_is_pool_is_protected: Option<bool>,
    pub ip_prefixes_broadcast_address_value: Option<String>,
    pub ip_prefixes_broadcast_address_values: Option<Vec<String>>,
    pub ip_prefixes_broadcast_address_source_id: Option<String>,
    pub ip_prefixes_broadcast_address_owner_id: Option<String>,
    pub ip_prefixes_broadcast_address_is_protected: Option<bool>,
    pub ip_prefixes_member_type_value: Option<String>,
    pub ip_prefixes_member_type_values: Option<Vec<String>>,
    pub ip_prefixes_member_type_source_id: Option<String>,
    pub ip_prefixes_member_type_owner_id: Option<String>,
    pub ip_prefixes_member_type_is_protected: Option<bool>,
    pub ip_prefixes_network_address_value: Option<String>,
    pub ip_prefixes_network_address_values: Option<Vec<String>>,
    pub ip_prefixes_network_address_source_id: Option<String>,
    pub ip_prefixes_network_address_owner_id: Option<String>,
    pub ip_prefixes_network_address_is_protected: Option<bool>,
    pub ip_prefixes_prefix_value: Option<String>,
    pub ip_prefixes_prefix_values: Option<Vec<String>>,
    pub ip_prefixes_prefix_source_id: Option<String>,
    pub ip_prefixes_prefix_owner_id: Option<String>,
    pub ip_prefixes_prefix_is_protected: Option<bool>,
    pub ip_prefixes_description_value: Option<String>,
    pub ip_prefixes_description_values: Option<Vec<String>>,
    pub ip_prefixes_description_source_id: Option<String>,
    pub ip_prefixes_description_owner_id: Option<String>,
    pub ip_prefixes_description_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub ip_addresses_ids: Option<Vec<String>>,
    pub ip_addresses_isnull: Option<bool>,
    pub ip_addresses_display_label_value: Option<String>,
    pub ip_addresses_display_label_values: Option<Vec<String>>,
    pub ip_addresses_display_label_isnull: Option<bool>,
    pub ip_addresses_address_value: Option<String>,
    pub ip_addresses_address_values: Option<Vec<String>>,
    pub ip_addresses_address_source_id: Option<String>,
    pub ip_addresses_address_owner_id: Option<String>,
    pub ip_addresses_address_is_protected: Option<bool>,
    pub ip_addresses_description_value: Option<String>,
    pub ip_addresses_description_values: Option<Vec<String>>,
    pub ip_addresses_description_source_id: Option<String>,
    pub ip_addresses_description_owner_id: Option<String>,
    pub ip_addresses_description_is_protected: Option<bool>,
    pub profiles_ids: Option<Vec<String>>,
    pub profiles_isnull: Option<bool>,
    pub profiles_display_label_value: Option<String>,
    pub profiles_display_label_values: Option<Vec<String>>,
    pub profiles_display_label_isnull: Option<bool>,
    pub profiles_profile_name_value: Option<String>,
    pub profiles_profile_name_values: Option<Vec<String>>,
    pub profiles_profile_name_source_id: Option<String>,
    pub profiles_profile_name_owner_id: Option<String>,
    pub profiles_profile_name_is_protected: Option<bool>,
    pub profiles_profile_priority_value: Option<i64>,
    pub profiles_profile_priority_values: Option<Vec<i64>>,
    pub profiles_profile_priority_source_id: Option<String>,
    pub profiles_profile_priority_owner_id: Option<String>,
    pub profiles_profile_priority_is_protected: Option<bool>,
}

impl BuiltinIPNamespaceFilters {
    fn to_vars(&self) -> Result<Value> {
        let mut vars = serde_json::Map::new();
        if let Some(value) = &self.offset {
            vars.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.limit {
            vars.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.order {
            vars.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ids {
            vars.insert("ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_value {
            vars.insert("display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_values {
            vars.insert("display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert("display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_value {
            vars.insert("description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_values {
            vars.insert("description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_isnull {
            vars.insert("description__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_source_id {
            vars.insert("description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_owner_id {
            vars.insert("description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_is_protected {
            vars.insert("description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_value {
            vars.insert("name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_values {
            vars.insert("name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_isnull {
            vars.insert("name__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_source_id {
            vars.insert("name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_owner_id {
            vars.insert("name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_is_protected {
            vars.insert("name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_value {
            vars.insert("any__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_values {
            vars.insert("any__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_source_id {
            vars.insert("any__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_owner_id {
            vars.insert("any__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_is_protected {
            vars.insert("any__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert("node_metadata__created_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert("node_metadata__created_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert("node_metadata__updated_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert("node_metadata__updated_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert("node_metadata__created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert("node_metadata__created_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert("node_metadata__created_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert("node_metadata__updated_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert("node_metadata__updated_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert("node_metadata__updated_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_ids {
            vars.insert("ip_prefixes__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_isnull {
            vars.insert("ip_prefixes__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_display_label_value {
            vars.insert("ip_prefixes__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_display_label_values {
            vars.insert("ip_prefixes__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_display_label_isnull {
            vars.insert("ip_prefixes__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_netmask_value {
            vars.insert("ip_prefixes__netmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_netmask_values {
            vars.insert("ip_prefixes__netmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_netmask_source_id {
            vars.insert("ip_prefixes__netmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_netmask_owner_id {
            vars.insert("ip_prefixes__netmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_netmask_is_protected {
            vars.insert("ip_prefixes__netmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_hostmask_value {
            vars.insert("ip_prefixes__hostmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_hostmask_values {
            vars.insert("ip_prefixes__hostmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_hostmask_source_id {
            vars.insert("ip_prefixes__hostmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_hostmask_owner_id {
            vars.insert("ip_prefixes__hostmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_hostmask_is_protected {
            vars.insert("ip_prefixes__hostmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_value {
            vars.insert("ip_prefixes__is_top_level__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_values {
            vars.insert("ip_prefixes__is_top_level__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_source_id {
            vars.insert("ip_prefixes__is_top_level__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_owner_id {
            vars.insert("ip_prefixes__is_top_level__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_is_protected {
            vars.insert("ip_prefixes__is_top_level__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_utilization_value {
            vars.insert("ip_prefixes__utilization__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_utilization_values {
            vars.insert("ip_prefixes__utilization__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_utilization_source_id {
            vars.insert("ip_prefixes__utilization__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_utilization_owner_id {
            vars.insert("ip_prefixes__utilization__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_utilization_is_protected {
            vars.insert("ip_prefixes__utilization__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_pool_value {
            vars.insert("ip_prefixes__is_pool__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_pool_values {
            vars.insert("ip_prefixes__is_pool__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_pool_source_id {
            vars.insert("ip_prefixes__is_pool__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_pool_owner_id {
            vars.insert("ip_prefixes__is_pool__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_is_pool_is_protected {
            vars.insert("ip_prefixes__is_pool__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_value {
            vars.insert("ip_prefixes__broadcast_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_values {
            vars.insert("ip_prefixes__broadcast_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_source_id {
            vars.insert("ip_prefixes__broadcast_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_owner_id {
            vars.insert("ip_prefixes__broadcast_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_is_protected {
            vars.insert("ip_prefixes__broadcast_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_member_type_value {
            vars.insert("ip_prefixes__member_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_member_type_values {
            vars.insert("ip_prefixes__member_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_member_type_source_id {
            vars.insert("ip_prefixes__member_type__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_member_type_owner_id {
            vars.insert("ip_prefixes__member_type__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_member_type_is_protected {
            vars.insert("ip_prefixes__member_type__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_network_address_value {
            vars.insert("ip_prefixes__network_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_network_address_values {
            vars.insert("ip_prefixes__network_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_network_address_source_id {
            vars.insert("ip_prefixes__network_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_network_address_owner_id {
            vars.insert("ip_prefixes__network_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_network_address_is_protected {
            vars.insert("ip_prefixes__network_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_prefix_value {
            vars.insert("ip_prefixes__prefix__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_prefix_values {
            vars.insert("ip_prefixes__prefix__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_prefix_source_id {
            vars.insert("ip_prefixes__prefix__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_prefix_owner_id {
            vars.insert("ip_prefixes__prefix__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_prefix_is_protected {
            vars.insert("ip_prefixes__prefix__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_description_value {
            vars.insert("ip_prefixes__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_description_values {
            vars.insert("ip_prefixes__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_description_source_id {
            vars.insert("ip_prefixes__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_description_owner_id {
            vars.insert("ip_prefixes__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_description_is_protected {
            vars.insert("ip_prefixes__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert("member_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert("member_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert("member_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert("member_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert("member_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert("member_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert("member_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert("member_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert("member_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert("member_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert("member_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert("member_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert("member_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert("subscriber_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert("subscriber_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert("subscriber_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert("subscriber_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert("subscriber_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert("subscriber_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert("subscriber_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert("subscriber_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert("subscriber_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert("subscriber_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert("subscriber_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert("subscriber_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert("subscriber_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_ids {
            vars.insert("ip_addresses__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_isnull {
            vars.insert("ip_addresses__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_display_label_value {
            vars.insert("ip_addresses__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_display_label_values {
            vars.insert("ip_addresses__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_display_label_isnull {
            vars.insert("ip_addresses__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_value {
            vars.insert("ip_addresses__address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_values {
            vars.insert("ip_addresses__address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_source_id {
            vars.insert("ip_addresses__address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_owner_id {
            vars.insert("ip_addresses__address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_is_protected {
            vars.insert("ip_addresses__address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_value {
            vars.insert("ip_addresses__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_values {
            vars.insert("ip_addresses__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_source_id {
            vars.insert("ip_addresses__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_owner_id {
            vars.insert("ip_addresses__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_is_protected {
            vars.insert("ip_addresses__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert("profiles__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert("profiles__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert("profiles__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert("profiles__profile_name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert("profiles__profile_name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert("profiles__profile_name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert("profiles__profile_name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert("profiles__profile_name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert("profiles__profile_priority__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert("profiles__profile_priority__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert("profiles__profile_priority__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert("profiles__profile_priority__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert("profiles__profile_priority__is_protected".to_string(), serde_json::to_value(value)?);
        }
        Ok(Value::Object(vars))
    }
}

pub struct BuiltinIPNamespaceClient<'a> {
    client: &'a Client,
}

impl<'a> BuiltinIPNamespaceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, filters: Option<BuiltinIPNamespaceFilters>, request_branch: Option<&str>) -> Result<Vec<serde_json::Value>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query BuiltinIPNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { BuiltinIPNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<BuiltinIPNamespaceResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.builtin_ip_namespace.edges {
            if let Some(node) = edge.node {
                items.push(node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<BuiltinIPNamespaceFilters>, request_branch: Option<&str>) -> DynPaginator<'a, serde_json::Value, String, (BuiltinIPNamespaceResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query BuiltinIPNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { BuiltinIPNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (BuiltinIPNamespaceResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (BuiltinIPNamespaceResponse, i64)> {
            let mut page_filters = base_filters.clone();
            let branch = request_branch.clone();
            let mut current_offset: i64 = 0;
            let base_offset = page_filters.offset.unwrap_or(0);
            current_offset = cursor
                .as_deref()
                .and_then(|c| c.parse::<i64>().ok())
                .unwrap_or(base_offset);
            page_filters.offset = Some(current_offset);
            Box::pin(async move {
            let vars = page_filters.to_vars()?;
                let response = client.execute::<BuiltinIPNamespaceResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, serde_json::Value, String, (BuiltinIPNamespaceResponse, i64)> = Box::new(move |(data, current_offset): (BuiltinIPNamespaceResponse, i64)| -> Result<EdgePage<serde_json::Value, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.builtin_ip_namespace.edges {
                if let Some(node) = edge.node {
                    items.push(node);
                }
            }
            if !items.is_empty() {
                next = Some((current_offset + items.len() as i64).to_string());
            }
            Ok(EdgePage { nodes: items, next_cursor: next })
        });
        infrahub::Paginator::new(fetch, extract)
    }

    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<serde_json::Value>> {
        let mut filters = BuiltinIPNamespaceFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }

}

#[derive(Debug, Clone, Default)]
pub struct BuiltinIPPrefixFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub netmask_value: Option<String>,
    pub netmask_values: Option<Vec<String>>,
    pub netmask_isnull: Option<bool>,
    pub netmask_source_id: Option<String>,
    pub netmask_owner_id: Option<String>,
    pub netmask_is_protected: Option<bool>,
    pub hostmask_value: Option<String>,
    pub hostmask_values: Option<Vec<String>>,
    pub hostmask_isnull: Option<bool>,
    pub hostmask_source_id: Option<String>,
    pub hostmask_owner_id: Option<String>,
    pub hostmask_is_protected: Option<bool>,
    pub is_top_level_value: Option<bool>,
    pub is_top_level_values: Option<Vec<bool>>,
    pub is_top_level_isnull: Option<bool>,
    pub is_top_level_source_id: Option<String>,
    pub is_top_level_owner_id: Option<String>,
    pub is_top_level_is_protected: Option<bool>,
    pub utilization_value: Option<i64>,
    pub utilization_values: Option<Vec<i64>>,
    pub utilization_isnull: Option<bool>,
    pub utilization_source_id: Option<String>,
    pub utilization_owner_id: Option<String>,
    pub utilization_is_protected: Option<bool>,
    pub is_pool_value: Option<bool>,
    pub is_pool_values: Option<Vec<bool>>,
    pub is_pool_isnull: Option<bool>,
    pub is_pool_source_id: Option<String>,
    pub is_pool_owner_id: Option<String>,
    pub is_pool_is_protected: Option<bool>,
    pub broadcast_address_value: Option<String>,
    pub broadcast_address_values: Option<Vec<String>>,
    pub broadcast_address_isnull: Option<bool>,
    pub broadcast_address_source_id: Option<String>,
    pub broadcast_address_owner_id: Option<String>,
    pub broadcast_address_is_protected: Option<bool>,
    pub member_type_value: Option<String>,
    pub member_type_values: Option<Vec<String>>,
    pub member_type_isnull: Option<bool>,
    pub member_type_source_id: Option<String>,
    pub member_type_owner_id: Option<String>,
    pub member_type_is_protected: Option<bool>,
    pub network_address_value: Option<String>,
    pub network_address_values: Option<Vec<String>>,
    pub network_address_isnull: Option<bool>,
    pub network_address_source_id: Option<String>,
    pub network_address_owner_id: Option<String>,
    pub network_address_is_protected: Option<bool>,
    pub prefix_value: Option<String>,
    pub prefix_values: Option<Vec<String>>,
    pub prefix_isnull: Option<bool>,
    pub prefix_source_id: Option<String>,
    pub prefix_owner_id: Option<String>,
    pub prefix_is_protected: Option<bool>,
    pub description_value: Option<String>,
    pub description_values: Option<Vec<String>>,
    pub description_isnull: Option<bool>,
    pub description_source_id: Option<String>,
    pub description_owner_id: Option<String>,
    pub description_is_protected: Option<bool>,
    pub any_value: Option<String>,
    pub any_values: Option<Vec<String>>,
    pub any_source_id: Option<String>,
    pub any_owner_id: Option<String>,
    pub any_is_protected: Option<bool>,
    pub partial_match: Option<bool>,
    pub node_metadata_created_by_id: Option<String>,
    pub node_metadata_created_by_ids: Option<Vec<String>>,
    pub node_metadata_updated_by_id: Option<String>,
    pub node_metadata_updated_by_ids: Option<Vec<String>>,
    pub node_metadata_created_at: Option<String>,
    pub node_metadata_created_at_before: Option<String>,
    pub node_metadata_created_at_after: Option<String>,
    pub node_metadata_updated_at: Option<String>,
    pub node_metadata_updated_at_before: Option<String>,
    pub node_metadata_updated_at_after: Option<String>,
    pub include_available: Option<bool>,
    pub kinds: Option<Vec<String>>,
    pub ip_namespace_ids: Option<Vec<String>>,
    pub ip_namespace_isnull: Option<bool>,
    pub ip_namespace_display_label_value: Option<String>,
    pub ip_namespace_display_label_values: Option<Vec<String>>,
    pub ip_namespace_display_label_isnull: Option<bool>,
    pub ip_namespace_description_value: Option<String>,
    pub ip_namespace_description_values: Option<Vec<String>>,
    pub ip_namespace_description_source_id: Option<String>,
    pub ip_namespace_description_owner_id: Option<String>,
    pub ip_namespace_description_is_protected: Option<bool>,
    pub ip_namespace_name_value: Option<String>,
    pub ip_namespace_name_values: Option<Vec<String>>,
    pub ip_namespace_name_source_id: Option<String>,
    pub ip_namespace_name_owner_id: Option<String>,
    pub ip_namespace_name_is_protected: Option<bool>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub ip_addresses_ids: Option<Vec<String>>,
    pub ip_addresses_isnull: Option<bool>,
    pub ip_addresses_display_label_value: Option<String>,
    pub ip_addresses_display_label_values: Option<Vec<String>>,
    pub ip_addresses_display_label_isnull: Option<bool>,
    pub ip_addresses_address_value: Option<String>,
    pub ip_addresses_address_values: Option<Vec<String>>,
    pub ip_addresses_address_source_id: Option<String>,
    pub ip_addresses_address_owner_id: Option<String>,
    pub ip_addresses_address_is_protected: Option<bool>,
    pub ip_addresses_description_value: Option<String>,
    pub ip_addresses_description_values: Option<Vec<String>>,
    pub ip_addresses_description_source_id: Option<String>,
    pub ip_addresses_description_owner_id: Option<String>,
    pub ip_addresses_description_is_protected: Option<bool>,
    pub children_ids: Option<Vec<String>>,
    pub children_isnull: Option<bool>,
    pub children_display_label_value: Option<String>,
    pub children_display_label_values: Option<Vec<String>>,
    pub children_display_label_isnull: Option<bool>,
    pub children_netmask_value: Option<String>,
    pub children_netmask_values: Option<Vec<String>>,
    pub children_netmask_source_id: Option<String>,
    pub children_netmask_owner_id: Option<String>,
    pub children_netmask_is_protected: Option<bool>,
    pub children_hostmask_value: Option<String>,
    pub children_hostmask_values: Option<Vec<String>>,
    pub children_hostmask_source_id: Option<String>,
    pub children_hostmask_owner_id: Option<String>,
    pub children_hostmask_is_protected: Option<bool>,
    pub children_is_top_level_value: Option<bool>,
    pub children_is_top_level_values: Option<Vec<bool>>,
    pub children_is_top_level_source_id: Option<String>,
    pub children_is_top_level_owner_id: Option<String>,
    pub children_is_top_level_is_protected: Option<bool>,
    pub children_utilization_value: Option<i64>,
    pub children_utilization_values: Option<Vec<i64>>,
    pub children_utilization_source_id: Option<String>,
    pub children_utilization_owner_id: Option<String>,
    pub children_utilization_is_protected: Option<bool>,
    pub children_is_pool_value: Option<bool>,
    pub children_is_pool_values: Option<Vec<bool>>,
    pub children_is_pool_source_id: Option<String>,
    pub children_is_pool_owner_id: Option<String>,
    pub children_is_pool_is_protected: Option<bool>,
    pub children_broadcast_address_value: Option<String>,
    pub children_broadcast_address_values: Option<Vec<String>>,
    pub children_broadcast_address_source_id: Option<String>,
    pub children_broadcast_address_owner_id: Option<String>,
    pub children_broadcast_address_is_protected: Option<bool>,
    pub children_member_type_value: Option<String>,
    pub children_member_type_values: Option<Vec<String>>,
    pub children_member_type_source_id: Option<String>,
    pub children_member_type_owner_id: Option<String>,
    pub children_member_type_is_protected: Option<bool>,
    pub children_network_address_value: Option<String>,
    pub children_network_address_values: Option<Vec<String>>,
    pub children_network_address_source_id: Option<String>,
    pub children_network_address_owner_id: Option<String>,
    pub children_network_address_is_protected: Option<bool>,
    pub children_prefix_value: Option<String>,
    pub children_prefix_values: Option<Vec<String>>,
    pub children_prefix_source_id: Option<String>,
    pub children_prefix_owner_id: Option<String>,
    pub children_prefix_is_protected: Option<bool>,
    pub children_description_value: Option<String>,
    pub children_description_values: Option<Vec<String>>,
    pub children_description_source_id: Option<String>,
    pub children_description_owner_id: Option<String>,
    pub children_description_is_protected: Option<bool>,
    pub resource_pool_ids: Option<Vec<String>>,
    pub resource_pool_isnull: Option<bool>,
    pub resource_pool_display_label_value: Option<String>,
    pub resource_pool_display_label_values: Option<Vec<String>>,
    pub resource_pool_display_label_isnull: Option<bool>,
    pub resource_pool_default_prefix_length_value: Option<i64>,
    pub resource_pool_default_prefix_length_values: Option<Vec<i64>>,
    pub resource_pool_default_prefix_length_source_id: Option<String>,
    pub resource_pool_default_prefix_length_owner_id: Option<String>,
    pub resource_pool_default_prefix_length_is_protected: Option<bool>,
    pub resource_pool_default_address_type_value: Option<String>,
    pub resource_pool_default_address_type_values: Option<Vec<String>>,
    pub resource_pool_default_address_type_source_id: Option<String>,
    pub resource_pool_default_address_type_owner_id: Option<String>,
    pub resource_pool_default_address_type_is_protected: Option<bool>,
    pub resource_pool_name_value: Option<String>,
    pub resource_pool_name_values: Option<Vec<String>>,
    pub resource_pool_name_source_id: Option<String>,
    pub resource_pool_name_owner_id: Option<String>,
    pub resource_pool_name_is_protected: Option<bool>,
    pub resource_pool_description_value: Option<String>,
    pub resource_pool_description_values: Option<Vec<String>>,
    pub resource_pool_description_source_id: Option<String>,
    pub resource_pool_description_owner_id: Option<String>,
    pub resource_pool_description_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub parent_ids: Option<Vec<String>>,
    pub parent_isnull: Option<bool>,
    pub parent_display_label_value: Option<String>,
    pub parent_display_label_values: Option<Vec<String>>,
    pub parent_display_label_isnull: Option<bool>,
    pub parent_netmask_value: Option<String>,
    pub parent_netmask_values: Option<Vec<String>>,
    pub parent_netmask_source_id: Option<String>,
    pub parent_netmask_owner_id: Option<String>,
    pub parent_netmask_is_protected: Option<bool>,
    pub parent_hostmask_value: Option<String>,
    pub parent_hostmask_values: Option<Vec<String>>,
    pub parent_hostmask_source_id: Option<String>,
    pub parent_hostmask_owner_id: Option<String>,
    pub parent_hostmask_is_protected: Option<bool>,
    pub parent_is_top_level_value: Option<bool>,
    pub parent_is_top_level_values: Option<Vec<bool>>,
    pub parent_is_top_level_source_id: Option<String>,
    pub parent_is_top_level_owner_id: Option<String>,
    pub parent_is_top_level_is_protected: Option<bool>,
    pub parent_utilization_value: Option<i64>,
    pub parent_utilization_values: Option<Vec<i64>>,
    pub parent_utilization_source_id: Option<String>,
    pub parent_utilization_owner_id: Option<String>,
    pub parent_utilization_is_protected: Option<bool>,
    pub parent_is_pool_value: Option<bool>,
    pub parent_is_pool_values: Option<Vec<bool>>,
    pub parent_is_pool_source_id: Option<String>,
    pub parent_is_pool_owner_id: Option<String>,
    pub parent_is_pool_is_protected: Option<bool>,
    pub parent_broadcast_address_value: Option<String>,
    pub parent_broadcast_address_values: Option<Vec<String>>,
    pub parent_broadcast_address_source_id: Option<String>,
    pub parent_broadcast_address_owner_id: Option<String>,
    pub parent_broadcast_address_is_protected: Option<bool>,
    pub parent_member_type_value: Option<String>,
    pub parent_member_type_values: Option<Vec<String>>,
    pub parent_member_type_source_id: Option<String>,
    pub parent_member_type_owner_id: Option<String>,
    pub parent_member_type_is_protected: Option<bool>,
    pub parent_network_address_value: Option<String>,
    pub parent_network_address_values: Option<Vec<String>>,
    pub parent_network_address_source_id: Option<String>,
    pub parent_network_address_owner_id: Option<String>,
    pub parent_network_address_is_protected: Option<bool>,
    pub parent_prefix_value: Option<String>,
    pub parent_prefix_values: Option<Vec<String>>,
    pub parent_prefix_source_id: Option<String>,
    pub parent_prefix_owner_id: Option<String>,
    pub parent_prefix_is_protected: Option<bool>,
    pub parent_description_value: Option<String>,
    pub parent_description_values: Option<Vec<String>>,
    pub parent_description_source_id: Option<String>,
    pub parent_description_owner_id: Option<String>,
    pub parent_description_is_protected: Option<bool>,
    pub profiles_ids: Option<Vec<String>>,
    pub profiles_isnull: Option<bool>,
    pub profiles_display_label_value: Option<String>,
    pub profiles_display_label_values: Option<Vec<String>>,
    pub profiles_display_label_isnull: Option<bool>,
    pub profiles_profile_name_value: Option<String>,
    pub profiles_profile_name_values: Option<Vec<String>>,
    pub profiles_profile_name_source_id: Option<String>,
    pub profiles_profile_name_owner_id: Option<String>,
    pub profiles_profile_name_is_protected: Option<bool>,
    pub profiles_profile_priority_value: Option<i64>,
    pub profiles_profile_priority_values: Option<Vec<i64>>,
    pub profiles_profile_priority_source_id: Option<String>,
    pub profiles_profile_priority_owner_id: Option<String>,
    pub profiles_profile_priority_is_protected: Option<bool>,
}

impl BuiltinIPPrefixFilters {
    fn to_vars(&self) -> Result<Value> {
        let mut vars = serde_json::Map::new();
        if let Some(value) = &self.offset {
            vars.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.limit {
            vars.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.order {
            vars.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ids {
            vars.insert("ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_value {
            vars.insert("display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_values {
            vars.insert("display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert("display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.netmask_value {
            vars.insert("netmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.netmask_values {
            vars.insert("netmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.netmask_isnull {
            vars.insert("netmask__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.netmask_source_id {
            vars.insert("netmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.netmask_owner_id {
            vars.insert("netmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.netmask_is_protected {
            vars.insert("netmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.hostmask_value {
            vars.insert("hostmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.hostmask_values {
            vars.insert("hostmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.hostmask_isnull {
            vars.insert("hostmask__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.hostmask_source_id {
            vars.insert("hostmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.hostmask_owner_id {
            vars.insert("hostmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.hostmask_is_protected {
            vars.insert("hostmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_top_level_value {
            vars.insert("is_top_level__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_top_level_values {
            vars.insert("is_top_level__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_top_level_isnull {
            vars.insert("is_top_level__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_top_level_source_id {
            vars.insert("is_top_level__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_top_level_owner_id {
            vars.insert("is_top_level__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_top_level_is_protected {
            vars.insert("is_top_level__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.utilization_value {
            vars.insert("utilization__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.utilization_values {
            vars.insert("utilization__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.utilization_isnull {
            vars.insert("utilization__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.utilization_source_id {
            vars.insert("utilization__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.utilization_owner_id {
            vars.insert("utilization__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.utilization_is_protected {
            vars.insert("utilization__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool_value {
            vars.insert("is_pool__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool_values {
            vars.insert("is_pool__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool_isnull {
            vars.insert("is_pool__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool_source_id {
            vars.insert("is_pool__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool_owner_id {
            vars.insert("is_pool__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool_is_protected {
            vars.insert("is_pool__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.broadcast_address_value {
            vars.insert("broadcast_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.broadcast_address_values {
            vars.insert("broadcast_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.broadcast_address_isnull {
            vars.insert("broadcast_address__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.broadcast_address_source_id {
            vars.insert("broadcast_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.broadcast_address_owner_id {
            vars.insert("broadcast_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.broadcast_address_is_protected {
            vars.insert("broadcast_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type_value {
            vars.insert("member_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type_values {
            vars.insert("member_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type_isnull {
            vars.insert("member_type__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type_source_id {
            vars.insert("member_type__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type_owner_id {
            vars.insert("member_type__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type_is_protected {
            vars.insert("member_type__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.network_address_value {
            vars.insert("network_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.network_address_values {
            vars.insert("network_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.network_address_isnull {
            vars.insert("network_address__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.network_address_source_id {
            vars.insert("network_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.network_address_owner_id {
            vars.insert("network_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.network_address_is_protected {
            vars.insert("network_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix_value {
            vars.insert("prefix__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix_values {
            vars.insert("prefix__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix_isnull {
            vars.insert("prefix__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix_source_id {
            vars.insert("prefix__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix_owner_id {
            vars.insert("prefix__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix_is_protected {
            vars.insert("prefix__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_value {
            vars.insert("description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_values {
            vars.insert("description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_isnull {
            vars.insert("description__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_source_id {
            vars.insert("description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_owner_id {
            vars.insert("description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_is_protected {
            vars.insert("description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_value {
            vars.insert("any__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_values {
            vars.insert("any__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_source_id {
            vars.insert("any__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_owner_id {
            vars.insert("any__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_is_protected {
            vars.insert("any__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert("node_metadata__created_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert("node_metadata__created_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert("node_metadata__updated_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert("node_metadata__updated_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert("node_metadata__created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert("node_metadata__created_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert("node_metadata__created_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert("node_metadata__updated_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert("node_metadata__updated_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert("node_metadata__updated_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.include_available {
            vars.insert("include_available".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.kinds {
            vars.insert("kinds".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_ids {
            vars.insert("ip_namespace__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_isnull {
            vars.insert("ip_namespace__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_display_label_value {
            vars.insert("ip_namespace__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_display_label_values {
            vars.insert("ip_namespace__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_display_label_isnull {
            vars.insert("ip_namespace__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_value {
            vars.insert("ip_namespace__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_values {
            vars.insert("ip_namespace__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_source_id {
            vars.insert("ip_namespace__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_owner_id {
            vars.insert("ip_namespace__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_description_is_protected {
            vars.insert("ip_namespace__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_value {
            vars.insert("ip_namespace__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_values {
            vars.insert("ip_namespace__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_source_id {
            vars.insert("ip_namespace__name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_owner_id {
            vars.insert("ip_namespace__name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace_name_is_protected {
            vars.insert("ip_namespace__name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert("subscriber_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert("subscriber_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert("subscriber_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert("subscriber_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert("subscriber_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert("subscriber_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert("subscriber_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert("subscriber_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert("subscriber_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert("subscriber_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert("subscriber_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert("subscriber_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert("subscriber_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_ids {
            vars.insert("ip_addresses__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_isnull {
            vars.insert("ip_addresses__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_display_label_value {
            vars.insert("ip_addresses__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_display_label_values {
            vars.insert("ip_addresses__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_display_label_isnull {
            vars.insert("ip_addresses__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_value {
            vars.insert("ip_addresses__address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_values {
            vars.insert("ip_addresses__address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_source_id {
            vars.insert("ip_addresses__address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_owner_id {
            vars.insert("ip_addresses__address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_address_is_protected {
            vars.insert("ip_addresses__address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_value {
            vars.insert("ip_addresses__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_values {
            vars.insert("ip_addresses__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_source_id {
            vars.insert("ip_addresses__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_owner_id {
            vars.insert("ip_addresses__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses_description_is_protected {
            vars.insert("ip_addresses__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_ids {
            vars.insert("children__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_isnull {
            vars.insert("children__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_display_label_value {
            vars.insert("children__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_display_label_values {
            vars.insert("children__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_display_label_isnull {
            vars.insert("children__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_netmask_value {
            vars.insert("children__netmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_netmask_values {
            vars.insert("children__netmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_netmask_source_id {
            vars.insert("children__netmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_netmask_owner_id {
            vars.insert("children__netmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_netmask_is_protected {
            vars.insert("children__netmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_hostmask_value {
            vars.insert("children__hostmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_hostmask_values {
            vars.insert("children__hostmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_hostmask_source_id {
            vars.insert("children__hostmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_hostmask_owner_id {
            vars.insert("children__hostmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_hostmask_is_protected {
            vars.insert("children__hostmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_top_level_value {
            vars.insert("children__is_top_level__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_top_level_values {
            vars.insert("children__is_top_level__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_top_level_source_id {
            vars.insert("children__is_top_level__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_top_level_owner_id {
            vars.insert("children__is_top_level__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_top_level_is_protected {
            vars.insert("children__is_top_level__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_utilization_value {
            vars.insert("children__utilization__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_utilization_values {
            vars.insert("children__utilization__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_utilization_source_id {
            vars.insert("children__utilization__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_utilization_owner_id {
            vars.insert("children__utilization__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_utilization_is_protected {
            vars.insert("children__utilization__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_pool_value {
            vars.insert("children__is_pool__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_pool_values {
            vars.insert("children__is_pool__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_pool_source_id {
            vars.insert("children__is_pool__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_pool_owner_id {
            vars.insert("children__is_pool__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_is_pool_is_protected {
            vars.insert("children__is_pool__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_broadcast_address_value {
            vars.insert("children__broadcast_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_broadcast_address_values {
            vars.insert("children__broadcast_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_broadcast_address_source_id {
            vars.insert("children__broadcast_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_broadcast_address_owner_id {
            vars.insert("children__broadcast_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_broadcast_address_is_protected {
            vars.insert("children__broadcast_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_member_type_value {
            vars.insert("children__member_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_member_type_values {
            vars.insert("children__member_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_member_type_source_id {
            vars.insert("children__member_type__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_member_type_owner_id {
            vars.insert("children__member_type__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_member_type_is_protected {
            vars.insert("children__member_type__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_network_address_value {
            vars.insert("children__network_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_network_address_values {
            vars.insert("children__network_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_network_address_source_id {
            vars.insert("children__network_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_network_address_owner_id {
            vars.insert("children__network_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_network_address_is_protected {
            vars.insert("children__network_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_prefix_value {
            vars.insert("children__prefix__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_prefix_values {
            vars.insert("children__prefix__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_prefix_source_id {
            vars.insert("children__prefix__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_prefix_owner_id {
            vars.insert("children__prefix__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_prefix_is_protected {
            vars.insert("children__prefix__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_description_value {
            vars.insert("children__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_description_values {
            vars.insert("children__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_description_source_id {
            vars.insert("children__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_description_owner_id {
            vars.insert("children__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.children_description_is_protected {
            vars.insert("children__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_ids {
            vars.insert("resource_pool__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_isnull {
            vars.insert("resource_pool__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_display_label_value {
            vars.insert("resource_pool__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_display_label_values {
            vars.insert("resource_pool__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_display_label_isnull {
            vars.insert("resource_pool__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_prefix_length_value {
            vars.insert("resource_pool__default_prefix_length__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_prefix_length_values {
            vars.insert("resource_pool__default_prefix_length__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_prefix_length_source_id {
            vars.insert("resource_pool__default_prefix_length__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_prefix_length_owner_id {
            vars.insert("resource_pool__default_prefix_length__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_prefix_length_is_protected {
            vars.insert("resource_pool__default_prefix_length__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_address_type_value {
            vars.insert("resource_pool__default_address_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_address_type_values {
            vars.insert("resource_pool__default_address_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_address_type_source_id {
            vars.insert("resource_pool__default_address_type__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_address_type_owner_id {
            vars.insert("resource_pool__default_address_type__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_default_address_type_is_protected {
            vars.insert("resource_pool__default_address_type__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_name_value {
            vars.insert("resource_pool__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_name_values {
            vars.insert("resource_pool__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_name_source_id {
            vars.insert("resource_pool__name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_name_owner_id {
            vars.insert("resource_pool__name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_name_is_protected {
            vars.insert("resource_pool__name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_description_value {
            vars.insert("resource_pool__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_description_values {
            vars.insert("resource_pool__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_description_source_id {
            vars.insert("resource_pool__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_description_owner_id {
            vars.insert("resource_pool__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.resource_pool_description_is_protected {
            vars.insert("resource_pool__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert("member_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert("member_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert("member_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert("member_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert("member_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert("member_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert("member_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert("member_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert("member_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert("member_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert("member_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert("member_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert("member_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_ids {
            vars.insert("parent__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_isnull {
            vars.insert("parent__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_display_label_value {
            vars.insert("parent__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_display_label_values {
            vars.insert("parent__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_display_label_isnull {
            vars.insert("parent__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_netmask_value {
            vars.insert("parent__netmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_netmask_values {
            vars.insert("parent__netmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_netmask_source_id {
            vars.insert("parent__netmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_netmask_owner_id {
            vars.insert("parent__netmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_netmask_is_protected {
            vars.insert("parent__netmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_hostmask_value {
            vars.insert("parent__hostmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_hostmask_values {
            vars.insert("parent__hostmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_hostmask_source_id {
            vars.insert("parent__hostmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_hostmask_owner_id {
            vars.insert("parent__hostmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_hostmask_is_protected {
            vars.insert("parent__hostmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_top_level_value {
            vars.insert("parent__is_top_level__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_top_level_values {
            vars.insert("parent__is_top_level__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_top_level_source_id {
            vars.insert("parent__is_top_level__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_top_level_owner_id {
            vars.insert("parent__is_top_level__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_top_level_is_protected {
            vars.insert("parent__is_top_level__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_utilization_value {
            vars.insert("parent__utilization__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_utilization_values {
            vars.insert("parent__utilization__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_utilization_source_id {
            vars.insert("parent__utilization__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_utilization_owner_id {
            vars.insert("parent__utilization__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_utilization_is_protected {
            vars.insert("parent__utilization__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_pool_value {
            vars.insert("parent__is_pool__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_pool_values {
            vars.insert("parent__is_pool__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_pool_source_id {
            vars.insert("parent__is_pool__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_pool_owner_id {
            vars.insert("parent__is_pool__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_is_pool_is_protected {
            vars.insert("parent__is_pool__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_broadcast_address_value {
            vars.insert("parent__broadcast_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_broadcast_address_values {
            vars.insert("parent__broadcast_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_broadcast_address_source_id {
            vars.insert("parent__broadcast_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_broadcast_address_owner_id {
            vars.insert("parent__broadcast_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_broadcast_address_is_protected {
            vars.insert("parent__broadcast_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_member_type_value {
            vars.insert("parent__member_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_member_type_values {
            vars.insert("parent__member_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_member_type_source_id {
            vars.insert("parent__member_type__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_member_type_owner_id {
            vars.insert("parent__member_type__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_member_type_is_protected {
            vars.insert("parent__member_type__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_network_address_value {
            vars.insert("parent__network_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_network_address_values {
            vars.insert("parent__network_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_network_address_source_id {
            vars.insert("parent__network_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_network_address_owner_id {
            vars.insert("parent__network_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_network_address_is_protected {
            vars.insert("parent__network_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_prefix_value {
            vars.insert("parent__prefix__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_prefix_values {
            vars.insert("parent__prefix__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_prefix_source_id {
            vars.insert("parent__prefix__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_prefix_owner_id {
            vars.insert("parent__prefix__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_prefix_is_protected {
            vars.insert("parent__prefix__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_description_value {
            vars.insert("parent__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_description_values {
            vars.insert("parent__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_description_source_id {
            vars.insert("parent__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_description_owner_id {
            vars.insert("parent__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.parent_description_is_protected {
            vars.insert("parent__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert("profiles__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert("profiles__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert("profiles__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert("profiles__profile_name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert("profiles__profile_name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert("profiles__profile_name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert("profiles__profile_name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert("profiles__profile_name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert("profiles__profile_priority__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert("profiles__profile_priority__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert("profiles__profile_priority__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert("profiles__profile_priority__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert("profiles__profile_priority__is_protected".to_string(), serde_json::to_value(value)?);
        }
        Ok(Value::Object(vars))
    }
}

pub struct BuiltinIPPrefixClient<'a> {
    client: &'a Client,
}

impl<'a> BuiltinIPPrefixClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, filters: Option<BuiltinIPPrefixFilters>, request_branch: Option<&str>) -> Result<Vec<serde_json::Value>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query BuiltinIPPrefix($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $netmask__value: String, $netmask__values: [String], $netmask__isnull: Boolean, $netmask__source__id: ID, $netmask__owner__id: ID, $netmask__is_protected: Boolean, $hostmask__value: String, $hostmask__values: [String], $hostmask__isnull: Boolean, $hostmask__source__id: ID, $hostmask__owner__id: ID, $hostmask__is_protected: Boolean, $is_top_level__value: Boolean, $is_top_level__values: [Boolean], $is_top_level__isnull: Boolean, $is_top_level__source__id: ID, $is_top_level__owner__id: ID, $is_top_level__is_protected: Boolean, $utilization__value: BigInt, $utilization__values: [BigInt], $utilization__isnull: Boolean, $utilization__source__id: ID, $utilization__owner__id: ID, $utilization__is_protected: Boolean, $is_pool__value: Boolean, $is_pool__values: [Boolean], $is_pool__isnull: Boolean, $is_pool__source__id: ID, $is_pool__owner__id: ID, $is_pool__is_protected: Boolean, $broadcast_address__value: String, $broadcast_address__values: [String], $broadcast_address__isnull: Boolean, $broadcast_address__source__id: ID, $broadcast_address__owner__id: ID, $broadcast_address__is_protected: Boolean, $member_type__value: String, $member_type__values: [String], $member_type__isnull: Boolean, $member_type__source__id: ID, $member_type__owner__id: ID, $member_type__is_protected: Boolean, $network_address__value: String, $network_address__values: [String], $network_address__isnull: Boolean, $network_address__source__id: ID, $network_address__owner__id: ID, $network_address__is_protected: Boolean, $prefix__value: String, $prefix__values: [String], $prefix__isnull: Boolean, $prefix__source__id: ID, $prefix__owner__id: ID, $prefix__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $include_available: Boolean, $kinds: [String!], $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $children__ids: [ID], $children__isnull: Boolean, $children__display_label__value: String, $children__display_label__values: [String], $children__display_label__isnull: Boolean, $children__netmask__value: String, $children__netmask__values: [String], $children__netmask__source__id: ID, $children__netmask__owner__id: ID, $children__netmask__is_protected: Boolean, $children__hostmask__value: String, $children__hostmask__values: [String], $children__hostmask__source__id: ID, $children__hostmask__owner__id: ID, $children__hostmask__is_protected: Boolean, $children__is_top_level__value: Boolean, $children__is_top_level__values: [Boolean], $children__is_top_level__source__id: ID, $children__is_top_level__owner__id: ID, $children__is_top_level__is_protected: Boolean, $children__utilization__value: BigInt, $children__utilization__values: [BigInt], $children__utilization__source__id: ID, $children__utilization__owner__id: ID, $children__utilization__is_protected: Boolean, $children__is_pool__value: Boolean, $children__is_pool__values: [Boolean], $children__is_pool__source__id: ID, $children__is_pool__owner__id: ID, $children__is_pool__is_protected: Boolean, $children__broadcast_address__value: String, $children__broadcast_address__values: [String], $children__broadcast_address__source__id: ID, $children__broadcast_address__owner__id: ID, $children__broadcast_address__is_protected: Boolean, $children__member_type__value: String, $children__member_type__values: [String], $children__member_type__source__id: ID, $children__member_type__owner__id: ID, $children__member_type__is_protected: Boolean, $children__network_address__value: String, $children__network_address__values: [String], $children__network_address__source__id: ID, $children__network_address__owner__id: ID, $children__network_address__is_protected: Boolean, $children__prefix__value: String, $children__prefix__values: [String], $children__prefix__source__id: ID, $children__prefix__owner__id: ID, $children__prefix__is_protected: Boolean, $children__description__value: String, $children__description__values: [String], $children__description__source__id: ID, $children__description__owner__id: ID, $children__description__is_protected: Boolean, $resource_pool__ids: [ID], $resource_pool__isnull: Boolean, $resource_pool__display_label__value: String, $resource_pool__display_label__values: [String], $resource_pool__display_label__isnull: Boolean, $resource_pool__default_prefix_length__value: BigInt, $resource_pool__default_prefix_length__values: [BigInt], $resource_pool__default_prefix_length__source__id: ID, $resource_pool__default_prefix_length__owner__id: ID, $resource_pool__default_prefix_length__is_protected: Boolean, $resource_pool__default_address_type__value: String, $resource_pool__default_address_type__values: [String], $resource_pool__default_address_type__source__id: ID, $resource_pool__default_address_type__owner__id: ID, $resource_pool__default_address_type__is_protected: Boolean, $resource_pool__name__value: String, $resource_pool__name__values: [String], $resource_pool__name__source__id: ID, $resource_pool__name__owner__id: ID, $resource_pool__name__is_protected: Boolean, $resource_pool__description__value: String, $resource_pool__description__values: [String], $resource_pool__description__source__id: ID, $resource_pool__description__owner__id: ID, $resource_pool__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $parent__ids: [ID], $parent__isnull: Boolean, $parent__display_label__value: String, $parent__display_label__values: [String], $parent__display_label__isnull: Boolean, $parent__netmask__value: String, $parent__netmask__values: [String], $parent__netmask__source__id: ID, $parent__netmask__owner__id: ID, $parent__netmask__is_protected: Boolean, $parent__hostmask__value: String, $parent__hostmask__values: [String], $parent__hostmask__source__id: ID, $parent__hostmask__owner__id: ID, $parent__hostmask__is_protected: Boolean, $parent__is_top_level__value: Boolean, $parent__is_top_level__values: [Boolean], $parent__is_top_level__source__id: ID, $parent__is_top_level__owner__id: ID, $parent__is_top_level__is_protected: Boolean, $parent__utilization__value: BigInt, $parent__utilization__values: [BigInt], $parent__utilization__source__id: ID, $parent__utilization__owner__id: ID, $parent__utilization__is_protected: Boolean, $parent__is_pool__value: Boolean, $parent__is_pool__values: [Boolean], $parent__is_pool__source__id: ID, $parent__is_pool__owner__id: ID, $parent__is_pool__is_protected: Boolean, $parent__broadcast_address__value: String, $parent__broadcast_address__values: [String], $parent__broadcast_address__source__id: ID, $parent__broadcast_address__owner__id: ID, $parent__broadcast_address__is_protected: Boolean, $parent__member_type__value: String, $parent__member_type__values: [String], $parent__member_type__source__id: ID, $parent__member_type__owner__id: ID, $parent__member_type__is_protected: Boolean, $parent__network_address__value: String, $parent__network_address__values: [String], $parent__network_address__source__id: ID, $parent__network_address__owner__id: ID, $parent__network_address__is_protected: Boolean, $parent__prefix__value: String, $parent__prefix__values: [String], $parent__prefix__source__id: ID, $parent__prefix__owner__id: ID, $parent__prefix__is_protected: Boolean, $parent__description__value: String, $parent__description__values: [String], $parent__description__source__id: ID, $parent__description__owner__id: ID, $parent__description__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { BuiltinIPPrefix(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, netmask__value: $netmask__value, netmask__values: $netmask__values, netmask__isnull: $netmask__isnull, netmask__source__id: $netmask__source__id, netmask__owner__id: $netmask__owner__id, netmask__is_protected: $netmask__is_protected, hostmask__value: $hostmask__value, hostmask__values: $hostmask__values, hostmask__isnull: $hostmask__isnull, hostmask__source__id: $hostmask__source__id, hostmask__owner__id: $hostmask__owner__id, hostmask__is_protected: $hostmask__is_protected, is_top_level__value: $is_top_level__value, is_top_level__values: $is_top_level__values, is_top_level__isnull: $is_top_level__isnull, is_top_level__source__id: $is_top_level__source__id, is_top_level__owner__id: $is_top_level__owner__id, is_top_level__is_protected: $is_top_level__is_protected, utilization__value: $utilization__value, utilization__values: $utilization__values, utilization__isnull: $utilization__isnull, utilization__source__id: $utilization__source__id, utilization__owner__id: $utilization__owner__id, utilization__is_protected: $utilization__is_protected, is_pool__value: $is_pool__value, is_pool__values: $is_pool__values, is_pool__isnull: $is_pool__isnull, is_pool__source__id: $is_pool__source__id, is_pool__owner__id: $is_pool__owner__id, is_pool__is_protected: $is_pool__is_protected, broadcast_address__value: $broadcast_address__value, broadcast_address__values: $broadcast_address__values, broadcast_address__isnull: $broadcast_address__isnull, broadcast_address__source__id: $broadcast_address__source__id, broadcast_address__owner__id: $broadcast_address__owner__id, broadcast_address__is_protected: $broadcast_address__is_protected, member_type__value: $member_type__value, member_type__values: $member_type__values, member_type__isnull: $member_type__isnull, member_type__source__id: $member_type__source__id, member_type__owner__id: $member_type__owner__id, member_type__is_protected: $member_type__is_protected, network_address__value: $network_address__value, network_address__values: $network_address__values, network_address__isnull: $network_address__isnull, network_address__source__id: $network_address__source__id, network_address__owner__id: $network_address__owner__id, network_address__is_protected: $network_address__is_protected, prefix__value: $prefix__value, prefix__values: $prefix__values, prefix__isnull: $prefix__isnull, prefix__source__id: $prefix__source__id, prefix__owner__id: $prefix__owner__id, prefix__is_protected: $prefix__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, include_available: $include_available, kinds: $kinds, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, children__ids: $children__ids, children__isnull: $children__isnull, children__display_label__value: $children__display_label__value, children__display_label__values: $children__display_label__values, children__display_label__isnull: $children__display_label__isnull, children__netmask__value: $children__netmask__value, children__netmask__values: $children__netmask__values, children__netmask__source__id: $children__netmask__source__id, children__netmask__owner__id: $children__netmask__owner__id, children__netmask__is_protected: $children__netmask__is_protected, children__hostmask__value: $children__hostmask__value, children__hostmask__values: $children__hostmask__values, children__hostmask__source__id: $children__hostmask__source__id, children__hostmask__owner__id: $children__hostmask__owner__id, children__hostmask__is_protected: $children__hostmask__is_protected, children__is_top_level__value: $children__is_top_level__value, children__is_top_level__values: $children__is_top_level__values, children__is_top_level__source__id: $children__is_top_level__source__id, children__is_top_level__owner__id: $children__is_top_level__owner__id, children__is_top_level__is_protected: $children__is_top_level__is_protected, children__utilization__value: $children__utilization__value, children__utilization__values: $children__utilization__values, children__utilization__source__id: $children__utilization__source__id, children__utilization__owner__id: $children__utilization__owner__id, children__utilization__is_protected: $children__utilization__is_protected, children__is_pool__value: $children__is_pool__value, children__is_pool__values: $children__is_pool__values, children__is_pool__source__id: $children__is_pool__source__id, children__is_pool__owner__id: $children__is_pool__owner__id, children__is_pool__is_protected: $children__is_pool__is_protected, children__broadcast_address__value: $children__broadcast_address__value, children__broadcast_address__values: $children__broadcast_address__values, children__broadcast_address__source__id: $children__broadcast_address__source__id, children__broadcast_address__owner__id: $children__broadcast_address__owner__id, children__broadcast_address__is_protected: $children__broadcast_address__is_protected, children__member_type__value: $children__member_type__value, children__member_type__values: $children__member_type__values, children__member_type__source__id: $children__member_type__source__id, children__member_type__owner__id: $children__member_type__owner__id, children__member_type__is_protected: $children__member_type__is_protected, children__network_address__value: $children__network_address__value, children__network_address__values: $children__network_address__values, children__network_address__source__id: $children__network_address__source__id, children__network_address__owner__id: $children__network_address__owner__id, children__network_address__is_protected: $children__network_address__is_protected, children__prefix__value: $children__prefix__value, children__prefix__values: $children__prefix__values, children__prefix__source__id: $children__prefix__source__id, children__prefix__owner__id: $children__prefix__owner__id, children__prefix__is_protected: $children__prefix__is_protected, children__description__value: $children__description__value, children__description__values: $children__description__values, children__description__source__id: $children__description__source__id, children__description__owner__id: $children__description__owner__id, children__description__is_protected: $children__description__is_protected, resource_pool__ids: $resource_pool__ids, resource_pool__isnull: $resource_pool__isnull, resource_pool__display_label__value: $resource_pool__display_label__value, resource_pool__display_label__values: $resource_pool__display_label__values, resource_pool__display_label__isnull: $resource_pool__display_label__isnull, resource_pool__default_prefix_length__value: $resource_pool__default_prefix_length__value, resource_pool__default_prefix_length__values: $resource_pool__default_prefix_length__values, resource_pool__default_prefix_length__source__id: $resource_pool__default_prefix_length__source__id, resource_pool__default_prefix_length__owner__id: $resource_pool__default_prefix_length__owner__id, resource_pool__default_prefix_length__is_protected: $resource_pool__default_prefix_length__is_protected, resource_pool__default_address_type__value: $resource_pool__default_address_type__value, resource_pool__default_address_type__values: $resource_pool__default_address_type__values, resource_pool__default_address_type__source__id: $resource_pool__default_address_type__source__id, resource_pool__default_address_type__owner__id: $resource_pool__default_address_type__owner__id, resource_pool__default_address_type__is_protected: $resource_pool__default_address_type__is_protected, resource_pool__name__value: $resource_pool__name__value, resource_pool__name__values: $resource_pool__name__values, resource_pool__name__source__id: $resource_pool__name__source__id, resource_pool__name__owner__id: $resource_pool__name__owner__id, resource_pool__name__is_protected: $resource_pool__name__is_protected, resource_pool__description__value: $resource_pool__description__value, resource_pool__description__values: $resource_pool__description__values, resource_pool__description__source__id: $resource_pool__description__source__id, resource_pool__description__owner__id: $resource_pool__description__owner__id, resource_pool__description__is_protected: $resource_pool__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, parent__ids: $parent__ids, parent__isnull: $parent__isnull, parent__display_label__value: $parent__display_label__value, parent__display_label__values: $parent__display_label__values, parent__display_label__isnull: $parent__display_label__isnull, parent__netmask__value: $parent__netmask__value, parent__netmask__values: $parent__netmask__values, parent__netmask__source__id: $parent__netmask__source__id, parent__netmask__owner__id: $parent__netmask__owner__id, parent__netmask__is_protected: $parent__netmask__is_protected, parent__hostmask__value: $parent__hostmask__value, parent__hostmask__values: $parent__hostmask__values, parent__hostmask__source__id: $parent__hostmask__source__id, parent__hostmask__owner__id: $parent__hostmask__owner__id, parent__hostmask__is_protected: $parent__hostmask__is_protected, parent__is_top_level__value: $parent__is_top_level__value, parent__is_top_level__values: $parent__is_top_level__values, parent__is_top_level__source__id: $parent__is_top_level__source__id, parent__is_top_level__owner__id: $parent__is_top_level__owner__id, parent__is_top_level__is_protected: $parent__is_top_level__is_protected, parent__utilization__value: $parent__utilization__value, parent__utilization__values: $parent__utilization__values, parent__utilization__source__id: $parent__utilization__source__id, parent__utilization__owner__id: $parent__utilization__owner__id, parent__utilization__is_protected: $parent__utilization__is_protected, parent__is_pool__value: $parent__is_pool__value, parent__is_pool__values: $parent__is_pool__values, parent__is_pool__source__id: $parent__is_pool__source__id, parent__is_pool__owner__id: $parent__is_pool__owner__id, parent__is_pool__is_protected: $parent__is_pool__is_protected, parent__broadcast_address__value: $parent__broadcast_address__value, parent__broadcast_address__values: $parent__broadcast_address__values, parent__broadcast_address__source__id: $parent__broadcast_address__source__id, parent__broadcast_address__owner__id: $parent__broadcast_address__owner__id, parent__broadcast_address__is_protected: $parent__broadcast_address__is_protected, parent__member_type__value: $parent__member_type__value, parent__member_type__values: $parent__member_type__values, parent__member_type__source__id: $parent__member_type__source__id, parent__member_type__owner__id: $parent__member_type__owner__id, parent__member_type__is_protected: $parent__member_type__is_protected, parent__network_address__value: $parent__network_address__value, parent__network_address__values: $parent__network_address__values, parent__network_address__source__id: $parent__network_address__source__id, parent__network_address__owner__id: $parent__network_address__owner__id, parent__network_address__is_protected: $parent__network_address__is_protected, parent__prefix__value: $parent__prefix__value, parent__prefix__values: $parent__prefix__values, parent__prefix__source__id: $parent__prefix__source__id, parent__prefix__owner__id: $parent__prefix__owner__id, parent__prefix__is_protected: $parent__prefix__is_protected, parent__description__value: $parent__description__value, parent__description__values: $parent__description__values, parent__description__source__id: $parent__description__source__id, parent__description__owner__id: $parent__description__owner__id, parent__description__is_protected: $parent__description__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<BuiltinIPPrefixResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.builtin_ip_prefix.edges {
            if let Some(node) = edge.node {
                items.push(node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<BuiltinIPPrefixFilters>, request_branch: Option<&str>) -> DynPaginator<'a, serde_json::Value, String, (BuiltinIPPrefixResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query BuiltinIPPrefix($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $netmask__value: String, $netmask__values: [String], $netmask__isnull: Boolean, $netmask__source__id: ID, $netmask__owner__id: ID, $netmask__is_protected: Boolean, $hostmask__value: String, $hostmask__values: [String], $hostmask__isnull: Boolean, $hostmask__source__id: ID, $hostmask__owner__id: ID, $hostmask__is_protected: Boolean, $is_top_level__value: Boolean, $is_top_level__values: [Boolean], $is_top_level__isnull: Boolean, $is_top_level__source__id: ID, $is_top_level__owner__id: ID, $is_top_level__is_protected: Boolean, $utilization__value: BigInt, $utilization__values: [BigInt], $utilization__isnull: Boolean, $utilization__source__id: ID, $utilization__owner__id: ID, $utilization__is_protected: Boolean, $is_pool__value: Boolean, $is_pool__values: [Boolean], $is_pool__isnull: Boolean, $is_pool__source__id: ID, $is_pool__owner__id: ID, $is_pool__is_protected: Boolean, $broadcast_address__value: String, $broadcast_address__values: [String], $broadcast_address__isnull: Boolean, $broadcast_address__source__id: ID, $broadcast_address__owner__id: ID, $broadcast_address__is_protected: Boolean, $member_type__value: String, $member_type__values: [String], $member_type__isnull: Boolean, $member_type__source__id: ID, $member_type__owner__id: ID, $member_type__is_protected: Boolean, $network_address__value: String, $network_address__values: [String], $network_address__isnull: Boolean, $network_address__source__id: ID, $network_address__owner__id: ID, $network_address__is_protected: Boolean, $prefix__value: String, $prefix__values: [String], $prefix__isnull: Boolean, $prefix__source__id: ID, $prefix__owner__id: ID, $prefix__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $include_available: Boolean, $kinds: [String!], $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $children__ids: [ID], $children__isnull: Boolean, $children__display_label__value: String, $children__display_label__values: [String], $children__display_label__isnull: Boolean, $children__netmask__value: String, $children__netmask__values: [String], $children__netmask__source__id: ID, $children__netmask__owner__id: ID, $children__netmask__is_protected: Boolean, $children__hostmask__value: String, $children__hostmask__values: [String], $children__hostmask__source__id: ID, $children__hostmask__owner__id: ID, $children__hostmask__is_protected: Boolean, $children__is_top_level__value: Boolean, $children__is_top_level__values: [Boolean], $children__is_top_level__source__id: ID, $children__is_top_level__owner__id: ID, $children__is_top_level__is_protected: Boolean, $children__utilization__value: BigInt, $children__utilization__values: [BigInt], $children__utilization__source__id: ID, $children__utilization__owner__id: ID, $children__utilization__is_protected: Boolean, $children__is_pool__value: Boolean, $children__is_pool__values: [Boolean], $children__is_pool__source__id: ID, $children__is_pool__owner__id: ID, $children__is_pool__is_protected: Boolean, $children__broadcast_address__value: String, $children__broadcast_address__values: [String], $children__broadcast_address__source__id: ID, $children__broadcast_address__owner__id: ID, $children__broadcast_address__is_protected: Boolean, $children__member_type__value: String, $children__member_type__values: [String], $children__member_type__source__id: ID, $children__member_type__owner__id: ID, $children__member_type__is_protected: Boolean, $children__network_address__value: String, $children__network_address__values: [String], $children__network_address__source__id: ID, $children__network_address__owner__id: ID, $children__network_address__is_protected: Boolean, $children__prefix__value: String, $children__prefix__values: [String], $children__prefix__source__id: ID, $children__prefix__owner__id: ID, $children__prefix__is_protected: Boolean, $children__description__value: String, $children__description__values: [String], $children__description__source__id: ID, $children__description__owner__id: ID, $children__description__is_protected: Boolean, $resource_pool__ids: [ID], $resource_pool__isnull: Boolean, $resource_pool__display_label__value: String, $resource_pool__display_label__values: [String], $resource_pool__display_label__isnull: Boolean, $resource_pool__default_prefix_length__value: BigInt, $resource_pool__default_prefix_length__values: [BigInt], $resource_pool__default_prefix_length__source__id: ID, $resource_pool__default_prefix_length__owner__id: ID, $resource_pool__default_prefix_length__is_protected: Boolean, $resource_pool__default_address_type__value: String, $resource_pool__default_address_type__values: [String], $resource_pool__default_address_type__source__id: ID, $resource_pool__default_address_type__owner__id: ID, $resource_pool__default_address_type__is_protected: Boolean, $resource_pool__name__value: String, $resource_pool__name__values: [String], $resource_pool__name__source__id: ID, $resource_pool__name__owner__id: ID, $resource_pool__name__is_protected: Boolean, $resource_pool__description__value: String, $resource_pool__description__values: [String], $resource_pool__description__source__id: ID, $resource_pool__description__owner__id: ID, $resource_pool__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $parent__ids: [ID], $parent__isnull: Boolean, $parent__display_label__value: String, $parent__display_label__values: [String], $parent__display_label__isnull: Boolean, $parent__netmask__value: String, $parent__netmask__values: [String], $parent__netmask__source__id: ID, $parent__netmask__owner__id: ID, $parent__netmask__is_protected: Boolean, $parent__hostmask__value: String, $parent__hostmask__values: [String], $parent__hostmask__source__id: ID, $parent__hostmask__owner__id: ID, $parent__hostmask__is_protected: Boolean, $parent__is_top_level__value: Boolean, $parent__is_top_level__values: [Boolean], $parent__is_top_level__source__id: ID, $parent__is_top_level__owner__id: ID, $parent__is_top_level__is_protected: Boolean, $parent__utilization__value: BigInt, $parent__utilization__values: [BigInt], $parent__utilization__source__id: ID, $parent__utilization__owner__id: ID, $parent__utilization__is_protected: Boolean, $parent__is_pool__value: Boolean, $parent__is_pool__values: [Boolean], $parent__is_pool__source__id: ID, $parent__is_pool__owner__id: ID, $parent__is_pool__is_protected: Boolean, $parent__broadcast_address__value: String, $parent__broadcast_address__values: [String], $parent__broadcast_address__source__id: ID, $parent__broadcast_address__owner__id: ID, $parent__broadcast_address__is_protected: Boolean, $parent__member_type__value: String, $parent__member_type__values: [String], $parent__member_type__source__id: ID, $parent__member_type__owner__id: ID, $parent__member_type__is_protected: Boolean, $parent__network_address__value: String, $parent__network_address__values: [String], $parent__network_address__source__id: ID, $parent__network_address__owner__id: ID, $parent__network_address__is_protected: Boolean, $parent__prefix__value: String, $parent__prefix__values: [String], $parent__prefix__source__id: ID, $parent__prefix__owner__id: ID, $parent__prefix__is_protected: Boolean, $parent__description__value: String, $parent__description__values: [String], $parent__description__source__id: ID, $parent__description__owner__id: ID, $parent__description__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { BuiltinIPPrefix(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, netmask__value: $netmask__value, netmask__values: $netmask__values, netmask__isnull: $netmask__isnull, netmask__source__id: $netmask__source__id, netmask__owner__id: $netmask__owner__id, netmask__is_protected: $netmask__is_protected, hostmask__value: $hostmask__value, hostmask__values: $hostmask__values, hostmask__isnull: $hostmask__isnull, hostmask__source__id: $hostmask__source__id, hostmask__owner__id: $hostmask__owner__id, hostmask__is_protected: $hostmask__is_protected, is_top_level__value: $is_top_level__value, is_top_level__values: $is_top_level__values, is_top_level__isnull: $is_top_level__isnull, is_top_level__source__id: $is_top_level__source__id, is_top_level__owner__id: $is_top_level__owner__id, is_top_level__is_protected: $is_top_level__is_protected, utilization__value: $utilization__value, utilization__values: $utilization__values, utilization__isnull: $utilization__isnull, utilization__source__id: $utilization__source__id, utilization__owner__id: $utilization__owner__id, utilization__is_protected: $utilization__is_protected, is_pool__value: $is_pool__value, is_pool__values: $is_pool__values, is_pool__isnull: $is_pool__isnull, is_pool__source__id: $is_pool__source__id, is_pool__owner__id: $is_pool__owner__id, is_pool__is_protected: $is_pool__is_protected, broadcast_address__value: $broadcast_address__value, broadcast_address__values: $broadcast_address__values, broadcast_address__isnull: $broadcast_address__isnull, broadcast_address__source__id: $broadcast_address__source__id, broadcast_address__owner__id: $broadcast_address__owner__id, broadcast_address__is_protected: $broadcast_address__is_protected, member_type__value: $member_type__value, member_type__values: $member_type__values, member_type__isnull: $member_type__isnull, member_type__source__id: $member_type__source__id, member_type__owner__id: $member_type__owner__id, member_type__is_protected: $member_type__is_protected, network_address__value: $network_address__value, network_address__values: $network_address__values, network_address__isnull: $network_address__isnull, network_address__source__id: $network_address__source__id, network_address__owner__id: $network_address__owner__id, network_address__is_protected: $network_address__is_protected, prefix__value: $prefix__value, prefix__values: $prefix__values, prefix__isnull: $prefix__isnull, prefix__source__id: $prefix__source__id, prefix__owner__id: $prefix__owner__id, prefix__is_protected: $prefix__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, include_available: $include_available, kinds: $kinds, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, children__ids: $children__ids, children__isnull: $children__isnull, children__display_label__value: $children__display_label__value, children__display_label__values: $children__display_label__values, children__display_label__isnull: $children__display_label__isnull, children__netmask__value: $children__netmask__value, children__netmask__values: $children__netmask__values, children__netmask__source__id: $children__netmask__source__id, children__netmask__owner__id: $children__netmask__owner__id, children__netmask__is_protected: $children__netmask__is_protected, children__hostmask__value: $children__hostmask__value, children__hostmask__values: $children__hostmask__values, children__hostmask__source__id: $children__hostmask__source__id, children__hostmask__owner__id: $children__hostmask__owner__id, children__hostmask__is_protected: $children__hostmask__is_protected, children__is_top_level__value: $children__is_top_level__value, children__is_top_level__values: $children__is_top_level__values, children__is_top_level__source__id: $children__is_top_level__source__id, children__is_top_level__owner__id: $children__is_top_level__owner__id, children__is_top_level__is_protected: $children__is_top_level__is_protected, children__utilization__value: $children__utilization__value, children__utilization__values: $children__utilization__values, children__utilization__source__id: $children__utilization__source__id, children__utilization__owner__id: $children__utilization__owner__id, children__utilization__is_protected: $children__utilization__is_protected, children__is_pool__value: $children__is_pool__value, children__is_pool__values: $children__is_pool__values, children__is_pool__source__id: $children__is_pool__source__id, children__is_pool__owner__id: $children__is_pool__owner__id, children__is_pool__is_protected: $children__is_pool__is_protected, children__broadcast_address__value: $children__broadcast_address__value, children__broadcast_address__values: $children__broadcast_address__values, children__broadcast_address__source__id: $children__broadcast_address__source__id, children__broadcast_address__owner__id: $children__broadcast_address__owner__id, children__broadcast_address__is_protected: $children__broadcast_address__is_protected, children__member_type__value: $children__member_type__value, children__member_type__values: $children__member_type__values, children__member_type__source__id: $children__member_type__source__id, children__member_type__owner__id: $children__member_type__owner__id, children__member_type__is_protected: $children__member_type__is_protected, children__network_address__value: $children__network_address__value, children__network_address__values: $children__network_address__values, children__network_address__source__id: $children__network_address__source__id, children__network_address__owner__id: $children__network_address__owner__id, children__network_address__is_protected: $children__network_address__is_protected, children__prefix__value: $children__prefix__value, children__prefix__values: $children__prefix__values, children__prefix__source__id: $children__prefix__source__id, children__prefix__owner__id: $children__prefix__owner__id, children__prefix__is_protected: $children__prefix__is_protected, children__description__value: $children__description__value, children__description__values: $children__description__values, children__description__source__id: $children__description__source__id, children__description__owner__id: $children__description__owner__id, children__description__is_protected: $children__description__is_protected, resource_pool__ids: $resource_pool__ids, resource_pool__isnull: $resource_pool__isnull, resource_pool__display_label__value: $resource_pool__display_label__value, resource_pool__display_label__values: $resource_pool__display_label__values, resource_pool__display_label__isnull: $resource_pool__display_label__isnull, resource_pool__default_prefix_length__value: $resource_pool__default_prefix_length__value, resource_pool__default_prefix_length__values: $resource_pool__default_prefix_length__values, resource_pool__default_prefix_length__source__id: $resource_pool__default_prefix_length__source__id, resource_pool__default_prefix_length__owner__id: $resource_pool__default_prefix_length__owner__id, resource_pool__default_prefix_length__is_protected: $resource_pool__default_prefix_length__is_protected, resource_pool__default_address_type__value: $resource_pool__default_address_type__value, resource_pool__default_address_type__values: $resource_pool__default_address_type__values, resource_pool__default_address_type__source__id: $resource_pool__default_address_type__source__id, resource_pool__default_address_type__owner__id: $resource_pool__default_address_type__owner__id, resource_pool__default_address_type__is_protected: $resource_pool__default_address_type__is_protected, resource_pool__name__value: $resource_pool__name__value, resource_pool__name__values: $resource_pool__name__values, resource_pool__name__source__id: $resource_pool__name__source__id, resource_pool__name__owner__id: $resource_pool__name__owner__id, resource_pool__name__is_protected: $resource_pool__name__is_protected, resource_pool__description__value: $resource_pool__description__value, resource_pool__description__values: $resource_pool__description__values, resource_pool__description__source__id: $resource_pool__description__source__id, resource_pool__description__owner__id: $resource_pool__description__owner__id, resource_pool__description__is_protected: $resource_pool__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, parent__ids: $parent__ids, parent__isnull: $parent__isnull, parent__display_label__value: $parent__display_label__value, parent__display_label__values: $parent__display_label__values, parent__display_label__isnull: $parent__display_label__isnull, parent__netmask__value: $parent__netmask__value, parent__netmask__values: $parent__netmask__values, parent__netmask__source__id: $parent__netmask__source__id, parent__netmask__owner__id: $parent__netmask__owner__id, parent__netmask__is_protected: $parent__netmask__is_protected, parent__hostmask__value: $parent__hostmask__value, parent__hostmask__values: $parent__hostmask__values, parent__hostmask__source__id: $parent__hostmask__source__id, parent__hostmask__owner__id: $parent__hostmask__owner__id, parent__hostmask__is_protected: $parent__hostmask__is_protected, parent__is_top_level__value: $parent__is_top_level__value, parent__is_top_level__values: $parent__is_top_level__values, parent__is_top_level__source__id: $parent__is_top_level__source__id, parent__is_top_level__owner__id: $parent__is_top_level__owner__id, parent__is_top_level__is_protected: $parent__is_top_level__is_protected, parent__utilization__value: $parent__utilization__value, parent__utilization__values: $parent__utilization__values, parent__utilization__source__id: $parent__utilization__source__id, parent__utilization__owner__id: $parent__utilization__owner__id, parent__utilization__is_protected: $parent__utilization__is_protected, parent__is_pool__value: $parent__is_pool__value, parent__is_pool__values: $parent__is_pool__values, parent__is_pool__source__id: $parent__is_pool__source__id, parent__is_pool__owner__id: $parent__is_pool__owner__id, parent__is_pool__is_protected: $parent__is_pool__is_protected, parent__broadcast_address__value: $parent__broadcast_address__value, parent__broadcast_address__values: $parent__broadcast_address__values, parent__broadcast_address__source__id: $parent__broadcast_address__source__id, parent__broadcast_address__owner__id: $parent__broadcast_address__owner__id, parent__broadcast_address__is_protected: $parent__broadcast_address__is_protected, parent__member_type__value: $parent__member_type__value, parent__member_type__values: $parent__member_type__values, parent__member_type__source__id: $parent__member_type__source__id, parent__member_type__owner__id: $parent__member_type__owner__id, parent__member_type__is_protected: $parent__member_type__is_protected, parent__network_address__value: $parent__network_address__value, parent__network_address__values: $parent__network_address__values, parent__network_address__source__id: $parent__network_address__source__id, parent__network_address__owner__id: $parent__network_address__owner__id, parent__network_address__is_protected: $parent__network_address__is_protected, parent__prefix__value: $parent__prefix__value, parent__prefix__values: $parent__prefix__values, parent__prefix__source__id: $parent__prefix__source__id, parent__prefix__owner__id: $parent__prefix__owner__id, parent__prefix__is_protected: $parent__prefix__is_protected, parent__description__value: $parent__description__value, parent__description__values: $parent__description__values, parent__description__source__id: $parent__description__source__id, parent__description__owner__id: $parent__description__owner__id, parent__description__is_protected: $parent__description__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (BuiltinIPPrefixResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (BuiltinIPPrefixResponse, i64)> {
            let mut page_filters = base_filters.clone();
            let branch = request_branch.clone();
            let mut current_offset: i64 = 0;
            let base_offset = page_filters.offset.unwrap_or(0);
            current_offset = cursor
                .as_deref()
                .and_then(|c| c.parse::<i64>().ok())
                .unwrap_or(base_offset);
            page_filters.offset = Some(current_offset);
            Box::pin(async move {
            let vars = page_filters.to_vars()?;
                let response = client.execute::<BuiltinIPPrefixResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, serde_json::Value, String, (BuiltinIPPrefixResponse, i64)> = Box::new(move |(data, current_offset): (BuiltinIPPrefixResponse, i64)| -> Result<EdgePage<serde_json::Value, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.builtin_ip_prefix.edges {
                if let Some(node) = edge.node {
                    items.push(node);
                }
            }
            if !items.is_empty() {
                next = Some((current_offset + items.len() as i64).to_string());
            }
            Ok(EdgePage { nodes: items, next_cursor: next })
        });
        infrahub::Paginator::new(fetch, extract)
    }

    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<serde_json::Value>> {
        let mut filters = BuiltinIPPrefixFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }

}

#[derive(Debug, Clone, Default)]
pub struct BuiltinTagFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub name_value: Option<String>,
    pub name_values: Option<Vec<String>>,
    pub name_isnull: Option<bool>,
    pub name_source_id: Option<String>,
    pub name_owner_id: Option<String>,
    pub name_is_protected: Option<bool>,
    pub description_value: Option<String>,
    pub description_values: Option<Vec<String>>,
    pub description_isnull: Option<bool>,
    pub description_source_id: Option<String>,
    pub description_owner_id: Option<String>,
    pub description_is_protected: Option<bool>,
    pub any_value: Option<String>,
    pub any_values: Option<Vec<String>>,
    pub any_source_id: Option<String>,
    pub any_owner_id: Option<String>,
    pub any_is_protected: Option<bool>,
    pub partial_match: Option<bool>,
    pub node_metadata_created_by_id: Option<String>,
    pub node_metadata_created_by_ids: Option<Vec<String>>,
    pub node_metadata_updated_by_id: Option<String>,
    pub node_metadata_updated_by_ids: Option<Vec<String>>,
    pub node_metadata_created_at: Option<String>,
    pub node_metadata_created_at_before: Option<String>,
    pub node_metadata_created_at_after: Option<String>,
    pub node_metadata_updated_at: Option<String>,
    pub node_metadata_updated_at_before: Option<String>,
    pub node_metadata_updated_at_after: Option<String>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub profiles_ids: Option<Vec<String>>,
    pub profiles_isnull: Option<bool>,
    pub profiles_display_label_value: Option<String>,
    pub profiles_display_label_values: Option<Vec<String>>,
    pub profiles_display_label_isnull: Option<bool>,
    pub profiles_profile_name_value: Option<String>,
    pub profiles_profile_name_values: Option<Vec<String>>,
    pub profiles_profile_name_source_id: Option<String>,
    pub profiles_profile_name_owner_id: Option<String>,
    pub profiles_profile_name_is_protected: Option<bool>,
    pub profiles_profile_priority_value: Option<i64>,
    pub profiles_profile_priority_values: Option<Vec<i64>>,
    pub profiles_profile_priority_source_id: Option<String>,
    pub profiles_profile_priority_owner_id: Option<String>,
    pub profiles_profile_priority_is_protected: Option<bool>,
}

impl BuiltinTagFilters {
    fn to_vars(&self) -> Result<Value> {
        let mut vars = serde_json::Map::new();
        if let Some(value) = &self.offset {
            vars.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.limit {
            vars.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.order {
            vars.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ids {
            vars.insert("ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_value {
            vars.insert("display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_values {
            vars.insert("display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert("display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_value {
            vars.insert("name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_values {
            vars.insert("name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_isnull {
            vars.insert("name__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_source_id {
            vars.insert("name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_owner_id {
            vars.insert("name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name_is_protected {
            vars.insert("name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_value {
            vars.insert("description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_values {
            vars.insert("description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_isnull {
            vars.insert("description__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_source_id {
            vars.insert("description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_owner_id {
            vars.insert("description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description_is_protected {
            vars.insert("description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_value {
            vars.insert("any__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_values {
            vars.insert("any__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_source_id {
            vars.insert("any__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_owner_id {
            vars.insert("any__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any_is_protected {
            vars.insert("any__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert("node_metadata__created_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert("node_metadata__created_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert("node_metadata__updated_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert("node_metadata__updated_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert("node_metadata__created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert("node_metadata__created_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert("node_metadata__created_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert("node_metadata__updated_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert("node_metadata__updated_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert("node_metadata__updated_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert("subscriber_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert("subscriber_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert("subscriber_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert("subscriber_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert("subscriber_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert("subscriber_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert("subscriber_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert("subscriber_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert("subscriber_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert("subscriber_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert("subscriber_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert("subscriber_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert("subscriber_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert("member_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert("member_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert("member_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert("member_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert("member_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert("member_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert("member_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert("member_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert("member_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert("member_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert("member_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert("member_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert("member_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert("profiles__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert("profiles__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert("profiles__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert("profiles__profile_name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert("profiles__profile_name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert("profiles__profile_name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert("profiles__profile_name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert("profiles__profile_name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert("profiles__profile_priority__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert("profiles__profile_priority__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert("profiles__profile_priority__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert("profiles__profile_priority__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert("profiles__profile_priority__is_protected".to_string(), serde_json::to_value(value)?);
        }
        Ok(Value::Object(vars))
    }
}

pub struct BuiltinTagClient<'a> {
    client: &'a Client,
}

impl<'a> BuiltinTagClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, filters: Option<BuiltinTagFilters>, request_branch: Option<&str>) -> Result<Vec<BuiltinTag>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query BuiltinTag($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { BuiltinTag(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<BuiltinTagResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.builtin_tag.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<BuiltinTagFilters>, request_branch: Option<&str>) -> DynPaginator<'a, BuiltinTag, String, (BuiltinTagResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query BuiltinTag($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { BuiltinTag(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (BuiltinTagResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (BuiltinTagResponse, i64)> {
            let mut page_filters = base_filters.clone();
            let branch = request_branch.clone();
            let mut current_offset: i64 = 0;
            let base_offset = page_filters.offset.unwrap_or(0);
            current_offset = cursor
                .as_deref()
                .and_then(|c| c.parse::<i64>().ok())
                .unwrap_or(base_offset);
            page_filters.offset = Some(current_offset);
            Box::pin(async move {
            let vars = page_filters.to_vars()?;
                let response = client.execute::<BuiltinTagResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, BuiltinTag, String, (BuiltinTagResponse, i64)> = Box::new(move |(data, current_offset): (BuiltinTagResponse, i64)| -> Result<EdgePage<BuiltinTag, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.builtin_tag.edges {
                if let Some(node) = edge.node {
                    items.push(*node);
                }
            }
            if !items.is_empty() {
                next = Some((current_offset + items.len() as i64).to_string());
            }
            Ok(EdgePage { nodes: items, next_cursor: next })
        });
        infrahub::Paginator::new(fetch, extract)
    }

    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<BuiltinTag>> {
        let mut filters = BuiltinTagFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }

}

