//! offline checks that generated types accept the payloads infrahub sends

use infrahub_test_client::types::{CoreGenericAccount, NestedEdgedCoreGroup};

#[test]
fn empty_generic_edge_deserializes_without_node_metadata() {
    let edge: NestedEdgedCoreGroup = serde_json::from_value(serde_json::json!({
        "properties": { "is_protected": null, "updated_at": null },
        "relationship_metadata": null
    }))
    .expect("empty edge");
    assert!(edge.node.is_none());
    assert!(edge.node_metadata.is_none());
}

#[test]
fn set_generic_edge_deserializes_its_peer() {
    let edge: NestedEdgedCoreGroup = serde_json::from_value(serde_json::json!({
        "node": {
            "__typename": "CoreStandardGroup",
            "id": "18a7d1c2-5b3e-4f1a-9c6d-0e2f4a6b8c9d",
            "hfid": ["smoke-parent-group"],
            "display_label": "smoke-parent-group"
        },
        "properties": { "__typename": "RelationshipProperty" },
        "relationship_metadata": { "__typename": "InfrahubRelationshipMetadata" }
    }))
    .expect("set edge");
    let peer = edge.node.expect("peer");
    assert_eq!(peer.typename.as_deref(), Some("CoreStandardGroup"));
    assert_eq!(
        peer.id.as_deref(),
        Some("18a7d1c2-5b3e-4f1a-9c6d-0e2f4a6b8c9d")
    );
    assert_eq!(peer.hfid, Some(vec!["smoke-parent-group".to_string()]));
    assert_eq!(peer.display_label.as_deref(), Some("smoke-parent-group"));
    assert!(edge.node_metadata.is_none());
}

#[test]
fn generic_peer_deserializes_from_a_capped_selection() {
    for node in [
        serde_json::json!({ "__typename": "CoreAccount" }),
        serde_json::json!({ "id": "18a7d1c2-5b3e-4f1a-9c6d-0e2f4a6b8c9d" }),
    ] {
        let account: CoreGenericAccount = serde_json::from_value(node).expect("capped peer");
        assert!(account.is_externally_managed.is_none());
    }
}
