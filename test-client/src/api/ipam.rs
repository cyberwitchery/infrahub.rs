//! generated api module

#![allow(non_snake_case, unused_imports, unused_assignments, clippy::field_reassign_with_default)]

use infrahub::{BoxExtract, BoxFetch, BoxFutureResult, Client, DynPaginator, EdgePage, Error, Result};
use serde_json::Value;

use crate::inputs::*;
use crate::responses::*;
use crate::types::*;

pub struct IpamApi<'a> {
    client: &'a Client,
}

impl<'a> IpamApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn namespace(&self) -> IpamNamespaceClient<'a> {
        IpamNamespaceClient::new(self.client)
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpamNamespaceFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub default_value: Option<bool>,
    pub default_values: Option<Vec<bool>>,
    pub default_isnull: Option<bool>,
    pub default_source_id: Option<String>,
    pub default_owner_id: Option<String>,
    pub default_is_protected: Option<bool>,
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

impl IpamNamespaceFilters {
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
        if let Some(value) = &self.default_value {
            vars.insert("default__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default_values {
            vars.insert("default__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default_isnull {
            vars.insert("default__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default_source_id {
            vars.insert("default__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default_owner_id {
            vars.insert("default__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default_is_protected {
            vars.insert("default__is_protected".to_string(), serde_json::to_value(value)?);
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

pub struct IpamNamespaceClient<'a> {
    client: &'a Client,
}

impl<'a> IpamNamespaceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, filters: Option<IpamNamespaceFilters>, request_branch: Option<&str>) -> Result<Vec<IpamNamespace>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query IpamNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $default__value: Boolean, $default__values: [Boolean], $default__isnull: Boolean, $default__source__id: ID, $default__owner__id: ID, $default__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean, $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { IpamNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, default__value: $default__value, default__values: $default__values, default__isnull: $default__isnull, default__source__id: $default__source__id, default__owner__id: $default__owner__id, default__is_protected: $default__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } default { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } ip_prefixes { count edges { __typename } } ip_addresses { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<IpamNamespaceResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.ipam_namespace.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<IpamNamespaceFilters>, request_branch: Option<&str>) -> DynPaginator<'a, IpamNamespace, String, (IpamNamespaceResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query IpamNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $default__value: Boolean, $default__values: [Boolean], $default__isnull: Boolean, $default__source__id: ID, $default__owner__id: ID, $default__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean, $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { IpamNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, default__value: $default__value, default__values: $default__values, default__isnull: $default__isnull, default__source__id: $default__source__id, default__owner__id: $default__owner__id, default__is_protected: $default__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } default { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } ip_prefixes { count edges { __typename } } ip_addresses { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (IpamNamespaceResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (IpamNamespaceResponse, i64)> {
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
                let response = client.execute::<IpamNamespaceResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, IpamNamespace, String, (IpamNamespaceResponse, i64)> = Box::new(move |(data, current_offset): (IpamNamespaceResponse, i64)| -> Result<EdgePage<IpamNamespace, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.ipam_namespace.edges {
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

    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<IpamNamespace>> {
        let mut filters = IpamNamespaceFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }

}

