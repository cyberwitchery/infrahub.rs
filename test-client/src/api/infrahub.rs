//! generated api module

#![allow(non_snake_case, unused_imports, unused_assignments, clippy::field_reassign_with_default)]

use ::infrahub::{BoxExtract, BoxFetch, BoxFutureResult, Client, DynPaginator, EdgePage, Error, Result};
use serde_json::Value;

use crate::inputs::*;
use crate::responses::*;
use crate::types::*;

pub struct InfrahubApi<'a> {
    client: &'a Client,
}

impl<'a> InfrahubApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn account_token(&self) -> InfrahubAccountTokenClient<'a> {
        InfrahubAccountTokenClient::new(self.client)
    }
}

pub struct InfrahubAccountTokenClient<'a> {
    client: &'a Client,
}

impl<'a> InfrahubAccountTokenClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, data: InfrahubAccountTokenCreateInput, request_branch: Option<&str>) -> Result<InfrahubAccountTokenType> {
        let mut vars = serde_json::Map::new();
        vars.insert("data".to_string(), serde_json::to_value(data)?);
        let vars = Value::Object(vars);
        let query = r#"mutation InfrahubAccountTokenCreate($data: InfrahubAccountTokenCreateInput!) { InfrahubAccountTokenCreate(data: $data) { ok object { id token { value } } } }"#;
        let response = self.client.execute::<InfrahubAccountTokenCreateResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let payload = data.infrahub_account_token_create.ok_or_else(|| Error::Config("missing payload".to_string()))?;
        let object = payload.object.ok_or_else(|| Error::Config("missing object".to_string()))?;
        Ok(*object)
    }

    pub async fn delete(&self, data: InfrahubAccountTokenDeleteInput, request_branch: Option<&str>) -> Result<bool> {
        let mut vars = serde_json::Map::new();
        vars.insert("data".to_string(), serde_json::to_value(data)?);
        let vars = Value::Object(vars);
        let query = r#"mutation InfrahubAccountTokenDelete($data: InfrahubAccountTokenDeleteInput!) { InfrahubAccountTokenDelete(data: $data) { ok } }"#;
        let response = self.client.execute::<InfrahubAccountTokenDeleteResponse>(query, Some(vars), request_branch).await?;
        let data = response.data.ok_or_else(|| Error::Config("missing data".to_string()))?;
        let payload = data.infrahub_account_token_delete.ok_or_else(|| Error::Config("missing payload".to_string()))?;
        Ok(payload.ok.unwrap_or(false))
    }

}

