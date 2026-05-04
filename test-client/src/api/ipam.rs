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
    pub display_label__value: Option<String>,
    pub display_label__values: Option<Vec<String>>,
    pub display_label__isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub default__value: Option<bool>,
    pub default__values: Option<Vec<bool>>,
    pub default__isnull: Option<bool>,
    pub default__source__id: Option<String>,
    pub default__owner__id: Option<String>,
    pub default__is_protected: Option<bool>,
    pub description__value: Option<String>,
    pub description__values: Option<Vec<String>>,
    pub description__isnull: Option<bool>,
    pub description__source__id: Option<String>,
    pub description__owner__id: Option<String>,
    pub description__is_protected: Option<bool>,
    pub name__value: Option<String>,
    pub name__values: Option<Vec<String>>,
    pub name__isnull: Option<bool>,
    pub name__source__id: Option<String>,
    pub name__owner__id: Option<String>,
    pub name__is_protected: Option<bool>,
    pub any__value: Option<String>,
    pub any__values: Option<Vec<String>>,
    pub any__source__id: Option<String>,
    pub any__owner__id: Option<String>,
    pub any__is_protected: Option<bool>,
    pub partial_match: Option<bool>,
    pub node_metadata__created_by__id: Option<String>,
    pub node_metadata__created_by__ids: Option<Vec<String>>,
    pub node_metadata__updated_by__id: Option<String>,
    pub node_metadata__updated_by__ids: Option<Vec<String>>,
    pub node_metadata__created_at: Option<String>,
    pub node_metadata__created_at__before: Option<String>,
    pub node_metadata__created_at__after: Option<String>,
    pub node_metadata__updated_at: Option<String>,
    pub node_metadata__updated_at__before: Option<String>,
    pub node_metadata__updated_at__after: Option<String>,
    pub profiles__ids: Option<Vec<String>>,
    pub profiles__isnull: Option<bool>,
    pub profiles__display_label__value: Option<String>,
    pub profiles__display_label__values: Option<Vec<String>>,
    pub profiles__display_label__isnull: Option<bool>,
    pub profiles__profile_priority__value: Option<i64>,
    pub profiles__profile_priority__values: Option<Vec<i64>>,
    pub profiles__profile_priority__source__id: Option<String>,
    pub profiles__profile_priority__owner__id: Option<String>,
    pub profiles__profile_priority__is_protected: Option<bool>,
    pub profiles__profile_name__value: Option<String>,
    pub profiles__profile_name__values: Option<Vec<String>>,
    pub profiles__profile_name__source__id: Option<String>,
    pub profiles__profile_name__owner__id: Option<String>,
    pub profiles__profile_name__is_protected: Option<bool>,
    pub subscriber_of_groups__ids: Option<Vec<String>>,
    pub subscriber_of_groups__isnull: Option<bool>,
    pub subscriber_of_groups__display_label__value: Option<String>,
    pub subscriber_of_groups__display_label__values: Option<Vec<String>>,
    pub subscriber_of_groups__display_label__isnull: Option<bool>,
    pub subscriber_of_groups__group_type__value: Option<String>,
    pub subscriber_of_groups__group_type__values: Option<Vec<String>>,
    pub subscriber_of_groups__name__value: Option<String>,
    pub subscriber_of_groups__name__values: Option<Vec<String>>,
    pub subscriber_of_groups__label__value: Option<String>,
    pub subscriber_of_groups__label__values: Option<Vec<String>>,
    pub subscriber_of_groups__description__value: Option<String>,
    pub subscriber_of_groups__description__values: Option<Vec<String>>,
    pub member_of_groups__ids: Option<Vec<String>>,
    pub member_of_groups__isnull: Option<bool>,
    pub member_of_groups__display_label__value: Option<String>,
    pub member_of_groups__display_label__values: Option<Vec<String>>,
    pub member_of_groups__display_label__isnull: Option<bool>,
    pub member_of_groups__group_type__value: Option<String>,
    pub member_of_groups__group_type__values: Option<Vec<String>>,
    pub member_of_groups__name__value: Option<String>,
    pub member_of_groups__name__values: Option<Vec<String>>,
    pub member_of_groups__label__value: Option<String>,
    pub member_of_groups__label__values: Option<Vec<String>>,
    pub member_of_groups__description__value: Option<String>,
    pub member_of_groups__description__values: Option<Vec<String>>,
    pub ip_addresses__ids: Option<Vec<String>>,
    pub ip_addresses__isnull: Option<bool>,
    pub ip_addresses__display_label__value: Option<String>,
    pub ip_addresses__display_label__values: Option<Vec<String>>,
    pub ip_addresses__display_label__isnull: Option<bool>,
    pub ip_addresses__address__value: Option<String>,
    pub ip_addresses__address__values: Option<Vec<String>>,
    pub ip_addresses__address__source__id: Option<String>,
    pub ip_addresses__address__owner__id: Option<String>,
    pub ip_addresses__address__is_protected: Option<bool>,
    pub ip_addresses__description__value: Option<String>,
    pub ip_addresses__description__values: Option<Vec<String>>,
    pub ip_addresses__description__source__id: Option<String>,
    pub ip_addresses__description__owner__id: Option<String>,
    pub ip_addresses__description__is_protected: Option<bool>,
    pub ip_prefixes__ids: Option<Vec<String>>,
    pub ip_prefixes__isnull: Option<bool>,
    pub ip_prefixes__display_label__value: Option<String>,
    pub ip_prefixes__display_label__values: Option<Vec<String>>,
    pub ip_prefixes__display_label__isnull: Option<bool>,
    pub ip_prefixes__is_top_level__value: Option<bool>,
    pub ip_prefixes__is_top_level__values: Option<Vec<bool>>,
    pub ip_prefixes__is_top_level__source__id: Option<String>,
    pub ip_prefixes__is_top_level__owner__id: Option<String>,
    pub ip_prefixes__is_top_level__is_protected: Option<bool>,
    pub ip_prefixes__broadcast_address__value: Option<String>,
    pub ip_prefixes__broadcast_address__values: Option<Vec<String>>,
    pub ip_prefixes__broadcast_address__source__id: Option<String>,
    pub ip_prefixes__broadcast_address__owner__id: Option<String>,
    pub ip_prefixes__broadcast_address__is_protected: Option<bool>,
    pub ip_prefixes__netmask__value: Option<String>,
    pub ip_prefixes__netmask__values: Option<Vec<String>>,
    pub ip_prefixes__netmask__source__id: Option<String>,
    pub ip_prefixes__netmask__owner__id: Option<String>,
    pub ip_prefixes__netmask__is_protected: Option<bool>,
    pub ip_prefixes__member_type__value: Option<String>,
    pub ip_prefixes__member_type__values: Option<Vec<String>>,
    pub ip_prefixes__member_type__source__id: Option<String>,
    pub ip_prefixes__member_type__owner__id: Option<String>,
    pub ip_prefixes__member_type__is_protected: Option<bool>,
    pub ip_prefixes__utilization__value: Option<i64>,
    pub ip_prefixes__utilization__values: Option<Vec<i64>>,
    pub ip_prefixes__utilization__source__id: Option<String>,
    pub ip_prefixes__utilization__owner__id: Option<String>,
    pub ip_prefixes__utilization__is_protected: Option<bool>,
    pub ip_prefixes__hostmask__value: Option<String>,
    pub ip_prefixes__hostmask__values: Option<Vec<String>>,
    pub ip_prefixes__hostmask__source__id: Option<String>,
    pub ip_prefixes__hostmask__owner__id: Option<String>,
    pub ip_prefixes__hostmask__is_protected: Option<bool>,
    pub ip_prefixes__network_address__value: Option<String>,
    pub ip_prefixes__network_address__values: Option<Vec<String>>,
    pub ip_prefixes__network_address__source__id: Option<String>,
    pub ip_prefixes__network_address__owner__id: Option<String>,
    pub ip_prefixes__network_address__is_protected: Option<bool>,
    pub ip_prefixes__is_pool__value: Option<bool>,
    pub ip_prefixes__is_pool__values: Option<Vec<bool>>,
    pub ip_prefixes__is_pool__source__id: Option<String>,
    pub ip_prefixes__is_pool__owner__id: Option<String>,
    pub ip_prefixes__is_pool__is_protected: Option<bool>,
    pub ip_prefixes__prefix__value: Option<String>,
    pub ip_prefixes__prefix__values: Option<Vec<String>>,
    pub ip_prefixes__prefix__source__id: Option<String>,
    pub ip_prefixes__prefix__owner__id: Option<String>,
    pub ip_prefixes__prefix__is_protected: Option<bool>,
    pub ip_prefixes__description__value: Option<String>,
    pub ip_prefixes__description__values: Option<Vec<String>>,
    pub ip_prefixes__description__source__id: Option<String>,
    pub ip_prefixes__description__owner__id: Option<String>,
    pub ip_prefixes__description__is_protected: Option<bool>,
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
        if let Some(value) = &self.display_label__value {
            vars.insert("display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label__values {
            vars.insert("display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.display_label__isnull {
            vars.insert("display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.hfid {
            vars.insert("hfid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default__value {
            vars.insert("default__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default__values {
            vars.insert("default__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default__isnull {
            vars.insert("default__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default__source__id {
            vars.insert("default__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default__owner__id {
            vars.insert("default__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.default__is_protected {
            vars.insert("default__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description__value {
            vars.insert("description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description__values {
            vars.insert("description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description__isnull {
            vars.insert("description__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description__source__id {
            vars.insert("description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description__owner__id {
            vars.insert("description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.description__is_protected {
            vars.insert("description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name__value {
            vars.insert("name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name__values {
            vars.insert("name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name__isnull {
            vars.insert("name__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name__source__id {
            vars.insert("name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name__owner__id {
            vars.insert("name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.name__is_protected {
            vars.insert("name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any__value {
            vars.insert("any__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any__values {
            vars.insert("any__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any__source__id {
            vars.insert("any__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any__owner__id {
            vars.insert("any__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.any__is_protected {
            vars.insert("any__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.partial_match {
            vars.insert("partial_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__created_by__id {
            vars.insert("node_metadata__created_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__created_by__ids {
            vars.insert("node_metadata__created_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__updated_by__id {
            vars.insert("node_metadata__updated_by__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__updated_by__ids {
            vars.insert("node_metadata__updated_by__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__created_at {
            vars.insert("node_metadata__created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__created_at__before {
            vars.insert("node_metadata__created_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__created_at__after {
            vars.insert("node_metadata__created_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__updated_at {
            vars.insert("node_metadata__updated_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__updated_at__before {
            vars.insert("node_metadata__updated_at__before".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.node_metadata__updated_at__after {
            vars.insert("node_metadata__updated_at__after".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__display_label__value {
            vars.insert("profiles__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__display_label__values {
            vars.insert("profiles__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__display_label__isnull {
            vars.insert("profiles__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_priority__value {
            vars.insert("profiles__profile_priority__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_priority__values {
            vars.insert("profiles__profile_priority__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_priority__source__id {
            vars.insert("profiles__profile_priority__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_priority__owner__id {
            vars.insert("profiles__profile_priority__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_priority__is_protected {
            vars.insert("profiles__profile_priority__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_name__value {
            vars.insert("profiles__profile_name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_name__values {
            vars.insert("profiles__profile_name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_name__source__id {
            vars.insert("profiles__profile_name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_name__owner__id {
            vars.insert("profiles__profile_name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles__profile_name__is_protected {
            vars.insert("profiles__profile_name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__ids {
            vars.insert("subscriber_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__isnull {
            vars.insert("subscriber_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__display_label__value {
            vars.insert("subscriber_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__display_label__values {
            vars.insert("subscriber_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__display_label__isnull {
            vars.insert("subscriber_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__value {
            vars.insert("subscriber_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__values {
            vars.insert("subscriber_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__value {
            vars.insert("subscriber_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__values {
            vars.insert("subscriber_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__label__value {
            vars.insert("subscriber_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__label__values {
            vars.insert("subscriber_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__value {
            vars.insert("subscriber_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__values {
            vars.insert("subscriber_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__ids {
            vars.insert("member_of_groups__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__isnull {
            vars.insert("member_of_groups__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__display_label__value {
            vars.insert("member_of_groups__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__display_label__values {
            vars.insert("member_of_groups__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__display_label__isnull {
            vars.insert("member_of_groups__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__value {
            vars.insert("member_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__values {
            vars.insert("member_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__value {
            vars.insert("member_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__values {
            vars.insert("member_of_groups__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__label__value {
            vars.insert("member_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__label__values {
            vars.insert("member_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__value {
            vars.insert("member_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__values {
            vars.insert("member_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__ids {
            vars.insert("ip_addresses__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__isnull {
            vars.insert("ip_addresses__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__display_label__value {
            vars.insert("ip_addresses__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__display_label__values {
            vars.insert("ip_addresses__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__display_label__isnull {
            vars.insert("ip_addresses__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__address__value {
            vars.insert("ip_addresses__address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__address__values {
            vars.insert("ip_addresses__address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__address__source__id {
            vars.insert("ip_addresses__address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__address__owner__id {
            vars.insert("ip_addresses__address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__address__is_protected {
            vars.insert("ip_addresses__address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__description__value {
            vars.insert("ip_addresses__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__description__values {
            vars.insert("ip_addresses__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__description__source__id {
            vars.insert("ip_addresses__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__description__owner__id {
            vars.insert("ip_addresses__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_addresses__description__is_protected {
            vars.insert("ip_addresses__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__ids {
            vars.insert("ip_prefixes__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__isnull {
            vars.insert("ip_prefixes__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__display_label__value {
            vars.insert("ip_prefixes__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__display_label__values {
            vars.insert("ip_prefixes__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__display_label__isnull {
            vars.insert("ip_prefixes__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_top_level__value {
            vars.insert("ip_prefixes__is_top_level__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_top_level__values {
            vars.insert("ip_prefixes__is_top_level__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_top_level__source__id {
            vars.insert("ip_prefixes__is_top_level__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_top_level__owner__id {
            vars.insert("ip_prefixes__is_top_level__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_top_level__is_protected {
            vars.insert("ip_prefixes__is_top_level__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__broadcast_address__value {
            vars.insert("ip_prefixes__broadcast_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__broadcast_address__values {
            vars.insert("ip_prefixes__broadcast_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__broadcast_address__source__id {
            vars.insert("ip_prefixes__broadcast_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__broadcast_address__owner__id {
            vars.insert("ip_prefixes__broadcast_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__broadcast_address__is_protected {
            vars.insert("ip_prefixes__broadcast_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__netmask__value {
            vars.insert("ip_prefixes__netmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__netmask__values {
            vars.insert("ip_prefixes__netmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__netmask__source__id {
            vars.insert("ip_prefixes__netmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__netmask__owner__id {
            vars.insert("ip_prefixes__netmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__netmask__is_protected {
            vars.insert("ip_prefixes__netmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__member_type__value {
            vars.insert("ip_prefixes__member_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__member_type__values {
            vars.insert("ip_prefixes__member_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__member_type__source__id {
            vars.insert("ip_prefixes__member_type__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__member_type__owner__id {
            vars.insert("ip_prefixes__member_type__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__member_type__is_protected {
            vars.insert("ip_prefixes__member_type__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__utilization__value {
            vars.insert("ip_prefixes__utilization__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__utilization__values {
            vars.insert("ip_prefixes__utilization__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__utilization__source__id {
            vars.insert("ip_prefixes__utilization__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__utilization__owner__id {
            vars.insert("ip_prefixes__utilization__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__utilization__is_protected {
            vars.insert("ip_prefixes__utilization__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__hostmask__value {
            vars.insert("ip_prefixes__hostmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__hostmask__values {
            vars.insert("ip_prefixes__hostmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__hostmask__source__id {
            vars.insert("ip_prefixes__hostmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__hostmask__owner__id {
            vars.insert("ip_prefixes__hostmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__hostmask__is_protected {
            vars.insert("ip_prefixes__hostmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__network_address__value {
            vars.insert("ip_prefixes__network_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__network_address__values {
            vars.insert("ip_prefixes__network_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__network_address__source__id {
            vars.insert("ip_prefixes__network_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__network_address__owner__id {
            vars.insert("ip_prefixes__network_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__network_address__is_protected {
            vars.insert("ip_prefixes__network_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_pool__value {
            vars.insert("ip_prefixes__is_pool__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_pool__values {
            vars.insert("ip_prefixes__is_pool__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_pool__source__id {
            vars.insert("ip_prefixes__is_pool__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_pool__owner__id {
            vars.insert("ip_prefixes__is_pool__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__is_pool__is_protected {
            vars.insert("ip_prefixes__is_pool__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__prefix__value {
            vars.insert("ip_prefixes__prefix__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__prefix__values {
            vars.insert("ip_prefixes__prefix__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__prefix__source__id {
            vars.insert("ip_prefixes__prefix__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__prefix__owner__id {
            vars.insert("ip_prefixes__prefix__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__prefix__is_protected {
            vars.insert("ip_prefixes__prefix__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__description__value {
            vars.insert("ip_prefixes__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__description__values {
            vars.insert("ip_prefixes__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__description__source__id {
            vars.insert("ip_prefixes__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__description__owner__id {
            vars.insert("ip_prefixes__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_prefixes__description__is_protected {
            vars.insert("ip_prefixes__description__is_protected".to_string(), serde_json::to_value(value)?);
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
        let query = r#"query IpamNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $default__value: Boolean, $default__values: [Boolean], $default__isnull: Boolean, $default__source__id: ID, $default__owner__id: ID, $default__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean) { IpamNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, default__value: $default__value, default__values: $default__values, default__isnull: $default__isnull, default__source__id: $default__source__id, default__owner__id: $default__owner__id, default__is_protected: $default__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected) { count edges { node { id hfid display_label description { is_default is_inherited is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_inherited is_protected updated_at id is_from_profile permissions { __typename } value } default { is_default is_inherited is_protected updated_at id is_from_profile permissions { __typename } value } profiles { count edges { __typename } } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } ip_addresses { count edges { __typename } } ip_prefixes { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
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
        let query = r#"query IpamNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $default__value: Boolean, $default__values: [Boolean], $default__isnull: Boolean, $default__source__id: ID, $default__owner__id: ID, $default__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean) { IpamNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, default__value: $default__value, default__values: $default__values, default__isnull: $default__isnull, default__source__id: $default__source__id, default__owner__id: $default__owner__id, default__is_protected: $default__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected) { count edges { node { id hfid display_label description { is_default is_inherited is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_inherited is_protected updated_at id is_from_profile permissions { __typename } value } default { is_default is_inherited is_protected updated_at id is_from_profile permissions { __typename } value } profiles { count edges { __typename } } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } ip_addresses { count edges { __typename } } ip_prefixes { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
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

