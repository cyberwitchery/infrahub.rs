//! main client
//!
//! includes helpers for raw graphql execution, typed responses, and schema fetch.

use crate::config::ClientConfig;
use crate::error::{Error, Result};
use crate::graphql::GraphQlResponse;
use crate::operation::Operation;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::sync::Arc;
use url::Url;

/// graphql client for infrahub
#[derive(Clone)]
pub struct Client {
    config: Arc<ClientConfig>,
    http: reqwest::Client,
}

impl Client {
    /// create a new client
    pub fn new(config: ClientConfig) -> Result<Self> {
        config.validate()?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-INFRAHUB-KEY",
            HeaderValue::from_str(&config.token)
                .map_err(|err| Error::Config(format!("invalid api token header value: {err}")))?,
        );
        headers.extend(config.extra_headers.clone());

        let builder = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(config.user_agent.clone())
            .timeout(config.timeout)
            .danger_accept_invalid_certs(!config.verify_ssl);

        let http = builder.build()?;

        Ok(Self {
            config: Arc::new(config),
            http,
        })
    }

    /// access the client configuration
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// execute a raw graphql query
    pub async fn execute_raw(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        branch: Option<&str>,
    ) -> Result<GraphQlResponse<serde_json::Value>> {
        self.execute_raw_with(query, variables, branch, |url, body| async move {
            let response = self.http.post(url).json(&body).send().await?;
            let status = response.status();
            let text = response.text().await?;
            Ok((status, text))
        })
        .await
    }

    /// execute a raw graphql query and deserialize into a typed response
    pub async fn execute<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        branch: Option<&str>,
    ) -> Result<GraphQlResponse<T>> {
        self.execute_with(query, variables, branch, |url, body| async move {
            let response = self.http.post(url).json(&body).send().await?;
            let status = response.status();
            let text = response.text().await?;
            Ok((status, text))
        })
        .await
    }

    /// execute a generated operation by name
    pub async fn execute_operation<O: Operation>(
        &self,
        variables: Option<serde_json::Value>,
        branch: Option<&str>,
    ) -> Result<GraphQlResponse<O::Response>> {
        self.execute(O::QUERY, variables, branch).await
    }

    /// fetch the graphql schema as text
    pub async fn fetch_schema(&self, branch: Option<&str>) -> Result<String> {
        self.fetch_schema_with(branch, |url| async move {
            let response = self.http.get(url).send().await?;
            let status = response.status();
            let text = response.text().await?;
            Ok((status, text))
        })
        .await
    }
}

fn parse_graphql_response<T: DeserializeOwned>(
    status: StatusCode,
    text: String,
) -> Result<GraphQlResponse<T>> {
    let parsed: GraphQlResponse<T> = serde_json::from_str(&text)?;
    if !parsed.errors.is_empty() {
        let message = parsed
            .errors
            .first()
            .map(|err| err.message.clone())
            .unwrap_or_else(|| "graphql error".to_string());
        return Err(Error::GraphQl {
            status: Some(status.as_u16()),
            errors: parsed.errors,
            body: text,
            message,
        });
    }

    if !status.is_success() {
        return Err(Error::GraphQl {
            status: Some(status.as_u16()),
            errors: Vec::new(),
            body: text,
            message: format!("graphql http error: {}", status),
        });
    }

    Ok(parsed)
}

fn parse_schema_response(status: StatusCode, text: String) -> Result<String> {
    if !status.is_success() {
        return Err(Error::GraphQl {
            status: Some(status.as_u16()),
            errors: Vec::new(),
            body: text,
            message: format!("schema http error: {}", status),
        });
    }

    Ok(text)
}

impl Client {
    pub(crate) async fn execute_raw_with<F, Fut>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        branch: Option<&str>,
        send: F,
    ) -> Result<GraphQlResponse<serde_json::Value>>
    where
        F: FnOnce(Url, serde_json::Value) -> Fut,
        Fut: Future<Output = Result<(StatusCode, String)>>,
    {
        let url = self.config.graphql_url(branch)?;
        let body = serde_json::json!({
            "query": query,
            "variables": variables.unwrap_or_else(|| serde_json::json!({})),
        });

        let (status, text) = send(url, body).await?;
        parse_graphql_response(status, text)
    }

    pub(crate) async fn execute_with<T: DeserializeOwned, F, Fut>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        branch: Option<&str>,
        send: F,
    ) -> Result<GraphQlResponse<T>>
    where
        F: FnOnce(Url, serde_json::Value) -> Fut,
        Fut: Future<Output = Result<(StatusCode, String)>>,
    {
        let url = self.config.graphql_url(branch)?;
        let body = serde_json::json!({
            "query": query,
            "variables": variables.unwrap_or_else(|| serde_json::json!({})),
        });

        let (status, text) = send(url, body).await?;
        parse_graphql_response(status, text)
    }

    pub(crate) async fn fetch_schema_with<F, Fut>(
        &self,
        branch: Option<&str>,
        send: F,
    ) -> Result<String>
    where
        F: FnOnce(Url) -> Fut,
        Fut: Future<Output = Result<(StatusCode, String)>>,
    {
        let url = self.config.schema_url(branch)?;
        let (status, text) = send(url).await?;
        parse_schema_response(status, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    fn test_client(config: ClientConfig) -> Client {
        config.validate().unwrap();
        let http = reqwest::Client::builder()
            .no_proxy()
            .build()
            .expect("test http client");
        Client {
            config: Arc::new(config),
            http,
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_raw_sets_header_and_url() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        let response = client
            .execute_raw_with("query { ok }", None, Some("main"), |url, body| async move {
                assert_eq!(url.path(), "/graphql/main");
                assert_eq!(body["query"], "query { ok }");
                Ok((StatusCode::OK, "{\"data\": {\"ok\": true}}".to_string()))
            })
            .await
            .unwrap();

        assert_eq!(response.data.unwrap()["ok"], true);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_graphql_error() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        let err = client
            .execute_raw_with("query { ok }", None, None, |_url, _body| async move {
                Ok((
                    StatusCode::OK,
                    "{\"data\": null, \"errors\": [{\"message\": \"boom\"}]}".to_string(),
                ))
            })
            .await;

        assert!(matches!(err, Err(Error::GraphQl { .. })));
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_typed_success() {
        #[derive(Debug, Deserialize)]
        struct Data {
            value: i64,
        }
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        let response = client
            .execute_with::<Data, _, _>("query { value }", None, None, |_url, _body| async move {
                Ok((StatusCode::OK, "{\"data\": {\"value\": 7}}".to_string()))
            })
            .await
            .unwrap();

        assert_eq!(response.data.unwrap().value, 7);
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_raw_http_error() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        let err = client
            .execute_raw_with("query { ok }", None, None, |_url, _body| async move {
                Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "{\"data\":null}".to_string(),
                ))
            })
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            Error::GraphQl {
                status: Some(500),
                ..
            }
        ));
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_fetch_schema_success_and_error() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);

        let schema = client
            .fetch_schema_with(None, |url| async move {
                assert_eq!(url.path(), "/schema.graphql");
                Ok((StatusCode::OK, "schema { query: Query }".to_string()))
            })
            .await
            .unwrap();
        assert!(schema.contains("schema"));

        let err = client
            .fetch_schema_with(Some("bad"), |url| async move {
                assert_eq!(url.path(), "/schema.graphql");
                assert_eq!(url.query(), Some("branch=bad"));
                Ok((StatusCode::NOT_FOUND, "not found".to_string()))
            })
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            Error::GraphQl {
                status: Some(404),
                ..
            }
        ));
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_raw_uses_default_branch() {
        let config =
            ClientConfig::new("http://localhost:1234", "test-token").with_default_branch("main");
        let client = test_client(config);
        let response = client
            .execute_raw_with("query { ok }", None, None, |url, _body| async move {
                assert_eq!(url.path(), "/graphql/main");
                Ok((StatusCode::OK, "{\"data\": {\"ok\": true}}".to_string()))
            })
            .await
            .unwrap();

        assert_eq!(response.data.unwrap()["ok"], true);
    }

    #[test]
    fn test_invalid_token_header() {
        let config = ClientConfig::new("http://localhost:1234", "bad\ntoken");
        let err = Client::new(config).err().expect("expected error");
        assert!(matches!(err, Error::Config(_)));
    }

    #[test]
    fn test_parse_graphql_response_success() {
        #[derive(Debug, Deserialize)]
        struct Data {
            value: i64,
        }

        let text = "{\"data\": {\"value\": 9}}".to_string();
        let parsed = parse_graphql_response::<Data>(StatusCode::OK, text).unwrap();
        assert_eq!(parsed.data.unwrap().value, 9);
    }

    #[test]
    fn test_parse_graphql_response_graphql_error() {
        let text = "{\"data\": null, \"errors\": [{\"message\": \"boom\"}]}".to_string();
        let err = parse_graphql_response::<serde_json::Value>(StatusCode::OK, text).unwrap_err();
        assert!(matches!(
            err,
            Error::GraphQl {
                status: Some(200),
                ..
            }
        ));
    }

    #[test]
    fn test_parse_graphql_response_http_error() {
        let text = "{\"data\": null}".to_string();
        let err =
            parse_graphql_response::<serde_json::Value>(StatusCode::BAD_GATEWAY, text).unwrap_err();
        assert!(matches!(
            err,
            Error::GraphQl {
                status: Some(502),
                ..
            }
        ));
    }

    #[test]
    fn test_parse_schema_response_error() {
        let err = parse_schema_response(StatusCode::NOT_FOUND, "nope".to_string()).unwrap_err();
        assert!(matches!(
            err,
            Error::GraphQl {
                status: Some(404),
                ..
            }
        ));
    }
}
