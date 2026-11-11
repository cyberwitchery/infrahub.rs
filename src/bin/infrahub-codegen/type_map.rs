//! mapping graphql types to rust types

use crate::conventions::{is_interface_peer, is_relationship_peer};
use crate::schema::{SchemaContext, PEER_REF};
use crate::selection::{is_required, is_selected};
use graphql_parser::schema::{Field, ObjectType, Type};

pub(crate) fn strip_option_box(ty: &str) -> (String, bool) {
    let inner = strip_wrapped(ty, "Option<", ">");
    let boxed = inner.starts_with("Box<") && inner.ends_with('>');
    let inner = if boxed {
        strip_wrapped(inner, "Box<", ">").to_string()
    } else {
        inner.to_string()
    };
    (inner, boxed)
}

fn strip_wrapped<'a>(value: &'a str, prefix: &str, suffix: &str) -> &'a str {
    if value.starts_with(prefix) && value.ends_with(suffix) {
        &value[prefix.len()..value.len() - suffix.len()]
    } else {
        value
    }
}

pub(crate) fn is_list(ty: &Type<String>) -> bool {
    match ty {
        Type::NonNullType(inner) => is_list(inner),
        Type::ListType(_) => true,
        Type::NamedType(_) => false,
    }
}

pub(crate) fn base_type_name(ty: &Type<String>) -> String {
    match ty {
        Type::NamedType(name) => name.clone(),
        Type::NonNullType(inner) => base_type_name(inner),
        Type::ListType(inner) => base_type_name(inner),
    }
}

pub(crate) fn is_optional(ty: &Type<String>) -> bool {
    !matches!(ty, Type::NonNullType(_))
}

pub(crate) fn is_scalar_type(name: &str) -> bool {
    matches!(
        name,
        "String"
            | "Int"
            | "Float"
            | "Boolean"
            | "ID"
            | "DateTime"
            | "BigInt"
            | "GenericScalar"
            | "FixedGenericScalar"
            | "Upload"
    )
}

/// the type `render_types` declares for `field` of `obj`.
pub(crate) fn field_rust_type(
    obj: &ObjectType<String>,
    field: &Field<String>,
    ctx: &SchemaContext,
) -> String {
    let selected = is_selected(obj, field, ctx);
    let ty = if selected && is_relationship_peer(obj, field, ctx) {
        peer_rust_type(&field.field_type, ctx.helper_ident(PEER_REF))
    } else if selected && is_interface_peer(field, ctx) {
        let base = base_type_name(&field.field_type);
        peer_rust_type(&field.field_type, ctx.type_ident(&base))
    } else {
        rust_type(&field.field_type, ctx, false)
    };
    if is_optional(&field.field_type) || is_required(obj, field, ctx) {
        ty
    } else {
        format!("Option<{ty}>")
    }
}

/// the type `render_responses` declares for a root field.
pub(crate) fn root_rust_type(field: &Field<String>, ctx: &SchemaContext) -> String {
    let base = base_type_name(&field.field_type);
    if ctx.interfaces.contains(&base) {
        peer_rust_type(&field.field_type, ctx.type_ident(&base))
    } else {
        rust_type(&field.field_type, ctx, false)
    }
}

/// `rust_type` for a peer, which is typed by its peer struct `ident`.
fn peer_rust_type(ty: &Type<String>, ident: &str) -> String {
    let peer = peer_rust_type_nonnull(ty, ident, false);
    if is_optional(ty) {
        format!("Option<{peer}>")
    } else {
        peer
    }
}

/// `rust_type_nonnull` for a peer, which is boxed like an object.
fn peer_rust_type_nonnull(ty: &Type<String>, ident: &str, in_list: bool) -> String {
    match ty {
        Type::ListType(inner) => format!("Vec<{}>", peer_rust_type_nonnull(inner, ident, true)),
        Type::NonNullType(inner) => peer_rust_type_nonnull(inner, ident, in_list),
        Type::NamedType(_) if in_list => ident.to_string(),
        Type::NamedType(_) => format!("Box<{ident}>"),
    }
}

pub(crate) fn rust_type(ty: &Type<String>, ctx: &SchemaContext, input: bool) -> String {
    match ty {
        Type::NonNullType(inner) => rust_type_nonnull(inner, ctx, input, false),
        _ => format!("Option<{}>", rust_type_nonnull(ty, ctx, input, false)),
    }
}

pub(crate) fn rust_type_nonnull(
    ty: &Type<String>,
    ctx: &SchemaContext,
    input: bool,
    in_list: bool,
) -> String {
    match ty {
        Type::ListType(inner) => format!("Vec<{}>", rust_type_nonnull(inner, ctx, input, true)),
        Type::NonNullType(inner) => rust_type_nonnull(inner, ctx, input, in_list),
        Type::NamedType(name) => match name.as_str() {
            "String" | "ID" | "DateTime" => "String".to_string(),
            "Int" => "i64".to_string(),
            "Float" => "f64".to_string(),
            "Boolean" => "bool".to_string(),
            "BigInt" => "i64".to_string(),
            "GenericScalar" | "FixedGenericScalar" => "serde_json::Value".to_string(),
            "Upload" => "Vec<u8>".to_string(),
            _ => {
                if ctx.enums.contains(name)
                    || ctx.inputs.contains(name)
                    || ctx.unions.contains(name)
                {
                    ctx.type_ident(name).to_string()
                } else if ctx.objects.contains(name) {
                    let ident = ctx.type_ident(name);
                    if input || in_list {
                        ident.to_string()
                    } else {
                        format!("Box<{ident}>")
                    }
                } else {
                    "serde_json::Value".to_string()
                }
            }
        },
    }
}

pub(crate) fn format_gql_type(ty: &Type<String>) -> String {
    match ty {
        Type::NamedType(name) => name.clone(),
        Type::NonNullType(inner) => format!("{}!", format_gql_type(inner)),
        Type::ListType(inner) => format!("[{}]", format_gql_type(inner)),
    }
}
