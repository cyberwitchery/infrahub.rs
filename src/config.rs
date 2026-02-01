//! client configuration
//!
//! build a [`ClientConfig`] with base url, token, and optional overrides.
//! pass it to [`crate::Client::new`] to create a client.

use crate::error::{Error, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
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

        if self.token.is_empty() {
            return Err(Error::Config("api token cannot be empty".to_string()));
        }

        Ok(())
    }

    /// build the graphql url for a branch (or default branch if none provided)
    pub(crate) fn graphql_url(&self, branch: Option<&str>) -> Result<Url> {
        let base = self.base_url.as_str().trim_end_matches('/');
        let branch = branch
            .map(|b| b.to_string())
            .or_else(|| self.default_branch.clone());
        let url_str = match branch {
            Some(branch) if !branch.is_empty() => format!("{}/graphql/{}", base, branch),
            _ => format!("{}/graphql", base),
        };
        Url::parse(&url_str).map_err(Error::from)
    }

    /// build the schema url for a branch (or default branch if none provided)
    pub(crate) fn schema_url(&self, branch: Option<&str>) -> Result<Url> {
        let base = self.base_url.as_str().trim_end_matches('/');
        let branch = branch
            .map(|b| b.to_string())
            .or_else(|| self.default_branch.clone());
        let url_str = match branch {
            Some(branch) if !branch.is_empty() => format!("{}/schema.graphql?branch={}", base, branch),
            _ => format!("{}/schema.graphql", base),
        };
        Url::parse(&url_str).map_err(Error::from)
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
        headers.insert(HeaderName::from_static("x-test"), HeaderValue::from_static("value"));

        let config = ClientConfig::new("https://infrahub.example.com", "token")
            .with_default_branch("main")
            .with_timeout(Duration::from_secs(5))
            .with_user_agent("infrahub-test")
            .with_ssl_verification(false)
            .with_headers(headers.clone())
            .with_header(HeaderName::from_static("x-other"), HeaderValue::from_static("other"));

        assert_eq!(config.default_branch.as_deref(), Some("main"));
        assert_eq!(config.timeout, Duration::from_secs(5));
        assert_eq!(config.user_agent, "infrahub-test");
        assert!(!config.verify_ssl);
        assert_eq!(
            config.extra_headers.get("x-test").unwrap(),
            "value"
        );
        assert_eq!(
            config.extra_headers.get("x-other").unwrap(),
            "other"
        );
        assert_eq!(config.extra_headers(), &config.extra_headers);
    }
}
