//! command line arguments and schema loading

use infrahub::{Client, ClientConfig};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct Args {
    pub(crate) url: Option<String>,
    pub(crate) token: Option<String>,
    pub(crate) branch: Option<String>,
    pub(crate) schema_path: Option<PathBuf>,
    pub(crate) out_dir: PathBuf,
    pub(crate) crate_name: Option<String>,
    pub(crate) infrahub_path: Option<String>,
}

pub(crate) enum ParseArgsError {
    Help,
    Message(String),
}

pub(crate) fn parse_args(args: Vec<String>) -> Result<Args, ParseArgsError> {
    let mut url = None;
    let mut token = None;
    let mut branch = None;
    let mut schema_path = None;
    let mut out_dir = None;
    let mut crate_name = None;
    let mut infrahub_path = None;

    let mut iter = args.into_iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--url" => url = iter.next(),
            "--token" => token = iter.next(),
            "--branch" => branch = iter.next(),
            "--schema" => schema_path = iter.next().map(PathBuf::from),
            "--out" => out_dir = iter.next().map(PathBuf::from),
            "--crate-name" => crate_name = iter.next(),
            "--infrahub-path" => infrahub_path = iter.next(),
            "--help" | "-h" => return Err(ParseArgsError::Help),
            _ => return Err(ParseArgsError::Message(format!("unknown argument: {arg}"))),
        }
    }

    let out_dir =
        out_dir.ok_or_else(|| ParseArgsError::Message("--out is required".to_string()))?;

    if url.is_none() && schema_path.is_none() {
        return Err(ParseArgsError::Message(
            "--url or --schema is required".to_string(),
        ));
    }

    Ok(Args {
        url,
        token,
        branch,
        schema_path,
        out_dir,
        crate_name,
        infrahub_path,
    })
}

pub(crate) fn load_schema(args: &Args) -> Result<String, String> {
    if let Some(schema_path) = &args.schema_path {
        return fs::read_to_string(schema_path)
            .map_err(|err| format!("failed to read {}: {err}", schema_path.display()));
    }

    let url = args
        .url
        .as_ref()
        .ok_or_else(|| "--url is required when --schema not provided".to_string())?;

    let client = schema_client(url, args.token.as_deref()).map_err(|err| err.to_string())?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|err| err.to_string())?;
    runtime
        .block_on(client.fetch_schema(args.branch.as_deref()))
        .map_err(describe_fetch_error)
}

/// without a token, fetch anonymously with `Client::new`'s user agent and timeout
fn schema_client(url: &str, token: Option<&str>) -> infrahub::Result<Client> {
    let config = match token.filter(|token| !token.is_empty()) {
        Some(token) => ClientConfig::new(url, token),
        None => {
            let http = reqwest::Client::builder()
                .user_agent(format!("infrahub-rs/{} (Rust)", env!("CARGO_PKG_VERSION")))
                .timeout(Duration::from_secs(30))
                .build()?;
            ClientConfig::new(url, "").with_http_client(http)
        }
    };
    Client::new(config)
}

fn describe_fetch_error(err: infrahub::Error) -> String {
    match &err {
        infrahub::Error::GraphQl { body, .. } if !body.is_empty() => {
            format!("{err}: {}", body.chars().take(200).collect::<String>())
        }
        _ => err.to_string(),
    }
}

#[cfg(test)]
mod fetch_tests {
    use super::*;

    #[test]
    fn missing_or_empty_token_fetches_anonymously() {
        assert!(schema_client("http://localhost:8000", None).is_ok());
        assert!(schema_client("http://localhost:8000", Some("")).is_ok());
        assert!(schema_client("http://localhost:8000", Some("token")).is_ok());
    }

    #[test]
    fn fetch_error_keeps_the_start_of_the_body() {
        let err = infrahub::Error::GraphQl {
            status: Some(404),
            errors: Vec::new(),
            body: "x".repeat(300),
            message: "schema http error: 404 Not Found".to_string(),
        };
        let expected = format!(
            "graphql error: schema http error: 404 Not Found: {}",
            "x".repeat(200)
        );
        assert_eq!(describe_fetch_error(err), expected);
    }
}
