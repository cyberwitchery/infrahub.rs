//! generated api module

#![allow(non_snake_case, unused_imports, unused_assignments, clippy::field_reassign_with_default)]

use ::infrahub::{BoxExtract, BoxFetch, BoxFutureResult, Client, DynPaginator, EdgePage, Error, Result};
use serde_json::Value;

use crate::inputs::*;
use crate::responses::*;
use crate::types::*;

pub struct BranchApi<'a> {
    client: &'a Client,
}

impl<'a> BranchApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn branch(&self) -> BranchClient<'a> {
        BranchClient::new(self.client)
    }
}

pub struct BranchClient<'a> {
    client: &'a Client,
}

impl<'a> BranchClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, background_execution: Option<bool>, context: Option<ContextInput>, data: BranchCreateInput, wait_until_completion: Option<bool>, request_branch: Option<&str>) -> Result<Branch> {
        let mut vars = serde_json::Map::new();
        if let Some(value) = background_execution {
            vars.insert("background_execution".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = context {
            vars.insert("context".to_string(), serde_json::to_value(value)?);
        }
        vars.insert("data".to_string(), serde_json::to_value(data)?);
        if let Some(value) = wait_until_completion {
            vars.insert("wait_until_completion".to_string(), serde_json::to_value(value)?);
        }
        let vars = Value::Object(vars);
        let query = r#"mutation BranchCreate($background_execution: Boolean, $context: ContextInput, $data: BranchCreateInput!, $wait_until_completion: Boolean) { BranchCreate(background_execution: $background_execution, context: $context, data: $data, wait_until_completion: $wait_until_completion) { ok object { id name description origin_branch branched_from status graph_version created_at sync_with_git is_default schema_differs_from_default_branch } task { id } } }"#;
        let response = self.client.execute::<BranchCreateResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let payload = data.branch_create.ok_or_else(|| Error::Config("missing payload".to_string()))?;
        let object = payload.object.ok_or_else(|| Error::Config("missing object".to_string()))?;
        Ok(*object)
    }

    pub async fn delete(&self, context: Option<ContextInput>, data: BranchDeleteInput, wait_until_completion: Option<bool>, request_branch: Option<&str>) -> Result<bool> {
        let mut vars = serde_json::Map::new();
        if let Some(value) = context {
            vars.insert("context".to_string(), serde_json::to_value(value)?);
        }
        vars.insert("data".to_string(), serde_json::to_value(data)?);
        if let Some(value) = wait_until_completion {
            vars.insert("wait_until_completion".to_string(), serde_json::to_value(value)?);
        }
        let vars = Value::Object(vars);
        let query = r#"mutation BranchDelete($context: ContextInput, $data: BranchDeleteInput!, $wait_until_completion: Boolean) { BranchDelete(context: $context, data: $data, wait_until_completion: $wait_until_completion) { ok task { id } } }"#;
        let response = self.client.execute::<BranchDeleteResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let payload = data.branch_delete.ok_or_else(|| Error::Config("missing payload".to_string()))?;
        Ok(payload.ok.unwrap_or(false))
    }

}

