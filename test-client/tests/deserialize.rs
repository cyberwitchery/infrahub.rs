//! offline checks that generated types accept the payloads infrahub sends

use infrahub_test_client::types::{
    CoreGenericAccount, NestedEdgedCoreArtifactDefinition, NestedEdgedCoreGroup,
    NestedPaginatedBuiltinTag, PeerRef,
};

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
fn empty_generic_edge_deserializes_null_node_metadata() {
    let edge: NestedEdgedCoreGroup = serde_json::from_value(serde_json::json!({
        "node": null,
        "node_metadata": null,
        "properties": { "is_protected": null, "updated_at": null },
        "relationship_metadata": null
    }))
    .expect("empty edge with null metadata");
    assert!(edge.node.is_none());
    assert!(edge.node_metadata.is_none());
}

#[test]
fn set_generic_edge_deserializes_its_peer() {
    let edge: NestedEdgedCoreGroup = serde_json::from_value(serde_json::json!({
        "node": {
            "__typename": "CoreStandardGroup",
            "id": "18a7d1c2-5b3e-4f1a-9c6d-0e2f4a6b8c9d"
        },
        "properties": { "__typename": "RelationshipProperty" },
        "relationship_metadata": { "__typename": "InfrahubRelationshipMetadata" }
    }))
    .expect("set edge");
    let PeerRef { typename, id } = *edge.node.expect("peer");
    assert_eq!(typename.as_deref(), Some("CoreStandardGroup"));
    assert_eq!(id.as_deref(), Some("18a7d1c2-5b3e-4f1a-9c6d-0e2f4a6b8c9d"));
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

#[test]
fn concrete_peer_deserializes_from_its_identity_selection() {
    for node in [
        serde_json::json!({ "__typename": "CoreArtifactDefinition" }),
        serde_json::json!({
            "__typename": "CoreArtifactDefinition",
            "id": "18a7d1c2-5b3e-4f1a-9c6d-0e2f4a6b8c9d"
        }),
    ] {
        let edge: NestedEdgedCoreArtifactDefinition =
            serde_json::from_value(serde_json::json!({ "node": node })).expect("concrete peer");
        let PeerRef { typename, id } = *edge.node.expect("peer");
        assert_eq!(typename.as_deref(), Some("CoreArtifactDefinition"));
        assert_eq!(id.as_deref(), node["id"].as_str());
    }
}

#[test]
fn capped_relationship_deserializes_without_permissions() {
    let tags: NestedPaginatedBuiltinTag = serde_json::from_value(serde_json::json!({
        "count": 1,
        "edges": [{ "__typename": "NestedEdgedBuiltinTag" }]
    }))
    .expect("capped relationship");
    assert_eq!(tags.edges.len(), 1);
    assert!(tags.permissions.is_none());
}
