//! generated api module

#![allow(non_snake_case, unused_imports, unused_assignments, clippy::field_reassign_with_default)]

use infrahub::{BoxExtract, BoxFetch, BoxFutureResult, Client, DynPaginator, EdgePage, Error, Result};
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
    pub display_label__value: Option<String>,
    pub display_label__values: Option<Vec<String>>,
    pub display_label__isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name__value: Option<String>,
    pub profile_name__values: Option<Vec<String>>,
    pub profile_name__isnull: Option<bool>,
    pub profile_name__source__id: Option<String>,
    pub profile_name__owner__id: Option<String>,
    pub profile_name__is_protected: Option<bool>,
    pub profile_priority__value: Option<i64>,
    pub profile_priority__values: Option<Vec<i64>>,
    pub profile_priority__isnull: Option<bool>,
    pub profile_priority__source__id: Option<String>,
    pub profile_priority__owner__id: Option<String>,
    pub profile_priority__is_protected: Option<bool>,
    pub address__value: Option<String>,
    pub address__values: Option<Vec<String>>,
    pub address__isnull: Option<bool>,
    pub address__source__id: Option<String>,
    pub address__owner__id: Option<String>,
    pub address__is_protected: Option<bool>,
    pub description__value: Option<String>,
    pub description__values: Option<Vec<String>>,
    pub description__isnull: Option<bool>,
    pub description__source__id: Option<String>,
    pub description__owner__id: Option<String>,
    pub description__is_protected: Option<bool>,
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
    pub related_nodes__ids: Option<Vec<String>>,
    pub related_nodes__isnull: Option<bool>,
    pub related_nodes__display_label__value: Option<String>,
    pub related_nodes__display_label__values: Option<Vec<String>>,
    pub related_nodes__display_label__isnull: Option<bool>,
    pub related_nodes__address__value: Option<String>,
    pub related_nodes__address__values: Option<Vec<String>>,
    pub related_nodes__address__source__id: Option<String>,
    pub related_nodes__address__owner__id: Option<String>,
    pub related_nodes__address__is_protected: Option<bool>,
    pub related_nodes__description__value: Option<String>,
    pub related_nodes__description__values: Option<Vec<String>>,
    pub related_nodes__description__source__id: Option<String>,
    pub related_nodes__description__owner__id: Option<String>,
    pub related_nodes__description__is_protected: Option<bool>,
    pub ip_namespace__ids: Option<Vec<String>>,
    pub ip_namespace__isnull: Option<bool>,
    pub ip_namespace__display_label__value: Option<String>,
    pub ip_namespace__display_label__values: Option<Vec<String>>,
    pub ip_namespace__display_label__isnull: Option<bool>,
    pub ip_namespace__description__value: Option<String>,
    pub ip_namespace__description__values: Option<Vec<String>>,
    pub ip_namespace__description__source__id: Option<String>,
    pub ip_namespace__description__owner__id: Option<String>,
    pub ip_namespace__description__is_protected: Option<bool>,
    pub ip_namespace__name__value: Option<String>,
    pub ip_namespace__name__values: Option<Vec<String>>,
    pub ip_namespace__name__source__id: Option<String>,
    pub ip_namespace__name__owner__id: Option<String>,
    pub ip_namespace__name__is_protected: Option<bool>,
    pub member_of_groups__ids: Option<Vec<String>>,
    pub member_of_groups__isnull: Option<bool>,
    pub member_of_groups__display_label__value: Option<String>,
    pub member_of_groups__display_label__values: Option<Vec<String>>,
    pub member_of_groups__display_label__isnull: Option<bool>,
    pub member_of_groups__label__value: Option<String>,
    pub member_of_groups__label__values: Option<Vec<String>>,
    pub member_of_groups__group_type__value: Option<String>,
    pub member_of_groups__group_type__values: Option<Vec<String>>,
    pub member_of_groups__description__value: Option<String>,
    pub member_of_groups__description__values: Option<Vec<String>>,
    pub member_of_groups__name__value: Option<String>,
    pub member_of_groups__name__values: Option<Vec<String>>,
    pub subscriber_of_groups__ids: Option<Vec<String>>,
    pub subscriber_of_groups__isnull: Option<bool>,
    pub subscriber_of_groups__display_label__value: Option<String>,
    pub subscriber_of_groups__display_label__values: Option<Vec<String>>,
    pub subscriber_of_groups__display_label__isnull: Option<bool>,
    pub subscriber_of_groups__label__value: Option<String>,
    pub subscriber_of_groups__label__values: Option<Vec<String>>,
    pub subscriber_of_groups__group_type__value: Option<String>,
    pub subscriber_of_groups__group_type__values: Option<Vec<String>>,
    pub subscriber_of_groups__description__value: Option<String>,
    pub subscriber_of_groups__description__values: Option<Vec<String>>,
    pub subscriber_of_groups__name__value: Option<String>,
    pub subscriber_of_groups__name__values: Option<Vec<String>>,
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
        if let Some(value) = &self.profile_name__value {
            vars.insert("profile_name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__values {
            vars.insert("profile_name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__isnull {
            vars.insert("profile_name__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__source__id {
            vars.insert("profile_name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__owner__id {
            vars.insert("profile_name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__is_protected {
            vars.insert("profile_name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__value {
            vars.insert("profile_priority__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__values {
            vars.insert("profile_priority__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__isnull {
            vars.insert("profile_priority__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__source__id {
            vars.insert("profile_priority__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__owner__id {
            vars.insert("profile_priority__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__is_protected {
            vars.insert("profile_priority__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address__value {
            vars.insert("address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address__values {
            vars.insert("address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address__isnull {
            vars.insert("address__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address__source__id {
            vars.insert("address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address__owner__id {
            vars.insert("address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.address__is_protected {
            vars.insert("address__is_protected".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.related_nodes__ids {
            vars.insert("related_nodes__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__isnull {
            vars.insert("related_nodes__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__value {
            vars.insert("related_nodes__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__values {
            vars.insert("related_nodes__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__isnull {
            vars.insert("related_nodes__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__address__value {
            vars.insert("related_nodes__address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__address__values {
            vars.insert("related_nodes__address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__address__source__id {
            vars.insert("related_nodes__address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__address__owner__id {
            vars.insert("related_nodes__address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__address__is_protected {
            vars.insert("related_nodes__address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__value {
            vars.insert("related_nodes__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__values {
            vars.insert("related_nodes__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__source__id {
            vars.insert("related_nodes__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__owner__id {
            vars.insert("related_nodes__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__is_protected {
            vars.insert("related_nodes__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__ids {
            vars.insert("ip_namespace__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__isnull {
            vars.insert("ip_namespace__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__display_label__value {
            vars.insert("ip_namespace__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__display_label__values {
            vars.insert("ip_namespace__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__display_label__isnull {
            vars.insert("ip_namespace__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__value {
            vars.insert("ip_namespace__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__values {
            vars.insert("ip_namespace__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__source__id {
            vars.insert("ip_namespace__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__owner__id {
            vars.insert("ip_namespace__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__is_protected {
            vars.insert("ip_namespace__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__value {
            vars.insert("ip_namespace__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__values {
            vars.insert("ip_namespace__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__source__id {
            vars.insert("ip_namespace__name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__owner__id {
            vars.insert("ip_namespace__name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__is_protected {
            vars.insert("ip_namespace__name__is_protected".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.member_of_groups__label__value {
            vars.insert("member_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__label__values {
            vars.insert("member_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__value {
            vars.insert("member_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__values {
            vars.insert("member_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__value {
            vars.insert("member_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__values {
            vars.insert("member_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__value {
            vars.insert("member_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__values {
            vars.insert("member_of_groups__name__values".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.subscriber_of_groups__label__value {
            vars.insert("subscriber_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__label__values {
            vars.insert("subscriber_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__value {
            vars.insert("subscriber_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__values {
            vars.insert("subscriber_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__value {
            vars.insert("subscriber_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__values {
            vars.insert("subscriber_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__value {
            vars.insert("subscriber_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__values {
            vars.insert("subscriber_of_groups__name__values".to_string(), serde_json::to_value(value)?);
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

    pub async fn list(&self, filters: Option<ProfileBuiltinIPAddressFilters>, request_branch: Option<&str>) -> Result<Vec<ProfileBuiltinIPAddress>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileBuiltinIPAddress($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $address__value: String, $address__values: [String], $address__isnull: Boolean, $address__source__id: ID, $address__owner__id: ID, $address__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__address__value: String, $related_nodes__address__values: [String], $related_nodes__address__source__id: ID, $related_nodes__address__owner__id: ID, $related_nodes__address__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String]) { ProfileBuiltinIPAddress(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, address__value: $address__value, address__values: $address__values, address__isnull: $address__isnull, address__source__id: $address__source__id, address__owner__id: $address__owner__id, address__is_protected: $address__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__address__value: $related_nodes__address__value, related_nodes__address__values: $related_nodes__address__values, related_nodes__address__source__id: $related_nodes__address__source__id, related_nodes__address__owner__id: $related_nodes__address__owner__id, related_nodes__address__is_protected: $related_nodes__address__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } address { is_default is_protected updated_at id is_from_profile permissions { __typename } value ip hostmask netmask prefixlen version with_hostmask with_netmask } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } } ip_namespace { node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<ProfileBuiltinIPAddressResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_builtin_i_p_address.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<ProfileBuiltinIPAddressFilters>, request_branch: Option<&str>) -> DynPaginator<'a, ProfileBuiltinIPAddress, String, (ProfileBuiltinIPAddressResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileBuiltinIPAddress($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $address__value: String, $address__values: [String], $address__isnull: Boolean, $address__source__id: ID, $address__owner__id: ID, $address__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__address__value: String, $related_nodes__address__values: [String], $related_nodes__address__source__id: ID, $related_nodes__address__owner__id: ID, $related_nodes__address__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String]) { ProfileBuiltinIPAddress(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, address__value: $address__value, address__values: $address__values, address__isnull: $address__isnull, address__source__id: $address__source__id, address__owner__id: $address__owner__id, address__is_protected: $address__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__address__value: $related_nodes__address__value, related_nodes__address__values: $related_nodes__address__values, related_nodes__address__source__id: $related_nodes__address__source__id, related_nodes__address__owner__id: $related_nodes__address__owner__id, related_nodes__address__is_protected: $related_nodes__address__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } address { is_default is_protected updated_at id is_from_profile permissions { __typename } value ip hostmask netmask prefixlen version with_hostmask with_netmask } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } } ip_namespace { node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
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
            for edge in data.profile_builtin_i_p_address.edges {
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

    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<ProfileBuiltinIPAddress>> {
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
    pub display_label__value: Option<String>,
    pub display_label__values: Option<Vec<String>>,
    pub display_label__isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name__value: Option<String>,
    pub profile_name__values: Option<Vec<String>>,
    pub profile_name__isnull: Option<bool>,
    pub profile_name__source__id: Option<String>,
    pub profile_name__owner__id: Option<String>,
    pub profile_name__is_protected: Option<bool>,
    pub profile_priority__value: Option<i64>,
    pub profile_priority__values: Option<Vec<i64>>,
    pub profile_priority__isnull: Option<bool>,
    pub profile_priority__source__id: Option<String>,
    pub profile_priority__owner__id: Option<String>,
    pub profile_priority__is_protected: Option<bool>,
    pub is_pool__value: Option<bool>,
    pub is_pool__values: Option<Vec<bool>>,
    pub is_pool__isnull: Option<bool>,
    pub is_pool__source__id: Option<String>,
    pub is_pool__owner__id: Option<String>,
    pub is_pool__is_protected: Option<bool>,
    pub member_type__value: Option<String>,
    pub member_type__values: Option<Vec<String>>,
    pub member_type__isnull: Option<bool>,
    pub member_type__source__id: Option<String>,
    pub member_type__owner__id: Option<String>,
    pub member_type__is_protected: Option<bool>,
    pub prefix__value: Option<String>,
    pub prefix__values: Option<Vec<String>>,
    pub prefix__isnull: Option<bool>,
    pub prefix__source__id: Option<String>,
    pub prefix__owner__id: Option<String>,
    pub prefix__is_protected: Option<bool>,
    pub description__value: Option<String>,
    pub description__values: Option<Vec<String>>,
    pub description__isnull: Option<bool>,
    pub description__source__id: Option<String>,
    pub description__owner__id: Option<String>,
    pub description__is_protected: Option<bool>,
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
    pub related_nodes__ids: Option<Vec<String>>,
    pub related_nodes__isnull: Option<bool>,
    pub related_nodes__display_label__value: Option<String>,
    pub related_nodes__display_label__values: Option<Vec<String>>,
    pub related_nodes__display_label__isnull: Option<bool>,
    pub related_nodes__netmask__value: Option<String>,
    pub related_nodes__netmask__values: Option<Vec<String>>,
    pub related_nodes__netmask__source__id: Option<String>,
    pub related_nodes__netmask__owner__id: Option<String>,
    pub related_nodes__netmask__is_protected: Option<bool>,
    pub related_nodes__hostmask__value: Option<String>,
    pub related_nodes__hostmask__values: Option<Vec<String>>,
    pub related_nodes__hostmask__source__id: Option<String>,
    pub related_nodes__hostmask__owner__id: Option<String>,
    pub related_nodes__hostmask__is_protected: Option<bool>,
    pub related_nodes__is_top_level__value: Option<bool>,
    pub related_nodes__is_top_level__values: Option<Vec<bool>>,
    pub related_nodes__is_top_level__source__id: Option<String>,
    pub related_nodes__is_top_level__owner__id: Option<String>,
    pub related_nodes__is_top_level__is_protected: Option<bool>,
    pub related_nodes__utilization__value: Option<i64>,
    pub related_nodes__utilization__values: Option<Vec<i64>>,
    pub related_nodes__utilization__source__id: Option<String>,
    pub related_nodes__utilization__owner__id: Option<String>,
    pub related_nodes__utilization__is_protected: Option<bool>,
    pub related_nodes__is_pool__value: Option<bool>,
    pub related_nodes__is_pool__values: Option<Vec<bool>>,
    pub related_nodes__is_pool__source__id: Option<String>,
    pub related_nodes__is_pool__owner__id: Option<String>,
    pub related_nodes__is_pool__is_protected: Option<bool>,
    pub related_nodes__broadcast_address__value: Option<String>,
    pub related_nodes__broadcast_address__values: Option<Vec<String>>,
    pub related_nodes__broadcast_address__source__id: Option<String>,
    pub related_nodes__broadcast_address__owner__id: Option<String>,
    pub related_nodes__broadcast_address__is_protected: Option<bool>,
    pub related_nodes__member_type__value: Option<String>,
    pub related_nodes__member_type__values: Option<Vec<String>>,
    pub related_nodes__member_type__source__id: Option<String>,
    pub related_nodes__member_type__owner__id: Option<String>,
    pub related_nodes__member_type__is_protected: Option<bool>,
    pub related_nodes__network_address__value: Option<String>,
    pub related_nodes__network_address__values: Option<Vec<String>>,
    pub related_nodes__network_address__source__id: Option<String>,
    pub related_nodes__network_address__owner__id: Option<String>,
    pub related_nodes__network_address__is_protected: Option<bool>,
    pub related_nodes__prefix__value: Option<String>,
    pub related_nodes__prefix__values: Option<Vec<String>>,
    pub related_nodes__prefix__source__id: Option<String>,
    pub related_nodes__prefix__owner__id: Option<String>,
    pub related_nodes__prefix__is_protected: Option<bool>,
    pub related_nodes__description__value: Option<String>,
    pub related_nodes__description__values: Option<Vec<String>>,
    pub related_nodes__description__source__id: Option<String>,
    pub related_nodes__description__owner__id: Option<String>,
    pub related_nodes__description__is_protected: Option<bool>,
    pub ip_namespace__ids: Option<Vec<String>>,
    pub ip_namespace__isnull: Option<bool>,
    pub ip_namespace__display_label__value: Option<String>,
    pub ip_namespace__display_label__values: Option<Vec<String>>,
    pub ip_namespace__display_label__isnull: Option<bool>,
    pub ip_namespace__description__value: Option<String>,
    pub ip_namespace__description__values: Option<Vec<String>>,
    pub ip_namespace__description__source__id: Option<String>,
    pub ip_namespace__description__owner__id: Option<String>,
    pub ip_namespace__description__is_protected: Option<bool>,
    pub ip_namespace__name__value: Option<String>,
    pub ip_namespace__name__values: Option<Vec<String>>,
    pub ip_namespace__name__source__id: Option<String>,
    pub ip_namespace__name__owner__id: Option<String>,
    pub ip_namespace__name__is_protected: Option<bool>,
    pub member_of_groups__ids: Option<Vec<String>>,
    pub member_of_groups__isnull: Option<bool>,
    pub member_of_groups__display_label__value: Option<String>,
    pub member_of_groups__display_label__values: Option<Vec<String>>,
    pub member_of_groups__display_label__isnull: Option<bool>,
    pub member_of_groups__label__value: Option<String>,
    pub member_of_groups__label__values: Option<Vec<String>>,
    pub member_of_groups__group_type__value: Option<String>,
    pub member_of_groups__group_type__values: Option<Vec<String>>,
    pub member_of_groups__description__value: Option<String>,
    pub member_of_groups__description__values: Option<Vec<String>>,
    pub member_of_groups__name__value: Option<String>,
    pub member_of_groups__name__values: Option<Vec<String>>,
    pub subscriber_of_groups__ids: Option<Vec<String>>,
    pub subscriber_of_groups__isnull: Option<bool>,
    pub subscriber_of_groups__display_label__value: Option<String>,
    pub subscriber_of_groups__display_label__values: Option<Vec<String>>,
    pub subscriber_of_groups__display_label__isnull: Option<bool>,
    pub subscriber_of_groups__label__value: Option<String>,
    pub subscriber_of_groups__label__values: Option<Vec<String>>,
    pub subscriber_of_groups__group_type__value: Option<String>,
    pub subscriber_of_groups__group_type__values: Option<Vec<String>>,
    pub subscriber_of_groups__description__value: Option<String>,
    pub subscriber_of_groups__description__values: Option<Vec<String>>,
    pub subscriber_of_groups__name__value: Option<String>,
    pub subscriber_of_groups__name__values: Option<Vec<String>>,
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
        if let Some(value) = &self.profile_name__value {
            vars.insert("profile_name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__values {
            vars.insert("profile_name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__isnull {
            vars.insert("profile_name__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__source__id {
            vars.insert("profile_name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__owner__id {
            vars.insert("profile_name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__is_protected {
            vars.insert("profile_name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__value {
            vars.insert("profile_priority__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__values {
            vars.insert("profile_priority__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__isnull {
            vars.insert("profile_priority__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__source__id {
            vars.insert("profile_priority__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__owner__id {
            vars.insert("profile_priority__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__is_protected {
            vars.insert("profile_priority__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool__value {
            vars.insert("is_pool__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool__values {
            vars.insert("is_pool__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool__isnull {
            vars.insert("is_pool__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool__source__id {
            vars.insert("is_pool__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool__owner__id {
            vars.insert("is_pool__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.is_pool__is_protected {
            vars.insert("is_pool__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type__value {
            vars.insert("member_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type__values {
            vars.insert("member_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type__isnull {
            vars.insert("member_type__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type__source__id {
            vars.insert("member_type__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type__owner__id {
            vars.insert("member_type__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_type__is_protected {
            vars.insert("member_type__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix__value {
            vars.insert("prefix__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix__values {
            vars.insert("prefix__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix__isnull {
            vars.insert("prefix__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix__source__id {
            vars.insert("prefix__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix__owner__id {
            vars.insert("prefix__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.prefix__is_protected {
            vars.insert("prefix__is_protected".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.related_nodes__ids {
            vars.insert("related_nodes__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__isnull {
            vars.insert("related_nodes__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__value {
            vars.insert("related_nodes__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__values {
            vars.insert("related_nodes__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__isnull {
            vars.insert("related_nodes__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__netmask__value {
            vars.insert("related_nodes__netmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__netmask__values {
            vars.insert("related_nodes__netmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__netmask__source__id {
            vars.insert("related_nodes__netmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__netmask__owner__id {
            vars.insert("related_nodes__netmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__netmask__is_protected {
            vars.insert("related_nodes__netmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__hostmask__value {
            vars.insert("related_nodes__hostmask__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__hostmask__values {
            vars.insert("related_nodes__hostmask__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__hostmask__source__id {
            vars.insert("related_nodes__hostmask__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__hostmask__owner__id {
            vars.insert("related_nodes__hostmask__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__hostmask__is_protected {
            vars.insert("related_nodes__hostmask__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_top_level__value {
            vars.insert("related_nodes__is_top_level__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_top_level__values {
            vars.insert("related_nodes__is_top_level__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_top_level__source__id {
            vars.insert("related_nodes__is_top_level__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_top_level__owner__id {
            vars.insert("related_nodes__is_top_level__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_top_level__is_protected {
            vars.insert("related_nodes__is_top_level__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__utilization__value {
            vars.insert("related_nodes__utilization__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__utilization__values {
            vars.insert("related_nodes__utilization__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__utilization__source__id {
            vars.insert("related_nodes__utilization__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__utilization__owner__id {
            vars.insert("related_nodes__utilization__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__utilization__is_protected {
            vars.insert("related_nodes__utilization__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_pool__value {
            vars.insert("related_nodes__is_pool__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_pool__values {
            vars.insert("related_nodes__is_pool__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_pool__source__id {
            vars.insert("related_nodes__is_pool__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_pool__owner__id {
            vars.insert("related_nodes__is_pool__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__is_pool__is_protected {
            vars.insert("related_nodes__is_pool__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__broadcast_address__value {
            vars.insert("related_nodes__broadcast_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__broadcast_address__values {
            vars.insert("related_nodes__broadcast_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__broadcast_address__source__id {
            vars.insert("related_nodes__broadcast_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__broadcast_address__owner__id {
            vars.insert("related_nodes__broadcast_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__broadcast_address__is_protected {
            vars.insert("related_nodes__broadcast_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__member_type__value {
            vars.insert("related_nodes__member_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__member_type__values {
            vars.insert("related_nodes__member_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__member_type__source__id {
            vars.insert("related_nodes__member_type__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__member_type__owner__id {
            vars.insert("related_nodes__member_type__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__member_type__is_protected {
            vars.insert("related_nodes__member_type__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__network_address__value {
            vars.insert("related_nodes__network_address__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__network_address__values {
            vars.insert("related_nodes__network_address__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__network_address__source__id {
            vars.insert("related_nodes__network_address__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__network_address__owner__id {
            vars.insert("related_nodes__network_address__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__network_address__is_protected {
            vars.insert("related_nodes__network_address__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__prefix__value {
            vars.insert("related_nodes__prefix__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__prefix__values {
            vars.insert("related_nodes__prefix__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__prefix__source__id {
            vars.insert("related_nodes__prefix__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__prefix__owner__id {
            vars.insert("related_nodes__prefix__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__prefix__is_protected {
            vars.insert("related_nodes__prefix__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__value {
            vars.insert("related_nodes__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__values {
            vars.insert("related_nodes__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__source__id {
            vars.insert("related_nodes__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__owner__id {
            vars.insert("related_nodes__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__is_protected {
            vars.insert("related_nodes__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__ids {
            vars.insert("ip_namespace__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__isnull {
            vars.insert("ip_namespace__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__display_label__value {
            vars.insert("ip_namespace__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__display_label__values {
            vars.insert("ip_namespace__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__display_label__isnull {
            vars.insert("ip_namespace__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__value {
            vars.insert("ip_namespace__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__values {
            vars.insert("ip_namespace__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__source__id {
            vars.insert("ip_namespace__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__owner__id {
            vars.insert("ip_namespace__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__description__is_protected {
            vars.insert("ip_namespace__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__value {
            vars.insert("ip_namespace__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__values {
            vars.insert("ip_namespace__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__source__id {
            vars.insert("ip_namespace__name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__owner__id {
            vars.insert("ip_namespace__name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.ip_namespace__name__is_protected {
            vars.insert("ip_namespace__name__is_protected".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.member_of_groups__label__value {
            vars.insert("member_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__label__values {
            vars.insert("member_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__value {
            vars.insert("member_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__values {
            vars.insert("member_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__value {
            vars.insert("member_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__values {
            vars.insert("member_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__value {
            vars.insert("member_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__values {
            vars.insert("member_of_groups__name__values".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.subscriber_of_groups__label__value {
            vars.insert("subscriber_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__label__values {
            vars.insert("subscriber_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__value {
            vars.insert("subscriber_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__values {
            vars.insert("subscriber_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__value {
            vars.insert("subscriber_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__values {
            vars.insert("subscriber_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__value {
            vars.insert("subscriber_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__values {
            vars.insert("subscriber_of_groups__name__values".to_string(), serde_json::to_value(value)?);
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

    pub async fn list(&self, filters: Option<ProfileBuiltinIPPrefixFilters>, request_branch: Option<&str>) -> Result<Vec<ProfileBuiltinIPPrefix>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileBuiltinIPPrefix($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $is_pool__value: Boolean, $is_pool__values: [Boolean], $is_pool__isnull: Boolean, $is_pool__source__id: ID, $is_pool__owner__id: ID, $is_pool__is_protected: Boolean, $member_type__value: String, $member_type__values: [String], $member_type__isnull: Boolean, $member_type__source__id: ID, $member_type__owner__id: ID, $member_type__is_protected: Boolean, $prefix__value: String, $prefix__values: [String], $prefix__isnull: Boolean, $prefix__source__id: ID, $prefix__owner__id: ID, $prefix__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__netmask__value: String, $related_nodes__netmask__values: [String], $related_nodes__netmask__source__id: ID, $related_nodes__netmask__owner__id: ID, $related_nodes__netmask__is_protected: Boolean, $related_nodes__hostmask__value: String, $related_nodes__hostmask__values: [String], $related_nodes__hostmask__source__id: ID, $related_nodes__hostmask__owner__id: ID, $related_nodes__hostmask__is_protected: Boolean, $related_nodes__is_top_level__value: Boolean, $related_nodes__is_top_level__values: [Boolean], $related_nodes__is_top_level__source__id: ID, $related_nodes__is_top_level__owner__id: ID, $related_nodes__is_top_level__is_protected: Boolean, $related_nodes__utilization__value: BigInt, $related_nodes__utilization__values: [BigInt], $related_nodes__utilization__source__id: ID, $related_nodes__utilization__owner__id: ID, $related_nodes__utilization__is_protected: Boolean, $related_nodes__is_pool__value: Boolean, $related_nodes__is_pool__values: [Boolean], $related_nodes__is_pool__source__id: ID, $related_nodes__is_pool__owner__id: ID, $related_nodes__is_pool__is_protected: Boolean, $related_nodes__broadcast_address__value: String, $related_nodes__broadcast_address__values: [String], $related_nodes__broadcast_address__source__id: ID, $related_nodes__broadcast_address__owner__id: ID, $related_nodes__broadcast_address__is_protected: Boolean, $related_nodes__member_type__value: String, $related_nodes__member_type__values: [String], $related_nodes__member_type__source__id: ID, $related_nodes__member_type__owner__id: ID, $related_nodes__member_type__is_protected: Boolean, $related_nodes__network_address__value: String, $related_nodes__network_address__values: [String], $related_nodes__network_address__source__id: ID, $related_nodes__network_address__owner__id: ID, $related_nodes__network_address__is_protected: Boolean, $related_nodes__prefix__value: String, $related_nodes__prefix__values: [String], $related_nodes__prefix__source__id: ID, $related_nodes__prefix__owner__id: ID, $related_nodes__prefix__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String]) { ProfileBuiltinIPPrefix(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, is_pool__value: $is_pool__value, is_pool__values: $is_pool__values, is_pool__isnull: $is_pool__isnull, is_pool__source__id: $is_pool__source__id, is_pool__owner__id: $is_pool__owner__id, is_pool__is_protected: $is_pool__is_protected, member_type__value: $member_type__value, member_type__values: $member_type__values, member_type__isnull: $member_type__isnull, member_type__source__id: $member_type__source__id, member_type__owner__id: $member_type__owner__id, member_type__is_protected: $member_type__is_protected, prefix__value: $prefix__value, prefix__values: $prefix__values, prefix__isnull: $prefix__isnull, prefix__source__id: $prefix__source__id, prefix__owner__id: $prefix__owner__id, prefix__is_protected: $prefix__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__netmask__value: $related_nodes__netmask__value, related_nodes__netmask__values: $related_nodes__netmask__values, related_nodes__netmask__source__id: $related_nodes__netmask__source__id, related_nodes__netmask__owner__id: $related_nodes__netmask__owner__id, related_nodes__netmask__is_protected: $related_nodes__netmask__is_protected, related_nodes__hostmask__value: $related_nodes__hostmask__value, related_nodes__hostmask__values: $related_nodes__hostmask__values, related_nodes__hostmask__source__id: $related_nodes__hostmask__source__id, related_nodes__hostmask__owner__id: $related_nodes__hostmask__owner__id, related_nodes__hostmask__is_protected: $related_nodes__hostmask__is_protected, related_nodes__is_top_level__value: $related_nodes__is_top_level__value, related_nodes__is_top_level__values: $related_nodes__is_top_level__values, related_nodes__is_top_level__source__id: $related_nodes__is_top_level__source__id, related_nodes__is_top_level__owner__id: $related_nodes__is_top_level__owner__id, related_nodes__is_top_level__is_protected: $related_nodes__is_top_level__is_protected, related_nodes__utilization__value: $related_nodes__utilization__value, related_nodes__utilization__values: $related_nodes__utilization__values, related_nodes__utilization__source__id: $related_nodes__utilization__source__id, related_nodes__utilization__owner__id: $related_nodes__utilization__owner__id, related_nodes__utilization__is_protected: $related_nodes__utilization__is_protected, related_nodes__is_pool__value: $related_nodes__is_pool__value, related_nodes__is_pool__values: $related_nodes__is_pool__values, related_nodes__is_pool__source__id: $related_nodes__is_pool__source__id, related_nodes__is_pool__owner__id: $related_nodes__is_pool__owner__id, related_nodes__is_pool__is_protected: $related_nodes__is_pool__is_protected, related_nodes__broadcast_address__value: $related_nodes__broadcast_address__value, related_nodes__broadcast_address__values: $related_nodes__broadcast_address__values, related_nodes__broadcast_address__source__id: $related_nodes__broadcast_address__source__id, related_nodes__broadcast_address__owner__id: $related_nodes__broadcast_address__owner__id, related_nodes__broadcast_address__is_protected: $related_nodes__broadcast_address__is_protected, related_nodes__member_type__value: $related_nodes__member_type__value, related_nodes__member_type__values: $related_nodes__member_type__values, related_nodes__member_type__source__id: $related_nodes__member_type__source__id, related_nodes__member_type__owner__id: $related_nodes__member_type__owner__id, related_nodes__member_type__is_protected: $related_nodes__member_type__is_protected, related_nodes__network_address__value: $related_nodes__network_address__value, related_nodes__network_address__values: $related_nodes__network_address__values, related_nodes__network_address__source__id: $related_nodes__network_address__source__id, related_nodes__network_address__owner__id: $related_nodes__network_address__owner__id, related_nodes__network_address__is_protected: $related_nodes__network_address__is_protected, related_nodes__prefix__value: $related_nodes__prefix__value, related_nodes__prefix__values: $related_nodes__prefix__values, related_nodes__prefix__source__id: $related_nodes__prefix__source__id, related_nodes__prefix__owner__id: $related_nodes__prefix__owner__id, related_nodes__prefix__is_protected: $related_nodes__prefix__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } is_pool { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_type { is_default is_protected updated_at value label color description id is_from_profile permissions { __typename } } prefix { is_default is_protected updated_at id is_from_profile permissions { __typename } value broadcast_address hostmask netmask prefixlen num_addresses version with_hostmask with_netmask } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } } ip_namespace { node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<ProfileBuiltinIPPrefixResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_builtin_i_p_prefix.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<ProfileBuiltinIPPrefixFilters>, request_branch: Option<&str>) -> DynPaginator<'a, ProfileBuiltinIPPrefix, String, (ProfileBuiltinIPPrefixResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileBuiltinIPPrefix($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $is_pool__value: Boolean, $is_pool__values: [Boolean], $is_pool__isnull: Boolean, $is_pool__source__id: ID, $is_pool__owner__id: ID, $is_pool__is_protected: Boolean, $member_type__value: String, $member_type__values: [String], $member_type__isnull: Boolean, $member_type__source__id: ID, $member_type__owner__id: ID, $member_type__is_protected: Boolean, $prefix__value: String, $prefix__values: [String], $prefix__isnull: Boolean, $prefix__source__id: ID, $prefix__owner__id: ID, $prefix__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__netmask__value: String, $related_nodes__netmask__values: [String], $related_nodes__netmask__source__id: ID, $related_nodes__netmask__owner__id: ID, $related_nodes__netmask__is_protected: Boolean, $related_nodes__hostmask__value: String, $related_nodes__hostmask__values: [String], $related_nodes__hostmask__source__id: ID, $related_nodes__hostmask__owner__id: ID, $related_nodes__hostmask__is_protected: Boolean, $related_nodes__is_top_level__value: Boolean, $related_nodes__is_top_level__values: [Boolean], $related_nodes__is_top_level__source__id: ID, $related_nodes__is_top_level__owner__id: ID, $related_nodes__is_top_level__is_protected: Boolean, $related_nodes__utilization__value: BigInt, $related_nodes__utilization__values: [BigInt], $related_nodes__utilization__source__id: ID, $related_nodes__utilization__owner__id: ID, $related_nodes__utilization__is_protected: Boolean, $related_nodes__is_pool__value: Boolean, $related_nodes__is_pool__values: [Boolean], $related_nodes__is_pool__source__id: ID, $related_nodes__is_pool__owner__id: ID, $related_nodes__is_pool__is_protected: Boolean, $related_nodes__broadcast_address__value: String, $related_nodes__broadcast_address__values: [String], $related_nodes__broadcast_address__source__id: ID, $related_nodes__broadcast_address__owner__id: ID, $related_nodes__broadcast_address__is_protected: Boolean, $related_nodes__member_type__value: String, $related_nodes__member_type__values: [String], $related_nodes__member_type__source__id: ID, $related_nodes__member_type__owner__id: ID, $related_nodes__member_type__is_protected: Boolean, $related_nodes__network_address__value: String, $related_nodes__network_address__values: [String], $related_nodes__network_address__source__id: ID, $related_nodes__network_address__owner__id: ID, $related_nodes__network_address__is_protected: Boolean, $related_nodes__prefix__value: String, $related_nodes__prefix__values: [String], $related_nodes__prefix__source__id: ID, $related_nodes__prefix__owner__id: ID, $related_nodes__prefix__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $ip_namespace__ids: [ID], $ip_namespace__isnull: Boolean, $ip_namespace__display_label__value: String, $ip_namespace__display_label__values: [String], $ip_namespace__display_label__isnull: Boolean, $ip_namespace__description__value: String, $ip_namespace__description__values: [String], $ip_namespace__description__source__id: ID, $ip_namespace__description__owner__id: ID, $ip_namespace__description__is_protected: Boolean, $ip_namespace__name__value: String, $ip_namespace__name__values: [String], $ip_namespace__name__source__id: ID, $ip_namespace__name__owner__id: ID, $ip_namespace__name__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String]) { ProfileBuiltinIPPrefix(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, is_pool__value: $is_pool__value, is_pool__values: $is_pool__values, is_pool__isnull: $is_pool__isnull, is_pool__source__id: $is_pool__source__id, is_pool__owner__id: $is_pool__owner__id, is_pool__is_protected: $is_pool__is_protected, member_type__value: $member_type__value, member_type__values: $member_type__values, member_type__isnull: $member_type__isnull, member_type__source__id: $member_type__source__id, member_type__owner__id: $member_type__owner__id, member_type__is_protected: $member_type__is_protected, prefix__value: $prefix__value, prefix__values: $prefix__values, prefix__isnull: $prefix__isnull, prefix__source__id: $prefix__source__id, prefix__owner__id: $prefix__owner__id, prefix__is_protected: $prefix__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__netmask__value: $related_nodes__netmask__value, related_nodes__netmask__values: $related_nodes__netmask__values, related_nodes__netmask__source__id: $related_nodes__netmask__source__id, related_nodes__netmask__owner__id: $related_nodes__netmask__owner__id, related_nodes__netmask__is_protected: $related_nodes__netmask__is_protected, related_nodes__hostmask__value: $related_nodes__hostmask__value, related_nodes__hostmask__values: $related_nodes__hostmask__values, related_nodes__hostmask__source__id: $related_nodes__hostmask__source__id, related_nodes__hostmask__owner__id: $related_nodes__hostmask__owner__id, related_nodes__hostmask__is_protected: $related_nodes__hostmask__is_protected, related_nodes__is_top_level__value: $related_nodes__is_top_level__value, related_nodes__is_top_level__values: $related_nodes__is_top_level__values, related_nodes__is_top_level__source__id: $related_nodes__is_top_level__source__id, related_nodes__is_top_level__owner__id: $related_nodes__is_top_level__owner__id, related_nodes__is_top_level__is_protected: $related_nodes__is_top_level__is_protected, related_nodes__utilization__value: $related_nodes__utilization__value, related_nodes__utilization__values: $related_nodes__utilization__values, related_nodes__utilization__source__id: $related_nodes__utilization__source__id, related_nodes__utilization__owner__id: $related_nodes__utilization__owner__id, related_nodes__utilization__is_protected: $related_nodes__utilization__is_protected, related_nodes__is_pool__value: $related_nodes__is_pool__value, related_nodes__is_pool__values: $related_nodes__is_pool__values, related_nodes__is_pool__source__id: $related_nodes__is_pool__source__id, related_nodes__is_pool__owner__id: $related_nodes__is_pool__owner__id, related_nodes__is_pool__is_protected: $related_nodes__is_pool__is_protected, related_nodes__broadcast_address__value: $related_nodes__broadcast_address__value, related_nodes__broadcast_address__values: $related_nodes__broadcast_address__values, related_nodes__broadcast_address__source__id: $related_nodes__broadcast_address__source__id, related_nodes__broadcast_address__owner__id: $related_nodes__broadcast_address__owner__id, related_nodes__broadcast_address__is_protected: $related_nodes__broadcast_address__is_protected, related_nodes__member_type__value: $related_nodes__member_type__value, related_nodes__member_type__values: $related_nodes__member_type__values, related_nodes__member_type__source__id: $related_nodes__member_type__source__id, related_nodes__member_type__owner__id: $related_nodes__member_type__owner__id, related_nodes__member_type__is_protected: $related_nodes__member_type__is_protected, related_nodes__network_address__value: $related_nodes__network_address__value, related_nodes__network_address__values: $related_nodes__network_address__values, related_nodes__network_address__source__id: $related_nodes__network_address__source__id, related_nodes__network_address__owner__id: $related_nodes__network_address__owner__id, related_nodes__network_address__is_protected: $related_nodes__network_address__is_protected, related_nodes__prefix__value: $related_nodes__prefix__value, related_nodes__prefix__values: $related_nodes__prefix__values, related_nodes__prefix__source__id: $related_nodes__prefix__source__id, related_nodes__prefix__owner__id: $related_nodes__prefix__owner__id, related_nodes__prefix__is_protected: $related_nodes__prefix__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, ip_namespace__ids: $ip_namespace__ids, ip_namespace__isnull: $ip_namespace__isnull, ip_namespace__display_label__value: $ip_namespace__display_label__value, ip_namespace__display_label__values: $ip_namespace__display_label__values, ip_namespace__display_label__isnull: $ip_namespace__display_label__isnull, ip_namespace__description__value: $ip_namespace__description__value, ip_namespace__description__values: $ip_namespace__description__values, ip_namespace__description__source__id: $ip_namespace__description__source__id, ip_namespace__description__owner__id: $ip_namespace__description__owner__id, ip_namespace__description__is_protected: $ip_namespace__description__is_protected, ip_namespace__name__value: $ip_namespace__name__value, ip_namespace__name__values: $ip_namespace__name__values, ip_namespace__name__source__id: $ip_namespace__name__source__id, ip_namespace__name__owner__id: $ip_namespace__name__owner__id, ip_namespace__name__is_protected: $ip_namespace__name__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } is_pool { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_type { is_default is_protected updated_at value label color description id is_from_profile permissions { __typename } } prefix { is_default is_protected updated_at id is_from_profile permissions { __typename } value broadcast_address hostmask netmask prefixlen num_addresses version with_hostmask with_netmask } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } } ip_namespace { node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
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
            for edge in data.profile_builtin_i_p_prefix.edges {
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

    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<ProfileBuiltinIPPrefix>> {
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
    pub display_label__value: Option<String>,
    pub display_label__values: Option<Vec<String>>,
    pub display_label__isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name__value: Option<String>,
    pub profile_name__values: Option<Vec<String>>,
    pub profile_name__isnull: Option<bool>,
    pub profile_name__source__id: Option<String>,
    pub profile_name__owner__id: Option<String>,
    pub profile_name__is_protected: Option<bool>,
    pub profile_priority__value: Option<i64>,
    pub profile_priority__values: Option<Vec<i64>>,
    pub profile_priority__isnull: Option<bool>,
    pub profile_priority__source__id: Option<String>,
    pub profile_priority__owner__id: Option<String>,
    pub profile_priority__is_protected: Option<bool>,
    pub description__value: Option<String>,
    pub description__values: Option<Vec<String>>,
    pub description__isnull: Option<bool>,
    pub description__source__id: Option<String>,
    pub description__owner__id: Option<String>,
    pub description__is_protected: Option<bool>,
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
    pub related_nodes__ids: Option<Vec<String>>,
    pub related_nodes__isnull: Option<bool>,
    pub related_nodes__display_label__value: Option<String>,
    pub related_nodes__display_label__values: Option<Vec<String>>,
    pub related_nodes__display_label__isnull: Option<bool>,
    pub related_nodes__name__value: Option<String>,
    pub related_nodes__name__values: Option<Vec<String>>,
    pub related_nodes__name__source__id: Option<String>,
    pub related_nodes__name__owner__id: Option<String>,
    pub related_nodes__name__is_protected: Option<bool>,
    pub related_nodes__description__value: Option<String>,
    pub related_nodes__description__values: Option<Vec<String>>,
    pub related_nodes__description__source__id: Option<String>,
    pub related_nodes__description__owner__id: Option<String>,
    pub related_nodes__description__is_protected: Option<bool>,
    pub member_of_groups__ids: Option<Vec<String>>,
    pub member_of_groups__isnull: Option<bool>,
    pub member_of_groups__display_label__value: Option<String>,
    pub member_of_groups__display_label__values: Option<Vec<String>>,
    pub member_of_groups__display_label__isnull: Option<bool>,
    pub member_of_groups__label__value: Option<String>,
    pub member_of_groups__label__values: Option<Vec<String>>,
    pub member_of_groups__group_type__value: Option<String>,
    pub member_of_groups__group_type__values: Option<Vec<String>>,
    pub member_of_groups__description__value: Option<String>,
    pub member_of_groups__description__values: Option<Vec<String>>,
    pub member_of_groups__name__value: Option<String>,
    pub member_of_groups__name__values: Option<Vec<String>>,
    pub subscriber_of_groups__ids: Option<Vec<String>>,
    pub subscriber_of_groups__isnull: Option<bool>,
    pub subscriber_of_groups__display_label__value: Option<String>,
    pub subscriber_of_groups__display_label__values: Option<Vec<String>>,
    pub subscriber_of_groups__display_label__isnull: Option<bool>,
    pub subscriber_of_groups__label__value: Option<String>,
    pub subscriber_of_groups__label__values: Option<Vec<String>>,
    pub subscriber_of_groups__group_type__value: Option<String>,
    pub subscriber_of_groups__group_type__values: Option<Vec<String>>,
    pub subscriber_of_groups__description__value: Option<String>,
    pub subscriber_of_groups__description__values: Option<Vec<String>>,
    pub subscriber_of_groups__name__value: Option<String>,
    pub subscriber_of_groups__name__values: Option<Vec<String>>,
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
        if let Some(value) = &self.profile_name__value {
            vars.insert("profile_name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__values {
            vars.insert("profile_name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__isnull {
            vars.insert("profile_name__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__source__id {
            vars.insert("profile_name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__owner__id {
            vars.insert("profile_name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__is_protected {
            vars.insert("profile_name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__value {
            vars.insert("profile_priority__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__values {
            vars.insert("profile_priority__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__isnull {
            vars.insert("profile_priority__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__source__id {
            vars.insert("profile_priority__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__owner__id {
            vars.insert("profile_priority__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__is_protected {
            vars.insert("profile_priority__is_protected".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.related_nodes__ids {
            vars.insert("related_nodes__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__isnull {
            vars.insert("related_nodes__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__value {
            vars.insert("related_nodes__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__values {
            vars.insert("related_nodes__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__isnull {
            vars.insert("related_nodes__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__value {
            vars.insert("related_nodes__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__values {
            vars.insert("related_nodes__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__source__id {
            vars.insert("related_nodes__name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__owner__id {
            vars.insert("related_nodes__name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__is_protected {
            vars.insert("related_nodes__name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__value {
            vars.insert("related_nodes__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__values {
            vars.insert("related_nodes__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__source__id {
            vars.insert("related_nodes__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__owner__id {
            vars.insert("related_nodes__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__is_protected {
            vars.insert("related_nodes__description__is_protected".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.member_of_groups__label__value {
            vars.insert("member_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__label__values {
            vars.insert("member_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__value {
            vars.insert("member_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__values {
            vars.insert("member_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__value {
            vars.insert("member_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__values {
            vars.insert("member_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__value {
            vars.insert("member_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__values {
            vars.insert("member_of_groups__name__values".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.subscriber_of_groups__label__value {
            vars.insert("subscriber_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__label__values {
            vars.insert("subscriber_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__value {
            vars.insert("subscriber_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__values {
            vars.insert("subscriber_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__value {
            vars.insert("subscriber_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__values {
            vars.insert("subscriber_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__value {
            vars.insert("subscriber_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__values {
            vars.insert("subscriber_of_groups__name__values".to_string(), serde_json::to_value(value)?);
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

    pub async fn list(&self, filters: Option<ProfileBuiltinTagFilters>, request_branch: Option<&str>) -> Result<Vec<ProfileBuiltinTag>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileBuiltinTag($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String]) { ProfileBuiltinTag(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<ProfileBuiltinTagResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_builtin_tag.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<ProfileBuiltinTagFilters>, request_branch: Option<&str>) -> DynPaginator<'a, ProfileBuiltinTag, String, (ProfileBuiltinTagResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileBuiltinTag($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String]) { ProfileBuiltinTag(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (ProfileBuiltinTagResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (ProfileBuiltinTagResponse, i64)> {
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
                let response = client.execute::<ProfileBuiltinTagResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
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

    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<ProfileBuiltinTag>> {
        let mut filters = ProfileBuiltinTagFilters::default();
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
    pub display_label__value: Option<String>,
    pub display_label__values: Option<Vec<String>>,
    pub display_label__isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub profile_name__value: Option<String>,
    pub profile_name__values: Option<Vec<String>>,
    pub profile_name__isnull: Option<bool>,
    pub profile_name__source__id: Option<String>,
    pub profile_name__owner__id: Option<String>,
    pub profile_name__is_protected: Option<bool>,
    pub profile_priority__value: Option<i64>,
    pub profile_priority__values: Option<Vec<i64>>,
    pub profile_priority__isnull: Option<bool>,
    pub profile_priority__source__id: Option<String>,
    pub profile_priority__owner__id: Option<String>,
    pub profile_priority__is_protected: Option<bool>,
    pub description__value: Option<String>,
    pub description__values: Option<Vec<String>>,
    pub description__isnull: Option<bool>,
    pub description__source__id: Option<String>,
    pub description__owner__id: Option<String>,
    pub description__is_protected: Option<bool>,
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
    pub related_nodes__ids: Option<Vec<String>>,
    pub related_nodes__isnull: Option<bool>,
    pub related_nodes__display_label__value: Option<String>,
    pub related_nodes__display_label__values: Option<Vec<String>>,
    pub related_nodes__display_label__isnull: Option<bool>,
    pub related_nodes__default__value: Option<bool>,
    pub related_nodes__default__values: Option<Vec<bool>>,
    pub related_nodes__default__source__id: Option<String>,
    pub related_nodes__default__owner__id: Option<String>,
    pub related_nodes__default__is_protected: Option<bool>,
    pub related_nodes__description__value: Option<String>,
    pub related_nodes__description__values: Option<Vec<String>>,
    pub related_nodes__description__source__id: Option<String>,
    pub related_nodes__description__owner__id: Option<String>,
    pub related_nodes__description__is_protected: Option<bool>,
    pub related_nodes__name__value: Option<String>,
    pub related_nodes__name__values: Option<Vec<String>>,
    pub related_nodes__name__source__id: Option<String>,
    pub related_nodes__name__owner__id: Option<String>,
    pub related_nodes__name__is_protected: Option<bool>,
    pub ip_prefixes__ids: Option<Vec<String>>,
    pub ip_prefixes__isnull: Option<bool>,
    pub ip_prefixes__display_label__value: Option<String>,
    pub ip_prefixes__display_label__values: Option<Vec<String>>,
    pub ip_prefixes__display_label__isnull: Option<bool>,
    pub ip_prefixes__netmask__value: Option<String>,
    pub ip_prefixes__netmask__values: Option<Vec<String>>,
    pub ip_prefixes__netmask__source__id: Option<String>,
    pub ip_prefixes__netmask__owner__id: Option<String>,
    pub ip_prefixes__netmask__is_protected: Option<bool>,
    pub ip_prefixes__hostmask__value: Option<String>,
    pub ip_prefixes__hostmask__values: Option<Vec<String>>,
    pub ip_prefixes__hostmask__source__id: Option<String>,
    pub ip_prefixes__hostmask__owner__id: Option<String>,
    pub ip_prefixes__hostmask__is_protected: Option<bool>,
    pub ip_prefixes__is_top_level__value: Option<bool>,
    pub ip_prefixes__is_top_level__values: Option<Vec<bool>>,
    pub ip_prefixes__is_top_level__source__id: Option<String>,
    pub ip_prefixes__is_top_level__owner__id: Option<String>,
    pub ip_prefixes__is_top_level__is_protected: Option<bool>,
    pub ip_prefixes__utilization__value: Option<i64>,
    pub ip_prefixes__utilization__values: Option<Vec<i64>>,
    pub ip_prefixes__utilization__source__id: Option<String>,
    pub ip_prefixes__utilization__owner__id: Option<String>,
    pub ip_prefixes__utilization__is_protected: Option<bool>,
    pub ip_prefixes__is_pool__value: Option<bool>,
    pub ip_prefixes__is_pool__values: Option<Vec<bool>>,
    pub ip_prefixes__is_pool__source__id: Option<String>,
    pub ip_prefixes__is_pool__owner__id: Option<String>,
    pub ip_prefixes__is_pool__is_protected: Option<bool>,
    pub ip_prefixes__broadcast_address__value: Option<String>,
    pub ip_prefixes__broadcast_address__values: Option<Vec<String>>,
    pub ip_prefixes__broadcast_address__source__id: Option<String>,
    pub ip_prefixes__broadcast_address__owner__id: Option<String>,
    pub ip_prefixes__broadcast_address__is_protected: Option<bool>,
    pub ip_prefixes__member_type__value: Option<String>,
    pub ip_prefixes__member_type__values: Option<Vec<String>>,
    pub ip_prefixes__member_type__source__id: Option<String>,
    pub ip_prefixes__member_type__owner__id: Option<String>,
    pub ip_prefixes__member_type__is_protected: Option<bool>,
    pub ip_prefixes__network_address__value: Option<String>,
    pub ip_prefixes__network_address__values: Option<Vec<String>>,
    pub ip_prefixes__network_address__source__id: Option<String>,
    pub ip_prefixes__network_address__owner__id: Option<String>,
    pub ip_prefixes__network_address__is_protected: Option<bool>,
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
    pub member_of_groups__ids: Option<Vec<String>>,
    pub member_of_groups__isnull: Option<bool>,
    pub member_of_groups__display_label__value: Option<String>,
    pub member_of_groups__display_label__values: Option<Vec<String>>,
    pub member_of_groups__display_label__isnull: Option<bool>,
    pub member_of_groups__label__value: Option<String>,
    pub member_of_groups__label__values: Option<Vec<String>>,
    pub member_of_groups__group_type__value: Option<String>,
    pub member_of_groups__group_type__values: Option<Vec<String>>,
    pub member_of_groups__description__value: Option<String>,
    pub member_of_groups__description__values: Option<Vec<String>>,
    pub member_of_groups__name__value: Option<String>,
    pub member_of_groups__name__values: Option<Vec<String>>,
    pub subscriber_of_groups__ids: Option<Vec<String>>,
    pub subscriber_of_groups__isnull: Option<bool>,
    pub subscriber_of_groups__display_label__value: Option<String>,
    pub subscriber_of_groups__display_label__values: Option<Vec<String>>,
    pub subscriber_of_groups__display_label__isnull: Option<bool>,
    pub subscriber_of_groups__label__value: Option<String>,
    pub subscriber_of_groups__label__values: Option<Vec<String>>,
    pub subscriber_of_groups__group_type__value: Option<String>,
    pub subscriber_of_groups__group_type__values: Option<Vec<String>>,
    pub subscriber_of_groups__description__value: Option<String>,
    pub subscriber_of_groups__description__values: Option<Vec<String>>,
    pub subscriber_of_groups__name__value: Option<String>,
    pub subscriber_of_groups__name__values: Option<Vec<String>>,
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
        if let Some(value) = &self.profile_name__value {
            vars.insert("profile_name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__values {
            vars.insert("profile_name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__isnull {
            vars.insert("profile_name__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__source__id {
            vars.insert("profile_name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__owner__id {
            vars.insert("profile_name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_name__is_protected {
            vars.insert("profile_name__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__value {
            vars.insert("profile_priority__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__values {
            vars.insert("profile_priority__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__isnull {
            vars.insert("profile_priority__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__source__id {
            vars.insert("profile_priority__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__owner__id {
            vars.insert("profile_priority__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profile_priority__is_protected {
            vars.insert("profile_priority__is_protected".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.related_nodes__ids {
            vars.insert("related_nodes__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__isnull {
            vars.insert("related_nodes__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__value {
            vars.insert("related_nodes__display_label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__values {
            vars.insert("related_nodes__display_label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__display_label__isnull {
            vars.insert("related_nodes__display_label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__default__value {
            vars.insert("related_nodes__default__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__default__values {
            vars.insert("related_nodes__default__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__default__source__id {
            vars.insert("related_nodes__default__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__default__owner__id {
            vars.insert("related_nodes__default__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__default__is_protected {
            vars.insert("related_nodes__default__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__value {
            vars.insert("related_nodes__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__values {
            vars.insert("related_nodes__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__source__id {
            vars.insert("related_nodes__description__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__owner__id {
            vars.insert("related_nodes__description__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__description__is_protected {
            vars.insert("related_nodes__description__is_protected".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__value {
            vars.insert("related_nodes__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__values {
            vars.insert("related_nodes__name__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__source__id {
            vars.insert("related_nodes__name__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__owner__id {
            vars.insert("related_nodes__name__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.related_nodes__name__is_protected {
            vars.insert("related_nodes__name__is_protected".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.member_of_groups__label__value {
            vars.insert("member_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__label__values {
            vars.insert("member_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__value {
            vars.insert("member_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__group_type__values {
            vars.insert("member_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__value {
            vars.insert("member_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__description__values {
            vars.insert("member_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__value {
            vars.insert("member_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.member_of_groups__name__values {
            vars.insert("member_of_groups__name__values".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &self.subscriber_of_groups__label__value {
            vars.insert("subscriber_of_groups__label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__label__values {
            vars.insert("subscriber_of_groups__label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__value {
            vars.insert("subscriber_of_groups__group_type__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__group_type__values {
            vars.insert("subscriber_of_groups__group_type__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__value {
            vars.insert("subscriber_of_groups__description__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__description__values {
            vars.insert("subscriber_of_groups__description__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__value {
            vars.insert("subscriber_of_groups__name__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.subscriber_of_groups__name__values {
            vars.insert("subscriber_of_groups__name__values".to_string(), serde_json::to_value(value)?);
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

    pub async fn list(&self, filters: Option<ProfileIpamNamespaceFilters>, request_branch: Option<&str>) -> Result<Vec<ProfileIpamNamespace>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query ProfileIpamNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__default__value: Boolean, $related_nodes__default__values: [Boolean], $related_nodes__default__source__id: ID, $related_nodes__default__owner__id: ID, $related_nodes__default__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean, $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String]) { ProfileIpamNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__default__value: $related_nodes__default__value, related_nodes__default__values: $related_nodes__default__values, related_nodes__default__source__id: $related_nodes__default__source__id, related_nodes__default__owner__id: $related_nodes__default__owner__id, related_nodes__default__is_protected: $related_nodes__default__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } ip_prefixes { count edges { __typename } } ip_addresses { count edges { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<ProfileIpamNamespaceResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.profile_ipam_namespace.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<ProfileIpamNamespaceFilters>, request_branch: Option<&str>) -> DynPaginator<'a, ProfileIpamNamespace, String, (ProfileIpamNamespaceResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query ProfileIpamNamespace($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $profile_name__value: String, $profile_name__values: [String], $profile_name__isnull: Boolean, $profile_name__source__id: ID, $profile_name__owner__id: ID, $profile_name__is_protected: Boolean, $profile_priority__value: BigInt, $profile_priority__values: [BigInt], $profile_priority__isnull: Boolean, $profile_priority__source__id: ID, $profile_priority__owner__id: ID, $profile_priority__is_protected: Boolean, $description__value: String, $description__values: [String], $description__isnull: Boolean, $description__source__id: ID, $description__owner__id: ID, $description__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $related_nodes__ids: [ID], $related_nodes__isnull: Boolean, $related_nodes__display_label__value: String, $related_nodes__display_label__values: [String], $related_nodes__display_label__isnull: Boolean, $related_nodes__default__value: Boolean, $related_nodes__default__values: [Boolean], $related_nodes__default__source__id: ID, $related_nodes__default__owner__id: ID, $related_nodes__default__is_protected: Boolean, $related_nodes__description__value: String, $related_nodes__description__values: [String], $related_nodes__description__source__id: ID, $related_nodes__description__owner__id: ID, $related_nodes__description__is_protected: Boolean, $related_nodes__name__value: String, $related_nodes__name__values: [String], $related_nodes__name__source__id: ID, $related_nodes__name__owner__id: ID, $related_nodes__name__is_protected: Boolean, $ip_prefixes__ids: [ID], $ip_prefixes__isnull: Boolean, $ip_prefixes__display_label__value: String, $ip_prefixes__display_label__values: [String], $ip_prefixes__display_label__isnull: Boolean, $ip_prefixes__netmask__value: String, $ip_prefixes__netmask__values: [String], $ip_prefixes__netmask__source__id: ID, $ip_prefixes__netmask__owner__id: ID, $ip_prefixes__netmask__is_protected: Boolean, $ip_prefixes__hostmask__value: String, $ip_prefixes__hostmask__values: [String], $ip_prefixes__hostmask__source__id: ID, $ip_prefixes__hostmask__owner__id: ID, $ip_prefixes__hostmask__is_protected: Boolean, $ip_prefixes__is_top_level__value: Boolean, $ip_prefixes__is_top_level__values: [Boolean], $ip_prefixes__is_top_level__source__id: ID, $ip_prefixes__is_top_level__owner__id: ID, $ip_prefixes__is_top_level__is_protected: Boolean, $ip_prefixes__utilization__value: BigInt, $ip_prefixes__utilization__values: [BigInt], $ip_prefixes__utilization__source__id: ID, $ip_prefixes__utilization__owner__id: ID, $ip_prefixes__utilization__is_protected: Boolean, $ip_prefixes__is_pool__value: Boolean, $ip_prefixes__is_pool__values: [Boolean], $ip_prefixes__is_pool__source__id: ID, $ip_prefixes__is_pool__owner__id: ID, $ip_prefixes__is_pool__is_protected: Boolean, $ip_prefixes__broadcast_address__value: String, $ip_prefixes__broadcast_address__values: [String], $ip_prefixes__broadcast_address__source__id: ID, $ip_prefixes__broadcast_address__owner__id: ID, $ip_prefixes__broadcast_address__is_protected: Boolean, $ip_prefixes__member_type__value: String, $ip_prefixes__member_type__values: [String], $ip_prefixes__member_type__source__id: ID, $ip_prefixes__member_type__owner__id: ID, $ip_prefixes__member_type__is_protected: Boolean, $ip_prefixes__network_address__value: String, $ip_prefixes__network_address__values: [String], $ip_prefixes__network_address__source__id: ID, $ip_prefixes__network_address__owner__id: ID, $ip_prefixes__network_address__is_protected: Boolean, $ip_prefixes__prefix__value: String, $ip_prefixes__prefix__values: [String], $ip_prefixes__prefix__source__id: ID, $ip_prefixes__prefix__owner__id: ID, $ip_prefixes__prefix__is_protected: Boolean, $ip_prefixes__description__value: String, $ip_prefixes__description__values: [String], $ip_prefixes__description__source__id: ID, $ip_prefixes__description__owner__id: ID, $ip_prefixes__description__is_protected: Boolean, $ip_addresses__ids: [ID], $ip_addresses__isnull: Boolean, $ip_addresses__display_label__value: String, $ip_addresses__display_label__values: [String], $ip_addresses__display_label__isnull: Boolean, $ip_addresses__address__value: String, $ip_addresses__address__values: [String], $ip_addresses__address__source__id: ID, $ip_addresses__address__owner__id: ID, $ip_addresses__address__is_protected: Boolean, $ip_addresses__description__value: String, $ip_addresses__description__values: [String], $ip_addresses__description__source__id: ID, $ip_addresses__description__owner__id: ID, $ip_addresses__description__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String]) { ProfileIpamNamespace(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, profile_name__value: $profile_name__value, profile_name__values: $profile_name__values, profile_name__isnull: $profile_name__isnull, profile_name__source__id: $profile_name__source__id, profile_name__owner__id: $profile_name__owner__id, profile_name__is_protected: $profile_name__is_protected, profile_priority__value: $profile_priority__value, profile_priority__values: $profile_priority__values, profile_priority__isnull: $profile_priority__isnull, profile_priority__source__id: $profile_priority__source__id, profile_priority__owner__id: $profile_priority__owner__id, profile_priority__is_protected: $profile_priority__is_protected, description__value: $description__value, description__values: $description__values, description__isnull: $description__isnull, description__source__id: $description__source__id, description__owner__id: $description__owner__id, description__is_protected: $description__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, related_nodes__ids: $related_nodes__ids, related_nodes__isnull: $related_nodes__isnull, related_nodes__display_label__value: $related_nodes__display_label__value, related_nodes__display_label__values: $related_nodes__display_label__values, related_nodes__display_label__isnull: $related_nodes__display_label__isnull, related_nodes__default__value: $related_nodes__default__value, related_nodes__default__values: $related_nodes__default__values, related_nodes__default__source__id: $related_nodes__default__source__id, related_nodes__default__owner__id: $related_nodes__default__owner__id, related_nodes__default__is_protected: $related_nodes__default__is_protected, related_nodes__description__value: $related_nodes__description__value, related_nodes__description__values: $related_nodes__description__values, related_nodes__description__source__id: $related_nodes__description__source__id, related_nodes__description__owner__id: $related_nodes__description__owner__id, related_nodes__description__is_protected: $related_nodes__description__is_protected, related_nodes__name__value: $related_nodes__name__value, related_nodes__name__values: $related_nodes__name__values, related_nodes__name__source__id: $related_nodes__name__source__id, related_nodes__name__owner__id: $related_nodes__name__owner__id, related_nodes__name__is_protected: $related_nodes__name__is_protected, ip_prefixes__ids: $ip_prefixes__ids, ip_prefixes__isnull: $ip_prefixes__isnull, ip_prefixes__display_label__value: $ip_prefixes__display_label__value, ip_prefixes__display_label__values: $ip_prefixes__display_label__values, ip_prefixes__display_label__isnull: $ip_prefixes__display_label__isnull, ip_prefixes__netmask__value: $ip_prefixes__netmask__value, ip_prefixes__netmask__values: $ip_prefixes__netmask__values, ip_prefixes__netmask__source__id: $ip_prefixes__netmask__source__id, ip_prefixes__netmask__owner__id: $ip_prefixes__netmask__owner__id, ip_prefixes__netmask__is_protected: $ip_prefixes__netmask__is_protected, ip_prefixes__hostmask__value: $ip_prefixes__hostmask__value, ip_prefixes__hostmask__values: $ip_prefixes__hostmask__values, ip_prefixes__hostmask__source__id: $ip_prefixes__hostmask__source__id, ip_prefixes__hostmask__owner__id: $ip_prefixes__hostmask__owner__id, ip_prefixes__hostmask__is_protected: $ip_prefixes__hostmask__is_protected, ip_prefixes__is_top_level__value: $ip_prefixes__is_top_level__value, ip_prefixes__is_top_level__values: $ip_prefixes__is_top_level__values, ip_prefixes__is_top_level__source__id: $ip_prefixes__is_top_level__source__id, ip_prefixes__is_top_level__owner__id: $ip_prefixes__is_top_level__owner__id, ip_prefixes__is_top_level__is_protected: $ip_prefixes__is_top_level__is_protected, ip_prefixes__utilization__value: $ip_prefixes__utilization__value, ip_prefixes__utilization__values: $ip_prefixes__utilization__values, ip_prefixes__utilization__source__id: $ip_prefixes__utilization__source__id, ip_prefixes__utilization__owner__id: $ip_prefixes__utilization__owner__id, ip_prefixes__utilization__is_protected: $ip_prefixes__utilization__is_protected, ip_prefixes__is_pool__value: $ip_prefixes__is_pool__value, ip_prefixes__is_pool__values: $ip_prefixes__is_pool__values, ip_prefixes__is_pool__source__id: $ip_prefixes__is_pool__source__id, ip_prefixes__is_pool__owner__id: $ip_prefixes__is_pool__owner__id, ip_prefixes__is_pool__is_protected: $ip_prefixes__is_pool__is_protected, ip_prefixes__broadcast_address__value: $ip_prefixes__broadcast_address__value, ip_prefixes__broadcast_address__values: $ip_prefixes__broadcast_address__values, ip_prefixes__broadcast_address__source__id: $ip_prefixes__broadcast_address__source__id, ip_prefixes__broadcast_address__owner__id: $ip_prefixes__broadcast_address__owner__id, ip_prefixes__broadcast_address__is_protected: $ip_prefixes__broadcast_address__is_protected, ip_prefixes__member_type__value: $ip_prefixes__member_type__value, ip_prefixes__member_type__values: $ip_prefixes__member_type__values, ip_prefixes__member_type__source__id: $ip_prefixes__member_type__source__id, ip_prefixes__member_type__owner__id: $ip_prefixes__member_type__owner__id, ip_prefixes__member_type__is_protected: $ip_prefixes__member_type__is_protected, ip_prefixes__network_address__value: $ip_prefixes__network_address__value, ip_prefixes__network_address__values: $ip_prefixes__network_address__values, ip_prefixes__network_address__source__id: $ip_prefixes__network_address__source__id, ip_prefixes__network_address__owner__id: $ip_prefixes__network_address__owner__id, ip_prefixes__network_address__is_protected: $ip_prefixes__network_address__is_protected, ip_prefixes__prefix__value: $ip_prefixes__prefix__value, ip_prefixes__prefix__values: $ip_prefixes__prefix__values, ip_prefixes__prefix__source__id: $ip_prefixes__prefix__source__id, ip_prefixes__prefix__owner__id: $ip_prefixes__prefix__owner__id, ip_prefixes__prefix__is_protected: $ip_prefixes__prefix__is_protected, ip_prefixes__description__value: $ip_prefixes__description__value, ip_prefixes__description__values: $ip_prefixes__description__values, ip_prefixes__description__source__id: $ip_prefixes__description__source__id, ip_prefixes__description__owner__id: $ip_prefixes__description__owner__id, ip_prefixes__description__is_protected: $ip_prefixes__description__is_protected, ip_addresses__ids: $ip_addresses__ids, ip_addresses__isnull: $ip_addresses__isnull, ip_addresses__display_label__value: $ip_addresses__display_label__value, ip_addresses__display_label__values: $ip_addresses__display_label__values, ip_addresses__display_label__isnull: $ip_addresses__display_label__isnull, ip_addresses__address__value: $ip_addresses__address__value, ip_addresses__address__values: $ip_addresses__address__values, ip_addresses__address__source__id: $ip_addresses__address__source__id, ip_addresses__address__owner__id: $ip_addresses__address__owner__id, ip_addresses__address__is_protected: $ip_addresses__address__is_protected, ip_addresses__description__value: $ip_addresses__description__value, ip_addresses__description__values: $ip_addresses__description__values, ip_addresses__description__source__id: $ip_addresses__description__source__id, ip_addresses__description__owner__id: $ip_addresses__description__owner__id, ip_addresses__description__is_protected: $ip_addresses__description__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values) { count edges { node { id hfid display_label profile_name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } profile_priority { is_default is_protected updated_at id is_from_profile permissions { __typename } value } description { is_default is_protected updated_at id is_from_profile permissions { __typename } value } related_nodes { count edges { __typename } permissions { __typename } } ip_prefixes { count edges { __typename } } ip_addresses { count edges { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
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

    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<ProfileIpamNamespace>> {
        let mut filters = ProfileIpamNamespaceFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }

}

