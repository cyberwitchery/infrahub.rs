//! generated api module

#![allow(
    non_snake_case,
    unused_imports,
    unused_assignments,
    clippy::field_reassign_with_default
)]

use infrahub::{
    BoxExtract, BoxFetch, BoxFutureResult, Client, DynPaginator, EdgePage, Error, Result,
};
use serde_json::Value;

use crate::inputs::*;
use crate::responses::*;
use crate::types::*;

pub struct ProfileApi<'a> {
    client: &'a Client,
}

impl<'a> ProfileApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn builtin_ip_address(&self) -> ProfileBuiltinIPAddressClient<'a> {
        ProfileBuiltinIPAddressClient::new(self.client)
    }
    pub fn builtin_ip_prefix(&self) -> ProfileBuiltinIPPrefixClient<'a> {
        ProfileBuiltinIPPrefixClient::new(self.client)
    }
    pub fn builtin_tag(&self) -> ProfileBuiltinTagClient<'a> {
        ProfileBuiltinTagClient::new(self.client)
    }
    pub fn dcim_cable(&self) -> ProfileDcimCableClient<'a> {
        ProfileDcimCableClient::new(self.client)
    }
    pub fn dcim_device(&self) -> ProfileDcimDeviceClient<'a> {
        ProfileDcimDeviceClient::new(self.client)
    }
    pub fn dcim_device_role(&self) -> ProfileDcimDeviceRoleClient<'a> {
        ProfileDcimDeviceRoleClient::new(self.client)
    }
    pub fn dcim_device_type(&self) -> ProfileDcimDeviceTypeClient<'a> {
        ProfileDcimDeviceTypeClient::new(self.client)
    }
    pub fn dcim_interface(&self) -> ProfileDcimInterfaceClient<'a> {
        ProfileDcimInterfaceClient::new(self.client)
    }
    pub fn dcim_manufacturer(&self) -> ProfileDcimManufacturerClient<'a> {
        ProfileDcimManufacturerClient::new(self.client)
    }
    pub fn dcim_platform(&self) -> ProfileDcimPlatformClient<'a> {
        ProfileDcimPlatformClient::new(self.client)
    }
    pub fn dcim_site(&self) -> ProfileDcimSiteClient<'a> {
        ProfileDcimSiteClient::new(self.client)
    }
    pub fn ipam_ip_address(&self) -> ProfileIpamIpAddressClient<'a> {
        ProfileIpamIpAddressClient::new(self.client)
    }
    pub fn ipam_namespace(&self) -> ProfileIpamNamespaceClient<'a> {
        ProfileIpamNamespaceClient::new(self.client)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileBuiltinIPAddressFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_address_value: Option<String>,
    pub related_nodes_address_values: Option<Vec<String>>,
    pub related_nodes_address_source_id: Option<String>,
    pub related_nodes_address_owner_id: Option<String>,
    pub related_nodes_address_is_protected: Option<bool>,
    pub related_nodes_description_value: Option<String>,
    pub related_nodes_description_values: Option<Vec<String>>,
    pub related_nodes_description_source_id: Option<String>,
    pub related_nodes_description_owner_id: Option<String>,
    pub related_nodes_description_is_protected: Option<bool>,
    pub ip_namespace_ids: Option<Vec<String>>,
    pub ip_namespace_isnull: Option<bool>,
    pub ip_namespace_display_label_value: Option<String>,
    pub ip_namespace_display_label_values: Option<Vec<String>>,
    pub ip_namespace_display_label_isnull: Option<bool>,
    pub ip_namespace_name_value: Option<String>,
    pub ip_namespace_name_values: Option<Vec<String>>,
    pub ip_namespace_name_source_id: Option<String>,
    pub ip_namespace_name_owner_id: Option<String>,
    pub ip_namespace_name_is_protected: Option<bool>,
    pub ip_namespace_description_value: Option<String>,
    pub ip_namespace_description_values: Option<Vec<String>>,
    pub ip_namespace_description_source_id: Option<String>,
    pub ip_namespace_description_owner_id: Option<String>,
    pub ip_namespace_description_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileBuiltinIPAddressFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "address__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.address_owner_id {
            vars.insert(
                "address__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.address_is_protected {
            vars.insert(
                "address__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_value {
            vars.insert(
                "description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_values {
            vars.insert(
                "description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_isnull {
            vars.insert(
                "description__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_source_id {
            vars.insert(
                "description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_owner_id {
            vars.insert(
                "description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_is_protected {
            vars.insert(
                "description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_value {
            vars.insert(
                "related_nodes__address__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_values {
            vars.insert(
                "related_nodes__address__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_source_id {
            vars.insert(
                "related_nodes__address__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_owner_id {
            vars.insert(
                "related_nodes__address__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_is_protected {
            vars.insert(
                "related_nodes__address__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_value {
            vars.insert(
                "related_nodes__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_values {
            vars.insert(
                "related_nodes__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_source_id {
            vars.insert(
                "related_nodes__description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_owner_id {
            vars.insert(
                "related_nodes__description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_is_protected {
            vars.insert(
                "related_nodes__description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_ids {
            vars.insert(
                "ip_namespace__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_isnull {
            vars.insert(
                "ip_namespace__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_display_label_value {
            vars.insert(
                "ip_namespace__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_display_label_values {
            vars.insert(
                "ip_namespace__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_display_label_isnull {
            vars.insert(
                "ip_namespace__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_value {
            vars.insert(
                "ip_namespace__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_values {
            vars.insert(
                "ip_namespace__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_source_id {
            vars.insert(
                "ip_namespace__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_owner_id {
            vars.insert(
                "ip_namespace__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_is_protected {
            vars.insert(
                "ip_namespace__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_value {
            vars.insert(
                "ip_namespace__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_values {
            vars.insert(
                "ip_namespace__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_source_id {
            vars.insert(
                "ip_namespace__description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_owner_id {
            vars.insert(
                "ip_namespace__description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_is_protected {
            vars.insert(
                "ip_namespace__description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileBuiltinIPAddressClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileBuiltinIPAddressClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileBuiltinIPAddressFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileBuiltinIPAddress>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileBuiltinIPAddress($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $address__value: String, $address__values: [String], $address__isnull: Boolean, $address__source__id: ID, $address__owner__id: ID, $address__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__address__value: String, $related_nodes__address__values: [String], $related_nodes__address__source__id: ID, $related_nodes__address__owner__id: ID, $related_nodes__address__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileBuiltinIPAddress(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, address__value: $address__value, address__values: $address__values, address__isnull: $address__isnull, address__source__id: $address__source__id, address__owner__id: $address__owner__id, address__is_protected: $address__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__address__value: $related_nodes__address__value, related_nodes__address__values: $related_nodes__address__values, related_nodes__address__source__id: $related_nodes__address__source__id, related_nodes__address__owner__id: $related_nodes__address__owner__id, related_nodes__address__is_protected: $related_nodes__address__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } address { is_default is_protected updated_at id is_from_profile permissions { __typename } value ip hostmask netmask prefixlen version with_hostmask with_netmask } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } } ip_namespace { node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileBuiltinIPAddressResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_builtin_ip_address.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileBuiltinIPAddressFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileBuiltinIPAddress, String, (ProfileBuiltinIPAddressResponse, i64)>
    {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileBuiltinIPAddress($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $address__value: String, $address__values: [String], $address__isnull: Boolean, $address__source__id: ID, $address__owner__id: ID, $address__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__address__value: String, $related_nodes__address__values: [String], $related_nodes__address__source__id: ID, $related_nodes__address__owner__id: ID, $related_nodes__address__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileBuiltinIPAddress(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, address__value: $address__value, address__values: $address__values, address__isnull: $address__isnull, address__source__id: $address__source__id, address__owner__id: $address__owner__id, address__is_protected: $address__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__address__value: $related_nodes__address__value, related_nodes__address__values: $related_nodes__address__values, related_nodes__address__source__id: $related_nodes__address__source__id, related_nodes__address__owner__id: $related_nodes__address__owner__id, related_nodes__address__is_protected: $related_nodes__address__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } address { is_default is_protected updated_at id is_from_profile permissions { __typename } value ip hostmask netmask prefixlen version with_hostmask with_netmask } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } } ip_namespace { node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileBuiltinIPAddressResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileBuiltinIPAddressResponse, i64)> {
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
                let response = client.execute::<ProfileBuiltinIPAddressResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, ProfileBuiltinIPAddress, String, (ProfileBuiltinIPAddressResponse, i64)> = Box::new(move |(data, current_offset): (ProfileBuiltinIPAddressResponse, i64)| -> Result<EdgePage<ProfileBuiltinIPAddress, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_builtin_ip_address.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileBuiltinIPAddress>> {
        let mut filters = ProfileBuiltinIPAddressFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileBuiltinIPPrefixFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
    pub member_type_value: Option<String>,
    pub member_type_values: Option<Vec<String>>,
    pub member_type_isnull: Option<bool>,
    pub member_type_source_id: Option<String>,
    pub member_type_owner_id: Option<String>,
    pub member_type_is_protected: Option<bool>,
    pub description_value: Option<String>,
    pub description_values: Option<Vec<String>>,
    pub description_isnull: Option<bool>,
    pub description_source_id: Option<String>,
    pub description_owner_id: Option<String>,
    pub description_is_protected: Option<bool>,
    pub is_pool_value: Option<bool>,
    pub is_pool_values: Option<Vec<bool>>,
    pub is_pool_isnull: Option<bool>,
    pub is_pool_source_id: Option<String>,
    pub is_pool_owner_id: Option<String>,
    pub is_pool_is_protected: Option<bool>,
    pub prefix_value: Option<String>,
    pub prefix_values: Option<Vec<String>>,
    pub prefix_isnull: Option<bool>,
    pub prefix_source_id: Option<String>,
    pub prefix_owner_id: Option<String>,
    pub prefix_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_member_type_value: Option<String>,
    pub related_nodes_member_type_values: Option<Vec<String>>,
    pub related_nodes_member_type_source_id: Option<String>,
    pub related_nodes_member_type_owner_id: Option<String>,
    pub related_nodes_member_type_is_protected: Option<bool>,
    pub related_nodes_description_value: Option<String>,
    pub related_nodes_description_values: Option<Vec<String>>,
    pub related_nodes_description_source_id: Option<String>,
    pub related_nodes_description_owner_id: Option<String>,
    pub related_nodes_description_is_protected: Option<bool>,
    pub related_nodes_hostmask_value: Option<String>,
    pub related_nodes_hostmask_values: Option<Vec<String>>,
    pub related_nodes_hostmask_source_id: Option<String>,
    pub related_nodes_hostmask_owner_id: Option<String>,
    pub related_nodes_hostmask_is_protected: Option<bool>,
    pub related_nodes_network_address_value: Option<String>,
    pub related_nodes_network_address_values: Option<Vec<String>>,
    pub related_nodes_network_address_source_id: Option<String>,
    pub related_nodes_network_address_owner_id: Option<String>,
    pub related_nodes_network_address_is_protected: Option<bool>,
    pub related_nodes_broadcast_address_value: Option<String>,
    pub related_nodes_broadcast_address_values: Option<Vec<String>>,
    pub related_nodes_broadcast_address_source_id: Option<String>,
    pub related_nodes_broadcast_address_owner_id: Option<String>,
    pub related_nodes_broadcast_address_is_protected: Option<bool>,
    pub related_nodes_utilization_value: Option<i64>,
    pub related_nodes_utilization_values: Option<Vec<i64>>,
    pub related_nodes_utilization_source_id: Option<String>,
    pub related_nodes_utilization_owner_id: Option<String>,
    pub related_nodes_utilization_is_protected: Option<bool>,
    pub related_nodes_is_pool_value: Option<bool>,
    pub related_nodes_is_pool_values: Option<Vec<bool>>,
    pub related_nodes_is_pool_source_id: Option<String>,
    pub related_nodes_is_pool_owner_id: Option<String>,
    pub related_nodes_is_pool_is_protected: Option<bool>,
    pub related_nodes_prefix_value: Option<String>,
    pub related_nodes_prefix_values: Option<Vec<String>>,
    pub related_nodes_prefix_source_id: Option<String>,
    pub related_nodes_prefix_owner_id: Option<String>,
    pub related_nodes_prefix_is_protected: Option<bool>,
    pub related_nodes_is_top_level_value: Option<bool>,
    pub related_nodes_is_top_level_values: Option<Vec<bool>>,
    pub related_nodes_is_top_level_source_id: Option<String>,
    pub related_nodes_is_top_level_owner_id: Option<String>,
    pub related_nodes_is_top_level_is_protected: Option<bool>,
    pub related_nodes_netmask_value: Option<String>,
    pub related_nodes_netmask_values: Option<Vec<String>>,
    pub related_nodes_netmask_source_id: Option<String>,
    pub related_nodes_netmask_owner_id: Option<String>,
    pub related_nodes_netmask_is_protected: Option<bool>,
    pub ip_namespace_ids: Option<Vec<String>>,
    pub ip_namespace_isnull: Option<bool>,
    pub ip_namespace_display_label_value: Option<String>,
    pub ip_namespace_display_label_values: Option<Vec<String>>,
    pub ip_namespace_display_label_isnull: Option<bool>,
    pub ip_namespace_name_value: Option<String>,
    pub ip_namespace_name_values: Option<Vec<String>>,
    pub ip_namespace_name_source_id: Option<String>,
    pub ip_namespace_name_owner_id: Option<String>,
    pub ip_namespace_name_is_protected: Option<bool>,
    pub ip_namespace_description_value: Option<String>,
    pub ip_namespace_description_values: Option<Vec<String>>,
    pub ip_namespace_description_source_id: Option<String>,
    pub ip_namespace_description_owner_id: Option<String>,
    pub ip_namespace_description_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileBuiltinIPPrefixFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_type_value {
            vars.insert(
                "member_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_type_values {
            vars.insert(
                "member_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_type_isnull {
            vars.insert(
                "member_type__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_type_source_id {
            vars.insert(
                "member_type__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_type_owner_id {
            vars.insert(
                "member_type__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_type_is_protected {
            vars.insert(
                "member_type__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_value {
            vars.insert(
                "description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_values {
            vars.insert(
                "description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_isnull {
            vars.insert(
                "description__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_source_id {
            vars.insert(
                "description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_owner_id {
            vars.insert(
                "description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_is_protected {
            vars.insert(
                "description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "is_pool__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.is_pool_owner_id {
            vars.insert(
                "is_pool__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.is_pool_is_protected {
            vars.insert(
                "is_pool__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "prefix__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.prefix_owner_id {
            vars.insert(
                "prefix__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.prefix_is_protected {
            vars.insert(
                "prefix__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_member_type_value {
            vars.insert(
                "related_nodes__member_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_member_type_values {
            vars.insert(
                "related_nodes__member_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_member_type_source_id {
            vars.insert(
                "related_nodes__member_type__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_member_type_owner_id {
            vars.insert(
                "related_nodes__member_type__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_member_type_is_protected {
            vars.insert(
                "related_nodes__member_type__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_value {
            vars.insert(
                "related_nodes__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_values {
            vars.insert(
                "related_nodes__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_source_id {
            vars.insert(
                "related_nodes__description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_owner_id {
            vars.insert(
                "related_nodes__description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_is_protected {
            vars.insert(
                "related_nodes__description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_hostmask_value {
            vars.insert(
                "related_nodes__hostmask__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_hostmask_values {
            vars.insert(
                "related_nodes__hostmask__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_hostmask_source_id {
            vars.insert(
                "related_nodes__hostmask__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_hostmask_owner_id {
            vars.insert(
                "related_nodes__hostmask__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_hostmask_is_protected {
            vars.insert(
                "related_nodes__hostmask__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_network_address_value {
            vars.insert(
                "related_nodes__network_address__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_network_address_values {
            vars.insert(
                "related_nodes__network_address__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_network_address_source_id {
            vars.insert(
                "related_nodes__network_address__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_network_address_owner_id {
            vars.insert(
                "related_nodes__network_address__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_network_address_is_protected {
            vars.insert(
                "related_nodes__network_address__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_broadcast_address_value {
            vars.insert(
                "related_nodes__broadcast_address__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_broadcast_address_values {
            vars.insert(
                "related_nodes__broadcast_address__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_broadcast_address_source_id {
            vars.insert(
                "related_nodes__broadcast_address__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_broadcast_address_owner_id {
            vars.insert(
                "related_nodes__broadcast_address__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_broadcast_address_is_protected {
            vars.insert(
                "related_nodes__broadcast_address__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_utilization_value {
            vars.insert(
                "related_nodes__utilization__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_utilization_values {
            vars.insert(
                "related_nodes__utilization__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_utilization_source_id {
            vars.insert(
                "related_nodes__utilization__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_utilization_owner_id {
            vars.insert(
                "related_nodes__utilization__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_utilization_is_protected {
            vars.insert(
                "related_nodes__utilization__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_pool_value {
            vars.insert(
                "related_nodes__is_pool__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_pool_values {
            vars.insert(
                "related_nodes__is_pool__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_pool_source_id {
            vars.insert(
                "related_nodes__is_pool__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_pool_owner_id {
            vars.insert(
                "related_nodes__is_pool__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_pool_is_protected {
            vars.insert(
                "related_nodes__is_pool__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_prefix_value {
            vars.insert(
                "related_nodes__prefix__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_prefix_values {
            vars.insert(
                "related_nodes__prefix__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_prefix_source_id {
            vars.insert(
                "related_nodes__prefix__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_prefix_owner_id {
            vars.insert(
                "related_nodes__prefix__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_prefix_is_protected {
            vars.insert(
                "related_nodes__prefix__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_top_level_value {
            vars.insert(
                "related_nodes__is_top_level__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_top_level_values {
            vars.insert(
                "related_nodes__is_top_level__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_top_level_source_id {
            vars.insert(
                "related_nodes__is_top_level__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_top_level_owner_id {
            vars.insert(
                "related_nodes__is_top_level__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_is_top_level_is_protected {
            vars.insert(
                "related_nodes__is_top_level__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_netmask_value {
            vars.insert(
                "related_nodes__netmask__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_netmask_values {
            vars.insert(
                "related_nodes__netmask__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_netmask_source_id {
            vars.insert(
                "related_nodes__netmask__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_netmask_owner_id {
            vars.insert(
                "related_nodes__netmask__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_netmask_is_protected {
            vars.insert(
                "related_nodes__netmask__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_ids {
            vars.insert(
                "ip_namespace__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_isnull {
            vars.insert(
                "ip_namespace__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_display_label_value {
            vars.insert(
                "ip_namespace__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_display_label_values {
            vars.insert(
                "ip_namespace__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_display_label_isnull {
            vars.insert(
                "ip_namespace__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_value {
            vars.insert(
                "ip_namespace__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_values {
            vars.insert(
                "ip_namespace__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_source_id {
            vars.insert(
                "ip_namespace__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_owner_id {
            vars.insert(
                "ip_namespace__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_name_is_protected {
            vars.insert(
                "ip_namespace__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_value {
            vars.insert(
                "ip_namespace__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_values {
            vars.insert(
                "ip_namespace__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_source_id {
            vars.insert(
                "ip_namespace__description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_owner_id {
            vars.insert(
                "ip_namespace__description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_namespace_description_is_protected {
            vars.insert(
                "ip_namespace__description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileBuiltinIPPrefixClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileBuiltinIPPrefixClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileBuiltinIPPrefixFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileBuiltinIPPrefix>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileBuiltinIPPrefix($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $member_type__value: String, $member_type__values: [String], $member_type__isnull: Boolean, $member_type__source__id: ID, $member_type__owner__id: ID, $member_type__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $is_pool__value: Boolean, $is_pool__values: [Boolean], $is_pool__isnull: Boolean, $is_pool__source__id: ID, $is_pool__owner__id: ID, $is_pool__is_protected: Boolean, $prefix__value: String, $prefix__values: [String], $prefix__isnull: Boolean, $prefix__source__id: ID, $prefix__owner__id: ID, $prefix__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__member_type__value: String, $related_nodes__member_type__values: [String], $related_nodes__member_type__source__id: ID, $related_nodes__member_type__owner__id: ID, $related_nodes__member_type__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $related_nodes__hostmask__value: String, $related_nodes__hostmask__values: [String], $related_nodes__hostmask__source__id: ID, $related_nodes__hostmask__owner__id: ID, $related_nodes__hostmask__is_protected: Boolean, $related_nodes__network_address__value: String, $related_nodes__network_address__values: [String], $related_nodes__network_address__source__id: ID, $related_nodes__network_address__owner__id: ID, $related_nodes__network_address__is_protected: Boolean, $related_nodes__broadcast_address__value: String, $related_nodes__broadcast_address__values: [String], $related_nodes__broadcast_address__source__id: ID, $related_nodes__broadcast_address__owner__id: ID, $related_nodes__broadcast_address__is_protected: Boolean, $related_nodes__utilization__value: BigInt, $related_nodes__utilization__values: [BigInt], $related_nodes__utilization__source__id: ID, $related_nodes__utilization__owner__id: ID, $related_nodes__utilization__is_protected: Boolean, $related_nodes__is_pool__value: Boolean, $related_nodes__is_pool__values: [Boolean], $related_nodes__is_pool__source__id: ID, $related_nodes__is_pool__owner__id: ID, $related_nodes__is_pool__is_protected: Boolean, $related_nodes__prefix__value: String, $related_nodes__prefix__values: [String], $related_nodes__prefix__source__id: ID, $related_nodes__prefix__owner__id: ID, $related_nodes__prefix__is_protected: Boolean, $related_nodes__is_top_level__value: Boolean, $related_nodes__is_top_level__values: [Boolean], $related_nodes__is_top_level__source__id: ID, $related_nodes__is_top_level__owner__id: ID, $related_nodes__is_top_level__is_protected: Boolean, $related_nodes__netmask__value: String, $related_nodes__netmask__values: [String], $related_nodes__netmask__source__id: ID, $related_nodes__netmask__owner__id: ID, $related_nodes__netmask__is_protected: Boolean, $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileBuiltinIPPrefix(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, member_type__value: $member_type__value, member_type__values: $member_type__values, member_type__isnull: $member_type__isnull, member_type__source__id: $member_type__source__id, member_type__owner__id: $member_type__owner__id, member_type__is_protected: $member_type__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, is_pool__value: $is_pool__value, is_pool__values: $is_pool__values, is_pool__isnull: $is_pool__isnull, is_pool__source__id: $is_pool__source__id, is_pool__owner__id: $is_pool__owner__id, is_pool__is_protected: $is_pool__is_protected, prefix__value: $prefix__value, prefix__values: $prefix__values, prefix__isnull: $prefix__isnull, prefix__source__id: $prefix__source__id, prefix__owner__id: $prefix__owner__id, prefix__is_protected: $prefix__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__member_type__value: $related_nodes__member_type__value, related_nodes__member_type__values: $related_nodes__member_type__values, related_nodes__member_type__source__id: $related_nodes__member_type__source__id, related_nodes__member_type__owner__id: $related_nodes__member_type__owner__id, related_nodes__member_type__is_protected: $related_nodes__member_type__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, related_nodes__hostmask__value: $related_nodes__hostmask__value, related_nodes__hostmask__values: $related_nodes__hostmask__values, related_nodes__hostmask__source__id: $related_nodes__hostmask__source__id, related_nodes__hostmask__owner__id: $related_nodes__hostmask__owner__id, related_nodes__hostmask__is_protected: $related_nodes__hostmask__is_protected, related_nodes__network_address__value: $related_nodes__network_address__value, related_nodes__network_address__values: $related_nodes__network_address__values, related_nodes__network_address__source__id: $related_nodes__network_address__source__id, related_nodes__network_address__owner__id: $related_nodes__network_address__owner__id, related_nodes__network_address__is_protected: $related_nodes__network_address__is_protected, related_nodes__broadcast_address__value: $related_nodes__broadcast_address__value, related_nodes__broadcast_address__values: $related_nodes__broadcast_address__values, related_nodes__broadcast_address__source__id: $related_nodes__broadcast_address__source__id, related_nodes__broadcast_address__owner__id: $related_nodes__broadcast_address__owner__id, related_nodes__broadcast_address__is_protected: $related_nodes__broadcast_address__is_protected, related_nodes__utilization__value: $related_nodes__utilization__value, related_nodes__utilization__values: $related_nodes__utilization__values, related_nodes__utilization__source__id: $related_nodes__utilization__source__id, related_nodes__utilization__owner__id: $related_nodes__utilization__owner__id, related_nodes__utilization__is_protected: $related_nodes__utilization__is_protected, related_nodes__is_pool__value: $related_nodes__is_pool__value, related_nodes__is_pool__values: $related_nodes__is_pool__values, related_nodes__is_pool__source__id: $related_nodes__is_pool__source__id, related_nodes__is_pool__owner__id: $related_nodes__is_pool__owner__id, related_nodes__is_pool__is_protected: $related_nodes__is_pool__is_protected, related_nodes__prefix__value: $related_nodes__prefix__value, related_nodes__prefix__values: $related_nodes__prefix__values, related_nodes__prefix__source__id: $related_nodes__prefix__source__id, related_nodes__prefix__owner__id: $related_nodes__prefix__owner__id, related_nodes__prefix__is_protected: $related_nodes__prefix__is_protected, related_nodes__is_top_level__value: $related_nodes__is_top_level__value, related_nodes__is_top_level__values: $related_nodes__is_top_level__values, related_nodes__is_top_level__source__id: $related_nodes__is_top_level__source__id, related_nodes__is_top_level__owner__id: $related_nodes__is_top_level__owner__id, related_nodes__is_top_level__is_protected: $related_nodes__is_top_level__is_protected, related_nodes__netmask__value: $related_nodes__netmask__value, related_nodes__netmask__values: $related_nodes__netmask__values, related_nodes__netmask__source__id: $related_nodes__netmask__source__id, related_nodes__netmask__owner__id: $related_nodes__netmask__owner__id, related_nodes__netmask__is_protected: $related_nodes__netmask__is_protected, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_type { is_default is_protected updated_at value label color description id is_from_profile permissions { __typename } } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } is_pool { is_default is_protected updated_at id is_from_profile permissions { __typename } value } prefix { is_default is_protected updated_at id is_from_profile permissions { __typename } value broadcast_address hostmask netmask prefixlen num_addresses version with_hostmask with_netmask } related_nodes { count edges { __typename } } ip_namespace { node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileBuiltinIPPrefixResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_builtin_ip_prefix.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileBuiltinIPPrefixFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileBuiltinIPPrefix, String, (ProfileBuiltinIPPrefixResponse, i64)>
    {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileBuiltinIPPrefix($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $member_type__value: String, $member_type__values: [String], $member_type__isnull: Boolean, $member_type__source__id: ID, $member_type__owner__id: ID, $member_type__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $is_pool__value: Boolean, $is_pool__values: [Boolean], $is_pool__isnull: Boolean, $is_pool__source__id: ID, $is_pool__owner__id: ID, $is_pool__is_protected: Boolean, $prefix__value: String, $prefix__values: [String], $prefix__isnull: Boolean, $prefix__source__id: ID, $prefix__owner__id: ID, $prefix__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__member_type__value: String, $related_nodes__member_type__values: [String], $related_nodes__member_type__source__id: ID, $related_nodes__member_type__owner__id: ID, $related_nodes__member_type__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $related_nodes__hostmask__value: String, $related_nodes__hostmask__values: [String], $related_nodes__hostmask__source__id: ID, $related_nodes__hostmask__owner__id: ID, $related_nodes__hostmask__is_protected: Boolean, $related_nodes__network_address__value: String, $related_nodes__network_address__values: [String], $related_nodes__network_address__source__id: ID, $related_nodes__network_address__owner__id: ID, $related_nodes__network_address__is_protected: Boolean, $related_nodes__broadcast_address__value: String, $related_nodes__broadcast_address__values: [String], $related_nodes__broadcast_address__source__id: ID, $related_nodes__broadcast_address__owner__id: ID, $related_nodes__broadcast_address__is_protected: Boolean, $related_nodes__utilization__value: BigInt, $related_nodes__utilization__values: [BigInt], $related_nodes__utilization__source__id: ID, $related_nodes__utilization__owner__id: ID, $related_nodes__utilization__is_protected: Boolean, $related_nodes__is_pool__value: Boolean, $related_nodes__is_pool__values: [Boolean], $related_nodes__is_pool__source__id: ID, $related_nodes__is_pool__owner__id: ID, $related_nodes__is_pool__is_protected: Boolean, $related_nodes__prefix__value: String, $related_nodes__prefix__values: [String], $related_nodes__prefix__source__id: ID, $related_nodes__prefix__owner__id: ID, $related_nodes__prefix__is_protected: Boolean, $related_nodes__is_top_level__value: Boolean, $related_nodes__is_top_level__values: [Boolean], $related_nodes__is_top_level__source__id: ID, $related_nodes__is_top_level__owner__id: ID, $related_nodes__is_top_level__is_protected: Boolean, $related_nodes__netmask__value: String, $related_nodes__netmask__values: [String], $related_nodes__netmask__source__id: ID, $related_nodes__netmask__owner__id: ID, $related_nodes__netmask__is_protected: Boolean, $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileBuiltinIPPrefix(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, member_type__value: $member_type__value, member_type__values: $member_type__values, member_type__isnull: $member_type__isnull, member_type__source__id: $member_type__source__id, member_type__owner__id: $member_type__owner__id, member_type__is_protected: $member_type__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, is_pool__value: $is_pool__value, is_pool__values: $is_pool__values, is_pool__isnull: $is_pool__isnull, is_pool__source__id: $is_pool__source__id, is_pool__owner__id: $is_pool__owner__id, is_pool__is_protected: $is_pool__is_protected, prefix__value: $prefix__value, prefix__values: $prefix__values, prefix__isnull: $prefix__isnull, prefix__source__id: $prefix__source__id, prefix__owner__id: $prefix__owner__id, prefix__is_protected: $prefix__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__member_type__value: $related_nodes__member_type__value, related_nodes__member_type__values: $related_nodes__member_type__values, related_nodes__member_type__source__id: $related_nodes__member_type__source__id, related_nodes__member_type__owner__id: $related_nodes__member_type__owner__id, related_nodes__member_type__is_protected: $related_nodes__member_type__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, related_nodes__hostmask__value: $related_nodes__hostmask__value, related_nodes__hostmask__values: $related_nodes__hostmask__values, related_nodes__hostmask__source__id: $related_nodes__hostmask__source__id, related_nodes__hostmask__owner__id: $related_nodes__hostmask__owner__id, related_nodes__hostmask__is_protected: $related_nodes__hostmask__is_protected, related_nodes__network_address__value: $related_nodes__network_address__value, related_nodes__network_address__values: $related_nodes__network_address__values, related_nodes__network_address__source__id: $related_nodes__network_address__source__id, related_nodes__network_address__owner__id: $related_nodes__network_address__owner__id, related_nodes__network_address__is_protected: $related_nodes__network_address__is_protected, related_nodes__broadcast_address__value: $related_nodes__broadcast_address__value, related_nodes__broadcast_address__values: $related_nodes__broadcast_address__values, related_nodes__broadcast_address__source__id: $related_nodes__broadcast_address__source__id, related_nodes__broadcast_address__owner__id: $related_nodes__broadcast_address__owner__id, related_nodes__broadcast_address__is_protected: $related_nodes__broadcast_address__is_protected, related_nodes__utilization__value: $related_nodes__utilization__value, related_nodes__utilization__values: $related_nodes__utilization__values, related_nodes__utilization__source__id: $related_nodes__utilization__source__id, related_nodes__utilization__owner__id: $related_nodes__utilization__owner__id, related_nodes__utilization__is_protected: $related_nodes__utilization__is_protected, related_nodes__is_pool__value: $related_nodes__is_pool__value, related_nodes__is_pool__values: $related_nodes__is_pool__values, related_nodes__is_pool__source__id: $related_nodes__is_pool__source__id, related_nodes__is_pool__owner__id: $related_nodes__is_pool__owner__id, related_nodes__is_pool__is_protected: $related_nodes__is_pool__is_protected, related_nodes__prefix__value: $related_nodes__prefix__value, related_nodes__prefix__values: $related_nodes__prefix__values, related_nodes__prefix__source__id: $related_nodes__prefix__source__id, related_nodes__prefix__owner__id: $related_nodes__prefix__owner__id, related_nodes__prefix__is_protected: $related_nodes__prefix__is_protected, related_nodes__is_top_level__value: $related_nodes__is_top_level__value, related_nodes__is_top_level__values: $related_nodes__is_top_level__values, related_nodes__is_top_level__source__id: $related_nodes__is_top_level__source__id, related_nodes__is_top_level__owner__id: $related_nodes__is_top_level__owner__id, related_nodes__is_top_level__is_protected: $related_nodes__is_top_level__is_protected, related_nodes__netmask__value: $related_nodes__netmask__value, related_nodes__netmask__values: $related_nodes__netmask__values, related_nodes__netmask__source__id: $related_nodes__netmask__source__id, related_nodes__netmask__owner__id: $related_nodes__netmask__owner__id, related_nodes__netmask__is_protected: $related_nodes__netmask__is_protected, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_type { is_default is_protected updated_at value label color description id is_from_profile permissions { __typename } } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } is_pool { is_default is_protected updated_at id is_from_profile permissions { __typename } value } prefix { is_default is_protected updated_at id is_from_profile permissions { __typename } value broadcast_address hostmask netmask prefixlen num_addresses version with_hostmask with_netmask } related_nodes { count edges { __typename } } ip_namespace { node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileBuiltinIPPrefixResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileBuiltinIPPrefixResponse, i64)> {
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
                let response = client.execute::<ProfileBuiltinIPPrefixResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, ProfileBuiltinIPPrefix, String, (ProfileBuiltinIPPrefixResponse, i64)> = Box::new(move |(data, current_offset): (ProfileBuiltinIPPrefixResponse, i64)| -> Result<EdgePage<ProfileBuiltinIPPrefix, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_builtin_ip_prefix.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileBuiltinIPPrefix>> {
        let mut filters = ProfileBuiltinIPPrefixFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileBuiltinTagFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_name_value: Option<String>,
    pub related_nodes_name_values: Option<Vec<String>>,
    pub related_nodes_name_source_id: Option<String>,
    pub related_nodes_name_owner_id: Option<String>,
    pub related_nodes_name_is_protected: Option<bool>,
    pub related_nodes_description_value: Option<String>,
    pub related_nodes_description_values: Option<Vec<String>>,
    pub related_nodes_description_source_id: Option<String>,
    pub related_nodes_description_owner_id: Option<String>,
    pub related_nodes_description_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileBuiltinTagFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_value {
            vars.insert(
                "description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_values {
            vars.insert(
                "description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_isnull {
            vars.insert(
                "description__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_source_id {
            vars.insert(
                "description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_owner_id {
            vars.insert(
                "description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_is_protected {
            vars.insert(
                "description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_value {
            vars.insert(
                "related_nodes__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_values {
            vars.insert(
                "related_nodes__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_source_id {
            vars.insert(
                "related_nodes__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_owner_id {
            vars.insert(
                "related_nodes__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_is_protected {
            vars.insert(
                "related_nodes__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_value {
            vars.insert(
                "related_nodes__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_values {
            vars.insert(
                "related_nodes__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_source_id {
            vars.insert(
                "related_nodes__description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_owner_id {
            vars.insert(
                "related_nodes__description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_is_protected {
            vars.insert(
                "related_nodes__description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileBuiltinTagClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileBuiltinTagClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileBuiltinTagFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileBuiltinTag>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileBuiltinTag($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileBuiltinTag(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileBuiltinTagResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_builtin_tag.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileBuiltinTagFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileBuiltinTag, String, (ProfileBuiltinTagResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileBuiltinTag($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileBuiltinTag(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileBuiltinTagResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileBuiltinTagResponse, i64)> {
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
                    let response = client
                        .execute::<ProfileBuiltinTagResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, ProfileBuiltinTag, String, (ProfileBuiltinTagResponse, i64)> = Box::new(move |(data, current_offset): (ProfileBuiltinTagResponse, i64)| -> Result<EdgePage<ProfileBuiltinTag, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_builtin_tag.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileBuiltinTag>> {
        let mut filters = ProfileBuiltinTagFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileDcimCableFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_label_value: Option<String>,
    pub related_nodes_label_values: Option<Vec<String>>,
    pub related_nodes_label_source_id: Option<String>,
    pub related_nodes_label_owner_id: Option<String>,
    pub related_nodes_label_is_protected: Option<bool>,
    pub b_terminations_ids: Option<Vec<String>>,
    pub b_terminations_isnull: Option<bool>,
    pub b_terminations_display_label_value: Option<String>,
    pub b_terminations_display_label_values: Option<Vec<String>>,
    pub b_terminations_display_label_isnull: Option<bool>,
    pub b_terminations_if_type_value: Option<String>,
    pub b_terminations_if_type_values: Option<Vec<String>>,
    pub b_terminations_if_type_source_id: Option<String>,
    pub b_terminations_if_type_owner_id: Option<String>,
    pub b_terminations_if_type_is_protected: Option<bool>,
    pub b_terminations_name_value: Option<String>,
    pub b_terminations_name_values: Option<Vec<String>>,
    pub b_terminations_name_source_id: Option<String>,
    pub b_terminations_name_owner_id: Option<String>,
    pub b_terminations_name_is_protected: Option<bool>,
    pub b_terminations_enabled_value: Option<bool>,
    pub b_terminations_enabled_values: Option<Vec<bool>>,
    pub b_terminations_enabled_source_id: Option<String>,
    pub b_terminations_enabled_owner_id: Option<String>,
    pub b_terminations_enabled_is_protected: Option<bool>,
    pub a_terminations_ids: Option<Vec<String>>,
    pub a_terminations_isnull: Option<bool>,
    pub a_terminations_display_label_value: Option<String>,
    pub a_terminations_display_label_values: Option<Vec<String>>,
    pub a_terminations_display_label_isnull: Option<bool>,
    pub a_terminations_if_type_value: Option<String>,
    pub a_terminations_if_type_values: Option<Vec<String>>,
    pub a_terminations_if_type_source_id: Option<String>,
    pub a_terminations_if_type_owner_id: Option<String>,
    pub a_terminations_if_type_is_protected: Option<bool>,
    pub a_terminations_name_value: Option<String>,
    pub a_terminations_name_values: Option<Vec<String>>,
    pub a_terminations_name_source_id: Option<String>,
    pub a_terminations_name_owner_id: Option<String>,
    pub a_terminations_name_is_protected: Option<bool>,
    pub a_terminations_enabled_value: Option<bool>,
    pub a_terminations_enabled_values: Option<Vec<bool>>,
    pub a_terminations_enabled_source_id: Option<String>,
    pub a_terminations_enabled_owner_id: Option<String>,
    pub a_terminations_enabled_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileDcimCableFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_label_value {
            vars.insert(
                "related_nodes__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_label_values {
            vars.insert(
                "related_nodes__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_label_source_id {
            vars.insert(
                "related_nodes__label__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_label_owner_id {
            vars.insert(
                "related_nodes__label__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_label_is_protected {
            vars.insert(
                "related_nodes__label__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_ids {
            vars.insert(
                "b_terminations__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_isnull {
            vars.insert(
                "b_terminations__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_display_label_value {
            vars.insert(
                "b_terminations__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_display_label_values {
            vars.insert(
                "b_terminations__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_display_label_isnull {
            vars.insert(
                "b_terminations__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_if_type_value {
            vars.insert(
                "b_terminations__if_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_if_type_values {
            vars.insert(
                "b_terminations__if_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_if_type_source_id {
            vars.insert(
                "b_terminations__if_type__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_if_type_owner_id {
            vars.insert(
                "b_terminations__if_type__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_if_type_is_protected {
            vars.insert(
                "b_terminations__if_type__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_name_value {
            vars.insert(
                "b_terminations__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_name_values {
            vars.insert(
                "b_terminations__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_name_source_id {
            vars.insert(
                "b_terminations__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_name_owner_id {
            vars.insert(
                "b_terminations__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_name_is_protected {
            vars.insert(
                "b_terminations__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_enabled_value {
            vars.insert(
                "b_terminations__enabled__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_enabled_values {
            vars.insert(
                "b_terminations__enabled__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_enabled_source_id {
            vars.insert(
                "b_terminations__enabled__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_enabled_owner_id {
            vars.insert(
                "b_terminations__enabled__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.b_terminations_enabled_is_protected {
            vars.insert(
                "b_terminations__enabled__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_ids {
            vars.insert(
                "a_terminations__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_isnull {
            vars.insert(
                "a_terminations__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_display_label_value {
            vars.insert(
                "a_terminations__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_display_label_values {
            vars.insert(
                "a_terminations__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_display_label_isnull {
            vars.insert(
                "a_terminations__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_if_type_value {
            vars.insert(
                "a_terminations__if_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_if_type_values {
            vars.insert(
                "a_terminations__if_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_if_type_source_id {
            vars.insert(
                "a_terminations__if_type__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_if_type_owner_id {
            vars.insert(
                "a_terminations__if_type__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_if_type_is_protected {
            vars.insert(
                "a_terminations__if_type__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_name_value {
            vars.insert(
                "a_terminations__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_name_values {
            vars.insert(
                "a_terminations__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_name_source_id {
            vars.insert(
                "a_terminations__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_name_owner_id {
            vars.insert(
                "a_terminations__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_name_is_protected {
            vars.insert(
                "a_terminations__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_enabled_value {
            vars.insert(
                "a_terminations__enabled__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_enabled_values {
            vars.insert(
                "a_terminations__enabled__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_enabled_source_id {
            vars.insert(
                "a_terminations__enabled__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_enabled_owner_id {
            vars.insert(
                "a_terminations__enabled__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.a_terminations_enabled_is_protected {
            vars.insert(
                "a_terminations__enabled__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileDcimCableClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileDcimCableClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileDcimCableFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileDcimCable>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileDcimCable($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__label__value: String, $related_nodes__label__values: [String], $related_nodes__label__source__id: ID, $related_nodes__label__owner__id: ID, $related_nodes__label__is_protected: Boolean, $b_terminations__ids: [ID], $b_terminations__isnull: Boolean, $b_terminations__display_label__value: String, $b_terminations__display_label__values: [String], $b_terminations__display_label__isnull: Boolean, $b_terminations__if_type__value: String, $b_terminations__if_type__values: [String], $b_terminations__if_type__source__id: ID, $b_terminations__if_type__owner__id: ID, $b_terminations__if_type__is_protected: Boolean, $b_terminations__name__value: String, $b_terminations__name__values: [String], $b_terminations__name__source__id: ID, $b_terminations__name__owner__id: ID, $b_terminations__name__is_protected: Boolean, $b_terminations__enabled__value: Boolean, $b_terminations__enabled__values: [Boolean], $b_terminations__enabled__source__id: ID, $b_terminations__enabled__owner__id: ID, $b_terminations__enabled__is_protected: Boolean, $a_terminations__ids: [ID], $a_terminations__isnull: Boolean, $a_terminations__display_label__value: String, $a_terminations__display_label__values: [String], $a_terminations__display_label__isnull: Boolean, $a_terminations__if_type__value: String, $a_terminations__if_type__values: [String], $a_terminations__if_type__source__id: ID, $a_terminations__if_type__owner__id: ID, $a_terminations__if_type__is_protected: Boolean, $a_terminations__name__value: String, $a_terminations__name__values: [String], $a_terminations__name__source__id: ID, $a_terminations__name__owner__id: ID, $a_terminations__name__is_protected: Boolean, $a_terminations__enabled__value: Boolean, $a_terminations__enabled__values: [Boolean], $a_terminations__enabled__source__id: ID, $a_terminations__enabled__owner__id: ID, $a_terminations__enabled__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimCable(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__label__value: $related_nodes__label__value, related_nodes__label__values: $related_nodes__label__values, related_nodes__label__source__id: $related_nodes__label__source__id, related_nodes__label__owner__id: $related_nodes__label__owner__id, related_nodes__label__is_protected: $related_nodes__label__is_protected, b_terminations__ids: $b_terminations__ids, b_terminations__isnull: $b_terminations__isnull, b_terminations__display_label__value: $b_terminations__display_label__value, b_terminations__display_label__values: $b_terminations__display_label__values, b_terminations__display_label__isnull: $b_terminations__display_label__isnull, b_terminations__if_type__value: $b_terminations__if_type__value, b_terminations__if_type__values: $b_terminations__if_type__values, b_terminations__if_type__source__id: $b_terminations__if_type__source__id, b_terminations__if_type__owner__id: $b_terminations__if_type__owner__id, b_terminations__if_type__is_protected: $b_terminations__if_type__is_protected, b_terminations__name__value: $b_terminations__name__value, b_terminations__name__values: $b_terminations__name__values, b_terminations__name__source__id: $b_terminations__name__source__id, b_terminations__name__owner__id: $b_terminations__name__owner__id, b_terminations__name__is_protected: $b_terminations__name__is_protected, b_terminations__enabled__value: $b_terminations__enabled__value, b_terminations__enabled__values: $b_terminations__enabled__values, b_terminations__enabled__source__id: $b_terminations__enabled__source__id, b_terminations__enabled__owner__id: $b_terminations__enabled__owner__id, b_terminations__enabled__is_protected: $b_terminations__enabled__is_protected, a_terminations__ids: $a_terminations__ids, a_terminations__isnull: $a_terminations__isnull, a_terminations__display_label__value: $a_terminations__display_label__value, a_terminations__display_label__values: $a_terminations__display_label__values, a_terminations__display_label__isnull: $a_terminations__display_label__isnull, a_terminations__if_type__value: $a_terminations__if_type__value, a_terminations__if_type__values: $a_terminations__if_type__values, a_terminations__if_type__source__id: $a_terminations__if_type__source__id, a_terminations__if_type__owner__id: $a_terminations__if_type__owner__id, a_terminations__if_type__is_protected: $a_terminations__if_type__is_protected, a_terminations__name__value: $a_terminations__name__value, a_terminations__name__values: $a_terminations__name__values, a_terminations__name__source__id: $a_terminations__name__source__id, a_terminations__name__owner__id: $a_terminations__name__owner__id, a_terminations__name__is_protected: $a_terminations__name__is_protected, a_terminations__enabled__value: $a_terminations__enabled__value, a_terminations__enabled__values: $a_terminations__enabled__values, a_terminations__enabled__source__id: $a_terminations__enabled__source__id, a_terminations__enabled__owner__id: $a_terminations__enabled__owner__id, a_terminations__enabled__is_protected: $a_terminations__enabled__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } b_terminations { count edges { __typename } permissions { __typename } } a_terminations { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileDcimCableResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_dcim_cable.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileDcimCableFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileDcimCable, String, (ProfileDcimCableResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileDcimCable($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__label__value: String, $related_nodes__label__values: [String], $related_nodes__label__source__id: ID, $related_nodes__label__owner__id: ID, $related_nodes__label__is_protected: Boolean, $b_terminations__ids: [ID], $b_terminations__isnull: Boolean, $b_terminations__display_label__value: String, $b_terminations__display_label__values: [String], $b_terminations__display_label__isnull: Boolean, $b_terminations__if_type__value: String, $b_terminations__if_type__values: [String], $b_terminations__if_type__source__id: ID, $b_terminations__if_type__owner__id: ID, $b_terminations__if_type__is_protected: Boolean, $b_terminations__name__value: String, $b_terminations__name__values: [String], $b_terminations__name__source__id: ID, $b_terminations__name__owner__id: ID, $b_terminations__name__is_protected: Boolean, $b_terminations__enabled__value: Boolean, $b_terminations__enabled__values: [Boolean], $b_terminations__enabled__source__id: ID, $b_terminations__enabled__owner__id: ID, $b_terminations__enabled__is_protected: Boolean, $a_terminations__ids: [ID], $a_terminations__isnull: Boolean, $a_terminations__display_label__value: String, $a_terminations__display_label__values: [String], $a_terminations__display_label__isnull: Boolean, $a_terminations__if_type__value: String, $a_terminations__if_type__values: [String], $a_terminations__if_type__source__id: ID, $a_terminations__if_type__owner__id: ID, $a_terminations__if_type__is_protected: Boolean, $a_terminations__name__value: String, $a_terminations__name__values: [String], $a_terminations__name__source__id: ID, $a_terminations__name__owner__id: ID, $a_terminations__name__is_protected: Boolean, $a_terminations__enabled__value: Boolean, $a_terminations__enabled__values: [Boolean], $a_terminations__enabled__source__id: ID, $a_terminations__enabled__owner__id: ID, $a_terminations__enabled__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimCable(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__label__value: $related_nodes__label__value, related_nodes__label__values: $related_nodes__label__values, related_nodes__label__source__id: $related_nodes__label__source__id, related_nodes__label__owner__id: $related_nodes__label__owner__id, related_nodes__label__is_protected: $related_nodes__label__is_protected, b_terminations__ids: $b_terminations__ids, b_terminations__isnull: $b_terminations__isnull, b_terminations__display_label__value: $b_terminations__display_label__value, b_terminations__display_label__values: $b_terminations__display_label__values, b_terminations__display_label__isnull: $b_terminations__display_label__isnull, b_terminations__if_type__value: $b_terminations__if_type__value, b_terminations__if_type__values: $b_terminations__if_type__values, b_terminations__if_type__source__id: $b_terminations__if_type__source__id, b_terminations__if_type__owner__id: $b_terminations__if_type__owner__id, b_terminations__if_type__is_protected: $b_terminations__if_type__is_protected, b_terminations__name__value: $b_terminations__name__value, b_terminations__name__values: $b_terminations__name__values, b_terminations__name__source__id: $b_terminations__name__source__id, b_terminations__name__owner__id: $b_terminations__name__owner__id, b_terminations__name__is_protected: $b_terminations__name__is_protected, b_terminations__enabled__value: $b_terminations__enabled__value, b_terminations__enabled__values: $b_terminations__enabled__values, b_terminations__enabled__source__id: $b_terminations__enabled__source__id, b_terminations__enabled__owner__id: $b_terminations__enabled__owner__id, b_terminations__enabled__is_protected: $b_terminations__enabled__is_protected, a_terminations__ids: $a_terminations__ids, a_terminations__isnull: $a_terminations__isnull, a_terminations__display_label__value: $a_terminations__display_label__value, a_terminations__display_label__values: $a_terminations__display_label__values, a_terminations__display_label__isnull: $a_terminations__display_label__isnull, a_terminations__if_type__value: $a_terminations__if_type__value, a_terminations__if_type__values: $a_terminations__if_type__values, a_terminations__if_type__source__id: $a_terminations__if_type__source__id, a_terminations__if_type__owner__id: $a_terminations__if_type__owner__id, a_terminations__if_type__is_protected: $a_terminations__if_type__is_protected, a_terminations__name__value: $a_terminations__name__value, a_terminations__name__values: $a_terminations__name__values, a_terminations__name__source__id: $a_terminations__name__source__id, a_terminations__name__owner__id: $a_terminations__name__owner__id, a_terminations__name__is_protected: $a_terminations__name__is_protected, a_terminations__enabled__value: $a_terminations__enabled__value, a_terminations__enabled__values: $a_terminations__enabled__values, a_terminations__enabled__source__id: $a_terminations__enabled__source__id, a_terminations__enabled__owner__id: $a_terminations__enabled__owner__id, a_terminations__enabled__is_protected: $a_terminations__enabled__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } b_terminations { count edges { __typename } permissions { __typename } } a_terminations { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileDcimCableResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileDcimCableResponse, i64)> {
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
                    let response = client
                        .execute::<ProfileDcimCableResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, ProfileDcimCable, String, (ProfileDcimCableResponse, i64)> = Box::new(move |(data, current_offset): (ProfileDcimCableResponse, i64)| -> Result<EdgePage<ProfileDcimCable, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_dcim_cable.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileDcimCable>> {
        let mut filters = ProfileDcimCableFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileDcimDeviceFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
    pub status_value: Option<String>,
    pub status_values: Option<Vec<String>>,
    pub status_isnull: Option<bool>,
    pub status_source_id: Option<String>,
    pub status_owner_id: Option<String>,
    pub status_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_status_value: Option<String>,
    pub related_nodes_status_values: Option<Vec<String>>,
    pub related_nodes_status_source_id: Option<String>,
    pub related_nodes_status_owner_id: Option<String>,
    pub related_nodes_status_is_protected: Option<bool>,
    pub related_nodes_name_value: Option<String>,
    pub related_nodes_name_values: Option<Vec<String>>,
    pub related_nodes_name_source_id: Option<String>,
    pub related_nodes_name_owner_id: Option<String>,
    pub related_nodes_name_is_protected: Option<bool>,
    pub role_ids: Option<Vec<String>>,
    pub role_isnull: Option<bool>,
    pub role_display_label_value: Option<String>,
    pub role_display_label_values: Option<Vec<String>>,
    pub role_display_label_isnull: Option<bool>,
    pub role_name_value: Option<String>,
    pub role_name_values: Option<Vec<String>>,
    pub role_name_source_id: Option<String>,
    pub role_name_owner_id: Option<String>,
    pub role_name_is_protected: Option<bool>,
    pub role_slug_value: Option<String>,
    pub role_slug_values: Option<Vec<String>>,
    pub role_slug_source_id: Option<String>,
    pub role_slug_owner_id: Option<String>,
    pub role_slug_is_protected: Option<bool>,
    pub platform_ids: Option<Vec<String>>,
    pub platform_isnull: Option<bool>,
    pub platform_display_label_value: Option<String>,
    pub platform_display_label_values: Option<Vec<String>>,
    pub platform_display_label_isnull: Option<bool>,
    pub platform_name_value: Option<String>,
    pub platform_name_values: Option<Vec<String>>,
    pub platform_name_source_id: Option<String>,
    pub platform_name_owner_id: Option<String>,
    pub platform_name_is_protected: Option<bool>,
    pub platform_slug_value: Option<String>,
    pub platform_slug_values: Option<Vec<String>>,
    pub platform_slug_source_id: Option<String>,
    pub platform_slug_owner_id: Option<String>,
    pub platform_slug_is_protected: Option<bool>,
    pub site_ids: Option<Vec<String>>,
    pub site_isnull: Option<bool>,
    pub site_display_label_value: Option<String>,
    pub site_display_label_values: Option<Vec<String>>,
    pub site_display_label_isnull: Option<bool>,
    pub site_slug_value: Option<String>,
    pub site_slug_values: Option<Vec<String>>,
    pub site_slug_source_id: Option<String>,
    pub site_slug_owner_id: Option<String>,
    pub site_slug_is_protected: Option<bool>,
    pub site_name_value: Option<String>,
    pub site_name_values: Option<Vec<String>>,
    pub site_name_source_id: Option<String>,
    pub site_name_owner_id: Option<String>,
    pub site_name_is_protected: Option<bool>,
    pub site_status_value: Option<String>,
    pub site_status_values: Option<Vec<String>>,
    pub site_status_source_id: Option<String>,
    pub site_status_owner_id: Option<String>,
    pub site_status_is_protected: Option<bool>,
    pub primary_ip4_ids: Option<Vec<String>>,
    pub primary_ip4_isnull: Option<bool>,
    pub primary_ip4_display_label_value: Option<String>,
    pub primary_ip4_display_label_values: Option<Vec<String>>,
    pub primary_ip4_display_label_isnull: Option<bool>,
    pub primary_ip4_address_value: Option<String>,
    pub primary_ip4_address_values: Option<Vec<String>>,
    pub primary_ip4_address_source_id: Option<String>,
    pub primary_ip4_address_owner_id: Option<String>,
    pub primary_ip4_address_is_protected: Option<bool>,
    pub primary_ip4_status_value: Option<String>,
    pub primary_ip4_status_values: Option<Vec<String>>,
    pub primary_ip4_status_source_id: Option<String>,
    pub primary_ip4_status_owner_id: Option<String>,
    pub primary_ip4_status_is_protected: Option<bool>,
    pub device_type_ids: Option<Vec<String>>,
    pub device_type_isnull: Option<bool>,
    pub device_type_display_label_value: Option<String>,
    pub device_type_display_label_values: Option<Vec<String>>,
    pub device_type_display_label_isnull: Option<bool>,
    pub device_type_model_value: Option<String>,
    pub device_type_model_values: Option<Vec<String>>,
    pub device_type_model_source_id: Option<String>,
    pub device_type_model_owner_id: Option<String>,
    pub device_type_model_is_protected: Option<bool>,
    pub device_type_slug_value: Option<String>,
    pub device_type_slug_values: Option<Vec<String>>,
    pub device_type_slug_source_id: Option<String>,
    pub device_type_slug_owner_id: Option<String>,
    pub device_type_slug_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileDcimDeviceFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.status_value {
            vars.insert("status__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.status_values {
            vars.insert("status__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.status_isnull {
            vars.insert("status__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.status_source_id {
            vars.insert(
                "status__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.status_owner_id {
            vars.insert(
                "status__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.status_is_protected {
            vars.insert(
                "status__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_value {
            vars.insert(
                "related_nodes__status__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_values {
            vars.insert(
                "related_nodes__status__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_source_id {
            vars.insert(
                "related_nodes__status__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_owner_id {
            vars.insert(
                "related_nodes__status__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_is_protected {
            vars.insert(
                "related_nodes__status__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_value {
            vars.insert(
                "related_nodes__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_values {
            vars.insert(
                "related_nodes__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_source_id {
            vars.insert(
                "related_nodes__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_owner_id {
            vars.insert(
                "related_nodes__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_is_protected {
            vars.insert(
                "related_nodes__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_ids {
            vars.insert("role__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.role_isnull {
            vars.insert("role__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.role_display_label_value {
            vars.insert(
                "role__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_display_label_values {
            vars.insert(
                "role__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_display_label_isnull {
            vars.insert(
                "role__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_name_value {
            vars.insert(
                "role__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_name_values {
            vars.insert(
                "role__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_name_source_id {
            vars.insert(
                "role__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_name_owner_id {
            vars.insert(
                "role__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_name_is_protected {
            vars.insert(
                "role__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_slug_value {
            vars.insert(
                "role__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_slug_values {
            vars.insert(
                "role__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_slug_source_id {
            vars.insert(
                "role__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_slug_owner_id {
            vars.insert(
                "role__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.role_slug_is_protected {
            vars.insert(
                "role__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_ids {
            vars.insert("platform__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.platform_isnull {
            vars.insert("platform__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.platform_display_label_value {
            vars.insert(
                "platform__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_display_label_values {
            vars.insert(
                "platform__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_display_label_isnull {
            vars.insert(
                "platform__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_name_value {
            vars.insert(
                "platform__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_name_values {
            vars.insert(
                "platform__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_name_source_id {
            vars.insert(
                "platform__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_name_owner_id {
            vars.insert(
                "platform__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_name_is_protected {
            vars.insert(
                "platform__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_slug_value {
            vars.insert(
                "platform__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_slug_values {
            vars.insert(
                "platform__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_slug_source_id {
            vars.insert(
                "platform__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_slug_owner_id {
            vars.insert(
                "platform__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.platform_slug_is_protected {
            vars.insert(
                "platform__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_ids {
            vars.insert("site__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.site_isnull {
            vars.insert("site__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.site_display_label_value {
            vars.insert(
                "site__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_display_label_values {
            vars.insert(
                "site__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_display_label_isnull {
            vars.insert(
                "site__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_slug_value {
            vars.insert(
                "site__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_slug_values {
            vars.insert(
                "site__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_slug_source_id {
            vars.insert(
                "site__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_slug_owner_id {
            vars.insert(
                "site__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_slug_is_protected {
            vars.insert(
                "site__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_name_value {
            vars.insert(
                "site__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_name_values {
            vars.insert(
                "site__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_name_source_id {
            vars.insert(
                "site__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_name_owner_id {
            vars.insert(
                "site__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_name_is_protected {
            vars.insert(
                "site__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_status_value {
            vars.insert(
                "site__status__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_status_values {
            vars.insert(
                "site__status__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_status_source_id {
            vars.insert(
                "site__status__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_status_owner_id {
            vars.insert(
                "site__status__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.site_status_is_protected {
            vars.insert(
                "site__status__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_ids {
            vars.insert("primary_ip4__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.primary_ip4_isnull {
            vars.insert(
                "primary_ip4__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_display_label_value {
            vars.insert(
                "primary_ip4__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_display_label_values {
            vars.insert(
                "primary_ip4__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_display_label_isnull {
            vars.insert(
                "primary_ip4__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_address_value {
            vars.insert(
                "primary_ip4__address__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_address_values {
            vars.insert(
                "primary_ip4__address__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_address_source_id {
            vars.insert(
                "primary_ip4__address__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_address_owner_id {
            vars.insert(
                "primary_ip4__address__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_address_is_protected {
            vars.insert(
                "primary_ip4__address__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_status_value {
            vars.insert(
                "primary_ip4__status__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_status_values {
            vars.insert(
                "primary_ip4__status__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_status_source_id {
            vars.insert(
                "primary_ip4__status__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_status_owner_id {
            vars.insert(
                "primary_ip4__status__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.primary_ip4_status_is_protected {
            vars.insert(
                "primary_ip4__status__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_ids {
            vars.insert("device_type__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.device_type_isnull {
            vars.insert(
                "device_type__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_display_label_value {
            vars.insert(
                "device_type__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_display_label_values {
            vars.insert(
                "device_type__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_display_label_isnull {
            vars.insert(
                "device_type__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_model_value {
            vars.insert(
                "device_type__model__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_model_values {
            vars.insert(
                "device_type__model__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_model_source_id {
            vars.insert(
                "device_type__model__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_model_owner_id {
            vars.insert(
                "device_type__model__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_model_is_protected {
            vars.insert(
                "device_type__model__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_slug_value {
            vars.insert(
                "device_type__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_slug_values {
            vars.insert(
                "device_type__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_slug_source_id {
            vars.insert(
                "device_type__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_slug_owner_id {
            vars.insert(
                "device_type__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_type_slug_is_protected {
            vars.insert(
                "device_type__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileDcimDeviceClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileDcimDeviceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileDcimDeviceFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileDcimDevice>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileDcimDevice($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__status__value: String, $related_nodes__status__values: [String], $related_nodes__status__source__id: ID, $related_nodes__status__owner__id: ID, $related_nodes__status__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $role__ids: [ID], $role__isnull: Boolean, $role__display_label__value: String, $role__display_label__values: [String], $role__display_label__isnull: Boolean, $role__name__value: String, $role__name__values: [String], $role__name__source__id: ID, $role__name__owner__id: ID, $role__name__is_protected: Boolean, $role__slug__value: String, $role__slug__values: [String], $role__slug__source__id: ID, $role__slug__owner__id: ID, $role__slug__is_protected: Boolean, $platform__ids: [ID], $platform__isnull: Boolean, $platform__display_label__value: String, $platform__display_label__values: [String], $platform__display_label__isnull: Boolean, $platform__name__value: String, $platform__name__values: [String], $platform__name__source__id: ID, $platform__name__owner__id: ID, $platform__name__is_protected: Boolean, $platform__slug__value: String, $platform__slug__values: [String], $platform__slug__source__id: ID, $platform__slug__owner__id: ID, $platform__slug__is_protected: Boolean, $site__ids: [ID], $site__isnull: Boolean, $site__display_label__value: String, $site__display_label__values: [String], $site__display_label__isnull: Boolean, $site__slug__value: String, $site__slug__values: [String], $site__slug__source__id: ID, $site__slug__owner__id: ID, $site__slug__is_protected: Boolean, $site__name__value: String, $site__name__values: [String], $site__name__source__id: ID, $site__name__owner__id: ID, $site__name__is_protected: Boolean, $site__status__value: String, $site__status__values: [String], $site__status__source__id: ID, $site__status__owner__id: ID, $site__status__is_protected: Boolean, $primary_ip4__ids: [ID], $primary_ip4__isnull: Boolean, $primary_ip4__display_label__value: String, $primary_ip4__display_label__values: [String], $primary_ip4__display_label__isnull: Boolean, $primary_ip4__address__value: String, $primary_ip4__address__values: [String], $primary_ip4__address__source__id: ID, $primary_ip4__address__owner__id: ID, $primary_ip4__address__is_protected: Boolean, $primary_ip4__status__value: String, $primary_ip4__status__values: [String], $primary_ip4__status__source__id: ID, $primary_ip4__status__owner__id: ID, $primary_ip4__status__is_protected: Boolean, $device_type__ids: [ID], $device_type__isnull: Boolean, $device_type__display_label__value: String, $device_type__display_label__values: [String], $device_type__display_label__isnull: Boolean, $device_type__model__value: String, $device_type__model__values: [String], $device_type__model__source__id: ID, $device_type__model__owner__id: ID, $device_type__model__is_protected: Boolean, $device_type__slug__value: String, $device_type__slug__values: [String], $device_type__slug__source__id: ID, $device_type__slug__owner__id: ID, $device_type__slug__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimDevice(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__status__value: $related_nodes__status__value, related_nodes__status__values: $related_nodes__status__values, related_nodes__status__source__id: $related_nodes__status__source__id, related_nodes__status__owner__id: $related_nodes__status__owner__id, related_nodes__status__is_protected: $related_nodes__status__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, role__ids: $role__ids, role__isnull: $role__isnull, role__display_label__value: $role__display_label__value, role__display_label__values: $role__display_label__values, role__display_label__isnull: $role__display_label__isnull, role__name__value: $role__name__value, role__name__values: $role__name__values, role__name__source__id: $role__name__source__id, role__name__owner__id: $role__name__owner__id, role__name__is_protected: $role__name__is_protected, role__slug__value: $role__slug__value, role__slug__values: $role__slug__values, role__slug__source__id: $role__slug__source__id, role__slug__owner__id: $role__slug__owner__id, role__slug__is_protected: $role__slug__is_protected, platform__ids: $platform__ids, platform__isnull: $platform__isnull, platform__display_label__value: $platform__display_label__value, platform__display_label__values: $platform__display_label__values, platform__display_label__isnull: $platform__display_label__isnull, platform__name__value: $platform__name__value, platform__name__values: $platform__name__values, platform__name__source__id: $platform__name__source__id, platform__name__owner__id: $platform__name__owner__id, platform__name__is_protected: $platform__name__is_protected, platform__slug__value: $platform__slug__value, platform__slug__values: $platform__slug__values, platform__slug__source__id: $platform__slug__source__id, platform__slug__owner__id: $platform__slug__owner__id, platform__slug__is_protected: $platform__slug__is_protected, site__ids: $site__ids, site__isnull: $site__isnull, site__display_label__value: $site__display_label__value, site__display_label__values: $site__display_label__values, site__display_label__isnull: $site__display_label__isnull, site__slug__value: $site__slug__value, site__slug__values: $site__slug__values, site__slug__source__id: $site__slug__source__id, site__slug__owner__id: $site__slug__owner__id, site__slug__is_protected: $site__slug__is_protected, site__name__value: $site__name__value, site__name__values: $site__name__values, site__name__source__id: $site__name__source__id, site__name__owner__id: $site__name__owner__id, site__name__is_protected: $site__name__is_protected, site__status__value: $site__status__value, site__status__values: $site__status__values, site__status__source__id: $site__status__source__id, site__status__owner__id: $site__status__owner__id, site__status__is_protected: $site__status__is_protected, primary_ip4__ids: $primary_ip4__ids, primary_ip4__isnull: $primary_ip4__isnull, primary_ip4__display_label__value: $primary_ip4__display_label__value, primary_ip4__display_label__values: $primary_ip4__display_label__values, primary_ip4__display_label__isnull: $primary_ip4__display_label__isnull, primary_ip4__address__value: $primary_ip4__address__value, primary_ip4__address__values: $primary_ip4__address__values, primary_ip4__address__source__id: $primary_ip4__address__source__id, primary_ip4__address__owner__id: $primary_ip4__address__owner__id, primary_ip4__address__is_protected: $primary_ip4__address__is_protected, primary_ip4__status__value: $primary_ip4__status__value, primary_ip4__status__values: $primary_ip4__status__values, primary_ip4__status__source__id: $primary_ip4__status__source__id, primary_ip4__status__owner__id: $primary_ip4__status__owner__id, primary_ip4__status__is_protected: $primary_ip4__status__is_protected, device_type__ids: $device_type__ids, device_type__isnull: $device_type__isnull, device_type__display_label__value: $device_type__display_label__value, device_type__display_label__values: $device_type__display_label__values, device_type__display_label__isnull: $device_type__display_label__isnull, device_type__model__value: $device_type__model__value, device_type__model__values: $device_type__model__values, device_type__model__source__id: $device_type__model__source__id, device_type__model__owner__id: $device_type__model__owner__id, device_type__model__is_protected: $device_type__model__is_protected, device_type__slug__value: $device_type__slug__value, device_type__slug__values: $device_type__slug__values, device_type__slug__source__id: $device_type__slug__source__id, device_type__slug__owner__id: $device_type__slug__owner__id, device_type__slug__is_protected: $device_type__slug__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } role { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } platform { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } site { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } primary_ip4 { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } device_type { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileDcimDeviceResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_dcim_device.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileDcimDeviceFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileDcimDevice, String, (ProfileDcimDeviceResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileDcimDevice($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__status__value: String, $related_nodes__status__values: [String], $related_nodes__status__source__id: ID, $related_nodes__status__owner__id: ID, $related_nodes__status__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $role__ids: [ID], $role__isnull: Boolean, $role__display_label__value: String, $role__display_label__values: [String], $role__display_label__isnull: Boolean, $role__name__value: String, $role__name__values: [String], $role__name__source__id: ID, $role__name__owner__id: ID, $role__name__is_protected: Boolean, $role__slug__value: String, $role__slug__values: [String], $role__slug__source__id: ID, $role__slug__owner__id: ID, $role__slug__is_protected: Boolean, $platform__ids: [ID], $platform__isnull: Boolean, $platform__display_label__value: String, $platform__display_label__values: [String], $platform__display_label__isnull: Boolean, $platform__name__value: String, $platform__name__values: [String], $platform__name__source__id: ID, $platform__name__owner__id: ID, $platform__name__is_protected: Boolean, $platform__slug__value: String, $platform__slug__values: [String], $platform__slug__source__id: ID, $platform__slug__owner__id: ID, $platform__slug__is_protected: Boolean, $site__ids: [ID], $site__isnull: Boolean, $site__display_label__value: String, $site__display_label__values: [String], $site__display_label__isnull: Boolean, $site__slug__value: String, $site__slug__values: [String], $site__slug__source__id: ID, $site__slug__owner__id: ID, $site__slug__is_protected: Boolean, $site__name__value: String, $site__name__values: [String], $site__name__source__id: ID, $site__name__owner__id: ID, $site__name__is_protected: Boolean, $site__status__value: String, $site__status__values: [String], $site__status__source__id: ID, $site__status__owner__id: ID, $site__status__is_protected: Boolean, $primary_ip4__ids: [ID], $primary_ip4__isnull: Boolean, $primary_ip4__display_label__value: String, $primary_ip4__display_label__values: [String], $primary_ip4__display_label__isnull: Boolean, $primary_ip4__address__value: String, $primary_ip4__address__values: [String], $primary_ip4__address__source__id: ID, $primary_ip4__address__owner__id: ID, $primary_ip4__address__is_protected: Boolean, $primary_ip4__status__value: String, $primary_ip4__status__values: [String], $primary_ip4__status__source__id: ID, $primary_ip4__status__owner__id: ID, $primary_ip4__status__is_protected: Boolean, $device_type__ids: [ID], $device_type__isnull: Boolean, $device_type__display_label__value: String, $device_type__display_label__values: [String], $device_type__display_label__isnull: Boolean, $device_type__model__value: String, $device_type__model__values: [String], $device_type__model__source__id: ID, $device_type__model__owner__id: ID, $device_type__model__is_protected: Boolean, $device_type__slug__value: String, $device_type__slug__values: [String], $device_type__slug__source__id: ID, $device_type__slug__owner__id: ID, $device_type__slug__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimDevice(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__status__value: $related_nodes__status__value, related_nodes__status__values: $related_nodes__status__values, related_nodes__status__source__id: $related_nodes__status__source__id, related_nodes__status__owner__id: $related_nodes__status__owner__id, related_nodes__status__is_protected: $related_nodes__status__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, role__ids: $role__ids, role__isnull: $role__isnull, role__display_label__value: $role__display_label__value, role__display_label__values: $role__display_label__values, role__display_label__isnull: $role__display_label__isnull, role__name__value: $role__name__value, role__name__values: $role__name__values, role__name__source__id: $role__name__source__id, role__name__owner__id: $role__name__owner__id, role__name__is_protected: $role__name__is_protected, role__slug__value: $role__slug__value, role__slug__values: $role__slug__values, role__slug__source__id: $role__slug__source__id, role__slug__owner__id: $role__slug__owner__id, role__slug__is_protected: $role__slug__is_protected, platform__ids: $platform__ids, platform__isnull: $platform__isnull, platform__display_label__value: $platform__display_label__value, platform__display_label__values: $platform__display_label__values, platform__display_label__isnull: $platform__display_label__isnull, platform__name__value: $platform__name__value, platform__name__values: $platform__name__values, platform__name__source__id: $platform__name__source__id, platform__name__owner__id: $platform__name__owner__id, platform__name__is_protected: $platform__name__is_protected, platform__slug__value: $platform__slug__value, platform__slug__values: $platform__slug__values, platform__slug__source__id: $platform__slug__source__id, platform__slug__owner__id: $platform__slug__owner__id, platform__slug__is_protected: $platform__slug__is_protected, site__ids: $site__ids, site__isnull: $site__isnull, site__display_label__value: $site__display_label__value, site__display_label__values: $site__display_label__values, site__display_label__isnull: $site__display_label__isnull, site__slug__value: $site__slug__value, site__slug__values: $site__slug__values, site__slug__source__id: $site__slug__source__id, site__slug__owner__id: $site__slug__owner__id, site__slug__is_protected: $site__slug__is_protected, site__name__value: $site__name__value, site__name__values: $site__name__values, site__name__source__id: $site__name__source__id, site__name__owner__id: $site__name__owner__id, site__name__is_protected: $site__name__is_protected, site__status__value: $site__status__value, site__status__values: $site__status__values, site__status__source__id: $site__status__source__id, site__status__owner__id: $site__status__owner__id, site__status__is_protected: $site__status__is_protected, primary_ip4__ids: $primary_ip4__ids, primary_ip4__isnull: $primary_ip4__isnull, primary_ip4__display_label__value: $primary_ip4__display_label__value, primary_ip4__display_label__values: $primary_ip4__display_label__values, primary_ip4__display_label__isnull: $primary_ip4__display_label__isnull, primary_ip4__address__value: $primary_ip4__address__value, primary_ip4__address__values: $primary_ip4__address__values, primary_ip4__address__source__id: $primary_ip4__address__source__id, primary_ip4__address__owner__id: $primary_ip4__address__owner__id, primary_ip4__address__is_protected: $primary_ip4__address__is_protected, primary_ip4__status__value: $primary_ip4__status__value, primary_ip4__status__values: $primary_ip4__status__values, primary_ip4__status__source__id: $primary_ip4__status__source__id, primary_ip4__status__owner__id: $primary_ip4__status__owner__id, primary_ip4__status__is_protected: $primary_ip4__status__is_protected, device_type__ids: $device_type__ids, device_type__isnull: $device_type__isnull, device_type__display_label__value: $device_type__display_label__value, device_type__display_label__values: $device_type__display_label__values, device_type__display_label__isnull: $device_type__display_label__isnull, device_type__model__value: $device_type__model__value, device_type__model__values: $device_type__model__values, device_type__model__source__id: $device_type__model__source__id, device_type__model__owner__id: $device_type__model__owner__id, device_type__model__is_protected: $device_type__model__is_protected, device_type__slug__value: $device_type__slug__value, device_type__slug__values: $device_type__slug__values, device_type__slug__source__id: $device_type__slug__source__id, device_type__slug__owner__id: $device_type__slug__owner__id, device_type__slug__is_protected: $device_type__slug__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } role { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } platform { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } site { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } primary_ip4 { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } device_type { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileDcimDeviceResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileDcimDeviceResponse, i64)> {
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
                    let response = client
                        .execute::<ProfileDcimDeviceResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, ProfileDcimDevice, String, (ProfileDcimDeviceResponse, i64)> = Box::new(move |(data, current_offset): (ProfileDcimDeviceResponse, i64)| -> Result<EdgePage<ProfileDcimDevice, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_dcim_device.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileDcimDevice>> {
        let mut filters = ProfileDcimDeviceFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileDcimDeviceRoleFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_name_value: Option<String>,
    pub related_nodes_name_values: Option<Vec<String>>,
    pub related_nodes_name_source_id: Option<String>,
    pub related_nodes_name_owner_id: Option<String>,
    pub related_nodes_name_is_protected: Option<bool>,
    pub related_nodes_slug_value: Option<String>,
    pub related_nodes_slug_values: Option<Vec<String>>,
    pub related_nodes_slug_source_id: Option<String>,
    pub related_nodes_slug_owner_id: Option<String>,
    pub related_nodes_slug_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileDcimDeviceRoleFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_value {
            vars.insert(
                "related_nodes__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_values {
            vars.insert(
                "related_nodes__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_source_id {
            vars.insert(
                "related_nodes__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_owner_id {
            vars.insert(
                "related_nodes__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_is_protected {
            vars.insert(
                "related_nodes__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_value {
            vars.insert(
                "related_nodes__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_values {
            vars.insert(
                "related_nodes__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_source_id {
            vars.insert(
                "related_nodes__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_owner_id {
            vars.insert(
                "related_nodes__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_is_protected {
            vars.insert(
                "related_nodes__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileDcimDeviceRoleClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileDcimDeviceRoleClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileDcimDeviceRoleFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileDcimDeviceRole>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileDcimDeviceRole($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimDeviceRole(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileDcimDeviceRoleResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_dcim_device_role.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileDcimDeviceRoleFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileDcimDeviceRole, String, (ProfileDcimDeviceRoleResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileDcimDeviceRole($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimDeviceRole(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileDcimDeviceRoleResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileDcimDeviceRoleResponse, i64)> {
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
                let response = client.execute::<ProfileDcimDeviceRoleResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, ProfileDcimDeviceRole, String, (ProfileDcimDeviceRoleResponse, i64)> = Box::new(move |(data, current_offset): (ProfileDcimDeviceRoleResponse, i64)| -> Result<EdgePage<ProfileDcimDeviceRole, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_dcim_device_role.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileDcimDeviceRole>> {
        let mut filters = ProfileDcimDeviceRoleFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileDcimDeviceTypeFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
    pub model_value: Option<String>,
    pub model_values: Option<Vec<String>>,
    pub model_isnull: Option<bool>,
    pub model_source_id: Option<String>,
    pub model_owner_id: Option<String>,
    pub model_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_model_value: Option<String>,
    pub related_nodes_model_values: Option<Vec<String>>,
    pub related_nodes_model_source_id: Option<String>,
    pub related_nodes_model_owner_id: Option<String>,
    pub related_nodes_model_is_protected: Option<bool>,
    pub related_nodes_slug_value: Option<String>,
    pub related_nodes_slug_values: Option<Vec<String>>,
    pub related_nodes_slug_source_id: Option<String>,
    pub related_nodes_slug_owner_id: Option<String>,
    pub related_nodes_slug_is_protected: Option<bool>,
    pub manufacturer_ids: Option<Vec<String>>,
    pub manufacturer_isnull: Option<bool>,
    pub manufacturer_display_label_value: Option<String>,
    pub manufacturer_display_label_values: Option<Vec<String>>,
    pub manufacturer_display_label_isnull: Option<bool>,
    pub manufacturer_slug_value: Option<String>,
    pub manufacturer_slug_values: Option<Vec<String>>,
    pub manufacturer_slug_source_id: Option<String>,
    pub manufacturer_slug_owner_id: Option<String>,
    pub manufacturer_slug_is_protected: Option<bool>,
    pub manufacturer_name_value: Option<String>,
    pub manufacturer_name_values: Option<Vec<String>>,
    pub manufacturer_name_source_id: Option<String>,
    pub manufacturer_name_owner_id: Option<String>,
    pub manufacturer_name_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileDcimDeviceTypeFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.model_value {
            vars.insert("model__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.model_values {
            vars.insert("model__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.model_isnull {
            vars.insert("model__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.model_source_id {
            vars.insert(
                "model__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.model_owner_id {
            vars.insert("model__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.model_is_protected {
            vars.insert(
                "model__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_model_value {
            vars.insert(
                "related_nodes__model__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_model_values {
            vars.insert(
                "related_nodes__model__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_model_source_id {
            vars.insert(
                "related_nodes__model__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_model_owner_id {
            vars.insert(
                "related_nodes__model__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_model_is_protected {
            vars.insert(
                "related_nodes__model__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_value {
            vars.insert(
                "related_nodes__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_values {
            vars.insert(
                "related_nodes__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_source_id {
            vars.insert(
                "related_nodes__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_owner_id {
            vars.insert(
                "related_nodes__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_is_protected {
            vars.insert(
                "related_nodes__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_ids {
            vars.insert(
                "manufacturer__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_isnull {
            vars.insert(
                "manufacturer__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_display_label_value {
            vars.insert(
                "manufacturer__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_display_label_values {
            vars.insert(
                "manufacturer__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_display_label_isnull {
            vars.insert(
                "manufacturer__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_slug_value {
            vars.insert(
                "manufacturer__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_slug_values {
            vars.insert(
                "manufacturer__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_slug_source_id {
            vars.insert(
                "manufacturer__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_slug_owner_id {
            vars.insert(
                "manufacturer__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_slug_is_protected {
            vars.insert(
                "manufacturer__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_name_value {
            vars.insert(
                "manufacturer__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_name_values {
            vars.insert(
                "manufacturer__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_name_source_id {
            vars.insert(
                "manufacturer__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_name_owner_id {
            vars.insert(
                "manufacturer__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.manufacturer_name_is_protected {
            vars.insert(
                "manufacturer__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileDcimDeviceTypeClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileDcimDeviceTypeClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileDcimDeviceTypeFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileDcimDeviceType>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileDcimDeviceType($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $model__value: String, $model__values: [String], $model__isnull: Boolean, $model__source__id: ID, $model__owner__id: ID, $model__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__model__value: String, $related_nodes__model__values: [String], $related_nodes__model__source__id: ID, $related_nodes__model__owner__id: ID, $related_nodes__model__is_protected: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $manufacturer__ids: [ID], $manufacturer__isnull: Boolean, $manufacturer__display_label__value: String, $manufacturer__display_label__values: [String], $manufacturer__display_label__isnull: Boolean, $manufacturer__slug__value: String, $manufacturer__slug__values: [String], $manufacturer__slug__source__id: ID, $manufacturer__slug__owner__id: ID, $manufacturer__slug__is_protected: Boolean, $manufacturer__name__value: String, $manufacturer__name__values: [String], $manufacturer__name__source__id: ID, $manufacturer__name__owner__id: ID, $manufacturer__name__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimDeviceType(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, model__value: $model__value, model__values: $model__values, model__isnull: $model__isnull, model__source__id: $model__source__id, model__owner__id: $model__owner__id, model__is_protected: $model__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__model__value: $related_nodes__model__value, related_nodes__model__values: $related_nodes__model__values, related_nodes__model__source__id: $related_nodes__model__source__id, related_nodes__model__owner__id: $related_nodes__model__owner__id, related_nodes__model__is_protected: $related_nodes__model__is_protected, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, manufacturer__ids: $manufacturer__ids, manufacturer__isnull: $manufacturer__isnull, manufacturer__display_label__value: $manufacturer__display_label__value, manufacturer__display_label__values: $manufacturer__display_label__values, manufacturer__display_label__isnull: $manufacturer__display_label__isnull, manufacturer__slug__value: $manufacturer__slug__value, manufacturer__slug__values: $manufacturer__slug__values, manufacturer__slug__source__id: $manufacturer__slug__source__id, manufacturer__slug__owner__id: $manufacturer__slug__owner__id, manufacturer__slug__is_protected: $manufacturer__slug__is_protected, manufacturer__name__value: $manufacturer__name__value, manufacturer__name__values: $manufacturer__name__values, manufacturer__name__source__id: $manufacturer__name__source__id, manufacturer__name__owner__id: $manufacturer__name__owner__id, manufacturer__name__is_protected: $manufacturer__name__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } model { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } manufacturer { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileDcimDeviceTypeResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_dcim_device_type.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileDcimDeviceTypeFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileDcimDeviceType, String, (ProfileDcimDeviceTypeResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileDcimDeviceType($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $model__value: String, $model__values: [String], $model__isnull: Boolean, $model__source__id: ID, $model__owner__id: ID, $model__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__model__value: String, $related_nodes__model__values: [String], $related_nodes__model__source__id: ID, $related_nodes__model__owner__id: ID, $related_nodes__model__is_protected: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $manufacturer__ids: [ID], $manufacturer__isnull: Boolean, $manufacturer__display_label__value: String, $manufacturer__display_label__values: [String], $manufacturer__display_label__isnull: Boolean, $manufacturer__slug__value: String, $manufacturer__slug__values: [String], $manufacturer__slug__source__id: ID, $manufacturer__slug__owner__id: ID, $manufacturer__slug__is_protected: Boolean, $manufacturer__name__value: String, $manufacturer__name__values: [String], $manufacturer__name__source__id: ID, $manufacturer__name__owner__id: ID, $manufacturer__name__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimDeviceType(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, model__value: $model__value, model__values: $model__values, model__isnull: $model__isnull, model__source__id: $model__source__id, model__owner__id: $model__owner__id, model__is_protected: $model__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__model__value: $related_nodes__model__value, related_nodes__model__values: $related_nodes__model__values, related_nodes__model__source__id: $related_nodes__model__source__id, related_nodes__model__owner__id: $related_nodes__model__owner__id, related_nodes__model__is_protected: $related_nodes__model__is_protected, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, manufacturer__ids: $manufacturer__ids, manufacturer__isnull: $manufacturer__isnull, manufacturer__display_label__value: $manufacturer__display_label__value, manufacturer__display_label__values: $manufacturer__display_label__values, manufacturer__display_label__isnull: $manufacturer__display_label__isnull, manufacturer__slug__value: $manufacturer__slug__value, manufacturer__slug__values: $manufacturer__slug__values, manufacturer__slug__source__id: $manufacturer__slug__source__id, manufacturer__slug__owner__id: $manufacturer__slug__owner__id, manufacturer__slug__is_protected: $manufacturer__slug__is_protected, manufacturer__name__value: $manufacturer__name__value, manufacturer__name__values: $manufacturer__name__values, manufacturer__name__source__id: $manufacturer__name__source__id, manufacturer__name__owner__id: $manufacturer__name__owner__id, manufacturer__name__is_protected: $manufacturer__name__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } model { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } manufacturer { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileDcimDeviceTypeResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileDcimDeviceTypeResponse, i64)> {
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
                let response = client.execute::<ProfileDcimDeviceTypeResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, ProfileDcimDeviceType, String, (ProfileDcimDeviceTypeResponse, i64)> = Box::new(move |(data, current_offset): (ProfileDcimDeviceTypeResponse, i64)| -> Result<EdgePage<ProfileDcimDeviceType, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_dcim_device_type.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileDcimDeviceType>> {
        let mut filters = ProfileDcimDeviceTypeFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileDcimInterfaceFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
    pub if_type_value: Option<String>,
    pub if_type_values: Option<Vec<String>>,
    pub if_type_isnull: Option<bool>,
    pub if_type_source_id: Option<String>,
    pub if_type_owner_id: Option<String>,
    pub if_type_is_protected: Option<bool>,
    pub enabled_value: Option<bool>,
    pub enabled_values: Option<Vec<bool>>,
    pub enabled_isnull: Option<bool>,
    pub enabled_source_id: Option<String>,
    pub enabled_owner_id: Option<String>,
    pub enabled_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_if_type_value: Option<String>,
    pub related_nodes_if_type_values: Option<Vec<String>>,
    pub related_nodes_if_type_source_id: Option<String>,
    pub related_nodes_if_type_owner_id: Option<String>,
    pub related_nodes_if_type_is_protected: Option<bool>,
    pub related_nodes_name_value: Option<String>,
    pub related_nodes_name_values: Option<Vec<String>>,
    pub related_nodes_name_source_id: Option<String>,
    pub related_nodes_name_owner_id: Option<String>,
    pub related_nodes_name_is_protected: Option<bool>,
    pub related_nodes_enabled_value: Option<bool>,
    pub related_nodes_enabled_values: Option<Vec<bool>>,
    pub related_nodes_enabled_source_id: Option<String>,
    pub related_nodes_enabled_owner_id: Option<String>,
    pub related_nodes_enabled_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileDcimInterfaceFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.if_type_value {
            vars.insert("if_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.if_type_values {
            vars.insert("if_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.if_type_isnull {
            vars.insert("if_type__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.if_type_source_id {
            vars.insert(
                "if_type__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.if_type_owner_id {
            vars.insert(
                "if_type__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.if_type_is_protected {
            vars.insert(
                "if_type__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.enabled_value {
            vars.insert("enabled__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.enabled_values {
            vars.insert("enabled__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.enabled_isnull {
            vars.insert("enabled__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.enabled_source_id {
            vars.insert(
                "enabled__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.enabled_owner_id {
            vars.insert(
                "enabled__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.enabled_is_protected {
            vars.insert(
                "enabled__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_if_type_value {
            vars.insert(
                "related_nodes__if_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_if_type_values {
            vars.insert(
                "related_nodes__if_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_if_type_source_id {
            vars.insert(
                "related_nodes__if_type__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_if_type_owner_id {
            vars.insert(
                "related_nodes__if_type__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_if_type_is_protected {
            vars.insert(
                "related_nodes__if_type__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_value {
            vars.insert(
                "related_nodes__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_values {
            vars.insert(
                "related_nodes__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_source_id {
            vars.insert(
                "related_nodes__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_owner_id {
            vars.insert(
                "related_nodes__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_is_protected {
            vars.insert(
                "related_nodes__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_enabled_value {
            vars.insert(
                "related_nodes__enabled__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_enabled_values {
            vars.insert(
                "related_nodes__enabled__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_enabled_source_id {
            vars.insert(
                "related_nodes__enabled__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_enabled_owner_id {
            vars.insert(
                "related_nodes__enabled__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_enabled_is_protected {
            vars.insert(
                "related_nodes__enabled__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileDcimInterfaceClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileDcimInterfaceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileDcimInterfaceFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileDcimInterface>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileDcimInterface($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $if_type__value: String, $if_type__values: [String], $if_type__isnull: Boolean, $if_type__source__id: ID, $if_type__owner__id: ID, $if_type__is_protected: Boolean, $enabled__value: Boolean, $enabled__values: [Boolean], $enabled__isnull: Boolean, $enabled__source__id: ID, $enabled__owner__id: ID, $enabled__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__if_type__value: String, $related_nodes__if_type__values: [String], $related_nodes__if_type__source__id: ID, $related_nodes__if_type__owner__id: ID, $related_nodes__if_type__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__enabled__value: Boolean, $related_nodes__enabled__values: [Boolean], $related_nodes__enabled__source__id: ID, $related_nodes__enabled__owner__id: ID, $related_nodes__enabled__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimInterface(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, if_type__value: $if_type__value, if_type__values: $if_type__values, if_type__isnull: $if_type__isnull, if_type__source__id: $if_type__source__id, if_type__owner__id: $if_type__owner__id, if_type__is_protected: $if_type__is_protected, enabled__value: $enabled__value, enabled__values: $enabled__values, enabled__isnull: $enabled__isnull, enabled__source__id: $enabled__source__id, enabled__owner__id: $enabled__owner__id, enabled__is_protected: $enabled__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__if_type__value: $related_nodes__if_type__value, related_nodes__if_type__values: $related_nodes__if_type__values, related_nodes__if_type__source__id: $related_nodes__if_type__source__id, related_nodes__if_type__owner__id: $related_nodes__if_type__owner__id, related_nodes__if_type__is_protected: $related_nodes__if_type__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__enabled__value: $related_nodes__enabled__value, related_nodes__enabled__values: $related_nodes__enabled__values, related_nodes__enabled__source__id: $related_nodes__enabled__source__id, related_nodes__enabled__owner__id: $related_nodes__enabled__owner__id, related_nodes__enabled__is_protected: $related_nodes__enabled__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } if_type { is_default is_protected updated_at id is_from_profile permissions { __typename } value } enabled { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileDcimInterfaceResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_dcim_interface.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileDcimInterfaceFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileDcimInterface, String, (ProfileDcimInterfaceResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileDcimInterface($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $if_type__value: String, $if_type__values: [String], $if_type__isnull: Boolean, $if_type__source__id: ID, $if_type__owner__id: ID, $if_type__is_protected: Boolean, $enabled__value: Boolean, $enabled__values: [Boolean], $enabled__isnull: Boolean, $enabled__source__id: ID, $enabled__owner__id: ID, $enabled__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__if_type__value: String, $related_nodes__if_type__values: [String], $related_nodes__if_type__source__id: ID, $related_nodes__if_type__owner__id: ID, $related_nodes__if_type__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__enabled__value: Boolean, $related_nodes__enabled__values: [Boolean], $related_nodes__enabled__source__id: ID, $related_nodes__enabled__owner__id: ID, $related_nodes__enabled__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimInterface(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, if_type__value: $if_type__value, if_type__values: $if_type__values, if_type__isnull: $if_type__isnull, if_type__source__id: $if_type__source__id, if_type__owner__id: $if_type__owner__id, if_type__is_protected: $if_type__is_protected, enabled__value: $enabled__value, enabled__values: $enabled__values, enabled__isnull: $enabled__isnull, enabled__source__id: $enabled__source__id, enabled__owner__id: $enabled__owner__id, enabled__is_protected: $enabled__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__if_type__value: $related_nodes__if_type__value, related_nodes__if_type__values: $related_nodes__if_type__values, related_nodes__if_type__source__id: $related_nodes__if_type__source__id, related_nodes__if_type__owner__id: $related_nodes__if_type__owner__id, related_nodes__if_type__is_protected: $related_nodes__if_type__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__enabled__value: $related_nodes__enabled__value, related_nodes__enabled__values: $related_nodes__enabled__values, related_nodes__enabled__source__id: $related_nodes__enabled__source__id, related_nodes__enabled__owner__id: $related_nodes__enabled__owner__id, related_nodes__enabled__is_protected: $related_nodes__enabled__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } if_type { is_default is_protected updated_at id is_from_profile permissions { __typename } value } enabled { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileDcimInterfaceResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileDcimInterfaceResponse, i64)> {
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
                let response = client.execute::<ProfileDcimInterfaceResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, ProfileDcimInterface, String, (ProfileDcimInterfaceResponse, i64)> = Box::new(move |(data, current_offset): (ProfileDcimInterfaceResponse, i64)| -> Result<EdgePage<ProfileDcimInterface, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_dcim_interface.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileDcimInterface>> {
        let mut filters = ProfileDcimInterfaceFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileDcimManufacturerFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_slug_value: Option<String>,
    pub related_nodes_slug_values: Option<Vec<String>>,
    pub related_nodes_slug_source_id: Option<String>,
    pub related_nodes_slug_owner_id: Option<String>,
    pub related_nodes_slug_is_protected: Option<bool>,
    pub related_nodes_name_value: Option<String>,
    pub related_nodes_name_values: Option<Vec<String>>,
    pub related_nodes_name_source_id: Option<String>,
    pub related_nodes_name_owner_id: Option<String>,
    pub related_nodes_name_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileDcimManufacturerFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_value {
            vars.insert(
                "related_nodes__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_values {
            vars.insert(
                "related_nodes__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_source_id {
            vars.insert(
                "related_nodes__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_owner_id {
            vars.insert(
                "related_nodes__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_is_protected {
            vars.insert(
                "related_nodes__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_value {
            vars.insert(
                "related_nodes__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_values {
            vars.insert(
                "related_nodes__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_source_id {
            vars.insert(
                "related_nodes__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_owner_id {
            vars.insert(
                "related_nodes__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_is_protected {
            vars.insert(
                "related_nodes__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileDcimManufacturerClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileDcimManufacturerClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileDcimManufacturerFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileDcimManufacturer>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileDcimManufacturer($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimManufacturer(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileDcimManufacturerResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_dcim_manufacturer.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileDcimManufacturerFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileDcimManufacturer, String, (ProfileDcimManufacturerResponse, i64)>
    {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileDcimManufacturer($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimManufacturer(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileDcimManufacturerResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileDcimManufacturerResponse, i64)> {
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
                let response = client.execute::<ProfileDcimManufacturerResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, ProfileDcimManufacturer, String, (ProfileDcimManufacturerResponse, i64)> = Box::new(move |(data, current_offset): (ProfileDcimManufacturerResponse, i64)| -> Result<EdgePage<ProfileDcimManufacturer, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_dcim_manufacturer.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileDcimManufacturer>> {
        let mut filters = ProfileDcimManufacturerFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileDcimPlatformFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_name_value: Option<String>,
    pub related_nodes_name_values: Option<Vec<String>>,
    pub related_nodes_name_source_id: Option<String>,
    pub related_nodes_name_owner_id: Option<String>,
    pub related_nodes_name_is_protected: Option<bool>,
    pub related_nodes_slug_value: Option<String>,
    pub related_nodes_slug_values: Option<Vec<String>>,
    pub related_nodes_slug_source_id: Option<String>,
    pub related_nodes_slug_owner_id: Option<String>,
    pub related_nodes_slug_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileDcimPlatformFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_value {
            vars.insert(
                "related_nodes__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_values {
            vars.insert(
                "related_nodes__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_source_id {
            vars.insert(
                "related_nodes__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_owner_id {
            vars.insert(
                "related_nodes__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_is_protected {
            vars.insert(
                "related_nodes__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_value {
            vars.insert(
                "related_nodes__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_values {
            vars.insert(
                "related_nodes__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_source_id {
            vars.insert(
                "related_nodes__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_owner_id {
            vars.insert(
                "related_nodes__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_is_protected {
            vars.insert(
                "related_nodes__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileDcimPlatformClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileDcimPlatformClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileDcimPlatformFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileDcimPlatform>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileDcimPlatform($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimPlatform(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileDcimPlatformResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_dcim_platform.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileDcimPlatformFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileDcimPlatform, String, (ProfileDcimPlatformResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileDcimPlatform($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimPlatform(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileDcimPlatformResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileDcimPlatformResponse, i64)> {
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
                let response = client.execute::<ProfileDcimPlatformResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, ProfileDcimPlatform, String, (ProfileDcimPlatformResponse, i64)> = Box::new(move |(data, current_offset): (ProfileDcimPlatformResponse, i64)| -> Result<EdgePage<ProfileDcimPlatform, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_dcim_platform.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileDcimPlatform>> {
        let mut filters = ProfileDcimPlatformFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileDcimSiteFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
    pub name_value: Option<String>,
    pub name_values: Option<Vec<String>>,
    pub name_isnull: Option<bool>,
    pub name_source_id: Option<String>,
    pub name_owner_id: Option<String>,
    pub name_is_protected: Option<bool>,
    pub status_value: Option<String>,
    pub status_values: Option<Vec<String>>,
    pub status_isnull: Option<bool>,
    pub status_source_id: Option<String>,
    pub status_owner_id: Option<String>,
    pub status_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_slug_value: Option<String>,
    pub related_nodes_slug_values: Option<Vec<String>>,
    pub related_nodes_slug_source_id: Option<String>,
    pub related_nodes_slug_owner_id: Option<String>,
    pub related_nodes_slug_is_protected: Option<bool>,
    pub related_nodes_name_value: Option<String>,
    pub related_nodes_name_values: Option<Vec<String>>,
    pub related_nodes_name_source_id: Option<String>,
    pub related_nodes_name_owner_id: Option<String>,
    pub related_nodes_name_is_protected: Option<bool>,
    pub related_nodes_status_value: Option<String>,
    pub related_nodes_status_values: Option<Vec<String>>,
    pub related_nodes_status_source_id: Option<String>,
    pub related_nodes_status_owner_id: Option<String>,
    pub related_nodes_status_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileDcimSiteFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.status_value {
            vars.insert("status__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.status_values {
            vars.insert("status__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.status_isnull {
            vars.insert("status__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.status_source_id {
            vars.insert(
                "status__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.status_owner_id {
            vars.insert(
                "status__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.status_is_protected {
            vars.insert(
                "status__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_value {
            vars.insert(
                "related_nodes__slug__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_values {
            vars.insert(
                "related_nodes__slug__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_source_id {
            vars.insert(
                "related_nodes__slug__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_owner_id {
            vars.insert(
                "related_nodes__slug__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_slug_is_protected {
            vars.insert(
                "related_nodes__slug__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_value {
            vars.insert(
                "related_nodes__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_values {
            vars.insert(
                "related_nodes__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_source_id {
            vars.insert(
                "related_nodes__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_owner_id {
            vars.insert(
                "related_nodes__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_is_protected {
            vars.insert(
                "related_nodes__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_value {
            vars.insert(
                "related_nodes__status__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_values {
            vars.insert(
                "related_nodes__status__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_source_id {
            vars.insert(
                "related_nodes__status__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_owner_id {
            vars.insert(
                "related_nodes__status__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_is_protected {
            vars.insert(
                "related_nodes__status__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileDcimSiteClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileDcimSiteClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileDcimSiteFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileDcimSite>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileDcimSite($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__status__value: String, $related_nodes__status__values: [String], $related_nodes__status__source__id: ID, $related_nodes__status__owner__id: ID, $related_nodes__status__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimSite(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__status__value: $related_nodes__status__value, related_nodes__status__values: $related_nodes__status__values, related_nodes__status__source__id: $related_nodes__status__source__id, related_nodes__status__owner__id: $related_nodes__status__owner__id, related_nodes__status__is_protected: $related_nodes__status__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileDcimSiteResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_dcim_site.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileDcimSiteFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileDcimSite, String, (ProfileDcimSiteResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileDcimSite($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__slug__value: String, $related_nodes__slug__values: [String], $related_nodes__slug__source__id: ID, $related_nodes__slug__owner__id: ID, $related_nodes__slug__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__status__value: String, $related_nodes__status__values: [String], $related_nodes__status__source__id: ID, $related_nodes__status__owner__id: ID, $related_nodes__status__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileDcimSite(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__slug__value: $related_nodes__slug__value, related_nodes__slug__values: $related_nodes__slug__values, related_nodes__slug__source__id: $related_nodes__slug__source__id, related_nodes__slug__owner__id: $related_nodes__slug__owner__id, related_nodes__slug__is_protected: $related_nodes__slug__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__status__value: $related_nodes__status__value, related_nodes__status__values: $related_nodes__status__values, related_nodes__status__source__id: $related_nodes__status__source__id, related_nodes__status__owner__id: $related_nodes__status__owner__id, related_nodes__status__is_protected: $related_nodes__status__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileDcimSiteResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileDcimSiteResponse, i64)> {
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
                    let response = client
                        .execute::<ProfileDcimSiteResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, ProfileDcimSite, String, (ProfileDcimSiteResponse, i64)> = Box::new(move |(data, current_offset): (ProfileDcimSiteResponse, i64)| -> Result<EdgePage<ProfileDcimSite, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_dcim_site.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileDcimSite>> {
        let mut filters = ProfileDcimSiteFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileIpamIpAddressFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
    pub status_value: Option<String>,
    pub status_values: Option<Vec<String>>,
    pub status_isnull: Option<bool>,
    pub status_source_id: Option<String>,
    pub status_owner_id: Option<String>,
    pub status_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_address_value: Option<String>,
    pub related_nodes_address_values: Option<Vec<String>>,
    pub related_nodes_address_source_id: Option<String>,
    pub related_nodes_address_owner_id: Option<String>,
    pub related_nodes_address_is_protected: Option<bool>,
    pub related_nodes_status_value: Option<String>,
    pub related_nodes_status_values: Option<Vec<String>>,
    pub related_nodes_status_source_id: Option<String>,
    pub related_nodes_status_owner_id: Option<String>,
    pub related_nodes_status_is_protected: Option<bool>,
    pub assigned_object_ids: Option<Vec<String>>,
    pub assigned_object_isnull: Option<bool>,
    pub assigned_object_display_label_value: Option<String>,
    pub assigned_object_display_label_values: Option<Vec<String>>,
    pub assigned_object_display_label_isnull: Option<bool>,
    pub assigned_object_if_type_value: Option<String>,
    pub assigned_object_if_type_values: Option<Vec<String>>,
    pub assigned_object_if_type_source_id: Option<String>,
    pub assigned_object_if_type_owner_id: Option<String>,
    pub assigned_object_if_type_is_protected: Option<bool>,
    pub assigned_object_name_value: Option<String>,
    pub assigned_object_name_values: Option<Vec<String>>,
    pub assigned_object_name_source_id: Option<String>,
    pub assigned_object_name_owner_id: Option<String>,
    pub assigned_object_name_is_protected: Option<bool>,
    pub assigned_object_enabled_value: Option<bool>,
    pub assigned_object_enabled_values: Option<Vec<bool>>,
    pub assigned_object_enabled_source_id: Option<String>,
    pub assigned_object_enabled_owner_id: Option<String>,
    pub assigned_object_enabled_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileIpamIpAddressFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.status_value {
            vars.insert("status__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.status_values {
            vars.insert("status__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.status_isnull {
            vars.insert("status__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.status_source_id {
            vars.insert(
                "status__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.status_owner_id {
            vars.insert(
                "status__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.status_is_protected {
            vars.insert(
                "status__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_value {
            vars.insert(
                "related_nodes__address__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_values {
            vars.insert(
                "related_nodes__address__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_source_id {
            vars.insert(
                "related_nodes__address__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_owner_id {
            vars.insert(
                "related_nodes__address__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_address_is_protected {
            vars.insert(
                "related_nodes__address__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_value {
            vars.insert(
                "related_nodes__status__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_values {
            vars.insert(
                "related_nodes__status__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_source_id {
            vars.insert(
                "related_nodes__status__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_owner_id {
            vars.insert(
                "related_nodes__status__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_status_is_protected {
            vars.insert(
                "related_nodes__status__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_ids {
            vars.insert(
                "assigned_object__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_isnull {
            vars.insert(
                "assigned_object__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_display_label_value {
            vars.insert(
                "assigned_object__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_display_label_values {
            vars.insert(
                "assigned_object__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_display_label_isnull {
            vars.insert(
                "assigned_object__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_if_type_value {
            vars.insert(
                "assigned_object__if_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_if_type_values {
            vars.insert(
                "assigned_object__if_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_if_type_source_id {
            vars.insert(
                "assigned_object__if_type__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_if_type_owner_id {
            vars.insert(
                "assigned_object__if_type__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_if_type_is_protected {
            vars.insert(
                "assigned_object__if_type__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_name_value {
            vars.insert(
                "assigned_object__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_name_values {
            vars.insert(
                "assigned_object__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_name_source_id {
            vars.insert(
                "assigned_object__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_name_owner_id {
            vars.insert(
                "assigned_object__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_name_is_protected {
            vars.insert(
                "assigned_object__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_enabled_value {
            vars.insert(
                "assigned_object__enabled__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_enabled_values {
            vars.insert(
                "assigned_object__enabled__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_enabled_source_id {
            vars.insert(
                "assigned_object__enabled__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_enabled_owner_id {
            vars.insert(
                "assigned_object__enabled__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.assigned_object_enabled_is_protected {
            vars.insert(
                "assigned_object__enabled__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileIpamIpAddressClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileIpamIpAddressClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileIpamIpAddressFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileIpamIpAddress>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileIpamIpAddress($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__address__value: String, $related_nodes__address__values: [String], $related_nodes__address__source__id: ID, $related_nodes__address__owner__id: ID, $related_nodes__address__is_protected: Boolean, $related_nodes__status__value: String, $related_nodes__status__values: [String], $related_nodes__status__source__id: ID, $related_nodes__status__owner__id: ID, $related_nodes__status__is_protected: Boolean, $assigned_object__ids: [ID], $assigned_object__isnull: Boolean, $assigned_object__display_label__value: String, $assigned_object__display_label__values: [String], $assigned_object__display_label__isnull: Boolean, $assigned_object__if_type__value: String, $assigned_object__if_type__values: [String], $assigned_object__if_type__source__id: ID, $assigned_object__if_type__owner__id: ID, $assigned_object__if_type__is_protected: Boolean, $assigned_object__name__value: String, $assigned_object__name__values: [String], $assigned_object__name__source__id: ID, $assigned_object__name__owner__id: ID, $assigned_object__name__is_protected: Boolean, $assigned_object__enabled__value: Boolean, $assigned_object__enabled__values: [Boolean], $assigned_object__enabled__source__id: ID, $assigned_object__enabled__owner__id: ID, $assigned_object__enabled__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileIpamIpAddress(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__address__value: $related_nodes__address__value, related_nodes__address__values: $related_nodes__address__values, related_nodes__address__source__id: $related_nodes__address__source__id, related_nodes__address__owner__id: $related_nodes__address__owner__id, related_nodes__address__is_protected: $related_nodes__address__is_protected, related_nodes__status__value: $related_nodes__status__value, related_nodes__status__values: $related_nodes__status__values, related_nodes__status__source__id: $related_nodes__status__source__id, related_nodes__status__owner__id: $related_nodes__status__owner__id, related_nodes__status__is_protected: $related_nodes__status__is_protected, assigned_object__ids: $assigned_object__ids, assigned_object__isnull: $assigned_object__isnull, assigned_object__display_label__value: $assigned_object__display_label__value, assigned_object__display_label__values: $assigned_object__display_label__values, assigned_object__display_label__isnull: $assigned_object__display_label__isnull, assigned_object__if_type__value: $assigned_object__if_type__value, assigned_object__if_type__values: $assigned_object__if_type__values, assigned_object__if_type__source__id: $assigned_object__if_type__source__id, assigned_object__if_type__owner__id: $assigned_object__if_type__owner__id, assigned_object__if_type__is_protected: $assigned_object__if_type__is_protected, assigned_object__name__value: $assigned_object__name__value, assigned_object__name__values: $assigned_object__name__values, assigned_object__name__source__id: $assigned_object__name__source__id, assigned_object__name__owner__id: $assigned_object__name__owner__id, assigned_object__name__is_protected: $assigned_object__name__is_protected, assigned_object__enabled__value: $assigned_object__enabled__value, assigned_object__enabled__values: $assigned_object__enabled__values, assigned_object__enabled__source__id: $assigned_object__enabled__source__id, assigned_object__enabled__owner__id: $assigned_object__enabled__owner__id, assigned_object__enabled__is_protected: $assigned_object__enabled__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } assigned_object { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileIpamIpAddressResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_ipam_ip_address.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileIpamIpAddressFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileIpamIpAddress, String, (ProfileIpamIpAddressResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileIpamIpAddress($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__address__value: String, $related_nodes__address__values: [String], $related_nodes__address__source__id: ID, $related_nodes__address__owner__id: ID, $related_nodes__address__is_protected: Boolean, $related_nodes__status__value: String, $related_nodes__status__values: [String], $related_nodes__status__source__id: ID, $related_nodes__status__owner__id: ID, $related_nodes__status__is_protected: Boolean, $assigned_object__ids: [ID], $assigned_object__isnull: Boolean, $assigned_object__display_label__value: String, $assigned_object__display_label__values: [String], $assigned_object__display_label__isnull: Boolean, $assigned_object__if_type__value: String, $assigned_object__if_type__values: [String], $assigned_object__if_type__source__id: ID, $assigned_object__if_type__owner__id: ID, $assigned_object__if_type__is_protected: Boolean, $assigned_object__name__value: String, $assigned_object__name__values: [String], $assigned_object__name__source__id: ID, $assigned_object__name__owner__id: ID, $assigned_object__name__is_protected: Boolean, $assigned_object__enabled__value: Boolean, $assigned_object__enabled__values: [Boolean], $assigned_object__enabled__source__id: ID, $assigned_object__enabled__owner__id: ID, $assigned_object__enabled__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileIpamIpAddress(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__address__value: $related_nodes__address__value, related_nodes__address__values: $related_nodes__address__values, related_nodes__address__source__id: $related_nodes__address__source__id, related_nodes__address__owner__id: $related_nodes__address__owner__id, related_nodes__address__is_protected: $related_nodes__address__is_protected, related_nodes__status__value: $related_nodes__status__value, related_nodes__status__values: $related_nodes__status__values, related_nodes__status__source__id: $related_nodes__status__source__id, related_nodes__status__owner__id: $related_nodes__status__owner__id, related_nodes__status__is_protected: $related_nodes__status__is_protected, assigned_object__ids: $assigned_object__ids, assigned_object__isnull: $assigned_object__isnull, assigned_object__display_label__value: $assigned_object__display_label__value, assigned_object__display_label__values: $assigned_object__display_label__values, assigned_object__display_label__isnull: $assigned_object__display_label__isnull, assigned_object__if_type__value: $assigned_object__if_type__value, assigned_object__if_type__values: $assigned_object__if_type__values, assigned_object__if_type__source__id: $assigned_object__if_type__source__id, assigned_object__if_type__owner__id: $assigned_object__if_type__owner__id, assigned_object__if_type__is_protected: $assigned_object__if_type__is_protected, assigned_object__name__value: $assigned_object__name__value, assigned_object__name__values: $assigned_object__name__values, assigned_object__name__source__id: $assigned_object__name__source__id, assigned_object__name__owner__id: $assigned_object__name__owner__id, assigned_object__name__is_protected: $assigned_object__name__is_protected, assigned_object__enabled__value: $assigned_object__enabled__value, assigned_object__enabled__values: $assigned_object__enabled__values, assigned_object__enabled__source__id: $assigned_object__enabled__source__id, assigned_object__enabled__owner__id: $assigned_object__enabled__owner__id, assigned_object__enabled__is_protected: $assigned_object__enabled__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } assigned_object { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileIpamIpAddressResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileIpamIpAddressResponse, i64)> {
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
                let response = client.execute::<ProfileIpamIpAddressResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, ProfileIpamIpAddress, String, (ProfileIpamIpAddressResponse, i64)> = Box::new(move |(data, current_offset): (ProfileIpamIpAddressResponse, i64)| -> Result<EdgePage<ProfileIpamIpAddress, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_ipam_ip_address.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileIpamIpAddress>> {
        let mut filters = ProfileIpamIpAddressFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileIpamNamespaceFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name_value: Option<String>,
    pub profile_name_values: Option<Vec<String>>,
    pub profile_name_isnull: Option<bool>,
    pub profile_name_source_id: Option<String>,
    pub profile_name_owner_id: Option<String>,
    pub profile_name_is_protected: Option<bool>,
    pub profile_priority_value: Option<i64>,
    pub profile_priority_values: Option<Vec<i64>>,
    pub profile_priority_isnull: Option<bool>,
    pub profile_priority_source_id: Option<String>,
    pub profile_priority_owner_id: Option<String>,
    pub profile_priority_is_protected: Option<bool>,
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
    pub related_nodes_ids: Option<Vec<String>>,
    pub related_nodes_isnull: Option<bool>,
    pub related_nodes_display_label_value: Option<String>,
    pub related_nodes_display_label_values: Option<Vec<String>>,
    pub related_nodes_display_label_isnull: Option<bool>,
    pub related_nodes_default_value: Option<bool>,
    pub related_nodes_default_values: Option<Vec<bool>>,
    pub related_nodes_default_source_id: Option<String>,
    pub related_nodes_default_owner_id: Option<String>,
    pub related_nodes_default_is_protected: Option<bool>,
    pub related_nodes_name_value: Option<String>,
    pub related_nodes_name_values: Option<Vec<String>>,
    pub related_nodes_name_source_id: Option<String>,
    pub related_nodes_name_owner_id: Option<String>,
    pub related_nodes_name_is_protected: Option<bool>,
    pub related_nodes_description_value: Option<String>,
    pub related_nodes_description_values: Option<Vec<String>>,
    pub related_nodes_description_source_id: Option<String>,
    pub related_nodes_description_owner_id: Option<String>,
    pub related_nodes_description_is_protected: Option<bool>,
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
    pub ip_prefixes_ids: Option<Vec<String>>,
    pub ip_prefixes_isnull: Option<bool>,
    pub ip_prefixes_display_label_value: Option<String>,
    pub ip_prefixes_display_label_values: Option<Vec<String>>,
    pub ip_prefixes_display_label_isnull: Option<bool>,
    pub ip_prefixes_member_type_value: Option<String>,
    pub ip_prefixes_member_type_values: Option<Vec<String>>,
    pub ip_prefixes_member_type_source_id: Option<String>,
    pub ip_prefixes_member_type_owner_id: Option<String>,
    pub ip_prefixes_member_type_is_protected: Option<bool>,
    pub ip_prefixes_description_value: Option<String>,
    pub ip_prefixes_description_values: Option<Vec<String>>,
    pub ip_prefixes_description_source_id: Option<String>,
    pub ip_prefixes_description_owner_id: Option<String>,
    pub ip_prefixes_description_is_protected: Option<bool>,
    pub ip_prefixes_hostmask_value: Option<String>,
    pub ip_prefixes_hostmask_values: Option<Vec<String>>,
    pub ip_prefixes_hostmask_source_id: Option<String>,
    pub ip_prefixes_hostmask_owner_id: Option<String>,
    pub ip_prefixes_hostmask_is_protected: Option<bool>,
    pub ip_prefixes_network_address_value: Option<String>,
    pub ip_prefixes_network_address_values: Option<Vec<String>>,
    pub ip_prefixes_network_address_source_id: Option<String>,
    pub ip_prefixes_network_address_owner_id: Option<String>,
    pub ip_prefixes_network_address_is_protected: Option<bool>,
    pub ip_prefixes_broadcast_address_value: Option<String>,
    pub ip_prefixes_broadcast_address_values: Option<Vec<String>>,
    pub ip_prefixes_broadcast_address_source_id: Option<String>,
    pub ip_prefixes_broadcast_address_owner_id: Option<String>,
    pub ip_prefixes_broadcast_address_is_protected: Option<bool>,
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
    pub ip_prefixes_prefix_value: Option<String>,
    pub ip_prefixes_prefix_values: Option<Vec<String>>,
    pub ip_prefixes_prefix_source_id: Option<String>,
    pub ip_prefixes_prefix_owner_id: Option<String>,
    pub ip_prefixes_prefix_is_protected: Option<bool>,
    pub ip_prefixes_is_top_level_value: Option<bool>,
    pub ip_prefixes_is_top_level_values: Option<Vec<bool>>,
    pub ip_prefixes_is_top_level_source_id: Option<String>,
    pub ip_prefixes_is_top_level_owner_id: Option<String>,
    pub ip_prefixes_is_top_level_is_protected: Option<bool>,
    pub ip_prefixes_netmask_value: Option<String>,
    pub ip_prefixes_netmask_values: Option<Vec<String>>,
    pub ip_prefixes_netmask_source_id: Option<String>,
    pub ip_prefixes_netmask_owner_id: Option<String>,
    pub ip_prefixes_netmask_is_protected: Option<bool>,
    pub member_of_groups_ids: Option<Vec<String>>,
    pub member_of_groups_isnull: Option<bool>,
    pub member_of_groups_display_label_value: Option<String>,
    pub member_of_groups_display_label_values: Option<Vec<String>>,
    pub member_of_groups_display_label_isnull: Option<bool>,
    pub member_of_groups_name_value: Option<String>,
    pub member_of_groups_name_values: Option<Vec<String>>,
    pub member_of_groups_group_type_value: Option<String>,
    pub member_of_groups_group_type_values: Option<Vec<String>>,
    pub member_of_groups_description_value: Option<String>,
    pub member_of_groups_description_values: Option<Vec<String>>,
    pub member_of_groups_label_value: Option<String>,
    pub member_of_groups_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_ids: Option<Vec<String>>,
    pub subscriber_of_groups_isnull: Option<bool>,
    pub subscriber_of_groups_display_label_value: Option<String>,
    pub subscriber_of_groups_display_label_values: Option<Vec<String>>,
    pub subscriber_of_groups_display_label_isnull: Option<bool>,
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
}

impl ProfileIpamNamespaceFilters {
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
            vars.insert(
                "display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_values {
            vars.insert(
                "display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.display_label_isnull {
            vars.insert(
                "display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name_value {
            vars.insert(
                "profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_values {
            vars.insert(
                "profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_isnull {
            vars.insert(
                "profile_name__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_source_id {
            vars.insert(
                "profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_owner_id {
            vars.insert(
                "profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_name_is_protected {
            vars.insert(
                "profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_value {
            vars.insert(
                "profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_values {
            vars.insert(
                "profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_isnull {
            vars.insert(
                "profile_priority__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_source_id {
            vars.insert(
                "profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_owner_id {
            vars.insert(
                "profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profile_priority_is_protected {
            vars.insert(
                "profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_value {
            vars.insert(
                "description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_values {
            vars.insert(
                "description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_isnull {
            vars.insert(
                "description__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_source_id {
            vars.insert(
                "description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_owner_id {
            vars.insert(
                "description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.description_is_protected {
            vars.insert(
                "description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
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
            vars.insert(
                "any__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata_created_by_id {
            vars.insert(
                "node_metadata__created_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_by_ids {
            vars.insert(
                "node_metadata__created_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_id {
            vars.insert(
                "node_metadata__updated_by__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_by_ids {
            vars.insert(
                "node_metadata__updated_by__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at {
            vars.insert(
                "node_metadata__created_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_before {
            vars.insert(
                "node_metadata__created_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_created_at_after {
            vars.insert(
                "node_metadata__created_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at {
            vars.insert(
                "node_metadata__updated_at".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_before {
            vars.insert(
                "node_metadata__updated_at__before".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.node_metadata_updated_at_after {
            vars.insert(
                "node_metadata__updated_at__after".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_ids {
            vars.insert(
                "related_nodes__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_isnull {
            vars.insert(
                "related_nodes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_value {
            vars.insert(
                "related_nodes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_values {
            vars.insert(
                "related_nodes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_display_label_isnull {
            vars.insert(
                "related_nodes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_default_value {
            vars.insert(
                "related_nodes__default__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_default_values {
            vars.insert(
                "related_nodes__default__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_default_source_id {
            vars.insert(
                "related_nodes__default__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_default_owner_id {
            vars.insert(
                "related_nodes__default__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_default_is_protected {
            vars.insert(
                "related_nodes__default__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_value {
            vars.insert(
                "related_nodes__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_values {
            vars.insert(
                "related_nodes__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_source_id {
            vars.insert(
                "related_nodes__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_owner_id {
            vars.insert(
                "related_nodes__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_name_is_protected {
            vars.insert(
                "related_nodes__name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_value {
            vars.insert(
                "related_nodes__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_values {
            vars.insert(
                "related_nodes__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_source_id {
            vars.insert(
                "related_nodes__description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_owner_id {
            vars.insert(
                "related_nodes__description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.related_nodes_description_is_protected {
            vars.insert(
                "related_nodes__description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_ids {
            vars.insert(
                "ip_addresses__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_isnull {
            vars.insert(
                "ip_addresses__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_display_label_value {
            vars.insert(
                "ip_addresses__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_display_label_values {
            vars.insert(
                "ip_addresses__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_display_label_isnull {
            vars.insert(
                "ip_addresses__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_address_value {
            vars.insert(
                "ip_addresses__address__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_address_values {
            vars.insert(
                "ip_addresses__address__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_address_source_id {
            vars.insert(
                "ip_addresses__address__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_address_owner_id {
            vars.insert(
                "ip_addresses__address__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_address_is_protected {
            vars.insert(
                "ip_addresses__address__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_description_value {
            vars.insert(
                "ip_addresses__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_description_values {
            vars.insert(
                "ip_addresses__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_description_source_id {
            vars.insert(
                "ip_addresses__description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_description_owner_id {
            vars.insert(
                "ip_addresses__description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_addresses_description_is_protected {
            vars.insert(
                "ip_addresses__description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_ids {
            vars.insert("ip_prefixes__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes_isnull {
            vars.insert(
                "ip_prefixes__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_display_label_value {
            vars.insert(
                "ip_prefixes__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_display_label_values {
            vars.insert(
                "ip_prefixes__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_display_label_isnull {
            vars.insert(
                "ip_prefixes__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_member_type_value {
            vars.insert(
                "ip_prefixes__member_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_member_type_values {
            vars.insert(
                "ip_prefixes__member_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_member_type_source_id {
            vars.insert(
                "ip_prefixes__member_type__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_member_type_owner_id {
            vars.insert(
                "ip_prefixes__member_type__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_member_type_is_protected {
            vars.insert(
                "ip_prefixes__member_type__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_description_value {
            vars.insert(
                "ip_prefixes__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_description_values {
            vars.insert(
                "ip_prefixes__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_description_source_id {
            vars.insert(
                "ip_prefixes__description__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_description_owner_id {
            vars.insert(
                "ip_prefixes__description__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_description_is_protected {
            vars.insert(
                "ip_prefixes__description__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_hostmask_value {
            vars.insert(
                "ip_prefixes__hostmask__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_hostmask_values {
            vars.insert(
                "ip_prefixes__hostmask__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_hostmask_source_id {
            vars.insert(
                "ip_prefixes__hostmask__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_hostmask_owner_id {
            vars.insert(
                "ip_prefixes__hostmask__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_hostmask_is_protected {
            vars.insert(
                "ip_prefixes__hostmask__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_network_address_value {
            vars.insert(
                "ip_prefixes__network_address__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_network_address_values {
            vars.insert(
                "ip_prefixes__network_address__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_network_address_source_id {
            vars.insert(
                "ip_prefixes__network_address__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_network_address_owner_id {
            vars.insert(
                "ip_prefixes__network_address__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_network_address_is_protected {
            vars.insert(
                "ip_prefixes__network_address__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_value {
            vars.insert(
                "ip_prefixes__broadcast_address__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_values {
            vars.insert(
                "ip_prefixes__broadcast_address__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_source_id {
            vars.insert(
                "ip_prefixes__broadcast_address__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_owner_id {
            vars.insert(
                "ip_prefixes__broadcast_address__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_broadcast_address_is_protected {
            vars.insert(
                "ip_prefixes__broadcast_address__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_utilization_value {
            vars.insert(
                "ip_prefixes__utilization__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_utilization_values {
            vars.insert(
                "ip_prefixes__utilization__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_utilization_source_id {
            vars.insert(
                "ip_prefixes__utilization__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_utilization_owner_id {
            vars.insert(
                "ip_prefixes__utilization__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_utilization_is_protected {
            vars.insert(
                "ip_prefixes__utilization__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_pool_value {
            vars.insert(
                "ip_prefixes__is_pool__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_pool_values {
            vars.insert(
                "ip_prefixes__is_pool__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_pool_source_id {
            vars.insert(
                "ip_prefixes__is_pool__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_pool_owner_id {
            vars.insert(
                "ip_prefixes__is_pool__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_pool_is_protected {
            vars.insert(
                "ip_prefixes__is_pool__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_prefix_value {
            vars.insert(
                "ip_prefixes__prefix__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_prefix_values {
            vars.insert(
                "ip_prefixes__prefix__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_prefix_source_id {
            vars.insert(
                "ip_prefixes__prefix__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_prefix_owner_id {
            vars.insert(
                "ip_prefixes__prefix__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_prefix_is_protected {
            vars.insert(
                "ip_prefixes__prefix__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_value {
            vars.insert(
                "ip_prefixes__is_top_level__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_values {
            vars.insert(
                "ip_prefixes__is_top_level__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_source_id {
            vars.insert(
                "ip_prefixes__is_top_level__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_owner_id {
            vars.insert(
                "ip_prefixes__is_top_level__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_is_top_level_is_protected {
            vars.insert(
                "ip_prefixes__is_top_level__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_netmask_value {
            vars.insert(
                "ip_prefixes__netmask__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_netmask_values {
            vars.insert(
                "ip_prefixes__netmask__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_netmask_source_id {
            vars.insert(
                "ip_prefixes__netmask__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_netmask_owner_id {
            vars.insert(
                "ip_prefixes__netmask__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.ip_prefixes_netmask_is_protected {
            vars.insert(
                "ip_prefixes__netmask__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_ids {
            vars.insert(
                "member_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_isnull {
            vars.insert(
                "member_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_value {
            vars.insert(
                "member_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_values {
            vars.insert(
                "member_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_display_label_isnull {
            vars.insert(
                "member_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_value {
            vars.insert(
                "member_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_name_values {
            vars.insert(
                "member_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_value {
            vars.insert(
                "member_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_group_type_values {
            vars.insert(
                "member_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_value {
            vars.insert(
                "member_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_description_values {
            vars.insert(
                "member_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_value {
            vars.insert(
                "member_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.member_of_groups_label_values {
            vars.insert(
                "member_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_ids {
            vars.insert(
                "subscriber_of_groups__ids".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_isnull {
            vars.insert(
                "subscriber_of_groups__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_value {
            vars.insert(
                "subscriber_of_groups__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_values {
            vars.insert(
                "subscriber_of_groups__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_display_label_isnull {
            vars.insert(
                "subscriber_of_groups__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_value {
            vars.insert(
                "subscriber_of_groups__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_name_values {
            vars.insert(
                "subscriber_of_groups__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_value {
            vars.insert(
                "subscriber_of_groups__group_type__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_group_type_values {
            vars.insert(
                "subscriber_of_groups__group_type__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_value {
            vars.insert(
                "subscriber_of_groups__description__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_description_values {
            vars.insert(
                "subscriber_of_groups__description__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_value {
            vars.insert(
                "subscriber_of_groups__label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.subscriber_of_groups_label_values {
            vars.insert(
                "subscriber_of_groups__label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct ProfileIpamNamespaceClient<'a> {
    client: &'a Client,
}

impl<'a> ProfileIpamNamespaceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<ProfileIpamNamespaceFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<ProfileIpamNamespace>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileIpamNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__default__value: Boolean, $related_nodes__default__values: [Boolean], $related_nodes__default__source__id: ID, $related_nodes__default__owner__id: ID, $related_nodes__default__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileIpamNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__default__value: $related_nodes__default__value, related_nodes__default__values: $related_nodes__default__values, related_nodes__default__source__id: $related_nodes__default__source__id, related_nodes__default__owner__id: $related_nodes__default__owner__id, related_nodes__default__is_protected: $related_nodes__default__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } ip_addresses { count edges { __typename } } ip_prefixes { count edges { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<ProfileIpamNamespaceResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_ipam_namespace.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<ProfileIpamNamespaceFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, ProfileIpamNamespace, String, (ProfileIpamNamespaceResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileIpamNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__default__value: Boolean, $related_nodes__default__values: [Boolean], $related_nodes__default__source__id: ID, $related_nodes__default__owner__id: ID, $related_nodes__default__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String]) { ProfileIpamNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__default__value: $related_nodes__default__value, related_nodes__default__values: $related_nodes__default__values, related_nodes__default__source__id: $related_nodes__default__source__id, related_nodes__default__owner__id: $related_nodes__default__owner__id, related_nodes__default__is_protected: $related_nodes__default__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } ip_addresses { count edges { __typename } } ip_prefixes { count edges { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileIpamNamespaceResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileIpamNamespaceResponse, i64)> {
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
                let response = client.execute::<ProfileIpamNamespaceResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, ProfileIpamNamespace, String, (ProfileIpamNamespaceResponse, i64)> = Box::new(move |(data, current_offset): (ProfileIpamNamespaceResponse, i64)| -> Result<EdgePage<ProfileIpamNamespace, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.profile_ipam_namespace.edges {
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

    pub async fn get_by_id(
        &self,
        id: impl Into<String>,
        request_branch: Option<&str>,
    ) -> Result<Option<ProfileIpamNamespace>> {
        let mut filters = ProfileIpamNamespaceFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}
