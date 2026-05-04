//! generated api module

#![allow(non_snake_case, unused_imports, unused_assignments, clippy::field_reassign_with_default)]

use infrahub::{BoxExtract, BoxFetch, BoxFutureResult, Client, DynPaginator, EdgePage, Error, Result};
use serde_json::Value;

use crate::inputs::*;
use crate::responses::*;
use crate::types::*;

pub struct LineageApi<'a> {
    client: &'a Client,
}

impl<'a> LineageApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn owner(&self) -> LineageOwnerClient<'a> {
        LineageOwnerClient::new(self.client)
    }
    pub fn source(&self) -> LineageSourceClient<'a> {
        LineageSourceClient::new(self.client)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineageOwnerFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
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
}

impl LineageOwnerFilters {
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
        Ok(Value::Object(vars))
    }
}

pub struct LineageOwnerClient<'a> {
    client: &'a Client,
}

impl<'a> LineageOwnerClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, filters: Option<LineageOwnerFilters>, request_branch: Option<&str>) -> Result<Vec<serde_json::Value>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query LineageOwner($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime) { LineageOwner(offset: $offset, limit: $limit, order: $order, ids: $ids, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<LineageOwnerResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.lineage_owner.edges {
            if let Some(node) = edge.node {
                items.push(node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<LineageOwnerFilters>, request_branch: Option<&str>) -> DynPaginator<'a, serde_json::Value, String, (LineageOwnerResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query LineageOwner($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime) { LineageOwner(offset: $offset, limit: $limit, order: $order, ids: $ids, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (LineageOwnerResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (LineageOwnerResponse, i64)> {
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
                let response = client.execute::<LineageOwnerResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, serde_json::Value, String, (LineageOwnerResponse, i64)> = Box::new(move |(data, current_offset): (LineageOwnerResponse, i64)| -> Result<EdgePage<serde_json::Value, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.lineage_owner.edges {
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
        let mut filters = LineageOwnerFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }

}

#[derive(Debug, Clone, Default)]
pub struct LineageSourceFilters {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order: Option<OrderInput>,
    pub ids: Option<Vec<String>>,
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
}

impl LineageSourceFilters {
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
        Ok(Value::Object(vars))
    }
}

pub struct LineageSourceClient<'a> {
    client: &'a Client,
}

impl<'a> LineageSourceClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, filters: Option<LineageSourceFilters>, request_branch: Option<&str>) -> Result<Vec<serde_json::Value>> {
        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let query = r#"query LineageSource($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime) { LineageSource(offset: $offset, limit: $limit, order: $order, ids: $ids, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let response = self.client.execute::<LineageSourceResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let mut items = Vec::new();
        for edge in data.lineage_source.edges {
            if let Some(node) = edge.node {
                items.push(node);
            }
        }
        Ok(items)
    }

    pub fn paginate(&self, filters: Option<LineageSourceFilters>, request_branch: Option<&str>) -> DynPaginator<'a, serde_json::Value, String, (LineageSourceResponse, i64)> {
        let client = self.client;
        let base_filters = filters.unwrap_or_default();
        let request_branch = request_branch.map(|b| b.to_string());
        let query = r#"query LineageSource($offset: Int, $limit: Int, $order: OrderInput, $ids: [ID], $any__value: String, $any__values: [String], $any__source__id: ID, $any__owner__id: ID, $any__is_protected: Boolean, $partial_match: Boolean, $node_metadata__created_by__id: ID, $node_metadata__created_by__ids: [ID], $node_metadata__updated_by__id: ID, $node_metadata__updated_by__ids: [ID], $node_metadata__created_at: DateTime, $node_metadata__created_at__before: DateTime, $node_metadata__created_at__after: DateTime, $node_metadata__updated_at: DateTime, $node_metadata__updated_at__before: DateTime, $node_metadata__updated_at__after: DateTime) { LineageSource(offset: $offset, limit: $limit, order: $order, ids: $ids, any__value: $any__value, any__values: $any__values, any__source__id: $any__source__id, any__owner__id: $any__owner__id, any__is_protected: $any__is_protected, partial_match: $partial_match, node_metadata__created_by__id: $node_metadata__created_by__id, node_metadata__created_by__ids: $node_metadata__created_by__ids, node_metadata__updated_by__id: $node_metadata__updated_by__id, node_metadata__updated_by__ids: $node_metadata__updated_by__ids, node_metadata__created_at: $node_metadata__created_at, node_metadata__created_at__before: $node_metadata__created_at__before, node_metadata__created_at__after: $node_metadata__created_at__after, node_metadata__updated_at: $node_metadata__updated_at, node_metadata__updated_at__before: $node_metadata__updated_at__before, node_metadata__updated_at__after: $node_metadata__updated_at__after) { count edges { node_metadata { created_at updated_at } } permissions { count edges { node { kind view create update delete } } } } }"#;
        let fetch: BoxFetch<'a, String, (LineageSourceResponse, i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (LineageSourceResponse, i64)> {
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
                let response = client.execute::<LineageSourceResponse>(query, Some(vars), branch.as_deref()).await?;
                let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
                Ok((data, current_offset))
            })
        });
        let extract: BoxExtract<'a, serde_json::Value, String, (LineageSourceResponse, i64)> = Box::new(move |(data, current_offset): (LineageSourceResponse, i64)| -> Result<EdgePage<serde_json::Value, String>> {
            let mut items = Vec::new();
            let mut next: Option<String> = None;
            for edge in data.lineage_source.edges {
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
        let mut filters = LineageSourceFilters::default();
        filters.ids = Some(vec![id.into()]);
        let mut items = self.list(Some(filters), request_branch).await?;
        Ok(items.pop())
    }

}

