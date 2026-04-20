//! client configuration
//!
//! build a [`ClientConfig`] with base url, token, and optional overrides.
//! pass it to [`crate::Client::new`] to create a client.

use crate::error::{Error, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/// configuration for the infrahub client
#[derive(Clone)]
pub struct ClientConfig {
    /// original base url input
    pub(crate) raw_base_url: String,

    /// base url of the infrahub instance (e.g., "<https://infrahub.example.com>")
    pub(crate) base_url: Url,

    /// whether the provided base url parsed successfully
    pub(crate) base_url_valid: bool,

    /// api authentication token
    pub(crate) token: String,

    /// default branch for graphql queries
    pub(crate) default_branch: Option<String>,

    /// request timeout duration
    pub(crate) timeout: Duration,

    /// user agent string
    pub(crate) user_agent: String,

    /// whether to verify ssl certificates
    pub(crate) verify_ssl: bool,

    /// additional headers to send with every request
    pub(crate) extra_headers: HeaderMap,

    /// prebuilt http client (takes precedence over http_client_builder)
    pub(crate) http_client: Option<reqwest::Client>,

    /// callback to customize the http client builder before building
    pub(crate) http_client_builder:
        Option<Arc<dyn Fn(reqwest::ClientBuilder) -> reqwest::ClientBuilder + Send + Sync>>,
}

impl ClientConfig {
    /// create a new client configuration
    ///
    /// # arguments
    ///
    /// * `base_url` - the base url of the infrahub instance (with or without trailing slash)
    /// * `token` - the api authentication token
    ///
    /// # example
    ///
    /// ```
    /// use infrahub::ClientConfig;
    ///
    /// let config = ClientConfig::new("https://infrahub.example.com", "your-token-here");
    /// ```
    pub fn new(base_url: impl AsRef<str>, token: impl Into<String>) -> Self {
        let base_url_str = base_url.as_ref();

        let normalized = base_url_str.trim_end_matches('/');

        let (base_url, base_url_valid) = match Url::parse(normalized)
            .or_else(|_| Url::parse(&format!("https://{}", normalized)))
        {
            Ok(url) => (url, true),
            Err(_) => (Url::parse("https://invalid.invalid").unwrap(), false),
        };

        Self {
            raw_base_url: base_url_str.to_string(),
            base_url,
            base_url_valid,
            token: token.into(),
            default_branch: None,
            timeout: Duration::from_secs(30),
            user_agent: format!("infrahub-rs/{} (Rust)", env!("CARGO_PKG_VERSION")),
            verify_ssl: true,
            extra_headers: HeaderMap::new(),
            http_client: None,
            http_client_builder: None,
        }
    }

    /// set the default branch for graphql queries
    pub fn with_default_branch(mut self, branch: impl Into<String>) -> Self {
        self.default_branch = Some(branch.into());
        self
    }

    /// set the request timeout
    ///
    /// default: 30 seconds
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// set a custom user agent string
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// disable ssl certificate verification (not recommended for production)
    ///
    /// default: enabled
    pub fn with_ssl_verification(mut self, verify: bool) -> Self {
        self.verify_ssl = verify;
        self
    }

    /// add a header to every request
    pub fn with_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.extra_headers.insert(name, value);
        self
    }

    /// add a set of headers to every request
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.extra_headers.extend(headers);
        self
    }

    /// access extra headers configured on this client
    pub fn extra_headers(&self) -> &HeaderMap {
        &self.extra_headers
    }

    /// inject a prebuilt http client.
    ///
    /// when set, this client is used as-is and takes precedence over
    /// `with_http_client_builder`. all transport configuration — auth headers,
    /// tls, timeouts, ssl verification, user agent — comes from the prebuilt
    /// client; the corresponding `ClientConfig` fields are ignored.
    ///
    /// because auth is managed by the caller, an empty token is accepted when
    /// this option is set.
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    /// customize the http client builder before the client is created.
    ///
    /// the callback receives a builder that already has the auth header,
    /// extra headers, user agent, timeout, and ssl settings applied.
    /// use this to add proxy config, custom tls roots, or other transport
    /// settings without reimplementing the defaults.
    ///
    /// ignored if `with_http_client` is also set.
    pub fn with_http_client_builder<F>(mut self, f: F) -> Self
    where
        F: Fn(reqwest::ClientBuilder) -> reqwest::ClientBuilder + Send + Sync + 'static,
    {
        self.http_client_builder = Some(Arc::new(f));
        self
    }

    /// validate the configuration
    pub(crate) fn validate(&self) -> Result<()> {
        if !self.base_url_valid {
            return Err(Error::Config(format!(
                "invalid base url: {}",
                self.raw_base_url
            )));
        }

        if self.base_url.scheme() != "http" && self.base_url.scheme() != "https" {
            return Err(Error::Config(format!(
                "invalid url scheme: {}. must be http or https",
                self.base_url.scheme()
            )));
        }

        // token is only required when the client is not managing its own transport
        if self.http_client.is_none() && self.token.is_empty() {
            return Err(Error::Config("api token cannot be empty".to_string()));
        }

        Ok(())
    }

    /// resolve the effective branch: use the explicit argument if non-empty,
    /// fall back to `default_branch`, or return `None`.
    fn resolve_branch(&self, branch: Option<&str>) -> Option<String> {
        branch
            .map(|b| b.to_string())
            .or_else(|| self.default_branch.clone())
            .filter(|b| !b.is_empty())
    }

    /// build the graphql url for a branch (or default branch if none provided)
    pub(crate) fn graphql_url(&self, branch: Option<&str>) -> Result<Url> {
        let base = self.base_url.as_str().trim_end_matches('/');
        let mut url = Url::parse(&format!("{}/graphql", base))?;
        if let Some(branch) = self.resolve_branch(branch) {
            url.path_segments_mut()
                .expect("HTTP URL supports path segments")
                .push(&branch);
        }
        Ok(url)
    }

    /// build a file download url by node id
    pub(crate) fn file_url(&self, node_id: &str, branch: Option<&str>) -> Result<Url> {
        let base = self.base_url.as_str().trim_end_matches('/');
        let mut url = Url::parse(&format!("{}/api/files", base))?;
        url.path_segments_mut()
            .expect("HTTP URL supports path segments")
            .push(node_id);
        if let Some(branch) = self.resolve_branch(branch) {
            url.query_pairs_mut().append_pair("branch", &branch);
        }
        Ok(url)
    }

    /// build a file download url by human-friendly id
    pub(crate) fn file_by_hfid_url(
        &self,
        kind: &str,
        hfid: &[&str],
        branch: Option<&str>,
    ) -> Result<Url> {
        let base = self.base_url.as_str().trim_end_matches('/');
        let mut url = Url::parse(&format!("{}/api/files/by-hfid", base))?;
        {
            let mut segments = url
                .path_segments_mut()
                .expect("HTTP URL supports path segments");
            segments.push(kind);
            for segment in hfid {
                segments.push(segment);
            }
        }
        if let Some(branch) = self.resolve_branch(branch) {
            url.query_pairs_mut().append_pair("branch", &branch);
        }
        Ok(url)
    }

    /// build a file download url by storage id
    pub(crate) fn file_by_storage_id_url(
        &self,
        storage_id: &str,
        branch: Option<&str>,
    ) -> Result<Url> {
        let base = self.base_url.as_str().trim_end_matches('/');
        let mut url = Url::parse(&format!("{}/api/files/by-storage-id", base))?;
        url.path_segments_mut()
            .expect("HTTP URL supports path segments")
            .push(storage_id);
        if let Some(branch) = self.resolve_branch(branch) {
            url.query_pairs_mut().append_pair("branch", &branch);
        }
        Ok(url)
    }

    /// build the schema url for a branch (or default branch if none provided)
    pub(crate) fn schema_url(&self, branch: Option<&str>) -> Result<Url> {
        let base = self.base_url.as_str().trim_end_matches('/');
        let mut url = Url::parse(&format!("{}/schema.graphql", base))?;
        if let Some(branch) = self.resolve_branch(branch) {
            url.query_pairs_mut().append_pair("branch", &branch);
        }
        Ok(url)
    }
}

impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfig")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("user_agent", &self.user_agent)
            .field("verify_ssl", &self.verify_ssl)
            .field("extra_headers", &self.extra_headers.len())
            .field("default_branch", &self.default_branch)
            .field("http_client", &self.http_client.is_some())
            .field("http_client_builder", &self.http_client_builder.is_some())
            .field("token", &"<redacted>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = ClientConfig::new("https://infrahub.example.com", "test-token");
        assert_eq!(
            config.base_url.as_str().trim_end_matches('/'),
            "https://infrahub.example.com"
        );
        assert_eq!(config.token, "test-token");
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_graphql_url_default() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");
        let url = config.graphql_url(None).unwrap();
        assert_eq!(url.as_str(), "https://infrahub.example.com/graphql");
    }

    #[test]
    fn test_graphql_url_branch() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");
        let url = config.graphql_url(Some("test")).unwrap();
        assert_eq!(url.as_str(), "https://infrahub.example.com/graphql/test");
    }

    #[test]
    fn test_schema_url_branch() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");
        let url = config.schema_url(Some("test")).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/schema.graphql?branch=test"
        );
    }

    #[test]
    fn test_schema_url_default() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");
        let url = config.schema_url(None).unwrap();
        assert_eq!(url.as_str(), "https://infrahub.example.com/schema.graphql");
    }

    #[test]
    fn test_validation() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");
        assert!(config.validate().is_ok());

        let empty_token = ClientConfig::new("https://infrahub.example.com", "");
        assert!(empty_token.validate().is_err());

        // empty token is allowed when a prebuilt client handles auth
        let empty_token_prebuilt = ClientConfig::new("https://infrahub.example.com", "")
            .with_http_client(reqwest::Client::new());
        assert!(empty_token_prebuilt.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_url() {
        let mut config = ClientConfig::new("https://infrahub.example.com", "token");
        config.base_url_valid = false;
        let err = config.validate().unwrap_err();
        assert!(matches!(err, Error::Config(_)));
    }

    #[test]
    fn test_validation_invalid_scheme() {
        let config = ClientConfig::new("ftp://example.com", "token");
        let err = config.validate().unwrap_err();
        assert!(matches!(err, Error::Config(_)));
    }

    #[test]
    fn test_builder_helpers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-test"),
            HeaderValue::from_static("value"),
        );

        let config = ClientConfig::new("https://infrahub.example.com", "token")
            .with_default_branch("main")
            .with_timeout(Duration::from_secs(5))
            .with_user_agent("infrahub-test")
            .with_ssl_verification(false)
            .with_headers(headers.clone())
            .with_header(
                HeaderName::from_static("x-other"),
                HeaderValue::from_static("other"),
            );

        assert_eq!(config.default_branch.as_deref(), Some("main"));
        assert_eq!(config.timeout, Duration::from_secs(5));
        assert_eq!(config.user_agent, "infrahub-test");
        assert!(!config.verify_ssl);
        assert_eq!(config.extra_headers.get("x-test").unwrap(), "value");
        assert_eq!(config.extra_headers.get("x-other").unwrap(), "other");
        assert_eq!(config.extra_headers(), &config.extra_headers);
    }

    #[test]
    fn test_with_http_client() {
        let prebuilt = reqwest::Client::new();
        let config =
            ClientConfig::new("https://infrahub.example.com", "token").with_http_client(prebuilt);
        assert!(config.http_client.is_some());
        assert!(config.http_client_builder.is_none());
    }

    #[test]
    fn test_with_http_client_builder() {
        let config = ClientConfig::new("https://infrahub.example.com", "token")
            .with_http_client_builder(|b| b.connection_verbose(true));
        assert!(config.http_client.is_none());
        assert!(config.http_client_builder.is_some());
    }

    #[test]
    fn test_file_url() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");
        let url = config.file_url("abc-123", None).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/abc-123"
        );

        let url = config.file_url("abc-123", Some("dev")).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/abc-123?branch=dev"
        );
    }

    #[test]
    fn test_file_by_hfid_url() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");
        let url = config
            .file_by_hfid_url("MyFile", &["value1", "value2"], None)
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/by-hfid/MyFile/value1/value2"
        );
    }

    #[test]
    fn test_file_by_storage_id_url() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");
        let url = config.file_by_storage_id_url("store-456", None).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/by-storage-id/store-456"
        );
    }

    #[test]
    fn test_branch_with_special_chars_is_encoded() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");

        // ampersand in branch name would inject a second query param without encoding
        let url = config.file_url("abc-123", Some("feat&evil=1")).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/abc-123?branch=feat%26evil%3D1"
        );

        let url = config
            .file_by_hfid_url("MyFile", &["v1"], Some("has spaces"))
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/by-hfid/MyFile/v1?branch=has+spaces"
        );

        let url = config
            .file_by_storage_id_url("store-1", Some("a=b&c=d"))
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/by-storage-id/store-1?branch=a%3Db%26c%3Dd"
        );

        let url = config.schema_url(Some("release/1.0&drop")).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/schema.graphql?branch=release%2F1.0%26drop"
        );
    }

    #[test]
    fn test_path_segments_with_special_chars_are_encoded() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");

        // branch with slash in graphql_url path
        let url = config.graphql_url(Some("release/1.0")).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/graphql/release%2F1.0"
        );

        // branch with hash would truncate the URL without encoding
        let url = config.graphql_url(Some("fix#123")).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/graphql/fix%23123"
        );

        // node_id with special characters
        let url = config.file_url("id/with#special", None).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/id%2Fwith%23special"
        );

        // kind and hfid with special characters
        let url = config
            .file_by_hfid_url("My/Kind", &["val/1", "val#2"], None)
            .unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/by-hfid/My%2FKind/val%2F1/val%232"
        );

        // storage_id with special characters
        let url = config.file_by_storage_id_url("store/id#1", None).unwrap();
        assert_eq!(
            url.as_str(),
            "https://infrahub.example.com/api/files/by-storage-id/store%2Fid%231"
        );
    }

    #[test]
    fn test_debug_reflects_http_client_fields() {
        let config = ClientConfig::new("https://infrahub.example.com", "token");
        let debug = format!("{config:?}");
        assert!(debug.contains("http_client: false"));
        assert!(debug.contains("http_client_builder: false"));
        assert!(debug.contains("\"<redacted>\""));

        let config = config.with_http_client(reqwest::Client::new());
        let debug = format!("{config:?}");
        assert!(debug.contains("http_client: true"));
    }
}
