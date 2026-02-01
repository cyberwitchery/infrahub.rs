use infrahub::{Client, ClientConfig};

#[cfg_attr(miri, ignore)]
#[tokio::test]
async fn smoke_infrahub_info() {
    let base_url = std::env::var("INFRAHUB_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let token = match std::env::var("INFRAHUB_TOKEN") {
        Ok(token) => token,
        Err(_) => return,
    };
    let branch = std::env::var("INFRAHUB_BRANCH").ok();

    let client = Client::new(ClientConfig::new(base_url, token)).expect("client");
    let response = client
        .execute_raw("query { InfrahubInfo { deployment_id version } }", None, branch.as_deref())
        .await
        .expect("graphql query");

    assert!(response.data.is_some());
}
