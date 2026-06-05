//! main client
//!
//! includes helpers for raw graphql execution, typed responses, and schema fetch.

use crate::config::ClientConfig;
use crate::error::{Error, Result};
use crate::graphql::GraphQlResponse;
use crate::operation::Operation;
use crate::upload::FileUpload;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::multipart;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

/// base delay in milliseconds for exponential backoff (attempt 1 = 200ms)
const RETRY_BASE_MS: u64 = 200;
/// maximum jitter added to a single retry delay, in milliseconds
const RETRY_MAX_JITTER_MS: u64 = 500;
/// hard ceiling for any single retry delay (prevents unbounded growth)
const RETRY_MAX_BACKOFF: Duration = Duration::from_secs(30);

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

        let http = if let Some(http) = config.http_client.clone() {
            http
        } else {
            let mut headers = HeaderMap::new();
            headers.insert(
                "X-INFRAHUB-KEY",
                HeaderValue::from_str(&config.token).map_err(|err| {
                    Error::Config(format!("invalid api token header value: {err}"))
                })?,
            );
            headers.extend(config.extra_headers.clone());

            let builder = reqwest::Client::builder()
                .default_headers(headers)
                .user_agent(config.user_agent.clone())
                .timeout(config.timeout)
                .danger_accept_invalid_certs(!config.verify_ssl);

            let builder = if let Some(customize) = &config.http_client_builder {
                customize(builder)
            } else {
                builder
            };

            builder.build()?
        };

        Ok(Self {
            config: Arc::new(config),
            http,
        })
    }

    /// access the client configuration
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// execute a raw graphql query and return the untyped json response, retrying on transient errors
    pub async fn execute_raw(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        branch: Option<&str>,
    ) -> Result<GraphQlResponse<serde_json::Value>> {
        self.execute(query, variables, branch).await
    }

    /// execute a graphql query and deserialize into a typed response, retrying on transient errors
    pub async fn execute<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        branch: Option<&str>,
    ) -> Result<GraphQlResponse<T>> {
        let url = self.config.graphql_url(branch)?;
        let body = serde_json::json!({
            "query": query,
            "variables": variables.unwrap_or_else(|| serde_json::json!({})),
        });
        self.retry_loop(|| {
            let url = url.clone();
            let body = body.clone();
            async move {
                let response = self.http.post(url).json(&body).send().await?;
                let status = response.status();
                let text = response.text().await?;
                parse_graphql_response(status, text)
            }
        })
        .await
    }

    /// execute a generated operation by name, retrying on transient errors
    pub async fn execute_operation<O: Operation>(
        &self,
        variables: Option<serde_json::Value>,
        branch: Option<&str>,
    ) -> Result<GraphQlResponse<O::Response>> {
        self.execute(O::QUERY, variables, branch).await
    }

    /// fetch the graphql schema as text
    pub async fn fetch_schema(&self, branch: Option<&str>) -> Result<String> {
        let url = self.config.schema_url(branch)?;
        self.retry_loop(|| {
            let url = url.clone();
            async move {
                let response = self.http.get(url).send().await?;
                let status = response.status();
                let text = response.text().await?;
                parse_schema_response(status, text)
            }
        })
        .await
    }

    /// execute a graphql mutation with file uploads per the
    /// [graphql multipart request spec](https://github.com/jaydenseric/graphql-multipart-request-spec),
    /// retrying on transient errors.
    ///
    /// `files` maps variable paths (e.g. `"file"`) to [`FileUpload`] values.
    pub async fn execute_multipart<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        files: Vec<(&str, FileUpload)>,
        branch: Option<&str>,
    ) -> Result<GraphQlResponse<T>> {
        let url = self.config.graphql_url(branch)?;
        let owned_files: Vec<(String, FileUpload)> =
            files.into_iter().map(|(k, v)| (k.to_owned(), v)).collect();
        self.retry_loop(|| {
            let url = url.clone();
            let variables = variables.clone();
            let files_for_attempt: Vec<(&str, FileUpload)> = owned_files
                .iter()
                .map(|(k, v)| (k.as_str(), v.clone()))
                .collect();
            async move {
                let form = build_multipart_form(query, variables, files_for_attempt)?;
                let response = self.http.post(url).multipart(form).send().await?;
                let status = response.status();
                let text = response.text().await?;
                parse_graphql_response(status, text)
            }
        })
        .await
    }

    /// download a file by node id
    pub async fn download_file(&self, node_id: &str, branch: Option<&str>) -> Result<Vec<u8>> {
        let url = self.config.file_url(node_id, branch)?;
        self.download_bytes(url).await
    }

    /// download a file by human-friendly id
    pub async fn download_file_by_hfid(
        &self,
        kind: &str,
        hfid: &[&str],
        branch: Option<&str>,
    ) -> Result<Vec<u8>> {
        let url = self.config.file_by_hfid_url(kind, hfid, branch)?;
        self.download_bytes(url).await
    }

    /// download a file by internal storage id
    pub async fn download_file_by_storage_id(
        &self,
        storage_id: &str,
        branch: Option<&str>,
    ) -> Result<Vec<u8>> {
        let url = self.config.file_by_storage_id_url(storage_id, branch)?;
        self.download_bytes(url).await
    }

    /// shared download helper — GET the given url and return response bytes
    async fn download_bytes(&self, url: Url) -> Result<Vec<u8>> {
        self.retry_loop(|| {
            let url = url.clone();
            async move {
                let response = self.http.get(url).send().await?;
                if !response.status().is_success() {
                    let status = response.status();
                    let body = response.text().await?;
                    return Err(Error::GraphQl {
                        status: Some(status.as_u16()),
                        errors: Vec::new(),
                        body,
                        message: format!("file download error: {status}"),
                    });
                }
                Ok(response.bytes().await?.to_vec())
            }
        })
        .await
    }
}

impl Client {
    async fn retry_loop<T, F, Fut>(&self, mut operation: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let mut attempts = 0;
        loop {
            let result = operation().await;
            match result {
                Ok(value) => return Ok(value),
                Err(err) => {
                    if attempts >= self.config.max_retries || !err.is_retryable() {
                        return Err(err);
                    }
                    attempts += 1;
                    let delay = Self::retry_delay(attempts);
                    sleep(delay).await;
                }
            }
        }
    }

    fn retry_delay(attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::from_millis(0);
        }

        let exp = attempt.saturating_sub(1);
        let backoff_ms = RETRY_BASE_MS.saturating_mul(2u64.saturating_pow(exp));
        let jitter = (backoff_ms / 4).min(RETRY_MAX_JITTER_MS);
        let offset = if jitter == 0 {
            0
        } else {
            Self::jitter_seed(attempt) % (jitter + 1)
        };
        let delay = Duration::from_millis(backoff_ms.saturating_add(offset));
        delay.min(RETRY_MAX_BACKOFF)
    }

    fn jitter_seed(attempt: u32) -> u64 {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64)
            .unwrap_or_else(|_| std::process::id() as u64);
        nanos.wrapping_mul(31).wrapping_add(attempt as u64)
    }
}

fn parse_graphql_response<T: DeserializeOwned>(
    status: StatusCode,
    text: String,
) -> Result<GraphQlResponse<T>> {
    let parsed: GraphQlResponse<T> = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(json_err) => {
            if !status.is_success() {
                return Err(Error::GraphQl {
                    status: Some(status.as_u16()),
                    errors: Vec::new(),
                    body: text,
                    message: format!("http {status}: non-JSON response"),
                });
            }
            return Err(json_err.into());
        }
    };
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

/// build a multipart form per the graphql multipart request spec.
///
/// the spec requires three named parts:
/// - `operations`: the json body with `null` placeholders for file variables
/// - `map`: a json object mapping part names ("0", "1", ...) to variable paths
/// - one part per file, named "0", "1", etc.
fn build_multipart_form(
    query: &str,
    variables: Option<serde_json::Value>,
    files: Vec<(&str, FileUpload)>,
) -> Result<multipart::Form> {
    let mut vars = variables.unwrap_or_else(|| serde_json::json!({}));
    let mut map = serde_json::Map::new();

    for (idx, (var_path, _)) in files.iter().enumerate() {
        if let Some(obj) = vars.as_object_mut() {
            obj.insert(var_path.to_string(), serde_json::Value::Null);
        }
        map.insert(
            idx.to_string(),
            serde_json::json!([format!("variables.{}", var_path)]),
        );
    }

    let operations = serde_json::json!({
        "query": query,
        "variables": vars,
    });

    let mut form = multipart::Form::new()
        .text("operations", operations.to_string())
        .text("map", serde_json::Value::Object(map).to_string());

    for (idx, (_, file)) in files.into_iter().enumerate() {
        let part = multipart::Part::bytes(file.data)
            .file_name(file.filename)
            .mime_str(&file.content_type)?;
        form = form.part(idx.to_string(), part);
    }

    Ok(form)
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

#[cfg(test)]
impl Client {
    async fn execute_multipart_with<T: DeserializeOwned, F, Fut>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        files: Vec<(&str, FileUpload)>,
        branch: Option<&str>,
        send: F,
    ) -> Result<GraphQlResponse<T>>
    where
        F: FnOnce(Url, multipart::Form) -> Fut,
        Fut: Future<Output = Result<(StatusCode, String)>>,
    {
        let url = self.config.graphql_url(branch)?;
        let form = build_multipart_form(query, variables, files)?;
        let (status, text) = send(url, form).await?;
        parse_graphql_response(status, text)
    }

    async fn execute_with<T: DeserializeOwned, F, Fut>(
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

    async fn fetch_schema_with<F, Fut>(&self, branch: Option<&str>, send: F) -> Result<String>
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
            .execute_with::<serde_json::Value, _, _>(
                "query { ok }",
                None,
                Some("main"),
                |url, body| async move {
                    assert_eq!(url.path(), "/graphql/main");
                    assert_eq!(body["query"], "query { ok }");
                    Ok((StatusCode::OK, "{\"data\": {\"ok\": true}}".to_string()))
                },
            )
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
            .execute_with::<serde_json::Value, _, _>(
                "query { ok }",
                None,
                None,
                |_url, _body| async move {
                    Ok((
                        StatusCode::OK,
                        "{\"data\": null, \"errors\": [{\"message\": \"boom\"}]}".to_string(),
                    ))
                },
            )
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
            .execute_with::<serde_json::Value, _, _>(
                "query { ok }",
                None,
                None,
                |_url, _body| async move {
                    Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "{\"data\":null}".to_string(),
                    ))
                },
            )
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
            .execute_with::<serde_json::Value, _, _>(
                "query { ok }",
                None,
                None,
                |url, _body| async move {
                    assert_eq!(url.path(), "/graphql/main");
                    Ok((StatusCode::OK, "{\"data\": {\"ok\": true}}".to_string()))
                },
            )
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

    #[test]
    fn test_new_with_prebuilt_http_client() {
        let prebuilt = reqwest::Client::new();
        let config = ClientConfig::new("http://localhost:1234", "token").with_http_client(prebuilt);
        let client = Client::new(config).expect("client with prebuilt http");
        assert!(client.config().http_client.is_some());
    }

    #[test]
    fn test_new_with_http_client_builder() {
        let config = ClientConfig::new("http://localhost:1234", "token")
            .with_http_client_builder(|b| b.connection_verbose(true));
        let client = Client::new(config).expect("client with builder callback");
        assert!(client.config().http_client_builder.is_some());
    }

    #[test]
    fn test_new_rejects_empty_token() {
        let config = ClientConfig::new("http://localhost:1234", "");
        let err = Client::new(config).err().expect("expected error");
        assert!(matches!(err, Error::Config(_)));
    }

    #[test]
    fn test_new_accepts_empty_token_with_prebuilt_client() {
        let config =
            ClientConfig::new("http://localhost:1234", "").with_http_client(reqwest::Client::new());
        assert!(Client::new(config).is_ok());
    }

    #[test]
    fn test_new_trailing_slash_url() {
        let config = ClientConfig::new("http://localhost:1234/", "token");
        let client = Client::new(config).unwrap();
        let url = client.config().graphql_url(None).unwrap();
        assert_eq!(url.as_str(), "http://localhost:1234/graphql");
    }

    #[test]
    fn test_new_schemeless_url_gets_https() {
        let config = ClientConfig::new("infrahub.example.com", "token");
        let client = Client::new(config).unwrap();
        assert_eq!(client.config().base_url.scheme(), "https");
    }

    #[test]
    fn test_parse_graphql_response_malformed_json() {
        let err = parse_graphql_response::<serde_json::Value>(
            StatusCode::OK,
            "not json at all".to_string(),
        )
        .unwrap_err();
        assert!(matches!(err, Error::Json(_)));
    }

    #[test]
    fn test_parse_graphql_response_non_json_error_body() {
        let html = "<html><body><h1>502 Bad Gateway</h1></body></html>".to_string();
        let err =
            parse_graphql_response::<serde_json::Value>(StatusCode::BAD_GATEWAY, html.clone())
                .unwrap_err();
        match err {
            Error::GraphQl {
                status,
                body,
                message,
                ..
            } => {
                assert_eq!(status, Some(502));
                assert_eq!(body, html);
                assert!(
                    message.contains("502"),
                    "message should contain status code"
                );
            }
            other => panic!("expected GraphQl error, got: {other:?}"),
        }
    }

    #[test]
    fn test_parse_graphql_response_error_preserves_message() {
        let text =
            r#"{"data": null, "errors": [{"message": "field 'name' not found on type 'Device'"}]}"#
                .to_string();
        let err = parse_graphql_response::<serde_json::Value>(StatusCode::OK, text).unwrap_err();
        match err {
            Error::GraphQl {
                message, status, ..
            } => {
                assert_eq!(message, "field 'name' not found on type 'Device'");
                assert_eq!(status, Some(200));
            }
            other => panic!("expected GraphQl error, got: {other:?}"),
        }
    }

    #[test]
    fn test_build_multipart_form_with_no_variables() {
        let file = FileUpload::new("test.txt", "text/plain", b"hello".to_vec());
        let form = build_multipart_form("mutation { upload }", None, vec![("file", file)]);
        assert!(form.is_ok());
    }

    #[test]
    fn test_build_multipart_form_multiple_files() {
        let files = vec![
            (
                "avatar",
                FileUpload::new("avatar.png", "image/png", vec![1, 2, 3]),
            ),
            (
                "document",
                FileUpload::new("doc.pdf", "application/pdf", vec![4, 5]),
            ),
        ];
        let form = build_multipart_form(
            "mutation Upload($avatar: Upload!, $document: Upload!) { upload }",
            Some(serde_json::json!({"avatar": "placeholder", "document": "placeholder"})),
            files,
        );
        assert!(form.is_ok());
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_raw_none_variables_become_empty_object() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        client
            .execute_with::<serde_json::Value, _, _>(
                "query { ok }",
                None,
                None,
                |_url, body| async move {
                    assert_eq!(body["variables"], serde_json::json!({}));
                    Ok((StatusCode::OK, r#"{"data": {"ok": true}}"#.to_string()))
                },
            )
            .await
            .unwrap();
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_raw_passes_variables() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        let vars = serde_json::json!({"id": "abc-123", "name": "test"});
        let response = client
            .execute_with::<serde_json::Value, _, _>(
                "query GetDevice($id: String!) { device(id: $id) { name } }",
                Some(vars),
                None,
                |_url, body| async move {
                    assert_eq!(body["variables"]["id"], "abc-123");
                    assert_eq!(body["variables"]["name"], "test");
                    Ok((
                        StatusCode::OK,
                        r#"{"data": {"device": {"name": "test"}}}"#.to_string(),
                    ))
                },
            )
            .await
            .unwrap();
        assert!(response.data.is_some());
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_server_error_is_retryable() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        let err = client
            .execute_with::<serde_json::Value, _, _>(
                "query { ok }",
                None,
                None,
                |_url, _body| async move {
                    Ok((
                        StatusCode::SERVICE_UNAVAILABLE,
                        r#"{"data": null}"#.to_string(),
                    ))
                },
            )
            .await
            .unwrap_err();
        assert!(err.is_retryable());
        assert!(!err.is_auth_error());
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_client_error_not_retryable() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        let err = client
            .execute_with::<serde_json::Value, _, _>(
                "query { ok }",
                None,
                None,
                |_url, _body| async move {
                    Ok((StatusCode::BAD_REQUEST, r#"{"data": null}"#.to_string()))
                },
            )
            .await
            .unwrap_err();
        assert!(!err.is_retryable());
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_auth_error_not_retryable() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        let err = client
            .execute_with::<serde_json::Value, _, _>(
                "query { ok }",
                None,
                None,
                |_url, _body| async move {
                    Ok((
                        StatusCode::UNAUTHORIZED,
                        r#"{"data": null, "errors": [{"message": "unauthorized"}]}"#.to_string(),
                    ))
                },
            )
            .await
            .unwrap_err();
        assert!(err.is_auth_error());
        assert!(!err.is_retryable());
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_execute_multipart_sends_form() {
        let config = ClientConfig::new("http://localhost:1234", "test-token");
        let client = test_client(config);
        let file = FileUpload::new("test.txt", "text/plain", b"hello".to_vec());
        let response = client
            .execute_multipart_with::<serde_json::Value, _, _>(
                "mutation Upload($file: Upload!) { upload(file: $file) { ok } }",
                None,
                vec![("file", file)],
                Some("main"),
                |url, _form| async move {
                    assert_eq!(url.path(), "/graphql/main");
                    Ok((
                        StatusCode::OK,
                        r#"{"data": {"upload": {"ok": true}}}"#.to_string(),
                    ))
                },
            )
            .await
            .unwrap();
        assert!(response.data.is_some());
    }

    #[test]
    fn test_retry_delay_backoff() {
        assert_eq!(Client::retry_delay(0), Duration::from_millis(0));
        let delay1 = Client::retry_delay(1).as_millis();
        let delay2 = Client::retry_delay(2).as_millis();
        let delay3 = Client::retry_delay(3).as_millis();
        assert!(
            (200..=250).contains(&delay1),
            "attempt 1 delay {delay1}ms outside expected 200..=250"
        );
        assert!(
            (400..=500).contains(&delay2),
            "attempt 2 delay {delay2}ms outside expected 400..=500"
        );
        assert!(
            (800..=1000).contains(&delay3),
            "attempt 3 delay {delay3}ms outside expected 800..=1000"
        );
    }

    #[test]
    fn test_retry_delay_capped_at_max() {
        // Raw backoff at attempt 9 is ~51s, at attempt 30 is ~107 billion ms.
        // Both exceed the 30s cap, so the delay must equal exactly RETRY_MAX_BACKOFF.
        for attempt in [9, 15, 30] {
            let delay = Client::retry_delay(attempt);
            assert_eq!(
                delay, RETRY_MAX_BACKOFF,
                "attempt {attempt} delay {delay:?} should hit cap {RETRY_MAX_BACKOFF:?}"
            );
        }
    }

    #[test]
    fn test_jitter_seed_varies_by_attempt() {
        let s1 = Client::jitter_seed(1);
        let s2 = Client::jitter_seed(2);
        assert_ne!(s1, s2, "jitter seed should differ across attempts");
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_retry_loop_succeeds_after_transient_errors() {
        let config = ClientConfig::new("http://localhost:1234", "test-token").with_max_retries(3);
        let client = test_client(config);
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let count = call_count.clone();
        let result: Result<String> = client
            .retry_loop(|| {
                let count = count.clone();
                async move {
                    let n = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    if n < 2 {
                        Err(Error::GraphQl {
                            status: Some(503),
                            errors: vec![],
                            body: String::new(),
                            message: "service unavailable".to_string(),
                        })
                    } else {
                        Ok("ok".to_string())
                    }
                }
            })
            .await;
        assert_eq!(result.unwrap(), "ok");
        assert_eq!(
            call_count.load(std::sync::atomic::Ordering::SeqCst),
            3,
            "should have been called 3 times (1 initial + 2 retries)"
        );
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_retry_loop_gives_up_after_max_retries() {
        let config = ClientConfig::new("http://localhost:1234", "test-token").with_max_retries(2);
        let client = test_client(config);
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let count = call_count.clone();
        let result: Result<String> = client
            .retry_loop(|| {
                let count = count.clone();
                async move {
                    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    Err(Error::GraphQl {
                        status: Some(500),
                        errors: vec![],
                        body: String::new(),
                        message: "server error".to_string(),
                    })
                }
            })
            .await;
        assert!(result.is_err());
        assert_eq!(
            call_count.load(std::sync::atomic::Ordering::SeqCst),
            3,
            "should have been called 3 times (1 initial + 2 retries)"
        );
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_retry_loop_does_not_retry_permanent_errors() {
        let config = ClientConfig::new("http://localhost:1234", "test-token").with_max_retries(3);
        let client = test_client(config);
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let count = call_count.clone();
        let result: Result<String> = client
            .retry_loop(|| {
                let count = count.clone();
                async move {
                    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    Err(Error::GraphQl {
                        status: Some(400),
                        errors: vec![],
                        body: String::new(),
                        message: "bad request".to_string(),
                    })
                }
            })
            .await;
        assert!(result.is_err());
        assert_eq!(
            call_count.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "permanent errors should not be retried"
        );
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_retry_loop_disabled_with_zero_retries() {
        let config = ClientConfig::new("http://localhost:1234", "test-token").with_max_retries(0);
        let client = test_client(config);
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let count = call_count.clone();
        let result: Result<String> = client
            .retry_loop(|| {
                let count = count.clone();
                async move {
                    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    Err(Error::GraphQl {
                        status: Some(503),
                        errors: vec![],
                        body: String::new(),
                        message: "service unavailable".to_string(),
                    })
                }
            })
            .await;
        assert!(result.is_err());
        assert_eq!(
            call_count.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "with max_retries=0 should only try once"
        );
    }
}
