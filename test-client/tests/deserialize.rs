//! offline checks that generated types accept the payloads infrahub sends

use infrahub_test_client::types::NestedEdgedCoreGroup;

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
