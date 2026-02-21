use infrahub::{Client, ClientConfig};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Data {
    #[serde(rename = "InfrahubInfo")]
    info: InfrahubInfo,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct InfrahubInfo {
    deployment_id: String,
    version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("INFRAHUB_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let token = env::var("INFRAHUB_TOKEN").expect("INFRAHUB_TOKEN is required");
    let branch = env::var("INFRAHUB_BRANCH").ok();

    let config = ClientConfig::new(base_url, token);
    let client = Client::new(config)?;

    let response = client
        .execute::<Data>(
            "query { InfrahubInfo { deployment_id version } }",
            None,
            branch.as_deref(),
        )
        .await?;

    println!("response: {response:?}");
    Ok(())
}
