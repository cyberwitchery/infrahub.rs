//! selection planning for generated queries

use crate::conventions::{
    edge_node, has_required_args, identifies_peer, is_interface_peer, is_relationship_peer,
    peer_fields, should_skip_field,
};
use crate::schema::{fields_of, SchemaContext};
use crate::type_map::{base_type_name, is_optional, is_scalar_type};
use graphql_parser::schema::{Field, ObjectType, TypeDefinition};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn selection_for_field(field: &Field<String>, ctx: &SchemaContext) -> String {
    let base = base_type_name(&field.field_type);
    if is_scalar_type(&base) || ctx.enums.contains(&base) || ctx.scalars.contains(&base) {
        return String::new();
    }
    if ctx.interfaces.contains(&base) {
        return format!(" {}", peer_selection(&base, ctx));
    }

    let mut stack = BTreeSet::new();
    format!(" {}", selection_for_type(&base, ctx, &mut stack, 0))
}

/// how much of an object a selection takes.
#[derive(Clone, Copy, PartialEq)]
enum Cut {
    /// every selectable field.
    Full,
    /// only what the struct requires: past the depth cap, or repeated on its own path.
    Required,
}

pub(crate) fn selection_for_type(
    type_name: &str,
    ctx: &SchemaContext,
    stack: &mut BTreeSet<String>,
    depth: usize,
) -> String {
    let Some(TypeDefinition::Object(obj)) = ctx.types.get(type_name) else {
        return "{ __typename }".to_string();
    };
    let cut = if depth > 3 || stack.contains(type_name) {
        Cut::Required
    } else {
        Cut::Full
    };
    let inserted = stack.insert(type_name.to_string());

    let mut fields = Vec::new();
    for field in &obj.fields {
        let wanted = match cut {
            Cut::Full => is_selected(obj, field, ctx),
            Cut::Required => is_required(obj, field, ctx),
        };
        if !wanted {
            continue;
        }
        let field_base = base_type_name(&field.field_type);
        if is_scalar_type(&field_base)
            || ctx.enums.contains(&field_base)
            || ctx.scalars.contains(&field_base)
        {
            fields.push(field.name.clone());
        } else if is_relationship_peer(obj, field, ctx) {
            let peer = peer_ref_selection(&field_base, ctx);
            fields.push(format!("{} {}", field.name, peer));
        } else if is_interface_peer(field, ctx) {
            let peer = peer_selection(&field_base, ctx);
            fields.push(format!("{} {}", field.name, peer));
        } else if ctx.objects.contains(&field_base) {
            // a nullable object the cap would cut short is left out rather than half-selected
            if depth >= 3 && is_optional(&field.field_type) && has_required_fields(&field_base, ctx)
            {
                continue;
            }
            let nested = selection_for_type(&field_base, ctx, stack, depth + 1);
            fields.push(format!("{} {}", field.name, nested));
        } else if ctx.unions.contains(&field_base) {
            fields.push(format!("{} {{ __typename }}", field.name));
        }
    }

    if inserted {
        stack.remove(type_name);
    }

    if cut == Cut::Required && !inserted && depth <= 3 && fields.is_empty() && selects_id(obj, ctx)
    {
        // a repeat keeps the id it needs to be told apart, as before the cap selected required fields
        return "{ id }".to_string();
    }
    if fields.is_empty() {
        "{ __typename }".to_string()
    } else {
        format!("{{ {} }}", fields.join(" "))
    }
}

fn selects_id(obj: &ObjectType<String>, ctx: &SchemaContext) -> bool {
    obj.fields
        .iter()
        .any(|field| field.name == "id" && is_selected(obj, field, ctx))
}

/// whether the struct of `obj` requires `field`: non-null, selected, and not closing a cycle of such fields.
pub(crate) fn is_required(
    obj: &ObjectType<String>,
    field: &Field<String>,
    ctx: &SchemaContext,
) -> bool {
    ctx.required
        .get(&obj.name)
        .is_some_and(|fields| fields.contains(&field.name))
}

/// whether `{ __typename }` cannot deserialize into the struct of type `name`.
fn has_required_fields(name: &str, ctx: &SchemaContext) -> bool {
    ctx.required
        .get(name)
        .is_some_and(|fields| !fields.is_empty())
}

/// the fields each object's struct requires.
///
/// a non-null selected field is required unless its type leads back to its own object through
/// required object fields: no finite selection fills such a cycle, so the field that closes it is
/// typed `Option` and stays selected where the query reaches it.
pub(crate) fn required_fields(ctx: &SchemaContext) -> BTreeMap<String, BTreeSet<String>> {
    let mut candidates: BTreeMap<&str, Vec<(&str, Option<String>)>> = BTreeMap::new();
    for name in &ctx.objects {
        let Some(TypeDefinition::Object(obj)) = ctx.types.get(name) else {
            continue;
        };
        let fields = obj
            .fields
            .iter()
            .filter(|field| !is_optional(&field.field_type) && is_selected(obj, field, ctx))
            .map(|field| {
                let base = base_type_name(&field.field_type);
                let nested = !is_relationship_peer(obj, field, ctx)
                    && !is_interface_peer(field, ctx)
                    && ctx.objects.contains(&base);
                (field.name.as_str(), nested.then_some(base))
            })
            .collect();
        candidates.insert(name, fields);
    }

    let leads_to = |from: &str, to: &str| {
        let mut seen = BTreeSet::new();
        let mut pending = vec![from.to_string()];
        while let Some(name) = pending.pop() {
            if name == to {
                return true;
            }
            if seen.insert(name.clone()) {
                let next = candidates.get(name.as_str()).into_iter().flatten();
                pending.extend(next.filter_map(|(_, target)| target.clone()));
            }
        }
        false
    };

    candidates
        .iter()
        .map(|(name, fields)| {
            let required = fields
                .iter()
                .filter(|(_, target)| target.as_deref().is_none_or(|t| !leads_to(t, name)))
                .map(|(field, _)| field.to_string())
                .collect();
            (name.to_string(), required)
        })
        .collect()
}

/// whether `selection_for_type` selects this field of `obj` within the cap.
pub(crate) fn is_selected(
    obj: &ObjectType<String>,
    field: &Field<String>,
    ctx: &SchemaContext,
) -> bool {
    if has_required_args(field) || should_skip_field(field) || nulled_on_empty_edge(obj, field) {
        return false;
    }
    let base = base_type_name(&field.field_type);
    if ctx.relationship_wrappers.contains(&obj.name)
        && ctx.objects.contains(&base)
        && edge_node(&base, ctx).is_none()
    {
        return false;
    }
    is_scalar_type(&base)
        || ctx.enums.contains(&base)
        || ctx.scalars.contains(&base)
        || ctx.objects.contains(&base)
        || ctx.unions.contains(&base)
        || is_interface_peer(field, ctx)
}

/// the selection of a relationship peer, which fills the shared peer struct.
fn peer_ref_selection(name: &str, ctx: &SchemaContext) -> &'static str {
    if fields_of(name, ctx)
        .iter()
        .any(|field| field.name == "id" && identifies_peer(field, ctx))
    {
        "{ __typename id }"
    } else {
        "{ __typename }"
    }
}

fn peer_selection(name: &str, ctx: &SchemaContext) -> String {
    let mut fields = vec!["__typename"];
    fields.extend(
        peer_fields(name, ctx)
            .iter()
            .map(|field| field.name.as_str()),
    );
    format!("{{ {} }}", fields.join(" "))
}

/// a non-null `node_metadata` beside a nullable `node`, which infrahub leaves null on an empty edge.
fn nulled_on_empty_edge(obj: &ObjectType<String>, field: &Field<String>) -> bool {
    field.name == "node_metadata"
        && !is_optional(&field.field_type)
        && obj
            .fields
            .iter()
            .any(|f| f.name == "node" && is_optional(&f.field_type))
}
