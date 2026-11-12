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
///
/// all requests go through a retry loop (transient network errors, 429,
/// and retryable 5xx, with backoff). mutations, documents that don't parse
/// and file uploads are sent once and never retried unless
/// [`ClientConfig::with_mutation_retries`] is set.
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

    /// execute a raw graphql query and return the untyped json response
    pub async fn execute_raw(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        branch: Option<&str>,
    ) -> Result<GraphQlResponse<serde_json::Value>> {
        self.execute(query, variables, branch).await
    }

    /// execute a graphql query and deserialize into a typed response
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
        let resend = self.config.retry_mutations || may_resend(query);
        self.request(
            resend,
            || Ok(self.http.post(url.clone()).json(&body)),
            reqwest::Response::text,
            parse_graphql_response,
        )
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
        let url = self.config.schema_url(branch)?;
        self.request(
            true,
            || Ok(self.http.get(url.clone())),
            reqwest::Response::text,
            parse_schema_response,
        )
        .await
    }

    /// fetch `/api/schema` and deserialize the schema snapshot json.
    pub async fn fetch_schema_snapshot<T: DeserializeOwned>(
        &self,
        branch: Option<&str>,
    ) -> Result<T> {
        let url = self.config.schema_snapshot_url(branch)?;
        self.request(
            true,
            || Ok(self.http.get(url.clone())),
            reqwest::Response::text,
            parse_schema_snapshot_response,
        )
        .await
    }

    /// execute a graphql mutation with file uploads per the
    /// [graphql multipart request spec](https://github.com/jaydenseric/graphql-multipart-request-spec).
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
        self.request(
            self.config.retry_mutations,
            || {
                let files = files.iter().map(|(name, file)| (*name, file.clone()));
                let form = build_multipart_form(query, variables.clone(), files.collect())?;
                Ok(self.http.post(url.clone()).multipart(form))
            },
            reqwest::Response::text,
            parse_graphql_response,
        )
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
        self.request(
            true,
            || Ok(self.http.get(url.clone())),
            read_download,
            parse_download,
        )
        .await
    }
}

impl Client {
    /// sends what `build` returns through the retry loop and hands the status and the body `read` returns to `parse`
    async fn request<B, T, R, Fut>(
        &self,
        resend: bool,
        build: impl Fn() -> Result<reqwest::RequestBuilder>,
        read: R,
        parse: impl Fn(StatusCode, B) -> Result<T>,
    ) -> Result<T>
    where
        R: Fn(reqwest::Response) -> Fut,
        Fut: Future<Output = reqwest::Result<B>>,
    {
        let (build, read, parse) = (&build, &read, &parse);
        self.retry_loop(resend, || async move {
            let response = build()?.send().await?;
            let status = response.status();
            let retry_after = parse_retry_after(response.headers());
            let body = read(response).await?;
            parse(status, body).map_err(|inner| RetryError { inner, retry_after })
        })
        .await
    }

    /// retries `operation` after a retryable error; with `resend` false it runs once
    async fn retry_loop<T, F, Fut>(&self, resend: bool, mut operation: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = std::result::Result<T, RetryError>>,
    {
        let mut attempts = 0;
        loop {
            match operation().await {
                Ok(value) => return Ok(value),
                Err(retry_err) => {
                    if !resend
                        || attempts >= self.config.max_retries
                        || !retry_err.inner.is_retryable()
                    {
                        return Err(retry_err.inner);
                    }
                    attempts += 1;
                    let delay =
                        Self::next_delay(Self::retry_delay(attempts), retry_err.retry_after);
                    sleep(delay).await;
                }
            }
        }
    }

    fn next_delay(backoff: Duration, hint: Option<Duration>) -> Duration {
        hint.map_or(backoff, |hint| hint.max(backoff))
            .min(RETRY_MAX_BACKOFF)
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

/// whether the document parses and has no mutation, so a resend can't apply anything twice
fn may_resend(document: &str) -> bool {
    use graphql_parser::query::{Definition, OperationDefinition};

    graphql_parser::parse_query::<&str>(document).is_ok_and(|doc| {
        !doc.definitions.iter().any(|definition| {
            matches!(
                definition,
                Definition::Operation(OperationDefinition::Mutation(_))
            )
        })
    })
}

struct RetryError {
    inner: Error,
    retry_after: Option<Duration>,
}

impl From<Error> for RetryError {
    fn from(inner: Error) -> Self {
        RetryError {
            inner,
            retry_after: None,
        }
    }
}

impl From<reqwest::Error> for RetryError {
    fn from(err: reqwest::Error) -> Self {
        RetryError {
            inner: Error::from(err),
            retry_after: None,
        }
    }
}

// only the delta-seconds form is parsed; http-date and non-integer values fall back to backoff (no date dependency)
fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(reqwest::header::RETRY_AFTER)?;
    let seconds: u64 = value.to_str().ok()?.trim().parse().ok()?;
    Some(Duration::from_secs(seconds))
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

fn parse_schema_snapshot_response<T: DeserializeOwned>(
    status: StatusCode,
    text: String,
) -> Result<T> {
    if !status.is_success() {
        return Err(Error::GraphQl {
            status: Some(status.as_u16()),
            errors: Vec::new(),
            body: text,
            message: format!("schema snapshot http error: {}", status),
        });
    }

    Ok(serde_json::from_str(&text)?)
}

/// the raw bytes of a successful download, the text of any other response
async fn read_download(
    response: reqwest::Response,
) -> reqwest::Result<std::result::Result<Vec<u8>, String>> {
    if response.status().is_success() {
        Ok(Ok(response.bytes().await?.to_vec()))
    } else {
        response.text().await.map(Err)
    }
}

fn parse_download(
    status: StatusCode,
    body: std::result::Result<Vec<u8>, String>,
) -> Result<Vec<u8>> {
    body.map_err(|body| Error::GraphQl {
        status: Some(status.as_u16()),
        errors: Vec::new(),
        body,
        message: format!("file download error: {status}"),
    })
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

    #[test]
    fn test_parse_schema_snapshot_response() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Snapshot {
            version: u64,
        }

        let parsed = parse_schema_snapshot_response::<Snapshot>(
            StatusCode::OK,
            r#"{"version": 1}"#.to_string(),
        )
        .unwrap();
        assert_eq!(parsed, Snapshot { version: 1 });

        let err = parse_schema_snapshot_response::<Snapshot>(
            StatusCode::TOO_MANY_REQUESTS,
            "slow down".to_string(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            Error::GraphQl {
                status: Some(429),
                ..
            }
        ));
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
        // raw backoff at attempt 9 is ~51s, at attempt 30 is ~107 billion ms.
        // both exceed the 30s cap, so the delay must equal exactly RETRY_MAX_BACKOFF.
        for attempt in [9, 15, 30] {
            let delay = Client::retry_delay(attempt);
            assert_eq!(
                delay, RETRY_MAX_BACKOFF,
                "attempt {attempt} delay {delay:?} should hit cap {RETRY_MAX_BACKOFF:?}"
            );
        }
    }

    fn headers_with_retry_after(value: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::RETRY_AFTER,
            HeaderValue::from_str(value).unwrap(),
        );
        headers
    }

    #[test]
    fn test_parse_retry_after() {
        // (header value, expected); None means the header is absent
        for (value, expected) in [
            (Some("5"), Some(Duration::from_secs(5))),
            (Some("0"), Some(Duration::from_secs(0))),
            (Some("  10 "), Some(Duration::from_secs(10))),
            (None, None),
            (Some(""), None),
            (Some("soon"), None),
            (Some("Wed, 21 Oct 2015 07:28:00 GMT"), None),
            (Some("100000"), Some(Duration::from_secs(100_000))),
        ] {
            let headers = value.map_or_else(HeaderMap::new, headers_with_retry_after);
            assert_eq!(parse_retry_after(&headers), expected, "{value:?}");
        }
    }

    #[test]
    fn test_next_delay() {
        let base = Duration::from_millis(RETRY_BASE_MS);
        let cap = RETRY_MAX_BACKOFF;
        // (backoff, retry-after hint, expected delay)
        for (backoff, hint, expected) in [
            (base, None, base),
            (base, Some(Duration::ZERO), base),
            (base, Some(base * 5), base * 5),
            (base, Some(cap * 2), cap),
            (cap, Some(base), cap),
        ] {
            assert_eq!(
                Client::next_delay(backoff, hint),
                expected,
                "backoff {backoff:?}, hint {hint:?}"
            );
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_retry_loop() {
        // (label, resend, max_retries, status, failures before success, succeeds, expected calls)
        for (label, resend, max_retries, status, failures, succeeds, expected_calls) in [
            ("transient errors then success", true, 3, 503, 2, true, 3),
            (
                "gives up after max_retries",
                true,
                2,
                500,
                u32::MAX,
                false,
                3,
            ),
            (
                "permanent errors are not retried",
                true,
                3,
                400,
                u32::MAX,
                false,
                1,
            ),
            (
                "max_retries=0 disables retries",
                true,
                0,
                503,
                u32::MAX,
                false,
                1,
            ),
            ("no resend after a 503", false, 3, 503, u32::MAX, false, 1),
            ("no resend after a 429", false, 3, 429, u32::MAX, false, 1),
        ] {
            let config = ClientConfig::new("http://localhost:1234", "test-token")
                .with_max_retries(max_retries);
            let client = test_client(config);
            let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
            let count = call_count.clone();
            let result: Result<String> = client
                .retry_loop(resend, || {
                    let count = count.clone();
                    async move {
                        let n = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        if n < failures {
                            Err(Error::GraphQl {
                                status: Some(status),
                                errors: vec![],
                                body: String::new(),
                                message: "error".to_string(),
                            }
                            .into())
                        } else {
                            Ok("ok".to_string())
                        }
                    }
                })
                .await;
            if succeeds {
                assert_eq!(result.unwrap(), "ok", "{label}");
            } else {
                assert!(
                    matches!(result, Err(Error::GraphQl { status: Some(s), .. }) if s == status),
                    "{label}: {result:?}"
                );
            }
            assert_eq!(
                call_count.load(std::sync::atomic::Ordering::SeqCst),
                expected_calls,
                "{label}: calls"
            );
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_retry_loop_does_not_retry_invalid_content_type() {
        let config = ClientConfig::new("http://localhost:1234", "test-token").with_max_retries(3);
        let client = test_client(config);
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let count = call_count.clone();
        let result = client
            .retry_loop(true, || {
                let count = count.clone();
                async move {
                    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let file = FileUpload::new("test.txt", "text", b"hello".to_vec());
                    build_multipart_form("mutation", None, vec![("file", file)])
                        .map_err(RetryError::from)
                }
            })
            .await;
        assert!(matches!(result, Err(Error::Http(ref err)) if err.is_builder()));
        assert_eq!(
            call_count.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "builder errors should not be retried"
        );
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_redirect_loop_not_retried() {
        use std::io::{BufRead, Write};

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let requests = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let seen = requests.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                seen.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let mut reader = std::io::BufReader::new(&stream);
                let mut line = String::new();
                while reader.read_line(&mut line).is_ok_and(|n| n > 2) {
                    line.clear();
                }
                let _ = write!(
                    &stream,
                    "HTTP/1.1 302 Found\r\nLocation: http://{addr}/\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                );
            }
        });

        let config = ClientConfig::new(format!("http://{addr}"), "test-token").with_max_retries(3);
        let client = test_client(config);
        let err = client.fetch_schema(None).await.unwrap_err();
        assert!(
            matches!(err, Error::Http(ref e) if e.is_redirect()),
            "{err}"
        );
        assert_eq!(
            requests.load(std::sync::atomic::Ordering::SeqCst),
            11,
            "one attempt of 1 request + 10 redirects"
        );
    }
    #[test]
    fn test_may_resend() {
        for (document, expected) in [
            ("{ ok }", true),
            ("query { ok }", true),
            ("query Get($id: ID!) { node(id: $id) { id } }", true),
            ("query A { a } query B { b }", true),
            ("subscription { ok }", true),
            ("fragment F on T { a }", true),
            (r#"query { a(s: "mutation { b }") }"#, true),
            ("# mutation { b }\nquery { ok }", true),
            ("query { mutation: ok mutation }", true),
            ("mutation { create { ok } }", false),
            (
                "mutation Delete($id: ID!) { delete(id: $id) { ok } }",
                false,
            ),
            ("fragment F on T { a } mutation { ...F }", false),
            ("query { a } mutation { b }", false),
            ("query { ok } # x\rmutation { b }", false),
            ("query { ok", false),
            (r#""creates a tag" mutation { create { ok } }"#, false),
        ] {
            assert_eq!(may_resend(document), expected, "{document:?}");
        }
    }

    /// reads a request in full and returns it, or `None` if the connection ends first
    fn read_request(stream: &std::net::TcpStream) -> Option<String> {
        use std::io::{BufRead, Read};

        let mut reader = std::io::BufReader::new(stream);
        let mut head = String::new();
        while reader.read_line(&mut head).is_ok_and(|n| n > 2) {}
        let length = head
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().unwrap())
            })
            .unwrap_or(0);
        let mut body = vec![0; length];
        reader.read_exact(&mut body).ok()?;
        Some(head + &String::from_utf8_lossy(&body))
    }

    /// answers the nth request with `replies[n]`, or the last reply once they run out, and records every request
    fn serve(replies: Vec<String>) -> (std::net::SocketAddr, Arc<std::sync::Mutex<Vec<String>>>) {
        use std::io::Write;

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let requests = Arc::new(std::sync::Mutex::new(Vec::new()));
        let seen = requests.clone();
        std::thread::spawn(move || {
            for mut stream in listener.incoming().flatten() {
                let Some(request) = read_request(&stream) else {
                    continue;
                };
                let n = {
                    let mut seen = seen.lock().unwrap();
                    seen.push(request);
                    seen.len().min(replies.len())
                };
                let _ = stream.write_all(replies[n - 1].as_bytes());
            }
        });
        (addr, requests)
    }

    fn reply(status: u16, headers: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {status} Reply\r\n{headers}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    }

    #[derive(Clone, Copy, Debug)]
    enum Call {
        Execute,
        Schema,
        Snapshot,
        Upload,
        File,
        FileByHfid,
        FileByStorageId,
    }

    /// runs `call` and renders what it returned
    async fn run(client: &Client, call: Call) -> Result<String> {
        let data = |response: GraphQlResponse<serde_json::Value>| {
            response.data.unwrap_or_default().to_string()
        };
        let text = |bytes: Vec<u8>| String::from_utf8(bytes).unwrap();
        match call {
            Call::Execute => client
                .execute_raw("query { ok }", None, None)
                .await
                .map(data),
            Call::Schema => client.fetch_schema(Some("dev")).await,
            Call::Snapshot => client
                .fetch_schema_snapshot::<serde_json::Value>(None)
                .await
                .map(|snapshot| snapshot.to_string()),
            Call::Upload => {
                let file = FileUpload::new("a.txt", "text/plain", b"hello".to_vec());
                let upload = "mutation($file: Upload!) { upload(file: $file) { ok } }";
                client
                    .execute_multipart(upload, None, vec![("file", file)], None)
                    .await
                    .map(data)
            }
            Call::File => client.download_file("f1", None).await.map(text),
            Call::FileByHfid => client
                .download_file_by_hfid("CoreFile", &["a", "b"], None)
                .await
                .map(text),
            Call::FileByStorageId => client
                .download_file_by_storage_id("s1", None)
                .await
                .map(text),
        }
    }

    type Case<'a> = (
        Call,
        u16,
        &'a str,
        &'a str,
        &'a [&'a str],
        std::result::Result<&'a str, (u16, &'a str)>,
    );

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_public_methods_end_to_end() {
        let query = r#"{"query":"query { ok }","variables":{}}"#;
        let operations = r#"{"query":"mutation($file: Upload!) { upload(file: $file) { ok } }","variables":{"file":null}}"#;
        let graphql = "POST /graphql/main HTTP/1.1";
        let file = "GET /api/files/f1?branch=main HTTP/1.1";
        // (call, status, response body, request line, request body parts, result or (status, message))
        let cases: [Case; 11] = [
            (
                Call::Execute,
                200,
                r#"{"data":{"ok":true}}"#,
                graphql,
                &[query],
                Ok(r#"{"ok":true}"#),
            ),
            (
                Call::Execute,
                400,
                r#"{"errors":[{"message":"boom"}]}"#,
                graphql,
                &[query],
                Err((400, "boom")),
            ),
            (
                Call::Schema,
                200,
                "type Query",
                "GET /schema.graphql?branch=dev HTTP/1.1",
                &[],
                Ok("type Query"),
            ),
            (
                Call::Schema,
                404,
                "nope",
                "GET /schema.graphql?branch=dev HTTP/1.1",
                &[],
                Err((404, "schema http error: 404 Not Found")),
            ),
            (
                Call::Snapshot,
                200,
                r#"{"version":1}"#,
                "GET /api/schema?branch=main HTTP/1.1",
                &[],
                Ok(r#"{"version":1}"#),
            ),
            (
                Call::Snapshot,
                404,
                "nope",
                "GET /api/schema?branch=main HTTP/1.1",
                &[],
                Err((404, "schema snapshot http error: 404 Not Found")),
            ),
            (
                Call::Upload,
                200,
                r#"{"data":{"upload":{"ok":true}}}"#,
                graphql,
                &[operations, r#"{"0":["variables.file"]}"#, "hello"],
                Ok(r#"{"upload":{"ok":true}}"#),
            ),
            (Call::File, 200, "bytes", file, &[], Ok("bytes")),
            (
                Call::File,
                404,
                "nope",
                file,
                &[],
                Err((404, "file download error: 404 Not Found")),
            ),
            (
                Call::FileByHfid,
                200,
                "bytes",
                "GET /api/files/by-hfid/CoreFile/a/b?branch=main HTTP/1.1",
                &[],
                Ok("bytes"),
            ),
            (
                Call::FileByStorageId,
                200,
                "bytes",
                "GET /api/files/by-storage-id/s1?branch=main HTTP/1.1",
                &[],
                Ok("bytes"),
            ),
        ];
        for (call, status, response, line, parts, expected) in cases {
            let (addr, requests) = serve(vec![reply(status, "", response)]);
            let config = ClientConfig::new(format!("http://{addr}"), "test-token")
                .with_default_branch("main");
            let result = run(&test_client(config), call)
                .await
                .map_err(|err| match err {
                    Error::GraphQl {
                        status: Some(status),
                        body,
                        message,
                        ..
                    } => {
                        assert_eq!(body, response, "{call:?} {status}: error body");
                        (status, message)
                    }
                    other => panic!("{call:?} {status}: {other}"),
                });
            let expected = expected
                .map(str::to_owned)
                .map_err(|(status, message)| (status, message.to_owned()));
            assert_eq!(result, expected, "{call:?} {status}");
            let requests = requests.lock().unwrap();
            assert_eq!(requests.len(), 1, "{call:?} {status}: requests");
            let (head, body) = requests[0].split_once("\r\n\r\n").unwrap();
            assert_eq!(head.lines().next(), Some(line), "{call:?} {status}");
            for part in parts {
                assert!(
                    body.contains(part),
                    "{call:?} {status}: {part} not in {body}"
                );
            }
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_retry_after_from_response() {
        // (document, first status, requests, succeeds)
        for (document, status, requests, succeeds) in [
            ("query { ok }", 429, 2, true),
            ("mutation { create { ok } }", 503, 1, false),
        ] {
            let (addr, seen) = serve(vec![
                reply(status, "Retry-After: 1\r\n", ""),
                reply(200, "", r#"{"data":{"ok":true}}"#),
            ]);
            let client = test_client(ClientConfig::new(format!("http://{addr}"), "test-token"));
            let start = std::time::Instant::now();
            let result = client.execute_raw(document, None, None).await;
            let waited = start.elapsed();
            assert_eq!(result.is_ok(), succeeds, "{document}: {result:?}");
            assert_eq!(seen.lock().unwrap().len(), requests, "{document}: requests");
            if succeeds {
                assert!(
                    waited >= Duration::from_secs(1),
                    "{document}: waited {waited:?}"
                );
            }
        }
    }

    /// answers every request except the first, which it reads and drops; counts POSTs
    fn drop_first_response() -> (std::net::SocketAddr, Arc<std::sync::atomic::AtomicU32>) {
        use std::io::Write;

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let posts = Arc::new(std::sync::atomic::AtomicU32::new(0));
        let seen = posts.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let Some(head) = read_request(&stream) else {
                    continue;
                };
                if head.starts_with("POST ")
                    && seen.fetch_add(1, std::sync::atomic::Ordering::SeqCst) == 0
                {
                    continue;
                }
                let data = r#"{"data": {"ok": true}}"#;
                let _ = write!(
                    &stream,
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{data}",
                    data.len()
                );
            }
        });
        (addr, posts)
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_mutation_not_resent_after_dropped_response() {
        let upload = "mutation($file: Upload!) { upload(file: $file) { ok } }";
        // (label, document, opt in, multipart, expected POSTs)
        for (label, document, opt_in, multipart, expected) in [
            ("query", "query { ok }", false, false, 2),
            ("mutation", "mutation { create { ok } }", false, false, 1),
            (
                "mutation, opted in",
                "mutation { create { ok } }",
                true,
                false,
                2,
            ),
            ("upload", upload, false, true, 1),
            (
                "upload, query document",
                "query($file: Upload!) { check(file: $file) { ok } }",
                false,
                true,
                1,
            ),
            ("upload, opted in", upload, true, true, 2),
        ] {
            let (addr, posts) = drop_first_response();
            let config = ClientConfig::new(format!("http://{addr}"), "test-token")
                .with_mutation_retries(opt_in);
            let client = test_client(config);
            let result = if multipart {
                let file = FileUpload::new("a.txt", "text/plain", b"a".to_vec());
                client
                    .execute_multipart::<serde_json::Value>(
                        document,
                        None,
                        vec![("file", file)],
                        None,
                    )
                    .await
            } else {
                client.execute_raw(document, None, None).await
            };
            assert_eq!(result.is_ok(), expected == 2, "{label}: {result:?}");
            assert_eq!(
                posts.load(std::sync::atomic::Ordering::SeqCst),
                expected,
                "{label}: POSTs"
            );
        }
    }

    /// resolves lookup `n` to `addrs[n % addrs.len()]` and counts lookups
    struct CyclingResolver(Vec<std::net::SocketAddr>, Arc<std::sync::atomic::AtomicU32>);

    impl reqwest::dns::Resolve for CyclingResolver {
        fn resolve(&self, _: reqwest::dns::Name) -> reqwest::dns::Resolving {
            let n = self.1.fetch_add(1, std::sync::atomic::Ordering::SeqCst) as usize;
            let addrs: reqwest::dns::Addrs = Box::new(std::iter::once(self.0[n % self.0.len()]));
            Box::pin(async move { Ok(addrs) })
        }
    }

    fn resolving_client(
        lookups: Vec<std::net::SocketAddr>,
    ) -> (Client, Arc<std::sync::atomic::AtomicU32>) {
        let count = Arc::new(std::sync::atomic::AtomicU32::new(0));
        let http = reqwest::Client::builder()
            .no_proxy()
            .dns_resolver(CyclingResolver(lookups, count.clone()))
            .build()
            .unwrap();
        let config = ClientConfig::new("http://infrahub.test", "test-token").with_max_retries(1);
        let client = Client {
            config: Arc::new(config),
            http,
        };
        (client, count)
    }

    fn closed_addr() -> std::net::SocketAddr {
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_connect_error_resends_only_queries() {
        for (document, expected) in [("query { ok }", 2), ("mutation { create { ok } }", 1)] {
            let (client, connects) = resolving_client(vec![closed_addr()]);
            let err = client.execute_raw(document, None, None).await.unwrap_err();
            assert!(matches!(err, Error::Http(ref e) if e.is_connect()), "{err}");
            assert_eq!(
                connects.load(std::sync::atomic::Ordering::SeqCst),
                expected,
                "{document}: connects"
            );
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn test_mutation_not_resent_after_redirect() {
        use std::io::Write;

        let mutation = "mutation { create { ok } }";
        let upload = "mutation($file: Upload!) { upload(file: $file) { ok } }";
        let to_closed = "http://{closed}/graphql";
        // (label, document, multipart, status, location, expected POSTs)
        for (label, document, multipart, status, location, expected) in [
            (
                "query, 303 to a closed port",
                "query { ok }",
                false,
                303,
                to_closed,
                2,
            ),
            ("303 to a closed port", mutation, false, 303, to_closed, 1),
            ("307 to a closed port", mutation, false, 307, to_closed, 1),
            ("308 to a closed port", mutation, false, 308, to_closed, 1),
            (
                "307 to the same url",
                mutation,
                false,
                307,
                "http://infrahub.test/graphql",
                1,
            ),
            (
                "303 to a 429",
                mutation,
                false,
                303,
                "http://{listener}/limited",
                1,
            ),
            (
                "upload, 303 to a closed port",
                upload,
                true,
                303,
                to_closed,
                1,
            ),
        ] {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let closed = closed_addr();
            let location = location
                .replace("{closed}", &closed.to_string())
                .replace("{listener}", &addr.to_string());
            let posts = Arc::new(std::sync::atomic::AtomicU32::new(0));
            let seen = posts.clone();
            std::thread::spawn(move || {
                for stream in listener.incoming().flatten() {
                    let Some(head) = read_request(&stream) else {
                        continue;
                    };
                    let _ = if head.starts_with("POST ") {
                        seen.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        write!(&stream, "HTTP/1.1 {status} Redirect\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                    } else {
                        write!(&stream, "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 0\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                    };
                }
            });
            // in the same-url row, every second lookup (the redirect hop) gets the closed port
            let lookups = if label.ends_with("same url") {
                vec![addr, closed]
            } else {
                vec![addr]
            };
            let (client, _) = resolving_client(lookups);
            let result = if multipart {
                let file = FileUpload::new("a.txt", "text/plain", b"a".to_vec());
                client
                    .execute_multipart::<serde_json::Value>(
                        document,
                        None,
                        vec![("file", file)],
                        None,
                    )
                    .await
            } else {
                client.execute_raw(document, None, None).await
            };
            let err = result.unwrap_err();
            assert!(
                matches!(err, Error::Http(ref e) if e.is_connect())
                    || matches!(
                        err,
                        Error::GraphQl {
                            status: Some(429),
                            ..
                        }
                    ),
                "{label}: {err}"
            );
            assert_eq!(
                posts.load(std::sync::atomic::Ordering::SeqCst),
                expected,
                "{label}: POSTs"
            );
        }
    }
}
