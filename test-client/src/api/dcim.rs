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

pub struct DcimApi<'a> {
    client: &'a Client,
}

impl<'a> DcimApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn cable(&self) -> DcimCableClient<'a> {
        DcimCableClient::new(self.client)
    }
    pub fn device(&self) -> DcimDeviceClient<'a> {
        DcimDeviceClient::new(self.client)
    }
    pub fn device_role(&self) -> DcimDeviceRoleClient<'a> {
        DcimDeviceRoleClient::new(self.client)
    }
    pub fn device_type(&self) -> DcimDeviceTypeClient<'a> {
        DcimDeviceTypeClient::new(self.client)
    }
    pub fn interface(&self) -> DcimInterfaceClient<'a> {
        DcimInterfaceClient::new(self.client)
    }
    pub fn manufacturer(&self) -> DcimManufacturerClient<'a> {
        DcimManufacturerClient::new(self.client)
    }
    pub fn platform(&self) -> DcimPlatformClient<'a> {
        DcimPlatformClient::new(self.client)
    }
    pub fn site(&self) -> DcimSiteClient<'a> {
        DcimSiteClient::new(self.client)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DcimCableFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub label_value: Option<String>,
    pub label_values: Option<Vec<String>>,
    pub label_isnull: Option<bool>,
    pub label_source_id: Option<String>,
    pub label_owner_id: Option<String>,
    pub label_is_protected: Option<bool>,
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

impl DcimCableFilters {
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
        if let Some(value) = &self.label_value {
            vars.insert("label__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.label_values {
            vars.insert("label__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.label_isnull {
            vars.insert("label__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.label_source_id {
            vars.insert(
                "label__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.label_owner_id {
            vars.insert("label__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.label_is_protected {
            vars.insert(
                "label__is_protected".to_string(),
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
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert(
                "profiles__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert(
                "profiles__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert(
                "profiles__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert(
                "profiles__profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert(
                "profiles__profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert(
                "profiles__profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert(
                "profiles__profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert(
                "profiles__profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert(
                "profiles__profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert(
                "profiles__profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert(
                "profiles__profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert(
                "profiles__profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert(
                "profiles__profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct DcimCableClient<'a> {
    client: &'a Client,
}

impl<'a> DcimCableClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<DcimCableFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<DcimCable>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query DcimCable($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $label__value: String, $label__values: [String], $label__isnull: Boolean, $label__source__id: ID, $label__owner__id: ID, $label__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $b_terminations__ids: [ID], $b_terminations__isnull: Boolean, $b_terminations__display_label__value: String, $b_terminations__display_label__values: [String], $b_terminations__display_label__isnull: Boolean, $b_terminations__if_type__value: String, $b_terminations__if_type__values: [String], $b_terminations__if_type__source__id: ID, $b_terminations__if_type__owner__id: ID, $b_terminations__if_type__is_protected: Boolean, $b_terminations__name__value: String, $b_terminations__name__values: [String], $b_terminations__name__source__id: ID, $b_terminations__name__owner__id: ID, $b_terminations__name__is_protected: Boolean, $b_terminations__enabled__value: Boolean, $b_terminations__enabled__values: [Boolean], $b_terminations__enabled__source__id: ID, $b_terminations__enabled__owner__id: ID, $b_terminations__enabled__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $a_terminations__ids: [ID], $a_terminations__isnull: Boolean, $a_terminations__display_label__value: String, $a_terminations__display_label__values: [String], $a_terminations__display_label__isnull: Boolean, $a_terminations__if_type__value: String, $a_terminations__if_type__values: [String], $a_terminations__if_type__source__id: ID, $a_terminations__if_type__owner__id: ID, $a_terminations__if_type__is_protected: Boolean, $a_terminations__name__value: String, $a_terminations__name__values: [String], $a_terminations__name__source__id: ID, $a_terminations__name__owner__id: ID, $a_terminations__name__is_protected: Boolean, $a_terminations__enabled__value: Boolean, $a_terminations__enabled__values: [Boolean], $a_terminations__enabled__source__id: ID, $a_terminations__enabled__owner__id: ID, $a_terminations__enabled__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimCable(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, label__value: $label__value, label__values: $label__values, label__isnull: $label__isnull, label__source__id: $label__source__id, label__owner__id: $label__owner__id, label__is_protected: $label__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, b_terminations__ids: $b_terminations__ids, b_terminations__isnull: $b_terminations__isnull, b_terminations__display_label__value: $b_terminations__display_label__value, b_terminations__display_label__values: $b_terminations__display_label__values, b_terminations__display_label__isnull: $b_terminations__display_label__isnull, b_terminations__if_type__value: $b_terminations__if_type__value, b_terminations__if_type__values: $b_terminations__if_type__values, b_terminations__if_type__source__id: $b_terminations__if_type__source__id, b_terminations__if_type__owner__id: $b_terminations__if_type__owner__id, b_terminations__if_type__is_protected: $b_terminations__if_type__is_protected, b_terminations__name__value: $b_terminations__name__value, b_terminations__name__values: $b_terminations__name__values, b_terminations__name__source__id: $b_terminations__name__source__id, b_terminations__name__owner__id: $b_terminations__name__owner__id, b_terminations__name__is_protected: $b_terminations__name__is_protected, b_terminations__enabled__value: $b_terminations__enabled__value, b_terminations__enabled__values: $b_terminations__enabled__values, b_terminations__enabled__source__id: $b_terminations__enabled__source__id, b_terminations__enabled__owner__id: $b_terminations__enabled__owner__id, b_terminations__enabled__is_protected: $b_terminations__enabled__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, a_terminations__ids: $a_terminations__ids, a_terminations__isnull: $a_terminations__isnull, a_terminations__display_label__value: $a_terminations__display_label__value, a_terminations__display_label__values: $a_terminations__display_label__values, a_terminations__display_label__isnull: $a_terminations__display_label__isnull, a_terminations__if_type__value: $a_terminations__if_type__value, a_terminations__if_type__values: $a_terminations__if_type__values, a_terminations__if_type__source__id: $a_terminations__if_type__source__id, a_terminations__if_type__owner__id: $a_terminations__if_type__owner__id, a_terminations__if_type__is_protected: $a_terminations__if_type__is_protected, a_terminations__name__value: $a_terminations__name__value, a_terminations__name__values: $a_terminations__name__values, a_terminations__name__source__id: $a_terminations__name__source__id, a_terminations__name__owner__id: $a_terminations__name__owner__id, a_terminations__name__is_protected: $a_terminations__name__is_protected, a_terminations__enabled__value: $a_terminations__enabled__value, a_terminations__enabled__values: $a_terminations__enabled__values, a_terminations__enabled__source__id: $a_terminations__enabled__source__id, a_terminations__enabled__owner__id: $a_terminations__enabled__owner__id, a_terminations__enabled__is_protected: $a_terminations__enabled__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label label { is_default is_protected updated_at id is_from_profile permissions { __typename } value } b_terminations { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } a_terminations { count edges { __typename } permissions { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<DcimCableResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.dcim_cable.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<DcimCableFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, DcimCable, String, (DcimCableResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query DcimCable($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $label__value: String, $label__values: [String], $label__isnull: Boolean, $label__source__id: ID, $label__owner__id: ID, $label__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $b_terminations__ids: [ID], $b_terminations__isnull: Boolean, $b_terminations__display_label__value: String, $b_terminations__display_label__values: [String], $b_terminations__display_label__isnull: Boolean, $b_terminations__if_type__value: String, $b_terminations__if_type__values: [String], $b_terminations__if_type__source__id: ID, $b_terminations__if_type__owner__id: ID, $b_terminations__if_type__is_protected: Boolean, $b_terminations__name__value: String, $b_terminations__name__values: [String], $b_terminations__name__source__id: ID, $b_terminations__name__owner__id: ID, $b_terminations__name__is_protected: Boolean, $b_terminations__enabled__value: Boolean, $b_terminations__enabled__values: [Boolean], $b_terminations__enabled__source__id: ID, $b_terminations__enabled__owner__id: ID, $b_terminations__enabled__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $a_terminations__ids: [ID], $a_terminations__isnull: Boolean, $a_terminations__display_label__value: String, $a_terminations__display_label__values: [String], $a_terminations__display_label__isnull: Boolean, $a_terminations__if_type__value: String, $a_terminations__if_type__values: [String], $a_terminations__if_type__source__id: ID, $a_terminations__if_type__owner__id: ID, $a_terminations__if_type__is_protected: Boolean, $a_terminations__name__value: String, $a_terminations__name__values: [String], $a_terminations__name__source__id: ID, $a_terminations__name__owner__id: ID, $a_terminations__name__is_protected: Boolean, $a_terminations__enabled__value: Boolean, $a_terminations__enabled__values: [Boolean], $a_terminations__enabled__source__id: ID, $a_terminations__enabled__owner__id: ID, $a_terminations__enabled__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimCable(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, label__value: $label__value, label__values: $label__values, label__isnull: $label__isnull, label__source__id: $label__source__id, label__owner__id: $label__owner__id, label__is_protected: $label__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, b_terminations__ids: $b_terminations__ids, b_terminations__isnull: $b_terminations__isnull, b_terminations__display_label__value: $b_terminations__display_label__value, b_terminations__display_label__values: $b_terminations__display_label__values, b_terminations__display_label__isnull: $b_terminations__display_label__isnull, b_terminations__if_type__value: $b_terminations__if_type__value, b_terminations__if_type__values: $b_terminations__if_type__values, b_terminations__if_type__source__id: $b_terminations__if_type__source__id, b_terminations__if_type__owner__id: $b_terminations__if_type__owner__id, b_terminations__if_type__is_protected: $b_terminations__if_type__is_protected, b_terminations__name__value: $b_terminations__name__value, b_terminations__name__values: $b_terminations__name__values, b_terminations__name__source__id: $b_terminations__name__source__id, b_terminations__name__owner__id: $b_terminations__name__owner__id, b_terminations__name__is_protected: $b_terminations__name__is_protected, b_terminations__enabled__value: $b_terminations__enabled__value, b_terminations__enabled__values: $b_terminations__enabled__values, b_terminations__enabled__source__id: $b_terminations__enabled__source__id, b_terminations__enabled__owner__id: $b_terminations__enabled__owner__id, b_terminations__enabled__is_protected: $b_terminations__enabled__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, a_terminations__ids: $a_terminations__ids, a_terminations__isnull: $a_terminations__isnull, a_terminations__display_label__value: $a_terminations__display_label__value, a_terminations__display_label__values: $a_terminations__display_label__values, a_terminations__display_label__isnull: $a_terminations__display_label__isnull, a_terminations__if_type__value: $a_terminations__if_type__value, a_terminations__if_type__values: $a_terminations__if_type__values, a_terminations__if_type__source__id: $a_terminations__if_type__source__id, a_terminations__if_type__owner__id: $a_terminations__if_type__owner__id, a_terminations__if_type__is_protected: $a_terminations__if_type__is_protected, a_terminations__name__value: $a_terminations__name__value, a_terminations__name__values: $a_terminations__name__values, a_terminations__name__source__id: $a_terminations__name__source__id, a_terminations__name__owner__id: $a_terminations__name__owner__id, a_terminations__name__is_protected: $a_terminations__name__is_protected, a_terminations__enabled__value: $a_terminations__enabled__value, a_terminations__enabled__values: $a_terminations__enabled__values, a_terminations__enabled__source__id: $a_terminations__enabled__source__id, a_terminations__enabled__owner__id: $a_terminations__enabled__owner__id, a_terminations__enabled__is_protected: $a_terminations__enabled__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label label { is_default is_protected updated_at id is_from_profile permissions { __typename } value } b_terminations { count edges { __typename } permissions { __typename } } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } a_terminations { count edges { __typename } permissions { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (DcimCableResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (DcimCableResponse, i64)> {
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
                        .execute::<DcimCableResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, DcimCable, String, (DcimCableResponse, i64)> = Box::new(move |(data, current_offset): (DcimCableResponse, i64)| -> Result<EdgePage<DcimCable, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.dcim_cable.edges {
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
    ) -> Result<Option<DcimCable>> {
        let mut filters = DcimCableFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct DcimDeviceFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub status_value: Option<String>,
    pub status_values: Option<Vec<String>>,
    pub status_isnull: Option<bool>,
    pub status_source_id: Option<String>,
    pub status_owner_id: Option<String>,
    pub status_is_protected: Option<bool>,
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

impl DcimDeviceFilters {
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
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert(
                "profiles__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert(
                "profiles__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert(
                "profiles__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert(
                "profiles__profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert(
                "profiles__profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert(
                "profiles__profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert(
                "profiles__profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert(
                "profiles__profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert(
                "profiles__profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert(
                "profiles__profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert(
                "profiles__profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert(
                "profiles__profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert(
                "profiles__profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct DcimDeviceClient<'a> {
    client: &'a Client,
}

impl<'a> DcimDeviceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<DcimDeviceFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<DcimDevice>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query DcimDevice($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $role__ids: [ID], $role__isnull: Boolean, $role__display_label__value: String, $role__display_label__values: [String], $role__display_label__isnull: Boolean, $role__name__value: String, $role__name__values: [String], $role__name__source__id: ID, $role__name__owner__id: ID, $role__name__is_protected: Boolean, $role__slug__value: String, $role__slug__values: [String], $role__slug__source__id: ID, $role__slug__owner__id: ID, $role__slug__is_protected: Boolean, $platform__ids: [ID], $platform__isnull: Boolean, $platform__display_label__value: String, $platform__display_label__values: [String], $platform__display_label__isnull: Boolean, $platform__name__value: String, $platform__name__values: [String], $platform__name__source__id: ID, $platform__name__owner__id: ID, $platform__name__is_protected: Boolean, $platform__slug__value: String, $platform__slug__values: [String], $platform__slug__source__id: ID, $platform__slug__owner__id: ID, $platform__slug__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $site__ids: [ID], $site__isnull: Boolean, $site__display_label__value: String, $site__display_label__values: [String], $site__display_label__isnull: Boolean, $site__slug__value: String, $site__slug__values: [String], $site__slug__source__id: ID, $site__slug__owner__id: ID, $site__slug__is_protected: Boolean, $site__name__value: String, $site__name__values: [String], $site__name__source__id: ID, $site__name__owner__id: ID, $site__name__is_protected: Boolean, $site__status__value: String, $site__status__values: [String], $site__status__source__id: ID, $site__status__owner__id: ID, $site__status__is_protected: Boolean, $primary_ip4__ids: [ID], $primary_ip4__isnull: Boolean, $primary_ip4__display_label__value: String, $primary_ip4__display_label__values: [String], $primary_ip4__display_label__isnull: Boolean, $primary_ip4__address__value: String, $primary_ip4__address__values: [String], $primary_ip4__address__source__id: ID, $primary_ip4__address__owner__id: ID, $primary_ip4__address__is_protected: Boolean, $primary_ip4__status__value: String, $primary_ip4__status__values: [String], $primary_ip4__status__source__id: ID, $primary_ip4__status__owner__id: ID, $primary_ip4__status__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $device_type__ids: [ID], $device_type__isnull: Boolean, $device_type__display_label__value: String, $device_type__display_label__values: [String], $device_type__display_label__isnull: Boolean, $device_type__model__value: String, $device_type__model__values: [String], $device_type__model__source__id: ID, $device_type__model__owner__id: ID, $device_type__model__is_protected: Boolean, $device_type__slug__value: String, $device_type__slug__values: [String], $device_type__slug__source__id: ID, $device_type__slug__owner__id: ID, $device_type__slug__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimDevice(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, role__ids: $role__ids, role__isnull: $role__isnull, role__display_label__value: $role__display_label__value, role__display_label__values: $role__display_label__values, role__display_label__isnull: $role__display_label__isnull, role__name__value: $role__name__value, role__name__values: $role__name__values, role__name__source__id: $role__name__source__id, role__name__owner__id: $role__name__owner__id, role__name__is_protected: $role__name__is_protected, role__slug__value: $role__slug__value, role__slug__values: $role__slug__values, role__slug__source__id: $role__slug__source__id, role__slug__owner__id: $role__slug__owner__id, role__slug__is_protected: $role__slug__is_protected, platform__ids: $platform__ids, platform__isnull: $platform__isnull, platform__display_label__value: $platform__display_label__value, platform__display_label__values: $platform__display_label__values, platform__display_label__isnull: $platform__display_label__isnull, platform__name__value: $platform__name__value, platform__name__values: $platform__name__values, platform__name__source__id: $platform__name__source__id, platform__name__owner__id: $platform__name__owner__id, platform__name__is_protected: $platform__name__is_protected, platform__slug__value: $platform__slug__value, platform__slug__values: $platform__slug__values, platform__slug__source__id: $platform__slug__source__id, platform__slug__owner__id: $platform__slug__owner__id, platform__slug__is_protected: $platform__slug__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, site__ids: $site__ids, site__isnull: $site__isnull, site__display_label__value: $site__display_label__value, site__display_label__values: $site__display_label__values, site__display_label__isnull: $site__display_label__isnull, site__slug__value: $site__slug__value, site__slug__values: $site__slug__values, site__slug__source__id: $site__slug__source__id, site__slug__owner__id: $site__slug__owner__id, site__slug__is_protected: $site__slug__is_protected, site__name__value: $site__name__value, site__name__values: $site__name__values, site__name__source__id: $site__name__source__id, site__name__owner__id: $site__name__owner__id, site__name__is_protected: $site__name__is_protected, site__status__value: $site__status__value, site__status__values: $site__status__values, site__status__source__id: $site__status__source__id, site__status__owner__id: $site__status__owner__id, site__status__is_protected: $site__status__is_protected, primary_ip4__ids: $primary_ip4__ids, primary_ip4__isnull: $primary_ip4__isnull, primary_ip4__display_label__value: $primary_ip4__display_label__value, primary_ip4__display_label__values: $primary_ip4__display_label__values, primary_ip4__display_label__isnull: $primary_ip4__display_label__isnull, primary_ip4__address__value: $primary_ip4__address__value, primary_ip4__address__values: $primary_ip4__address__values, primary_ip4__address__source__id: $primary_ip4__address__source__id, primary_ip4__address__owner__id: $primary_ip4__address__owner__id, primary_ip4__address__is_protected: $primary_ip4__address__is_protected, primary_ip4__status__value: $primary_ip4__status__value, primary_ip4__status__values: $primary_ip4__status__values, primary_ip4__status__source__id: $primary_ip4__status__source__id, primary_ip4__status__owner__id: $primary_ip4__status__owner__id, primary_ip4__status__is_protected: $primary_ip4__status__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, device_type__ids: $device_type__ids, device_type__isnull: $device_type__isnull, device_type__display_label__value: $device_type__display_label__value, device_type__display_label__values: $device_type__display_label__values, device_type__display_label__isnull: $device_type__display_label__isnull, device_type__model__value: $device_type__model__value, device_type__model__values: $device_type__model__values, device_type__model__source__id: $device_type__model__source__id, device_type__model__owner__id: $device_type__model__owner__id, device_type__model__is_protected: $device_type__model__is_protected, device_type__slug__value: $device_type__slug__value, device_type__slug__values: $device_type__slug__values, device_type__slug__source__id: $device_type__slug__source__id, device_type__slug__owner__id: $device_type__slug__owner__id, device_type__slug__is_protected: $device_type__slug__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } role { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } platform { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } site { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } primary_ip4 { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } subscriber_of_groups { count edges { __typename } } device_type { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<DcimDeviceResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.dcim_device.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<DcimDeviceFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, DcimDevice, String, (DcimDeviceResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query DcimDevice($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $role__ids: [ID], $role__isnull: Boolean, $role__display_label__value: String, $role__display_label__values: [String], $role__display_label__isnull: Boolean, $role__name__value: String, $role__name__values: [String], $role__name__source__id: ID, $role__name__owner__id: ID, $role__name__is_protected: Boolean, $role__slug__value: String, $role__slug__values: [String], $role__slug__source__id: ID, $role__slug__owner__id: ID, $role__slug__is_protected: Boolean, $platform__ids: [ID], $platform__isnull: Boolean, $platform__display_label__value: String, $platform__display_label__values: [String], $platform__display_label__isnull: Boolean, $platform__name__value: String, $platform__name__values: [String], $platform__name__source__id: ID, $platform__name__owner__id: ID, $platform__name__is_protected: Boolean, $platform__slug__value: String, $platform__slug__values: [String], $platform__slug__source__id: ID, $platform__slug__owner__id: ID, $platform__slug__is_protected: Boolean, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $site__ids: [ID], $site__isnull: Boolean, $site__display_label__value: String, $site__display_label__values: [String], $site__display_label__isnull: Boolean, $site__slug__value: String, $site__slug__values: [String], $site__slug__source__id: ID, $site__slug__owner__id: ID, $site__slug__is_protected: Boolean, $site__name__value: String, $site__name__values: [String], $site__name__source__id: ID, $site__name__owner__id: ID, $site__name__is_protected: Boolean, $site__status__value: String, $site__status__values: [String], $site__status__source__id: ID, $site__status__owner__id: ID, $site__status__is_protected: Boolean, $primary_ip4__ids: [ID], $primary_ip4__isnull: Boolean, $primary_ip4__display_label__value: String, $primary_ip4__display_label__values: [String], $primary_ip4__display_label__isnull: Boolean, $primary_ip4__address__value: String, $primary_ip4__address__values: [String], $primary_ip4__address__source__id: ID, $primary_ip4__address__owner__id: ID, $primary_ip4__address__is_protected: Boolean, $primary_ip4__status__value: String, $primary_ip4__status__values: [String], $primary_ip4__status__source__id: ID, $primary_ip4__status__owner__id: ID, $primary_ip4__status__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $device_type__ids: [ID], $device_type__isnull: Boolean, $device_type__display_label__value: String, $device_type__display_label__values: [String], $device_type__display_label__isnull: Boolean, $device_type__model__value: String, $device_type__model__values: [String], $device_type__model__source__id: ID, $device_type__model__owner__id: ID, $device_type__model__is_protected: Boolean, $device_type__slug__value: String, $device_type__slug__values: [String], $device_type__slug__source__id: ID, $device_type__slug__owner__id: ID, $device_type__slug__is_protected: Boolean, $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimDevice(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, role__ids: $role__ids, role__isnull: $role__isnull, role__display_label__value: $role__display_label__value, role__display_label__values: $role__display_label__values, role__display_label__isnull: $role__display_label__isnull, role__name__value: $role__name__value, role__name__values: $role__name__values, role__name__source__id: $role__name__source__id, role__name__owner__id: $role__name__owner__id, role__name__is_protected: $role__name__is_protected, role__slug__value: $role__slug__value, role__slug__values: $role__slug__values, role__slug__source__id: $role__slug__source__id, role__slug__owner__id: $role__slug__owner__id, role__slug__is_protected: $role__slug__is_protected, platform__ids: $platform__ids, platform__isnull: $platform__isnull, platform__display_label__value: $platform__display_label__value, platform__display_label__values: $platform__display_label__values, platform__display_label__isnull: $platform__display_label__isnull, platform__name__value: $platform__name__value, platform__name__values: $platform__name__values, platform__name__source__id: $platform__name__source__id, platform__name__owner__id: $platform__name__owner__id, platform__name__is_protected: $platform__name__is_protected, platform__slug__value: $platform__slug__value, platform__slug__values: $platform__slug__values, platform__slug__source__id: $platform__slug__source__id, platform__slug__owner__id: $platform__slug__owner__id, platform__slug__is_protected: $platform__slug__is_protected, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, site__ids: $site__ids, site__isnull: $site__isnull, site__display_label__value: $site__display_label__value, site__display_label__values: $site__display_label__values, site__display_label__isnull: $site__display_label__isnull, site__slug__value: $site__slug__value, site__slug__values: $site__slug__values, site__slug__source__id: $site__slug__source__id, site__slug__owner__id: $site__slug__owner__id, site__slug__is_protected: $site__slug__is_protected, site__name__value: $site__name__value, site__name__values: $site__name__values, site__name__source__id: $site__name__source__id, site__name__owner__id: $site__name__owner__id, site__name__is_protected: $site__name__is_protected, site__status__value: $site__status__value, site__status__values: $site__status__values, site__status__source__id: $site__status__source__id, site__status__owner__id: $site__status__owner__id, site__status__is_protected: $site__status__is_protected, primary_ip4__ids: $primary_ip4__ids, primary_ip4__isnull: $primary_ip4__isnull, primary_ip4__display_label__value: $primary_ip4__display_label__value, primary_ip4__display_label__values: $primary_ip4__display_label__values, primary_ip4__display_label__isnull: $primary_ip4__display_label__isnull, primary_ip4__address__value: $primary_ip4__address__value, primary_ip4__address__values: $primary_ip4__address__values, primary_ip4__address__source__id: $primary_ip4__address__source__id, primary_ip4__address__owner__id: $primary_ip4__address__owner__id, primary_ip4__address__is_protected: $primary_ip4__address__is_protected, primary_ip4__status__value: $primary_ip4__status__value, primary_ip4__status__values: $primary_ip4__status__values, primary_ip4__status__source__id: $primary_ip4__status__source__id, primary_ip4__status__owner__id: $primary_ip4__status__owner__id, primary_ip4__status__is_protected: $primary_ip4__status__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, device_type__ids: $device_type__ids, device_type__isnull: $device_type__isnull, device_type__display_label__value: $device_type__display_label__value, device_type__display_label__values: $device_type__display_label__values, device_type__display_label__isnull: $device_type__display_label__isnull, device_type__model__value: $device_type__model__value, device_type__model__values: $device_type__model__values, device_type__model__source__id: $device_type__model__source__id, device_type__model__owner__id: $device_type__model__owner__id, device_type__model__is_protected: $device_type__model__is_protected, device_type__slug__value: $device_type__slug__value, device_type__slug__values: $device_type__slug__values, device_type__slug__source__id: $device_type__slug__source__id, device_type__slug__owner__id: $device_type__slug__owner__id, device_type__slug__is_protected: $device_type__slug__is_protected, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } role { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } platform { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } member_of_groups { count edges { __typename } } site { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } primary_ip4 { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } subscriber_of_groups { count edges { __typename } } device_type { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (DcimDeviceResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (DcimDeviceResponse, i64)> {
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
                        .execute::<DcimDeviceResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, DcimDevice, String, (DcimDeviceResponse, i64)> = Box::new(move |(data, current_offset): (DcimDeviceResponse, i64)| -> Result<EdgePage<DcimDevice, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.dcim_device.edges {
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
    ) -> Result<Option<DcimDevice>> {
        let mut filters = DcimDeviceFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct DcimDeviceRoleFilters {
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
    pub slug_value: Option<String>,
    pub slug_values: Option<Vec<String>>,
    pub slug_isnull: Option<bool>,
    pub slug_source_id: Option<String>,
    pub slug_owner_id: Option<String>,
    pub slug_is_protected: Option<bool>,
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
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
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

impl DcimDeviceRoleFilters {
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
        if let Some(value) = &self.slug_value {
            vars.insert("slug__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_values {
            vars.insert("slug__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_isnull {
            vars.insert("slug__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_source_id {
            vars.insert("slug__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_owner_id {
            vars.insert("slug__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_is_protected {
            vars.insert(
                "slug__is_protected".to_string(),
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
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert(
                "profiles__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert(
                "profiles__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert(
                "profiles__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert(
                "profiles__profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert(
                "profiles__profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert(
                "profiles__profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert(
                "profiles__profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert(
                "profiles__profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert(
                "profiles__profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert(
                "profiles__profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert(
                "profiles__profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert(
                "profiles__profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert(
                "profiles__profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct DcimDeviceRoleClient<'a> {
    client: &'a Client,
}

impl<'a> DcimDeviceRoleClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<DcimDeviceRoleFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<DcimDeviceRole>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query DcimDeviceRole($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimDeviceRole(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<DcimDeviceRoleResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.dcim_device_role.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<DcimDeviceRoleFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, DcimDeviceRole, String, (DcimDeviceRoleResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query DcimDeviceRole($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimDeviceRole(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (DcimDeviceRoleResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (DcimDeviceRoleResponse, i64)> {
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
                        .execute::<DcimDeviceRoleResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, DcimDeviceRole, String, (DcimDeviceRoleResponse, i64)> = Box::new(move |(data, current_offset): (DcimDeviceRoleResponse, i64)| -> Result<EdgePage<DcimDeviceRole, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.dcim_device_role.edges {
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
    ) -> Result<Option<DcimDeviceRole>> {
        let mut filters = DcimDeviceRoleFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct DcimDeviceTypeFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub model_value: Option<String>,
    pub model_values: Option<Vec<String>>,
    pub model_isnull: Option<bool>,
    pub model_source_id: Option<String>,
    pub model_owner_id: Option<String>,
    pub model_is_protected: Option<bool>,
    pub slug_value: Option<String>,
    pub slug_values: Option<Vec<String>>,
    pub slug_isnull: Option<bool>,
    pub slug_source_id: Option<String>,
    pub slug_owner_id: Option<String>,
    pub slug_is_protected: Option<bool>,
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

impl DcimDeviceTypeFilters {
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
        if let Some(value) = &self.slug_value {
            vars.insert("slug__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_values {
            vars.insert("slug__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_isnull {
            vars.insert("slug__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_source_id {
            vars.insert("slug__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_owner_id {
            vars.insert("slug__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_is_protected {
            vars.insert(
                "slug__is_protected".to_string(),
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
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert(
                "profiles__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert(
                "profiles__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert(
                "profiles__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert(
                "profiles__profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert(
                "profiles__profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert(
                "profiles__profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert(
                "profiles__profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert(
                "profiles__profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert(
                "profiles__profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert(
                "profiles__profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert(
                "profiles__profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert(
                "profiles__profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert(
                "profiles__profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct DcimDeviceTypeClient<'a> {
    client: &'a Client,
}

impl<'a> DcimDeviceTypeClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<DcimDeviceTypeFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<DcimDeviceType>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query DcimDeviceType($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $model__value: String, $model__values: [String], $model__isnull: Boolean, $model__source__id: ID, $model__owner__id: ID, $model__is_protected: Boolean, $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $manufacturer__ids: [ID], $manufacturer__isnull: Boolean, $manufacturer__display_label__value: String, $manufacturer__display_label__values: [String], $manufacturer__display_label__isnull: Boolean, $manufacturer__slug__value: String, $manufacturer__slug__values: [String], $manufacturer__slug__source__id: ID, $manufacturer__slug__owner__id: ID, $manufacturer__slug__is_protected: Boolean, $manufacturer__name__value: String, $manufacturer__name__values: [String], $manufacturer__name__source__id: ID, $manufacturer__name__owner__id: ID, $manufacturer__name__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimDeviceType(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, model__value: $model__value, model__values: $model__values, model__isnull: $model__isnull, model__source__id: $model__source__id, model__owner__id: $model__owner__id, model__is_protected: $model__is_protected, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, manufacturer__ids: $manufacturer__ids, manufacturer__isnull: $manufacturer__isnull, manufacturer__display_label__value: $manufacturer__display_label__value, manufacturer__display_label__values: $manufacturer__display_label__values, manufacturer__display_label__isnull: $manufacturer__display_label__isnull, manufacturer__slug__value: $manufacturer__slug__value, manufacturer__slug__values: $manufacturer__slug__values, manufacturer__slug__source__id: $manufacturer__slug__source__id, manufacturer__slug__owner__id: $manufacturer__slug__owner__id, manufacturer__slug__is_protected: $manufacturer__slug__is_protected, manufacturer__name__value: $manufacturer__name__value, manufacturer__name__values: $manufacturer__name__values, manufacturer__name__source__id: $manufacturer__name__source__id, manufacturer__name__owner__id: $manufacturer__name__owner__id, manufacturer__name__is_protected: $manufacturer__name__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label model { is_default is_protected updated_at id is_from_profile permissions { __typename } value } slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } manufacturer { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<DcimDeviceTypeResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.dcim_device_type.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<DcimDeviceTypeFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, DcimDeviceType, String, (DcimDeviceTypeResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query DcimDeviceType($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $model__value: String, $model__values: [String], $model__isnull: Boolean, $model__source__id: ID, $model__owner__id: ID, $model__is_protected: Boolean, $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $manufacturer__ids: [ID], $manufacturer__isnull: Boolean, $manufacturer__display_label__value: String, $manufacturer__display_label__values: [String], $manufacturer__display_label__isnull: Boolean, $manufacturer__slug__value: String, $manufacturer__slug__values: [String], $manufacturer__slug__source__id: ID, $manufacturer__slug__owner__id: ID, $manufacturer__slug__is_protected: Boolean, $manufacturer__name__value: String, $manufacturer__name__values: [String], $manufacturer__name__source__id: ID, $manufacturer__name__owner__id: ID, $manufacturer__name__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimDeviceType(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, model__value: $model__value, model__values: $model__values, model__isnull: $model__isnull, model__source__id: $model__source__id, model__owner__id: $model__owner__id, model__is_protected: $model__is_protected, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, manufacturer__ids: $manufacturer__ids, manufacturer__isnull: $manufacturer__isnull, manufacturer__display_label__value: $manufacturer__display_label__value, manufacturer__display_label__values: $manufacturer__display_label__values, manufacturer__display_label__isnull: $manufacturer__display_label__isnull, manufacturer__slug__value: $manufacturer__slug__value, manufacturer__slug__values: $manufacturer__slug__values, manufacturer__slug__source__id: $manufacturer__slug__source__id, manufacturer__slug__owner__id: $manufacturer__slug__owner__id, manufacturer__slug__is_protected: $manufacturer__slug__is_protected, manufacturer__name__value: $manufacturer__name__value, manufacturer__name__values: $manufacturer__name__values, manufacturer__name__source__id: $manufacturer__name__source__id, manufacturer__name__owner__id: $manufacturer__name__owner__id, manufacturer__name__is_protected: $manufacturer__name__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label model { is_default is_protected updated_at id is_from_profile permissions { __typename } value } slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } manufacturer { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (DcimDeviceTypeResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (DcimDeviceTypeResponse, i64)> {
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
                        .execute::<DcimDeviceTypeResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, DcimDeviceType, String, (DcimDeviceTypeResponse, i64)> = Box::new(move |(data, current_offset): (DcimDeviceTypeResponse, i64)| -> Result<EdgePage<DcimDeviceType, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.dcim_device_type.edges {
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
    ) -> Result<Option<DcimDeviceType>> {
        let mut filters = DcimDeviceTypeFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct DcimInterfaceFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub if_type_value: Option<String>,
    pub if_type_values: Option<Vec<String>>,
    pub if_type_isnull: Option<bool>,
    pub if_type_source_id: Option<String>,
    pub if_type_owner_id: Option<String>,
    pub if_type_is_protected: Option<bool>,
    pub name_value: Option<String>,
    pub name_values: Option<Vec<String>>,
    pub name_isnull: Option<bool>,
    pub name_source_id: Option<String>,
    pub name_owner_id: Option<String>,
    pub name_is_protected: Option<bool>,
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
    pub device_ids: Option<Vec<String>>,
    pub device_isnull: Option<bool>,
    pub device_display_label_value: Option<String>,
    pub device_display_label_values: Option<Vec<String>>,
    pub device_display_label_isnull: Option<bool>,
    pub device_status_value: Option<String>,
    pub device_status_values: Option<Vec<String>>,
    pub device_status_source_id: Option<String>,
    pub device_status_owner_id: Option<String>,
    pub device_status_is_protected: Option<bool>,
    pub device_name_value: Option<String>,
    pub device_name_values: Option<Vec<String>>,
    pub device_name_source_id: Option<String>,
    pub device_name_owner_id: Option<String>,
    pub device_name_is_protected: Option<bool>,
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

impl DcimInterfaceFilters {
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
        if let Some(value) = &self.device_ids {
            vars.insert("device__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.device_isnull {
            vars.insert("device__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.device_display_label_value {
            vars.insert(
                "device__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_display_label_values {
            vars.insert(
                "device__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_display_label_isnull {
            vars.insert(
                "device__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_status_value {
            vars.insert(
                "device__status__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_status_values {
            vars.insert(
                "device__status__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_status_source_id {
            vars.insert(
                "device__status__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_status_owner_id {
            vars.insert(
                "device__status__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_status_is_protected {
            vars.insert(
                "device__status__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_name_value {
            vars.insert(
                "device__name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_name_values {
            vars.insert(
                "device__name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_name_source_id {
            vars.insert(
                "device__name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_name_owner_id {
            vars.insert(
                "device__name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.device_name_is_protected {
            vars.insert(
                "device__name__is_protected".to_string(),
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
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert(
                "profiles__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert(
                "profiles__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert(
                "profiles__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert(
                "profiles__profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert(
                "profiles__profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert(
                "profiles__profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert(
                "profiles__profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert(
                "profiles__profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert(
                "profiles__profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert(
                "profiles__profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert(
                "profiles__profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert(
                "profiles__profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert(
                "profiles__profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct DcimInterfaceClient<'a> {
    client: &'a Client,
}

impl<'a> DcimInterfaceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<DcimInterfaceFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<DcimInterface>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query DcimInterface($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $if_type__value: String, $if_type__values: [String], $if_type__isnull: Boolean, $if_type__source__id: ID, $if_type__owner__id: ID, $if_type__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $enabled__value: Boolean, $enabled__values: [Boolean], $enabled__isnull: Boolean, $enabled__source__id: ID, $enabled__owner__id: ID, $enabled__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $device__ids: [ID], $device__isnull: Boolean, $device__display_label__value: String, $device__display_label__values: [String], $device__display_label__isnull: Boolean, $device__status__value: String, $device__status__values: [String], $device__status__source__id: ID, $device__status__owner__id: ID, $device__status__is_protected: Boolean, $device__name__value: String, $device__name__values: [String], $device__name__source__id: ID, $device__name__owner__id: ID, $device__name__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimInterface(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, if_type__value: $if_type__value, if_type__values: $if_type__values, if_type__isnull: $if_type__isnull, if_type__source__id: $if_type__source__id, if_type__owner__id: $if_type__owner__id, if_type__is_protected: $if_type__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, enabled__value: $enabled__value, enabled__values: $enabled__values, enabled__isnull: $enabled__isnull, enabled__source__id: $enabled__source__id, enabled__owner__id: $enabled__owner__id, enabled__is_protected: $enabled__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, device__ids: $device__ids, device__isnull: $device__isnull, device__display_label__value: $device__display_label__value, device__display_label__values: $device__display_label__values, device__display_label__isnull: $device__display_label__isnull, device__status__value: $device__status__value, device__status__values: $device__status__values, device__status__source__id: $device__status__source__id, device__status__owner__id: $device__status__owner__id, device__status__is_protected: $device__status__is_protected, device__name__value: $device__name__value, device__name__values: $device__name__values, device__name__source__id: $device__name__source__id, device__name__owner__id: $device__name__owner__id, device__name__is_protected: $device__name__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label if_type { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } enabled { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_of_groups { count edges { __typename } } device { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } subscriber_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<DcimInterfaceResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.dcim_interface.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<DcimInterfaceFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, DcimInterface, String, (DcimInterfaceResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query DcimInterface($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $if_type__value: String, $if_type__values: [String], $if_type__isnull: Boolean, $if_type__source__id: ID, $if_type__owner__id: ID, $if_type__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $enabled__value: Boolean, $enabled__values: [Boolean], $enabled__isnull: Boolean, $enabled__source__id: ID, $enabled__owner__id: ID, $enabled__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $device__ids: [ID], $device__isnull: Boolean, $device__display_label__value: String, $device__display_label__values: [String], $device__display_label__isnull: Boolean, $device__status__value: String, $device__status__values: [String], $device__status__source__id: ID, $device__status__owner__id: ID, $device__status__is_protected: Boolean, $device__name__value: String, $device__name__values: [String], $device__name__source__id: ID, $device__name__owner__id: ID, $device__name__is_protected: Boolean, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimInterface(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, if_type__value: $if_type__value, if_type__values: $if_type__values, if_type__isnull: $if_type__isnull, if_type__source__id: $if_type__source__id, if_type__owner__id: $if_type__owner__id, if_type__is_protected: $if_type__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, enabled__value: $enabled__value, enabled__values: $enabled__values, enabled__isnull: $enabled__isnull, enabled__source__id: $enabled__source__id, enabled__owner__id: $enabled__owner__id, enabled__is_protected: $enabled__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, device__ids: $device__ids, device__isnull: $device__isnull, device__display_label__value: $device__display_label__value, device__display_label__values: $device__display_label__values, device__display_label__isnull: $device__display_label__isnull, device__status__value: $device__status__value, device__status__values: $device__status__values, device__status__source__id: $device__status__source__id, device__status__owner__id: $device__status__owner__id, device__status__is_protected: $device__status__is_protected, device__name__value: $device__name__value, device__name__values: $device__name__values, device__name__source__id: $device__name__source__id, device__name__owner__id: $device__name__owner__id, device__name__is_protected: $device__name__is_protected, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label if_type { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } enabled { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_of_groups { count edges { __typename } } device { node { __typename } node_metadata { __typename } properties { __typename } relationship_metadata { __typename } } subscriber_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (DcimInterfaceResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (DcimInterfaceResponse, i64)> {
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
                        .execute::<DcimInterfaceResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, DcimInterface, String, (DcimInterfaceResponse, i64)> = Box::new(move |(data, current_offset): (DcimInterfaceResponse, i64)| -> Result<EdgePage<DcimInterface, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.dcim_interface.edges {
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
    ) -> Result<Option<DcimInterface>> {
        let mut filters = DcimInterfaceFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct DcimManufacturerFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub slug_value: Option<String>,
    pub slug_values: Option<Vec<String>>,
    pub slug_isnull: Option<bool>,
    pub slug_source_id: Option<String>,
    pub slug_owner_id: Option<String>,
    pub slug_is_protected: Option<bool>,
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

impl DcimManufacturerFilters {
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
        if let Some(value) = &self.slug_value {
            vars.insert("slug__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_values {
            vars.insert("slug__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_isnull {
            vars.insert("slug__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_source_id {
            vars.insert("slug__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_owner_id {
            vars.insert("slug__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_is_protected {
            vars.insert(
                "slug__is_protected".to_string(),
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
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert(
                "profiles__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert(
                "profiles__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert(
                "profiles__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert(
                "profiles__profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert(
                "profiles__profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert(
                "profiles__profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert(
                "profiles__profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert(
                "profiles__profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert(
                "profiles__profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert(
                "profiles__profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert(
                "profiles__profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert(
                "profiles__profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert(
                "profiles__profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct DcimManufacturerClient<'a> {
    client: &'a Client,
}

impl<'a> DcimManufacturerClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<DcimManufacturerFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<DcimManufacturer>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query DcimManufacturer($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimManufacturer(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<DcimManufacturerResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.dcim_manufacturer.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<DcimManufacturerFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, DcimManufacturer, String, (DcimManufacturerResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query DcimManufacturer($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimManufacturer(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (DcimManufacturerResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (DcimManufacturerResponse, i64)> {
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
                        .execute::<DcimManufacturerResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, DcimManufacturer, String, (DcimManufacturerResponse, i64)> = Box::new(move |(data, current_offset): (DcimManufacturerResponse, i64)| -> Result<EdgePage<DcimManufacturer, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.dcim_manufacturer.edges {
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
    ) -> Result<Option<DcimManufacturer>> {
        let mut filters = DcimManufacturerFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct DcimPlatformFilters {
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
    pub slug_value: Option<String>,
    pub slug_values: Option<Vec<String>>,
    pub slug_isnull: Option<bool>,
    pub slug_source_id: Option<String>,
    pub slug_owner_id: Option<String>,
    pub slug_is_protected: Option<bool>,
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
    pub subscriber_of_groups_name_value: Option<String>,
    pub subscriber_of_groups_name_values: Option<Vec<String>>,
    pub subscriber_of_groups_group_type_value: Option<String>,
    pub subscriber_of_groups_group_type_values: Option<Vec<String>>,
    pub subscriber_of_groups_description_value: Option<String>,
    pub subscriber_of_groups_description_values: Option<Vec<String>>,
    pub subscriber_of_groups_label_value: Option<String>,
    pub subscriber_of_groups_label_values: Option<Vec<String>>,
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

impl DcimPlatformFilters {
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
        if let Some(value) = &self.slug_value {
            vars.insert("slug__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_values {
            vars.insert("slug__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_isnull {
            vars.insert("slug__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_source_id {
            vars.insert("slug__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_owner_id {
            vars.insert("slug__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_is_protected {
            vars.insert(
                "slug__is_protected".to_string(),
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
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert(
                "profiles__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert(
                "profiles__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert(
                "profiles__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert(
                "profiles__profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert(
                "profiles__profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert(
                "profiles__profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert(
                "profiles__profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert(
                "profiles__profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert(
                "profiles__profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert(
                "profiles__profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert(
                "profiles__profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert(
                "profiles__profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert(
                "profiles__profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct DcimPlatformClient<'a> {
    client: &'a Client,
}

impl<'a> DcimPlatformClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<DcimPlatformFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<DcimPlatform>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query DcimPlatform($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimPlatform(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<DcimPlatformResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.dcim_platform.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<DcimPlatformFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, DcimPlatform, String, (DcimPlatformResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query DcimPlatform($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimPlatform(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } subscriber_of_groups { count edges { __typename } } member_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (DcimPlatformResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (DcimPlatformResponse, i64)> {
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
                        .execute::<DcimPlatformResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, DcimPlatform, String, (DcimPlatformResponse, i64)> = Box::new(move |(data, current_offset): (DcimPlatformResponse, i64)| -> Result<EdgePage<DcimPlatform, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.dcim_platform.edges {
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
    ) -> Result<Option<DcimPlatform>> {
        let mut filters = DcimPlatformFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}

#[derive(Debug, Clone, Default)]
pub struct DcimSiteFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
    pub display_label_value: Option<String>,
    pub display_label_values: Option<Vec<String>>,
    pub display_label_isnull: Option<bool>,
    pub hfid: Option<Vec<String>>,
    pub slug_value: Option<String>,
    pub slug_values: Option<Vec<String>>,
    pub slug_isnull: Option<bool>,
    pub slug_source_id: Option<String>,
    pub slug_owner_id: Option<String>,
    pub slug_is_protected: Option<bool>,
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

impl DcimSiteFilters {
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
        if let Some(value) = &self.slug_value {
            vars.insert("slug__value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_values {
            vars.insert("slug__values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_isnull {
            vars.insert("slug__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_source_id {
            vars.insert("slug__source__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_owner_id {
            vars.insert("slug__owner__id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.slug_is_protected {
            vars.insert(
                "slug__is_protected".to_string(),
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
        if let Some(value) = &self.profiles_ids {
            vars.insert("profiles__ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_isnull {
            vars.insert("profiles__isnull".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &self.profiles_display_label_value {
            vars.insert(
                "profiles__display_label__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_values {
            vars.insert(
                "profiles__display_label__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_display_label_isnull {
            vars.insert(
                "profiles__display_label__isnull".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_value {
            vars.insert(
                "profiles__profile_name__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_values {
            vars.insert(
                "profiles__profile_name__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_source_id {
            vars.insert(
                "profiles__profile_name__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_owner_id {
            vars.insert(
                "profiles__profile_name__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_name_is_protected {
            vars.insert(
                "profiles__profile_name__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_value {
            vars.insert(
                "profiles__profile_priority__value".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_values {
            vars.insert(
                "profiles__profile_priority__values".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_source_id {
            vars.insert(
                "profiles__profile_priority__source__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_owner_id {
            vars.insert(
                "profiles__profile_priority__owner__id".to_string(),
                serde_json::to_value(value)?,
            );
        }
        if let Some(value) = &self.profiles_profile_priority_is_protected {
            vars.insert(
                "profiles__profile_priority__is_protected".to_string(),
                serde_json::to_value(value)?,
            );
        }
        Ok(Value::Object(vars))
    }
}

pub struct DcimSiteClient<'a> {
    client: &'a Client,
}

impl<'a> DcimSiteClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        filters: Option<DcimSiteFilters>,
        request_branch: Option<&str>,
    ) -> Result<Vec<DcimSite>> {
        let vars = filters
            .map(|f| f.to_vars())
            .transpose()?
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query DcimSite($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimSite(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self
            .client
            .execute::<DcimSiteResponse>(query, Some(vars), request_branch)
            .await?;
        let data = response
            .data
            .ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.dcim_site.edges {
            if let Some(node) = edge.node {
                items.push(*node);
            }
        }
        Ok(items)
    }

    pub fn paginate(
        &self,
        filters: Option<DcimSiteFilters>,
        request_branch: Option<&str>,
    ) -> DynPaginator<'a, DcimSite, String, (DcimSiteResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query DcimSite($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $display_label__value: String, $display_label__values: [String], $display_label__isnull: Boolean, $hfid: [String], $slug__value: String, $slug__values: [String], $slug__isnull: Boolean, $slug__source__id: ID, $slug__owner__id: ID, $slug__is_protected: Boolean, $name__value: String, $name__values: [String], $name__isnull: Boolean, $name__source__id: ID, $name__owner__id: ID, $name__is_protected: Boolean, $status__value: String, $status__values: [String], $status__isnull: Boolean, $status__source__id: ID, $status__owner__id: ID, $status__is_protected: Boolean, $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime, $member_of_groups__ids: [ID], $member_of_groups__isnull: Boolean, $member_of_groups__display_label__value: String, $member_of_groups__display_label__values: [String], $member_of_groups__display_label__isnull: Boolean, $member_of_groups__name__value: String, $member_of_groups__name__values: [String], $member_of_groups__group_type__value: String, $member_of_groups__group_type__values: [String], $member_of_groups__description__value: String, $member_of_groups__description__values: [String], $member_of_groups__label__value: String, $member_of_groups__label__values: [String], $subscriber_of_groups__ids: [ID], $subscriber_of_groups__isnull: Boolean, $subscriber_of_groups__display_label__value: String, $subscriber_of_groups__display_label__values: [String], $subscriber_of_groups__display_label__isnull: Boolean, $subscriber_of_groups__name__value: String, $subscriber_of_groups__name__values: [String], $subscriber_of_groups__group_type__value: String, $subscriber_of_groups__group_type__values: [String], $subscriber_of_groups__description__value: String, $subscriber_of_groups__description__values: [String], $subscriber_of_groups__label__value: String, $subscriber_of_groups__label__values: [String], $profiles__ids: [ID], $profiles__isnull: Boolean, $profiles__display_label__value: String, $profiles__display_label__values: [String], $profiles__display_label__isnull: Boolean, $profiles__profile_name__value: String, $profiles__profile_name__values: [String], $profiles__profile_name__source__id: ID, $profiles__profile_name__owner__id: ID, $profiles__profile_name__is_protected: Boolean, $profiles__profile_priority__value: BigInt, $profiles__profile_priority__values: [BigInt], $profiles__profile_priority__source__id: ID, $profiles__profile_priority__owner__id: ID, $profiles__profile_priority__is_protected: Boolean) { DcimSite(offset: $offset, limit: $limit, order: $order, ids: $ids, display_label__value: $display_label__value, display_label__values: $display_label__values, display_label__isnull: $display_label__isnull, hfid: $hfid, slug__value: $slug__value, slug__values: $slug__values, slug__isnull: $slug__isnull, slug__source__id: $slug__source__id, slug__owner__id: $slug__owner__id, slug__is_protected: $slug__is_protected, name__value: $name__value, name__values: $name__values, name__isnull: $name__isnull, name__source__id: $name__source__id, name__owner__id: $name__owner__id, name__is_protected: $name__is_protected, status__value: $status__value, status__values: $status__values, status__isnull: $status__isnull, status__source__id: $status__source__id, status__owner__id: $status__owner__id, status__is_protected: $status__is_protected, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after, member_of_groups__ids: $member_of_groups__ids, member_of_groups__isnull: $member_of_groups__isnull, member_of_groups__display_label__value: $member_of_groups__display_label__value, member_of_groups__display_label__values: $member_of_groups__display_label__values, member_of_groups__display_label__isnull: $member_of_groups__display_label__isnull, member_of_groups__name__value: $member_of_groups__name__value, member_of_groups__name__values: $member_of_groups__name__values, member_of_groups__group_type__value: $member_of_groups__group_type__value, member_of_groups__group_type__values: $member_of_groups__group_type__values, member_of_groups__description__value: $member_of_groups__description__value, member_of_groups__description__values: $member_of_groups__description__values, member_of_groups__label__value: $member_of_groups__label__value, member_of_groups__label__values: $member_of_groups__label__values, subscriber_of_groups__ids: $subscriber_of_groups__ids, subscriber_of_groups__isnull: $subscriber_of_groups__isnull, subscriber_of_groups__display_label__value: $subscriber_of_groups__display_label__value, subscriber_of_groups__display_label__values: $subscriber_of_groups__display_label__values, subscriber_of_groups__display_label__isnull: $subscriber_of_groups__display_label__isnull, subscriber_of_groups__name__value: $subscriber_of_groups__name__value, subscriber_of_groups__name__values: $subscriber_of_groups__name__values, subscriber_of_groups__group_type__value: $subscriber_of_groups__group_type__value, subscriber_of_groups__group_type__values: $subscriber_of_groups__group_type__values, subscriber_of_groups__description__value: $subscriber_of_groups__description__value, subscriber_of_groups__description__values: $subscriber_of_groups__description__values, subscriber_of_groups__label__value: $subscriber_of_groups__label__value, subscriber_of_groups__label__values: $subscriber_of_groups__label__values, profiles__ids: $profiles__ids, profiles__isnull: $profiles__isnull, profiles__display_label__value: $profiles__display_label__value, profiles__display_label__values: $profiles__display_label__values, profiles__display_label__isnull: $profiles__display_label__isnull, profiles__profile_name__value: $profiles__profile_name__value, profiles__profile_name__values: $profiles__profile_name__values, profiles__profile_name__source__id: $profiles__profile_name__source__id, profiles__profile_name__owner__id: $profiles__profile_name__owner__id, profiles__profile_name__is_protected: $profiles__profile_name__is_protected, profiles__profile_priority__value: $profiles__profile_priority__value, profiles__profile_priority__values: $profiles__profile_priority__values, profiles__profile_priority__source__id: $profiles__profile_priority__source__id, profiles__profile_priority__owner__id: $profiles__profile_priority__owner__id, profiles__profile_priority__is_protected: $profiles__profile_priority__is_protected) { count edges { node { id hfid display_label slug { is_default is_protected updated_at id is_from_profile permissions { __typename } value } name { is_default is_protected updated_at id is_from_profile permissions { __typename } value } status { is_default is_protected updated_at id is_from_profile permissions { __typename } value } member_of_groups { count edges { __typename } } subscriber_of_groups { count edges { __typename } } profiles { count edges { __typename } } } node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (DcimSiteResponse, i64)> = Box::new(
            move |cursor: Option<String>| -> BoxFutureResult<'a, (DcimSiteResponse, i64)> {
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
                        .execute::<DcimSiteResponse>(query, Some(vars), branch.as_deref())
                        .await?;
                    let data = response
                        .data
                        .ok_or_else(|| Error::Config("missing data".to_string()))?;
                    Ok((data, current_offset))
                })
            },
        );
        let extract: BoxExtract<'a, DcimSite, String, (DcimSiteResponse, i64)> = Box::new(move |(data, current_offset): (DcimSiteResponse, i64)| -> Result<EdgePage<DcimSite, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.dcim_site.edges {
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
    ) -> Result<Option<DcimSite>> {
        let mut filters = DcimSiteFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }
}
