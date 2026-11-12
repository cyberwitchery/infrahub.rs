//! infrahub conventions: models, namespaces, relationships and peers

use crate::schema::{fields_of, split_identifier_words, to_snake, SchemaContext};
use crate::selection::is_selected;
use crate::type_map::{
    base_type_name, field_rust_type, is_list, is_scalar_type, rust_type, strip_option_box,
};
use graphql_parser::schema::{EnumValue, Field, ObjectType, Type, TypeDefinition};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub(crate) struct ModelInfo<'a> {
    pub(crate) name: String,
    pub(crate) namespace: String,
    pub(crate) query_field: Option<Field<'a, String>>,
    pub(crate) query_return: Option<String>,
    pub(crate) node_type: String,
    pub(crate) node_boxed: bool,
    pub(crate) create: Option<Field<'a, String>>,
    pub(crate) update: Option<Field<'a, String>>,
    pub(crate) upsert: Option<Field<'a, String>>,
    pub(crate) delete: Option<Field<'a, String>>,
}

pub(crate) fn collect_models<'a>(ctx: &SchemaContext<'a>) -> BTreeMap<String, ModelInfo<'a>> {
    let mut models: BTreeMap<String, ModelInfo<'a>> = BTreeMap::new();

    if let Some(TypeDefinition::Object(query)) = ctx.types.get(&ctx.query_type) {
        for field in &query.fields {
            let return_type = base_type_name(&field.field_type);
            if let Some(model) = return_type.strip_prefix("Paginated") {
                let namespace = namespace_from_type(model);
                let (node_type, node_boxed) = node_type_for_model(model, ctx);
                models
                    .entry(model.to_string())
                    .and_modify(|info| {
                        info.query_field = Some(field.clone());
                        info.query_return = Some(return_type.clone());
                        info.node_type = node_type.clone();
                        info.node_boxed = node_boxed;
                    })
                    .or_insert(ModelInfo {
                        name: model.to_string(),
                        namespace,
                        query_field: Some(field.clone()),
                        query_return: Some(return_type),
                        node_type,
                        node_boxed,
                        create: None,
                        update: None,
                        upsert: None,
                        delete: None,
                    });
            }
        }
    }

    if let Some(mutation_name) = &ctx.mutation_type {
        if let Some(TypeDefinition::Object(mutation)) = ctx.types.get(mutation_name) {
            for field in &mutation.fields {
                let name = field.name.as_str();
                let (model, slot) = if let Some(model) = name.strip_suffix("Create") {
                    (model.to_string(), "create")
                } else if let Some(model) = name.strip_suffix("Update") {
                    (model.to_string(), "update")
                } else if let Some(model) = name.strip_suffix("Upsert") {
                    (model.to_string(), "upsert")
                } else if let Some(model) = name.strip_suffix("Delete") {
                    (model.to_string(), "delete")
                } else {
                    continue;
                };
                if !mutation_fits_slot(field, slot, ctx) {
                    continue;
                }
                let namespace = namespace_from_type(&model);
                let (node_type, node_boxed) = node_type_for_model(&model, ctx);
                let entry = models.entry(model.clone()).or_insert(ModelInfo {
                    name: model.clone(),
                    namespace,
                    query_field: None,
                    query_return: None,
                    node_type,
                    node_boxed,
                    create: None,
                    update: None,
                    upsert: None,
                    delete: None,
                });
                match slot {
                    "create" => entry.create = Some(field.clone()),
                    "update" => entry.update = Some(field.clone()),
                    "upsert" => entry.upsert = Some(field.clone()),
                    "delete" => entry.delete = Some(field.clone()),
                    _ => {}
                }
            }
        }
    }

    models
}

/// whether the payload has the nullable `object` (or `ok: Boolean` for delete) the helper reads.
fn mutation_fits_slot(field: &Field<String>, slot: &str, ctx: &SchemaContext) -> bool {
    let Type::NamedType(payload) = &field.field_type else {
        return false;
    };
    let Some(TypeDefinition::Object(obj)) = ctx.types.get(payload) else {
        return false;
    };
    let wanted = if slot == "delete" { "ok" } else { "object" };
    obj.fields.iter().any(|f| {
        f.name == wanted
            && is_selected(obj, f, ctx)
            && match &f.field_type {
                Type::NonNullType(_) => false,
                Type::NamedType(name) => slot != "delete" || name == "Boolean",
                Type::ListType(_) => slot != "delete",
            }
    })
}

pub(crate) fn namespace_from_type(name: &str) -> String {
    let words = split_identifier_words(name);
    if words.is_empty() {
        return name.to_string();
    }
    words[0].clone()
}

pub(crate) fn model_accessor_name(model: &str, namespace: &str) -> String {
    let name = model.strip_prefix(namespace).unwrap_or(model);
    let name = if name.is_empty() { model } else { name };
    to_snake(name)
}

fn node_type_for_model<'a>(model: &str, ctx: &SchemaContext<'a>) -> (String, bool) {
    let edge_type = format!("Edged{}", model);
    if let Some(TypeDefinition::Object(obj)) = ctx.types.get(&edge_type) {
        if let Some(node_field) = obj.fields.iter().find(|f| f.name == "node") {
            return strip_option_box(&field_rust_type(obj, node_field, ctx));
        }
    }
    ("serde_json::Value".to_string(), false)
}

pub(crate) fn object_type_for_return<'a>(
    return_type: &str,
    ctx: &SchemaContext<'a>,
) -> (String, bool) {
    if let Some(TypeDefinition::Object(obj)) = ctx.types.get(return_type) {
        if let Some(field) = obj.fields.iter().find(|f| f.name == "object") {
            let rust = rust_type(&field.field_type, ctx, false);
            return strip_option_box(&rust);
        }
    }
    ("serde_json::Value".to_string(), false)
}

/// an edge's `node` whose peer is an interface.
pub(crate) fn is_interface_peer(field: &Field<String>, ctx: &SchemaContext) -> bool {
    field.name == "node" && ctx.interfaces.contains(&base_type_name(&field.field_type))
}

/// the `node` of a relationship edge, read into the shared peer struct.
pub(crate) fn is_relationship_peer(
    obj: &ObjectType<String>,
    field: &Field<String>,
    ctx: &SchemaContext,
) -> bool {
    field.name == "node" && ctx.relationship_edges.contains(&obj.name)
}

/// what hangs off the fields of models.
pub(crate) struct Relationships {
    /// the edges a model's field returns, directly or in a list field of its type, and no root field does.
    pub(crate) edges: BTreeSet<String>,
    /// the types other than models that a model's field returns and that list edges, of which
    /// infrahub resolves only the scalars and those edges.
    pub(crate) wrappers: BTreeSet<String>,
}

pub(crate) fn relationships(ctx: &SchemaContext) -> Relationships {
    let models: BTreeSet<String> = ctx
        .objects
        .iter()
        .filter_map(|name| edge_node(name, ctx))
        .filter(|node| ctx.objects.contains(node))
        .collect();
    let (mut edges, mut wrappers) = (BTreeSet::new(), BTreeSet::new());
    let mut root = BTreeSet::new();
    hung_off(&ctx.query_type, ctx, &mut root, &mut BTreeSet::new());
    for model in &models {
        hung_off(model, ctx, &mut edges, &mut wrappers);
    }
    edges.retain(|edge| !root.contains(edge));
    wrappers.retain(|wrapper| !models.contains(wrapper));
    Relationships { edges, wrappers }
}

/// collects the edges a field of `holder` returns, itself or as the items of a list field of its
/// type, and the types holding such lists.
fn hung_off(
    holder: &str,
    ctx: &SchemaContext,
    edges: &mut BTreeSet<String>,
    wrappers: &mut BTreeSet<String>,
) {
    for field in fields_of(holder, ctx) {
        let base = base_type_name(&field.field_type);
        if edge_node(&base, ctx).is_some() {
            edges.insert(base);
            continue;
        }
        for item in fields_of(&base, ctx) {
            let item_base = base_type_name(&item.field_type);
            if is_list(&item.field_type) && edge_node(&item_base, ctx).is_some() {
                edges.insert(item_base);
                wrappers.insert(base.clone());
            }
        }
    }
}

/// the type of an edge's `node`, when `name` is an object type with an object or interface `node`.
pub(crate) fn edge_node(name: &str, ctx: &SchemaContext) -> Option<String> {
    if !ctx.objects.contains(name) {
        return None;
    }
    let node = fields_of(name, ctx)
        .iter()
        .find(|field| field.name == "node")?;
    let base = base_type_name(&node.field_type);
    (ctx.objects.contains(&base) || ctx.interfaces.contains(&base)).then_some(base)
}

/// the fields of an interface that go into its struct and its peer selection.
pub(crate) fn peer_fields<'c, 'a>(
    name: &str,
    ctx: &'c SchemaContext<'a>,
) -> Vec<&'c Field<'a, String>> {
    match ctx.types.get(name) {
        Some(TypeDefinition::Interface(iface)) => iface
            .fields
            .iter()
            .filter(|field| identifies_peer(field, ctx))
            .collect(),
        _ => Vec::new(),
    }
}

/// whether a field of a peer's type identifies the peer.
pub(crate) fn identifies_peer(field: &Field<String>, ctx: &SchemaContext) -> bool {
    let base = base_type_name(&field.field_type);
    !has_required_args(field)
        && !should_skip_field(field)
        && (is_scalar_type(&base) || ctx.enums.contains(&base) || ctx.scalars.contains(&base))
}

pub(crate) fn has_required_args(field: &Field<String>) -> bool {
    field
        .arguments
        .iter()
        .any(|arg| matches!(arg.value_type, Type::NonNullType(_)) && arg.default_value.is_none())
}

fn is_field_deprecated(field: &Field<String>) -> bool {
    field.directives.iter().any(|d| d.name == "deprecated")
}

pub(crate) fn is_enum_value_deprecated(value: &EnumValue<String>) -> bool {
    value.directives.iter().any(|d| d.name == "deprecated")
}

pub(crate) fn should_skip_field(field: &Field<String>) -> bool {
    is_field_deprecated(field)
}
