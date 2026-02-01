use infrahub::{Client, ClientConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("INFRAHUB_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let token = env::var("INFRAHUB_TOKEN").expect("INFRAHUB_TOKEN is required");
    let branch = env::var("INFRAHUB_BRANCH").ok();

    let config = ClientConfig::new(base_url, token);
    let client = Client::new(config)?;

    let response = client
        .execute_raw(
            "{ InfrahubInfo { deployment_id version } }",
            None,
            branch.as_deref(),
        )
        .await?;

    println!("data: {}", response.data.unwrap_or_default());

    Ok(())
}
