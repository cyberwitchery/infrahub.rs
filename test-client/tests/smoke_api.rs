//! integration smoke tests for the generated typed API
//!
//! these tests exercise list / get_by_id / paginate flows through the
//! generated client crate. they require a live infrahub instance and are
//! skipped when INFRAHUB_TOKEN is not set.

use infrahub::{Client, ClientConfig};
use infrahub_test_client::api::ApiClient;

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn make_client() -> Option<Client> {
    let base_url =
        std::env::var("INFRAHUB_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let token = std::env::var("INFRAHUB_TOKEN").ok()?;
    let config = ClientConfig::new(base_url, token);
    Some(Client::new(config).expect("client"))
}

fn branch() -> Option<String> {
    std::env::var("INFRAHUB_BRANCH").ok()
}

/// create a BuiltinTag via raw graphql and return its id
async fn create_tag(client: &Client, name: &str) -> String {
    let query = r#"mutation CreateTag($data: BuiltinTagCreateInput!) {
        BuiltinTagCreate(data: $data) { ok object { id } }
    }"#;
    let vars = serde_json::json!({
        "data": { "name": { "value": name } }
    });
    let resp: infrahub::GraphQlResponse<serde_json::Value> = client
        .execute(query, Some(vars), branch().as_deref())
        .await
        .expect("create tag");
    let data = resp.data.expect("create tag data");
    data["BuiltinTagCreate"]["object"]["id"]
        .as_str()
        .expect("tag id")
        .to_string()
}

/// delete a BuiltinTag by id
async fn delete_tag(client: &Client, id: &str) {
    let query = r#"mutation DeleteTag($data: DeleteInput!) {
        BuiltinTagDelete(data: $data) { ok }
    }"#;
    let vars = serde_json::json!({ "data": { "id": id } });
    let _: infrahub::GraphQlResponse<serde_json::Value> = client
        .execute(query, Some(vars), branch().as_deref())
        .await
        .expect("delete tag");
}

// ---------------------------------------------------------------------------
// CoreAccount — guaranteed to exist in a fresh infrahub (admin user)
// ---------------------------------------------------------------------------

#[cfg_attr(miri, ignore)]
#[tokio::test]
async fn account_list_returns_admin() {
    let Some(client) = make_client() else {
        return;
    };
    let accounts = client
        .api()
        .core()
        .account()
        .list(None, branch().as_deref())
        .await
        .expect("list accounts");

    assert!(!accounts.is_empty(), "fresh infrahub should have >=1 account");

    let admin = accounts
        .iter()
        .find(|a| {
            a.name
                .as_ref()
                .and_then(|n| n.value.as_deref())
                .is_some_and(|v| v == "admin")
        })
        .expect("admin account should exist");

    assert!(!admin.id.is_empty());
}

#[cfg_attr(miri, ignore)]
#[tokio::test]
async fn account_get_by_id() {
    let Some(client) = make_client() else {
        return;
    };
    let accounts = client
        .api()
        .core()
        .account()
        .list(None, branch().as_deref())
        .await
        .expect("list accounts");
    let first = accounts.first().expect("at least one account");

    let found = client
        .api()
        .core()
        .account()
        .get_by_id(&first.id, branch().as_deref())
        .await
        .expect("get_by_id");

    let found = found.expect("account should be found");
    assert_eq!(found.id, first.id);
}

#[cfg_attr(miri, ignore)]
#[tokio::test]
async fn account_paginate() {
    let Some(client) = make_client() else {
        return;
    };
    let mut paginator = client
        .api()
        .core()
        .account()
        .paginate(None, branch().as_deref());

    let page = paginator
        .next_page()
        .await
        .expect("first page")
        .expect("should return at least one page");

    assert!(!page.is_empty(), "first page should have items");
}

// ---------------------------------------------------------------------------
// BuiltinTag — create test data, exercise list / get_by_id / paginate,
//              then clean up
// ---------------------------------------------------------------------------

#[cfg_attr(miri, ignore)]
#[tokio::test]
async fn tag_list_and_get_by_id() {
    let Some(client) = make_client() else {
        return;
    };
    let tag_id = create_tag(&client, "smoke-test-tag").await;

    // list — should find the tag we just created
    let tags = client
        .api()
        .builtin()
        .tag()
        .list(None, branch().as_deref())
        .await
        .expect("list tags");

    let found = tags.iter().any(|t| t.id == tag_id);
    assert!(found, "created tag should appear in list");

    // get_by_id
    let tag = client
        .api()
        .builtin()
        .tag()
        .get_by_id(&tag_id, branch().as_deref())
        .await
        .expect("get_by_id");

    let tag = tag.expect("tag should be found");
    assert_eq!(tag.id, tag_id);
    let name_val = tag
        .name
        .as_ref()
        .and_then(|n| n.value.as_deref())
        .expect("tag name");
    assert_eq!(name_val, "smoke-test-tag");

    // cleanup
    delete_tag(&client, &tag_id).await;
}

#[cfg_attr(miri, ignore)]
#[tokio::test]
async fn tag_paginate_with_limit() {
    let Some(client) = make_client() else {
        return;
    };

    // create a few tags to have pagination content
    let ids: Vec<String> = {
        let mut v = Vec::new();
        for i in 0..3 {
            v.push(create_tag(&client, &format!("smoke-page-{i}")).await);
        }
        v
    };

    // paginate with limit=2 so we get multiple pages
    let mut filters =
        infrahub_test_client::api::builtin::BuiltinTagFilters::default();
    filters.limit = Some(2);
    let all = client
        .api()
        .builtin()
        .tag()
        .paginate(Some(filters), branch().as_deref())
        .collect_all()
        .await
        .expect("paginate collect_all");

    assert!(
        all.len() >= 3,
        "should collect all created tags (got {})",
        all.len()
    );

    // cleanup
    for id in &ids {
        delete_tag(&client, id).await;
    }
}

#[cfg_attr(miri, ignore)]
#[tokio::test]
async fn tag_get_by_id_missing() {
    let Some(client) = make_client() else {
        return;
    };
    let result = client
        .api()
        .builtin()
        .tag()
        .get_by_id("00000000-0000-0000-0000-000000000000", branch().as_deref())
        .await
        .expect("get_by_id non-existent");

    assert!(result.is_none(), "non-existent id should return None");
}
