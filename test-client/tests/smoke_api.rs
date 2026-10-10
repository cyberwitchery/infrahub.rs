//! integration smoke tests for the generated typed API
//!
//! these tests exercise create / list / get_by_id / paginate / update / delete
//! flows through the generated client crate. they require a live infrahub
//! instance and are skipped when INFRAHUB_TOKEN is not set.

use infrahub::{Client, ClientConfig};
use infrahub_test_client::api::ApiClient;
use infrahub_test_client::inputs::{
    BuiltinTagCreateInput, BuiltinTagUpdateInput, CoreStandardGroupCreateInput, DeleteInput,
    RelatedNodeInput, TextAttributeCreate, TextAttributeUpdate,
};
use infrahub_test_client::types::CoreStandardGroup;

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

/// create a BuiltinTag and return its id
async fn create_tag(client: &Client, name: &str) -> String {
    let data = BuiltinTagCreateInput {
        id: None,
        name: Some(TextAttributeCreate {
            is_protected: None,
            source: None,
            owner: None,
            value: Some(name.to_string()),
        }),
        description: None,
        profiles: None,
        member_of_groups: None,
        subscriber_of_groups: None,
    };
    let tag = client
        .api()
        .builtin()
        .tag()
        .create(None, data, branch().as_deref())
        .await
        .expect("create tag");
    tag.id
}

/// delete a BuiltinTag by id
async fn delete_tag(client: &Client, id: &str) {
    let data = DeleteInput {
        id: Some(id.to_string()),
        hfid: None,
    };
    let deleted = client
        .api()
        .builtin()
        .tag()
        .delete(None, data, branch().as_deref())
        .await
        .expect("delete tag");
    assert!(deleted, "delete should report ok");
}

fn related(id: &str) -> RelatedNodeInput {
    RelatedNodeInput {
        id: Some(id.to_string()),
        hfid: None,
        kind: None,
        from_pool: None,
        _relation_is_protected: None,
        _relation_owner: None,
        _relation_source: None,
    }
}

/// create a CoreStandardGroup
async fn create_group(
    client: &Client,
    name: &str,
    members: Option<Vec<RelatedNodeInput>>,
    parent: Option<RelatedNodeInput>,
) -> CoreStandardGroup {
    let data = CoreStandardGroupCreateInput {
        id: None,
        name: Some(TextAttributeCreate {
            is_protected: None,
            source: None,
            owner: None,
            value: Some(name.to_string()),
        }),
        label: None,
        description: None,
        group_type: None,
        members,
        subscribers: None,
        parent,
        children: None,
    };
    client
        .api()
        .core()
        .standard_group()
        .create(None, data, branch().as_deref())
        .await
        .expect("create group")
}

/// delete a CoreStandardGroup by id
async fn delete_group(client: &Client, id: &str) {
    let data = DeleteInput {
        id: Some(id.to_string()),
        hfid: None,
    };
    let deleted = client
        .api()
        .core()
        .standard_group()
        .delete(None, data, branch().as_deref())
        .await
        .expect("delete group");
    assert!(deleted, "delete should report ok");
}

// ---------------------------------------------------------------------------
// CoreAccount — guaranteed to exist in a fresh infrahub (admin user)
//
// These were ignored on 1.9.x, where the is_externally_managed_resolver did a
// NodeManager.query per account node and concurrent GraphQL resolution
// triggered "read() called while another coroutine is already waiting for
// incoming data". The generated CoreAccount query selects
// is_externally_managed, so every account query could hit it. Fixed upstream;
// verified against 1.10.6.
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

    assert!(
        !accounts.is_empty(),
        "fresh infrahub should have >=1 account"
    );

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

    delete_tag(&client, &tag_id).await;
    let gone = client
        .api()
        .builtin()
        .tag()
        .get_by_id(&tag_id, branch().as_deref())
        .await
        .expect("get_by_id after delete");
    assert!(gone.is_none(), "deleted tag should be gone");
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
    let filters = infrahub_test_client::api::builtin::BuiltinTagFilters {
        limit: Some(2),
        ..Default::default()
    };
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
async fn tag_update_keeps_group_membership() {
    let Some(client) = make_client() else {
        return;
    };
    let tag_id = create_tag(&client, "smoke-update-tag").await;

    let members = Some(vec![related(&tag_id)]);
    let group = create_group(&client, "smoke-update-group", members, None).await;
    assert_eq!(group.members.count, 1, "group should hold the tag");
    let member_ids: Vec<_> = group
        .members
        .edges
        .iter()
        .flatten()
        .filter_map(|edge| edge.node.as_ref()?.id.clone())
        .collect();
    assert_eq!(
        member_ids,
        [tag_id.as_str()],
        "create should return the member"
    );

    let data = BuiltinTagUpdateInput {
        id: Some(tag_id.clone()),
        hfid: None,
        name: Some(TextAttributeUpdate {
            is_default: None,
            is_protected: None,
            source: None,
            owner: None,
            value: Some("smoke-update-tag-renamed".to_string()),
        }),
        description: None,
        profiles: None,
        member_of_groups: None,
        subscriber_of_groups: None,
    };
    let renamed = client
        .api()
        .builtin()
        .tag()
        .update(None, data, branch().as_deref())
        .await
        .expect("rename tag");
    let group_ids: Vec<_> = renamed
        .member_of_groups
        .edges
        .iter()
        .flatten()
        .filter_map(|edge| edge.node.as_ref()?.id.clone())
        .collect();
    assert_eq!(
        group_ids,
        [group.id.as_str()],
        "update should return the group"
    );

    let tag = client
        .api()
        .builtin()
        .tag()
        .get_by_id(&tag_id, branch().as_deref())
        .await
        .expect("get_by_id after rename")
        .expect("tag should be found");
    let name_val = tag.name.as_ref().and_then(|n| n.value.as_deref());
    assert_eq!(name_val, Some("smoke-update-tag-renamed"));
    assert_eq!(
        tag.member_of_groups.count, 1,
        "rename should keep the tag in its group"
    );

    // cleanup
    delete_group(&client, &group.id).await;
    delete_tag(&client, &tag_id).await;
}

#[cfg_attr(miri, ignore)]
#[tokio::test]
async fn group_without_parent_list_and_get_by_id() {
    let Some(client) = make_client() else {
        return;
    };
    let group_id = create_group(&client, "smoke-parentless-group", None, None)
        .await
        .id;

    let groups = client
        .api()
        .core()
        .standard_group()
        .list(None, branch().as_deref())
        .await
        .expect("list groups");
    assert!(
        groups.iter().any(|g| g.id == group_id),
        "created group should appear in list"
    );

    let group = client
        .api()
        .core()
        .standard_group()
        .get_by_id(&group_id, branch().as_deref())
        .await
        .expect("get_by_id")
        .expect("group should be found");
    assert_eq!(group.id, group_id);
    let name_val = group.name.as_ref().and_then(|n| n.value.as_deref());
    assert_eq!(name_val, Some("smoke-parentless-group"));

    delete_group(&client, &group_id).await;
}

#[cfg_attr(miri, ignore)]
#[tokio::test]
async fn group_parent_comes_back_with_its_peer() {
    let Some(client) = make_client() else {
        return;
    };
    let parent = create_group(&client, "smoke-parent-group", None, None).await;
    let child = create_group(
        &client,
        "smoke-child-group",
        None,
        Some(related(&parent.id)),
    )
    .await;
    let created = child
        .parent
        .node
        .as_ref()
        .expect("create returns the parent");
    assert_eq!(created.id.as_deref(), Some(parent.id.as_str()));

    let groups = client
        .api()
        .core()
        .standard_group()
        .list(None, branch().as_deref())
        .await
        .expect("list groups");
    let listed = groups
        .iter()
        .find(|g| g.id == child.id)
        .expect("child should appear in list");
    let peer = listed
        .parent
        .node
        .as_ref()
        .expect("list returns the parent");
    assert_eq!(peer.id.as_deref(), Some(parent.id.as_str()));
    assert_eq!(peer.typename.as_deref(), Some("CoreStandardGroup"));

    let found = client
        .api()
        .core()
        .standard_group()
        .get_by_id(&child.id, branch().as_deref())
        .await
        .expect("get_by_id")
        .expect("child should be found");
    let peer = found
        .parent
        .node
        .as_ref()
        .expect("get_by_id returns the parent");
    assert_eq!(peer.id.as_deref(), Some(parent.id.as_str()));

    let generic = client
        .api()
        .core()
        .group()
        .get_by_id(&parent.id, branch().as_deref())
        .await
        .expect("generic get_by_id")
        .expect("parent should be found as a CoreGroup");
    assert_eq!(generic.id.as_deref(), Some(parent.id.as_str()));
    assert_eq!(generic.typename.as_deref(), Some("CoreStandardGroup"));

    delete_group(&client, &child.id).await;
    delete_group(&client, &parent.id).await;
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
