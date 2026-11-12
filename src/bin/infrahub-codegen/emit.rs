//! emitters for the generated crate

use crate::cli::Args;
use crate::conventions::{
    collect_models, is_enum_value_deprecated, model_accessor_name, object_type_for_return,
    peer_fields, should_skip_field, ModelInfo,
};
use crate::schema::{
    method_arg_names, scope_idents, scope_names, to_rust_field, to_rust_ident, to_snake,
    SchemaContext, PEER_REF,
};
use crate::selection::{selection_for_field, selection_for_type};
use crate::type_map::{
    base_type_name, field_rust_type, format_gql_type, is_optional, root_rust_type, rust_type,
    rust_type_nonnull,
};
use graphql_parser::schema::{Field, InputObjectType, InputValue, TypeDefinition, UnionType};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub(crate) fn generate_client(args: &Args, ctx: &SchemaContext) -> Result<(), String> {
    let out_dir = &args.out_dir;
    let manifest_path = out_dir.join("Cargo.toml");
    let crate_name = match &args.crate_name {
        Some(name) => Some(name.as_str()),
        None if manifest_path.exists() => None,
        None => Some(default_crate_name(out_dir)?),
    };
    let src_dir = out_dir.join("src");
    let api_dir = src_dir.join("api");
    fs::create_dir_all(&src_dir).map_err(|err| err.to_string())?;
    fs::create_dir_all(&api_dir).map_err(|err| err.to_string())?;

    if let Some(crate_name) = crate_name {
        let mut cargo = String::new();
        cargo.push_str("[package]\n");
        cargo.push_str(&format!("name = \"{}\"\n", crate_name));
        cargo.push_str(&format!("version = \"{}\"\n", env!("CARGO_PKG_VERSION")));
        cargo.push_str("edition = \"2021\"\n\n");
        cargo.push_str("[dependencies]\n");
        if let Some(path) = &args.infrahub_path {
            cargo.push_str(&format!("infrahub = {{ path = \"{}\" }}\n", path));
        } else {
            cargo.push_str(&format!("infrahub = \"{}\"\n", env!("CARGO_PKG_VERSION")));
        }
        cargo.push_str("serde = { version = \"1\", features = [\"derive\"] }\n");
        cargo.push_str("serde_json = \"1\"\n");
        fs::write(manifest_path, cargo).map_err(|err| err.to_string())?;
    }

    let types_rs = render_types(ctx);
    fs::write(src_dir.join("types.rs"), types_rs).map_err(|err| err.to_string())?;

    let inputs_rs = render_inputs(ctx);
    fs::write(src_dir.join("inputs.rs"), inputs_rs).map_err(|err| err.to_string())?;

    let responses_rs = render_responses(ctx);
    fs::write(src_dir.join("responses.rs"), responses_rs).map_err(|err| err.to_string())?;

    let client_rs = render_client(ctx);
    fs::write(src_dir.join("client.rs"), client_rs).map_err(|err| err.to_string())?;

    let api_mod = render_api_mod(ctx);
    fs::write(api_dir.join("mod.rs"), api_mod).map_err(|err| err.to_string())?;

    let api_modules = render_api_modules(ctx);
    for (name, content) in api_modules {
        fs::write(api_dir.join(format!("{name}.rs")), content).map_err(|err| err.to_string())?;
    }

    let lib_rs = render_lib();
    fs::write(src_dir.join("lib.rs"), lib_rs).map_err(|err| err.to_string())?;

    Ok(())
}

fn default_crate_name(out_dir: &Path) -> Result<&str, String> {
    let name = out_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let mut chars = name.chars();
    let valid = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if valid {
        Ok(name)
    } else {
        Err(format!(
            "the last component of --out {out_dir:?} is not a valid package name; pass --crate-name"
        ))
    }
}

fn render_lib() -> String {
    let mut out = String::new();
    out.push_str("//! generated infrahub client\n\n");
    out.push_str("pub mod api;\n");
    out.push_str("pub mod client;\n");
    out.push_str("pub mod inputs;\n");
    out.push_str("pub mod responses;\n");
    out.push_str("pub mod types;\n\n");
    out.push_str("pub use client::GeneratedClient;\n");
    out.push_str("pub use api::{Api, ApiClient};\n");
    out
}

/// emit a struct field, prefixed with `#[serde(rename = "...")]` when the rust
/// identifier diverges from the schema field name.
fn push_struct_field(out: &mut String, schema_name: &str, ty: &str) {
    push_named_field(out, &to_rust_field(schema_name), schema_name, ty);
}

fn push_named_field(out: &mut String, rust_name: &str, schema_name: &str, ty: &str) {
    if rust_name != schema_name {
        out.push_str(&format!("    #[serde(rename = \"{}\")]\n", schema_name));
    }
    out.push_str(&format!("    pub {}: {},\n", rust_name, ty));
}

pub(crate) fn render_types(ctx: &SchemaContext) -> String {
    let mut out = String::new();
    out.push_str("//! generated types\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");

    for enum_name in &ctx.enums {
        if let Some(TypeDefinition::Enum(enum_ty)) = ctx.types.get(enum_name) {
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]\n");
            out.push_str(&format!("pub enum {} {{\n", ctx.type_ident(enum_name)));
            let values: Vec<_> = enum_ty
                .values
                .iter()
                .filter(|value| !is_enum_value_deprecated(value))
                .collect();
            let variants = scope_idents(
                values.iter().map(|value| value.name.as_str()),
                &[],
                to_rust_ident,
                str::to_string,
            );
            for (value, variant) in values.iter().zip(&variants) {
                out.push_str(&format!("    #[serde(rename = \"{}\")]\n", value.name));
                out.push_str(&format!("    {},\n", variant));
            }
            out.push_str("}\n\n");
        }
    }

    for obj_name in &ctx.objects {
        if obj_name == &ctx.query_type {
            continue;
        }
        if let Some(TypeDefinition::Object(obj)) = ctx.types.get(obj_name) {
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", ctx.type_ident(obj_name)));
            let fields: Vec<_> = obj
                .fields
                .iter()
                .filter(|field| !should_skip_field(field))
                .collect();
            let names = scope_names(fields.iter().map(|field| field.name.as_str()), &[]);
            for (field, rust_name) in fields.iter().zip(&names) {
                push_named_field(
                    &mut out,
                    rust_name,
                    field.name.as_str(),
                    &field_rust_type(obj, field, ctx),
                );
            }
            out.push_str("}\n\n");
        }
    }

    for name in &ctx.interfaces {
        out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
        out.push_str(&format!("pub struct {} {{\n", ctx.type_ident(name)));
        out.push_str("    #[serde(rename = \"__typename\")]\n");
        out.push_str("    pub typename: Option<String>,\n");
        let fields = peer_fields(name, ctx);
        let names = scope_names(
            fields.iter().map(|field| field.name.as_str()),
            &["typename"],
        );
        for (field, rust_name) in fields.iter().zip(&names) {
            let ty = rust_type_nonnull(&field.field_type, ctx, false, false);
            push_named_field(
                &mut out,
                rust_name,
                field.name.as_str(),
                &format!("Option<{ty}>"),
            );
        }
        out.push_str("}\n\n");
    }

    if !ctx.relationship_edges.is_empty() {
        out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
        out.push_str(&format!("pub struct {} {{\n", ctx.helper_ident(PEER_REF)));
        out.push_str("    #[serde(rename = \"__typename\")]\n");
        out.push_str("    pub typename: Option<String>,\n");
        out.push_str("    pub id: Option<String>,\n");
        out.push_str("}\n\n");
    }

    for union_name in &ctx.unions {
        if let Some(TypeDefinition::Union(UnionType { name, .. })) = ctx.types.get(union_name) {
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            out.push_str(&format!(
                "pub struct {}(pub serde_json::Value);\n\n",
                ctx.type_ident(name)
            ));
        }
    }

    out
}

pub(crate) fn render_inputs(ctx: &SchemaContext) -> String {
    let mut out = String::new();
    out.push_str("//! generated input types\n\n");
    out.push_str("#![allow(non_snake_case)]\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");
    out.push_str("use crate::types::*;\n\n");

    for input_name in &ctx.inputs {
        if let Some(TypeDefinition::InputObject(InputObjectType { name, fields, .. })) =
            ctx.types.get(input_name)
        {
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", ctx.type_ident(name)));
            let names = scope_names(fields.iter().map(|field| field.name.as_str()), &[]);
            for (field, rust_name) in fields.iter().zip(&names) {
                let ty = rust_type(&field.value_type, ctx, true);
                if is_optional(&field.value_type) {
                    out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                }
                push_named_field(&mut out, rust_name, field.name.as_str(), &ty);
            }
            out.push_str("}\n\n");
        }
    }

    out
}

pub(crate) fn render_responses(ctx: &SchemaContext) -> String {
    let mut out = String::new();
    out.push_str("//! generated response wrappers\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");
    out.push_str("use crate::types::*;\n\n");

    let query = ctx.types.get(&ctx.query_type).and_then(|ty| match ty {
        TypeDefinition::Object(obj) => Some(obj),
        _ => None,
    });

    if let Some(query) = query {
        for field in &query.fields {
            let resp_name = ctx.response_ident(false, &field.name);
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", resp_name));
            let ty = root_rust_type(field, ctx);
            push_struct_field(&mut out, field.name.as_str(), &ty);
            out.push_str("}\n\n");
        }
    }

    if let Some(mutation_name) = &ctx.mutation_type {
        if let Some(TypeDefinition::Object(mutation)) = ctx.types.get(mutation_name) {
            for field in &mutation.fields {
                let resp_name = ctx.response_ident(true, &field.name);
                out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
                out.push_str(&format!("pub struct {} {{\n", resp_name));
                let ty = root_rust_type(field, ctx);
                push_struct_field(&mut out, field.name.as_str(), &ty);
                out.push_str("}\n\n");
            }
        }
    }

    out
}

pub(crate) fn render_client(ctx: &SchemaContext) -> String {
    let mut out = String::new();
    out.push_str("//! generated client\n\n");
    out.push_str("#![allow(non_snake_case, clippy::too_many_arguments, clippy::field_reassign_with_default)]\n\n");
    out.push_str("use ::infrahub::{Client, GraphQlResponse, Result};\n");
    out.push_str("use serde_json::Value;\n\n");
    out.push_str("use crate::inputs::*;\n");
    out.push_str("use crate::responses::*;\n");
    out.push_str("use crate::types::*;\n\n");

    out.push_str("pub trait GeneratedClient {\n");
    out.push_str("    fn generated(&self) -> GeneratedClientImpl<'_>;\n");
    out.push_str("}\n\n");

    out.push_str("impl GeneratedClient for Client {\n");
    out.push_str("    fn generated(&self) -> GeneratedClientImpl<'_> {\n");
    out.push_str("        GeneratedClientImpl { client: self }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("pub struct GeneratedClientImpl<'a> {\n");
    out.push_str("    client: &'a Client,\n");
    out.push_str("}\n\n");

    out.push_str("impl<'a> GeneratedClientImpl<'a> {\n");

    let mut fields = Vec::new();
    if let Some(TypeDefinition::Object(query)) = ctx.types.get(&ctx.query_type) {
        fields.extend(query.fields.iter().map(|field| (field, false)));
    }
    if let Some(mutation_name) = &ctx.mutation_type {
        if let Some(TypeDefinition::Object(mutation)) = ctx.types.get(mutation_name) {
            fields.extend(mutation.fields.iter().map(|field| (field, true)));
        }
    }
    let names = scope_names(fields.iter().map(|(field, _)| field.name.as_str()), &[]);
    for ((field, is_mutation), name) in fields.into_iter().zip(&names) {
        out.push_str(&render_field_method(field, name, ctx, is_mutation));
    }

    out.push_str("}\n");

    out
}

pub(crate) fn render_api_mod<'a>(ctx: &SchemaContext<'a>) -> String {
    let namespaces = api_namespaces(ctx);

    let mut out = String::new();
    out.push_str("//! generated ergonomic api\n\n");
    out.push_str("use ::infrahub::Client;\n\n");
    for (ns, _) in namespaces.values() {
        out.push_str(&format!("pub mod {};\n", ns));
    }
    out.push('\n');
    out.push_str("pub struct Api<'a> {\n");
    out.push_str("    client: &'a Client,\n");
    out.push_str("}\n\n");
    out.push_str("pub trait ApiClient {\n");
    out.push_str("    fn api(&self) -> Api<'_>;\n");
    out.push_str("}\n\n");
    out.push_str("impl ApiClient for Client {\n");
    out.push_str("    fn api(&self) -> Api<'_> {\n");
    out.push_str("        Api { client: self }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("impl<'a> Api<'a> {\n");
    for (snake, (ns, _)) in &namespaces {
        let struct_name = ctx.helper_ident(&format!("{}Api", to_rust_ident(snake)));
        out.push_str(&format!(
            "    pub fn {}(&self) -> {}::{}<'a> {{\n",
            ns, ns, struct_name
        ));
        out.push_str(&format!(
            "        {}::{}::new(self.client)\n",
            ns, struct_name
        ));
        out.push_str("    }\n");
    }
    out.push_str("}\n");
    out
}

/// the api modules' identifiers, and the models in each, keyed by `to_snake` of the namespace.
fn api_namespaces<'a>(ctx: &SchemaContext<'a>) -> BTreeMap<String, (String, Vec<ModelInfo<'a>>)> {
    let mut by_ns: BTreeMap<String, Vec<ModelInfo<'a>>> = BTreeMap::new();
    for model in collect_models(ctx).into_values() {
        by_ns
            .entry(to_snake(&model.namespace))
            .or_default()
            .push(model);
    }
    let idents = scope_names(by_ns.keys().map(String::as_str), &["r#mod"]);
    by_ns
        .into_iter()
        .zip(idents)
        .map(|((snake, models), ident)| (snake, (ident, models)))
        .collect()
}

/// each api module's source, keyed by its file stem.
pub(crate) fn render_api_modules<'a>(ctx: &SchemaContext<'a>) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for (snake, (ns, mut models)) in api_namespaces(ctx) {
        models.sort_by(|a, b| a.name.cmp(&b.name));
        let stem = ns.trim_start_matches("r#").to_string();
        out.insert(stem, render_api_module(&snake, &models, ctx));
    }
    out
}

fn render_api_module<'a>(
    namespace: &str,
    models: &[ModelInfo<'a>],
    ctx: &SchemaContext<'a>,
) -> String {
    let struct_name = ctx.helper_ident(&format!("{}Api", to_rust_ident(namespace)));
    let mut out = String::new();
    out.push_str("//! generated api module\n\n");
    out.push_str("#![allow(non_snake_case, unused_imports, unused_assignments, clippy::field_reassign_with_default)]\n\n");
    out.push_str(
        "use ::infrahub::{BoxExtract, BoxFetch, BoxFutureResult, Client, DynPaginator, EdgePage, Error, Result};\n",
    );
    out.push_str("use serde_json::Value;\n\n");
    out.push_str("use crate::inputs::*;\n");
    out.push_str("use crate::responses::*;\n");
    out.push_str("use crate::types::*;\n\n");

    out.push_str(&format!("pub struct {}<'a> {{\n", struct_name));
    out.push_str("    client: &'a Client,\n");
    out.push_str("}\n\n");
    out.push_str(&format!("impl<'a> {}<'a> {{\n", struct_name));
    out.push_str("    pub(crate) fn new(client: &'a Client) -> Self {\n");
    out.push_str("        Self { client }\n");
    out.push_str("    }\n\n");
    let accessors = models
        .iter()
        .map(|model| model_accessor_name(&model.name, &model.namespace))
        .collect::<Vec<_>>();
    let accessors = scope_names(accessors.iter().map(String::as_str), &["new"]);
    for (model, accessor) in models.iter().zip(&accessors) {
        let client_struct = ctx.helper_ident(&format!("{}Client", model.name));
        out.push_str(&format!(
            "    pub fn {}(&self) -> {}<'a> {{\n",
            accessor, client_struct
        ));
        out.push_str(&format!("        {}::new(self.client)\n", client_struct));
        out.push_str("    }\n");
    }
    out.push_str("}\n\n");

    for model in models {
        out.push_str(&render_model_client(model, ctx));
    }

    out
}

fn render_model_client<'a>(model: &ModelInfo<'a>, ctx: &SchemaContext<'a>) -> String {
    let mut out = String::new();
    let client_struct = ctx.helper_ident(&format!("{}Client", model.name));
    let filters_struct = ctx.helper_ident(&format!("{}Filters", model.name));
    let model_field = to_rust_field(model.name.as_str());

    if let Some(query_field) = &model.query_field {
        let args = &query_field.arguments;
        let names = scope_names(args.iter().map(|arg| arg.name.as_str()), &[]);
        out.push_str(&format!(
            "#[derive(Debug, Clone, Default)]\npub struct {} {{\n",
            filters_struct
        ));
        for (arg, rust_name) in args.iter().zip(&names) {
            let inner = rust_type_nonnull(&arg.value_type, ctx, true, false);
            out.push_str(&format!("    pub {}: Option<{}>,\n", rust_name, inner));
        }
        out.push_str("}\n\n");
        out.push_str(&format!("impl {} {{\n", filters_struct));
        out.push_str("    fn to_vars(&self) -> Result<Value> {\n");
        out.push_str("        let mut vars = serde_json::Map::new();\n");
        for (arg, rust_name) in args.iter().zip(&names) {
            out.push_str(&format!(
                "        if let Some(value) = &self.{rust_name} {{\n"
            ));
            out.push_str(&format!(
                "            vars.insert(\"{}\".to_string(), serde_json::to_value(value)?);\n",
                arg.name
            ));
            out.push_str("        }\n");
        }
        out.push_str("        Ok(Value::Object(vars))\n");
        out.push_str("    }\n");
        out.push_str("}\n\n");
    }

    out.push_str(&format!("pub struct {}<'a> {{\n", client_struct));
    out.push_str("    client: &'a Client,\n");
    out.push_str("}\n\n");
    out.push_str(&format!("impl<'a> {}<'a> {{\n", client_struct));
    out.push_str("    pub(crate) fn new(client: &'a Client) -> Self {\n");
    out.push_str("        Self { client }\n");
    out.push_str("    }\n\n");

    if let Some(query_field) = &model.query_field {
        let query_name = query_field.name.clone();
        let response_type = ctx.response_ident(false, &query_name);
        let vars_def = render_variable_defs(&query_field.arguments);
        let field_args = render_field_args(&query_field.arguments);
        let has_after = query_field.arguments.iter().any(|arg| arg.name == "after");
        let has_offset = query_field.arguments.iter().any(|arg| arg.name == "offset");
        let return_type = model
            .query_return
            .clone()
            .unwrap_or_else(|| "serde_json::Value".to_string());
        let selection = selection_for_type(&return_type, ctx, &mut BTreeSet::new(), 0);
        let op_header = if vars_def.is_empty() {
            format!("query {}", query_name)
        } else {
            format!("query {}({})", query_name, vars_def)
        };

        out.push_str(&format!(
            "    pub async fn list(&self, filters: Option<{filters_struct}>, request_branch: Option<&str>) -> Result<Vec<{model_type}>> {{\n",
            model_type = model.node_type
        ));
        out.push_str("        let vars = filters.map(|f| f.to_vars()).transpose()?.unwrap_or_else(|| Value::Object(serde_json::Map::new()));\n");
        out.push_str(&format!(
            "        let query = r#\"{op} {{ {name}{args} {sel} }}\"#;\n",
            op = op_header,
            name = query_name,
            args = field_args,
            sel = selection
        ));
        out.push_str(&format!(
            "        let response = self.client.execute::<{}>(query, Some(vars), request_branch).await?;\n",
            response_type
        ));
        out.push_str("        let data = response.data.ok_or_else(|| Error::Config(\"missing data\".to_string()))?;\n");
        out.push_str("        let mut items = Vec::new();\n");
        out.push_str(&format!(
            "        for edge in data.{field}.edges {{\n",
            field = model_field
        ));
        out.push_str("            if let Some(node) = edge.node {\n");
        if model.node_boxed {
            out.push_str("                items.push(*node);\n");
        } else {
            out.push_str("                items.push(node);\n");
        }
        out.push_str("            }\n");
        out.push_str("        }\n");
        out.push_str("        Ok(items)\n");
        out.push_str("    }\n\n");

        out.push_str(&format!(
            "    pub fn paginate(&self, filters: Option<{filters_struct}>, request_branch: Option<&str>) -> DynPaginator<'a, {model_type}, String, ({response_type}, i64)> {{\n",
            model_type = model.node_type,
            response_type = response_type,
        ));
        out.push_str("        let client = self.client;\n");
        out.push_str("        let base_filters = filters.unwrap_or_default();\n");
        out.push_str("        let request_branch = request_branch.map(|b| b.to_string());\n");
        out.push_str(&format!(
            "        let query = r#\"{op} {{ {name}{args} {sel} }}\"#;\n",
            op = op_header,
            name = query_name,
            args = field_args,
            sel = selection
        ));
        out.push_str("        let fetch: BoxFetch<'a, String, (");
        out.push_str(response_type);
        out.push_str(", i64)> = Box::new(move |cursor: Option<String>| -> BoxFutureResult<'a, (");
        out.push_str(response_type);
        out.push_str(", i64)> {\n");
        out.push_str("            let mut page_filters = base_filters.clone();\n");
        out.push_str("            let branch = request_branch.clone();\n");
        out.push_str("            let mut current_offset: i64 = 0;\n");
        if has_after {
            out.push_str("            page_filters.after = cursor.clone();\n");
        }
        if has_offset {
            out.push_str("            let base_offset = page_filters.offset.unwrap_or(0);\n");
            out.push_str("            current_offset = cursor\n");
            out.push_str("                .as_deref()\n");
            out.push_str("                .and_then(|c| c.parse::<i64>().ok())\n");
            out.push_str("                .unwrap_or(base_offset);\n");
            out.push_str("            page_filters.offset = Some(current_offset);\n");
        }
        out.push_str("            Box::pin(async move {\n");
        out.push_str("            let vars = page_filters.to_vars()?;\n");
        out.push_str(&format!(
            "                let response = client.execute::<{}>(query, Some(vars), branch.as_deref()).await?;\n",
            response_type
        ));
        out.push_str("                let data = response.data.ok_or_else(|| Error::Config(\"missing data\".to_string()))?;\n");
        out.push_str("                Ok((data, current_offset))\n");
        out.push_str("            })\n");
        out.push_str("        });\n");
        out.push_str("        let extract: BoxExtract<'a, ");
        out.push_str(&model.node_type);
        out.push_str(", String, (");
        out.push_str(response_type);
        out.push_str(", i64)> = Box::new(move |(data, current_offset): (");
        out.push_str(response_type);
        out.push_str(", i64)| -> Result<EdgePage<");
        out.push_str(&model.node_type);
        out.push_str(", String>> {\n");
        out.push_str("            let mut items = Vec::new();\n");
        out.push_str("            let mut next: Option<String> = None;\n");
        out.push_str(&format!(
            "            for edge in data.{field}.edges {{\n",
            field = model_field
        ));
        out.push_str("                if let Some(node) = edge.node {\n");
        if model.node_boxed {
            out.push_str("                    items.push(*node);\n");
        } else {
            out.push_str("                    items.push(node);\n");
        }
        out.push_str("                }\n");
        if has_after {
            out.push_str(
                "                if let Ok(cursor_value) = serde_json::to_value(&edge.cursor) {\n",
            );
            out.push_str("                    if let Some(cursor_str) = cursor_value.as_str() {\n");
            out.push_str("                        next = Some(cursor_str.to_string());\n");
            out.push_str("                    }\n");
            out.push_str("                }\n");
        }
        out.push_str("            }\n");
        if has_offset && !has_after {
            out.push_str("            if !items.is_empty() {\n");
            out.push_str(
                "                next = Some((current_offset + items.len() as i64).to_string());\n",
            );
            out.push_str("            }\n");
        }
        out.push_str("            Ok(EdgePage { nodes: items, next_cursor: next })\n");
        out.push_str("        });\n");
        out.push_str("        ::infrahub::Paginator::new(fetch, extract)\n");
        out.push_str("    }\n\n");

        if query_field.arguments.iter().any(|arg| arg.name == "ids") {
            out.push_str(&format!(
                "    pub async fn get_by_id(&self, id: impl Into<String>, request_branch: Option<&str>) -> Result<Option<{}>> {{\n",
                model.node_type
            ));
            out.push_str(&format!(
                "        let mut filters = {}::default();\n",
                filters_struct
            ));
            out.push_str("        filters.ids = Some(vec![id.into()]);\n");
            out.push_str(
                "        let mut items = self.list(Some(filters), request_branch).await?;\n",
            );
            out.push_str("        Ok(items.pop())\n");
            out.push_str("    }\n\n");
        }
    }

    out.push_str(&render_mutation_helpers(model, ctx));
    out.push_str("}\n\n");
    out
}

fn render_mutation_helpers<'a>(model: &ModelInfo<'a>, ctx: &SchemaContext<'a>) -> String {
    let mut out = String::new();
    let mutations = [
        ("create", &model.create),
        ("update", &model.update),
        ("upsert", &model.upsert),
        ("delete", &model.delete),
    ];

    for (name, field_opt) in mutations {
        let Some(field) = field_opt else { continue };
        let field_name = field.name.clone();
        let vars_def = render_variable_defs(&field.arguments);
        let field_args = render_field_args(&field.arguments);
        let return_type = base_type_name(&field.field_type);
        let selection = selection_for_type(&return_type, ctx, &mut BTreeSet::new(), 0);
        let (object_type, object_boxed) = object_type_for_return(&return_type, ctx);
        let response_type = ctx.response_ident(true, &field_name);
        let response_field = to_rust_field(&field_name);
        let op_header = if vars_def.is_empty() {
            format!("mutation {}", field_name)
        } else {
            format!("mutation {}({})", field_name, vars_def)
        };

        let mut method_args = Vec::new();
        for (arg, rust_name) in field
            .arguments
            .iter()
            .zip(method_arg_names(&field.arguments))
        {
            let ty = rust_type(&arg.value_type, ctx, true);
            method_args.push(format!("{rust_name}: {ty}"));
        }
        method_args.push("request_branch: Option<&str>".to_string());

        let ret = if name == "delete" {
            "bool".to_string()
        } else {
            object_type.clone()
        };
        out.push_str(&format!(
            "    pub async fn {name}(&self, {args}) -> Result<{ret}> {{\n",
            name = name,
            args = method_args.join(", "),
            ret = ret
        ));
        out.push_str("        let mut vars = serde_json::Map::new();\n");
        out.push_str(&render_vars_builder(&field.arguments));
        out.push_str("        let vars = Value::Object(vars);\n");
        out.push_str(&format!(
            "        let query = r#\"{op} {{ {fname}{args} {sel} }}\"#;\n",
            op = op_header,
            fname = field_name,
            args = field_args,
            sel = selection
        ));
        out.push_str(&format!(
            "        let response = self.client.execute::<{resp}>(query, Some(vars), request_branch).await?;\n",
            resp = response_type
        ));
        out.push_str("        let data = response.data.ok_or_else(|| Error::Config(\"missing data\".to_string()))?;\n");
        out.push_str(&format!(
            "        let payload = data.{field}.ok_or_else(|| Error::Config(\"missing payload\".to_string()))?;\n",
            field = response_field
        ));
        if name == "delete" {
            out.push_str("        Ok(payload.ok.unwrap_or(false))\n");
        } else {
            out.push_str("        let object = payload.object.ok_or_else(|| Error::Config(\"missing object\".to_string()))?;\n");
            if object_boxed {
                out.push_str("        Ok(*object)\n");
            } else {
                out.push_str("        Ok(object)\n");
            }
        }
        out.push_str("    }\n\n");
    }

    out
}

fn render_field_method(
    field: &Field<String>,
    method_name: &str,
    ctx: &SchemaContext,
    is_mutation: bool,
) -> String {
    let mut out = String::new();
    let op_name = if is_mutation { "mutation" } else { "query" };
    let query_name = to_rust_ident(field.name.as_str());
    let response_name = ctx.response_ident(is_mutation, &field.name);

    let args = render_args(&field.arguments, ctx);
    let vars_builder = render_vars_builder(&field.arguments);
    let field_args = render_field_args(&field.arguments);

    let selection = selection_for_field(field, ctx);
    let var_defs = render_variable_defs(&field.arguments);
    let op_header = if var_defs.is_empty() {
        format!("{} {}", op_name, query_name)
    } else {
        format!("{} {}({})", op_name, query_name, var_defs)
    };
    let query = format!(
        "{} {{ {}{}{} }}",
        op_header, field.name, field_args, selection
    );

    out.push_str(&format!("    pub async fn {}(&self{} , request_branch: Option<&str>) -> Result<GraphQlResponse<{}>> {{\n", method_name, args.signature, response_name));
    if field.arguments.is_empty() {
        out.push_str("        let vars = serde_json::Map::new();\n");
    } else {
        out.push_str("        let mut vars = serde_json::Map::new();\n");
        out.push_str(&vars_builder);
    }
    out.push_str(&format!("        let query = r#\"{}\"#;\n", query));
    out.push_str("        let vars = Value::Object(vars);\n");
    out.push_str("        self.client.execute(query, Some(vars), request_branch).await\n");
    out.push_str("    }\n\n");

    out
}

fn render_args(args: &[InputValue<String>], ctx: &SchemaContext) -> MethodArgs {
    let mut signature_parts = Vec::new();
    for (arg, rust_name) in args.iter().zip(method_arg_names(args)) {
        let ty = rust_type(&arg.value_type, ctx, true);
        signature_parts.push(format!("{}: {}", rust_name, ty));
    }
    let signature = if signature_parts.is_empty() {
        "".to_string()
    } else {
        format!(", {}", signature_parts.join(", "))
    };

    MethodArgs { signature }
}

fn render_vars_builder(args: &[InputValue<String>]) -> String {
    let mut out = String::new();
    for (arg, rust_name) in args.iter().zip(method_arg_names(args)) {
        let var_name = &arg.name;
        if is_optional(&arg.value_type) {
            out.push_str(&format!("        if let Some(value) = {} {{\n", rust_name));
            out.push_str(&format!(
                "            vars.insert(\"{}\".to_string(), serde_json::to_value(value)?);\n",
                var_name
            ));
            out.push_str("        }\n");
        } else {
            out.push_str(&format!(
                "        vars.insert(\"{}\".to_string(), serde_json::to_value({})?);\n",
                var_name, rust_name
            ));
        }
    }
    out
}

fn render_variable_defs(args: &[InputValue<String>]) -> String {
    let mut defs = Vec::new();
    for arg in args {
        let gql_type = format_gql_type(&arg.value_type);
        defs.push(format!("${}: {}", arg.name, gql_type));
    }
    defs.join(", ")
}

fn render_field_args(args: &[InputValue<String>]) -> String {
    if args.is_empty() {
        return String::new();
    }

    let mut parts = Vec::new();
    for arg in args {
        parts.push(format!("{}: ${}", arg.name, arg.name));
    }
    format!("({})", parts.join(", "))
}

struct MethodArgs {
    signature: String,
}

#[cfg(test)]
mod codegen_name_tests {
    use super::*;
    use crate::conventions::{edge_node, namespace_from_type};
    use crate::schema::fields_of;
    use graphql_parser::schema::parse_schema;

    #[test]
    fn test_namespace_from_type_prefers_first_word() {
        assert_eq!(namespace_from_type("CoreRepository"), "Core");
        assert_eq!(namespace_from_type("InfrahubTask"), "Infrahub");
        assert_eq!(namespace_from_type("BuiltinIPAddress"), "Builtin");
        assert_eq!(namespace_from_type("IPAMNamespace"), "IPAM");
    }

    #[test]
    fn test_to_snake() {
        for (input, expected) in [
            // acronyms
            ("IPAM", "ipam"),
            ("GraphQLQuery", "graph_ql_query"),
            ("IPAddressPool", "ip_address_pool"),
            // digit to alpha boundary
            ("CoreTransformJinja2Create", "core_transform_jinja2_create"),
            ("CoreTransformJinja2Update", "core_transform_jinja2_update"),
            ("Http2Client", "http2_client"),
            ("V2Api", "v2_api"),
            ("route53Zone", "route53_zone"),
        ] {
            assert_eq!(to_snake(input), expected, "to_snake({input:?})");
        }
    }

    #[test]
    fn test_to_rust_field() {
        for (input, expected) in [
            // acronyms
            ("nodeID", "node_id"),
            ("hFID", "h_fid"),
            ("ipAddress", "ip_address"),
            ("nodeUUID", "node_uuid"),
            ("simple", "simple"),
            ("camelCase", "camel_case"),
            // keywords that can be raw identifiers
            ("type", "r#type"),
            ("yield", "r#yield"),
            ("abstract", "r#abstract"),
            ("try", "r#try"),
            ("gen", "r#gen"),
            // keywords that cannot be raw identifiers get a suffix
            ("self", "self_"),
            ("Self", "self_"),
            ("super", "super_"),
            ("crate", "crate_"),
            // leading underscores
            ("_", "__"),
            ("__", "__"),
            ("_1st", "_1_st"),
            ("_updated_at", "_updated_at"),
        ] {
            assert_eq!(to_rust_field(input), expected, "to_rust_field({input:?})");
        }
    }

    #[test]
    fn test_to_rust_ident() {
        for (input, expected) in [
            // all-caps enum variants
            ("DENY", "Deny"),
            ("ALLOW_DEFAULT", "AllowDefault"),
            ("NEED_UPGRADE_REBASE", "NeedUpgradeRebase"),
            ("BASE_BRANCH", "BaseBranch"),
            ("ASC", "Asc"),
            // pascal case passes through
            ("CoreMenuItem", "CoreMenuItem"),
            ("InfrahubTask", "InfrahubTask"),
            // uppercase clusters are preserved
            ("BuiltinIPAddress", "BuiltinIPAddress"),
            ("CoreGraphQLQuery", "CoreGraphQLQuery"),
            ("CoreIPAddressPool", "CoreIPAddressPool"),
            ("JSONAttribute", "JSONAttribute"),
            ("UpdateHFID", "UpdateHFID"),
            // digit boundaries
            ("CoreTransformJinja2Create", "CoreTransformJinja2Create"),
            ("Http2Client", "Http2Client"),
            ("V2Api", "V2Api"),
            // reserved words
            ("Self", "SelfType"),
            ("Type", "TypeType"),
            ("Box", "BoxType"),
            ("Result", "ResultType"),
            // lowercase input
            ("core", "Core"),
            ("builtin", "Builtin"),
        ] {
            assert_eq!(to_rust_ident(input), expected, "to_rust_ident({input:?})");
        }
    }

    #[test]
    fn test_model_accessor_name_strips_namespace() {
        assert_eq!(
            model_accessor_name("CoreRepository", "Core"),
            "repository".to_string()
        );
        assert_eq!(
            model_accessor_name("IPAMNamespace", "IPAM"),
            "namespace".to_string()
        );
    }

    #[test]
    fn test_deprecated_fields_skipped_in_selection() {
        let schema = r#"
            type Query { info: Info }
            type Info {
                id: String
                _updated_at: DateTime @deprecated(reason: "use node_metadata")
                name: String
            }
            scalar DateTime
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let mut stack = BTreeSet::new();
        let sel = selection_for_type("Info", &ctx, &mut stack, 0);
        assert!(sel.contains("id"));
        assert!(sel.contains("name"));
        assert!(!sel.contains("_updated_at"));
    }

    #[test]
    fn test_deprecated_fields_skipped_in_types() {
        let schema = r#"
            type Query { info: Info }
            type Info {
                id: String
                _updated_at: DateTime @deprecated(reason: "use node_metadata")
                name: String
            }
            scalar DateTime
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let types_rs = render_types(&ctx);
        assert!(types_rs.contains("id"));
        assert!(types_rs.contains("name"));
        assert!(!types_rs.contains("_updated_at"));
    }

    #[test]
    fn test_deprecated_enum_values_skipped() {
        let schema = r#"
            type Query { status: Status }
            enum Status {
                ACTIVE
                DELETED @deprecated(reason: "no longer used")
                PENDING
            }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let types_rs = render_types(&ctx);
        assert!(types_rs.contains("Active"));
        assert!(types_rs.contains("Pending"));
        assert!(
            !types_rs.contains("Deleted"),
            "deprecated DELETED should be skipped, got:\n{types_rs}"
        );
    }

    #[test]
    fn test_enum_no_unknown_variant() {
        let schema = r#"
            type Query { status: Status }
            enum Status {
                ACTIVE
                PENDING
            }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let types_rs = render_types(&ctx);
        assert!(
            types_rs.contains("Active"),
            "ACTIVE should become Active, got:\n{types_rs}"
        );
        assert!(!types_rs.contains("Unknown"));
        assert!(!types_rs.contains("#[serde(other)]"));
    }

    #[test]
    fn test_acronym_fields_in_generated_types() {
        let schema = r#"
            type Query { node: Node }
            type Node {
                nodeID: String
                hFID: String
                ipAddress: String
                nodeUUID: String
                simpleName: String
            }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let types_rs = render_types(&ctx);
        assert!(
            types_rs.contains("pub node_id:"),
            "nodeID should become node_id, got:\n{types_rs}"
        );
        assert!(
            types_rs.contains("pub h_fid:"),
            "hFID should become h_fid, got:\n{types_rs}"
        );
        assert!(
            types_rs.contains("pub ip_address:"),
            "ipAddress should become ip_address, got:\n{types_rs}"
        );
        assert!(
            types_rs.contains("pub node_uuid:"),
            "nodeUUID should become node_uuid, got:\n{types_rs}"
        );
        assert!(
            types_rs.contains("pub simple_name:"),
            "simpleName should become simple_name, got:\n{types_rs}"
        );
    }

    #[test]
    fn test_enum_variants_use_pascal_case() {
        let schema = r#"
            type Query { perm: Permission }
            enum Permission {
                DENY
                ALLOW
                ALLOW_DEFAULT
                ALLOW_OTHER
            }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let types_rs = render_types(&ctx);
        assert!(
            types_rs.contains("Deny,"),
            "DENY should become Deny, got:\n{types_rs}"
        );
        assert!(
            types_rs.contains("Allow,"),
            "ALLOW should become Allow, got:\n{types_rs}"
        );
        assert!(
            types_rs.contains("AllowDefault,"),
            "ALLOW_DEFAULT should become AllowDefault, got:\n{types_rs}"
        );
        assert!(
            types_rs.contains("AllowOther,"),
            "ALLOW_OTHER should become AllowOther, got:\n{types_rs}"
        );
        assert!(
            types_rs.contains("serde(rename = \"DENY\")"),
            "serde rename should preserve original name"
        );
        assert!(
            types_rs.contains("serde(rename = \"ALLOW_DEFAULT\")"),
            "serde rename should preserve original name"
        );
    }

    #[test]
    fn test_custom_scalars_map_to_json_value() {
        for scalar in ["FixedGenericScalar", "JSON", "Decimal"] {
            let schema = format!(
                "scalar {scalar}
                type Query {{ pool(value: {scalar}): Pool }}
                input PoolInput {{ data: {scalar} }}
                type Pool {{ id: String data: {scalar} }}"
            );
            let doc = parse_schema::<String>(&schema).unwrap();
            let ctx = SchemaContext::new(&doc);
            for rendered in [render_types(&ctx), render_inputs(&ctx)] {
                assert!(
                    rendered.contains("pub data: Option<serde_json::Value>,"),
                    "{scalar}: {rendered}"
                );
            }
            let client_rs = render_client(&ctx);
            assert!(
                client_rs.contains("value: Option<serde_json::Value>"),
                "{scalar}: {client_rs}"
            );
        }
    }

    #[test]
    fn test_nullable_input_fields_skip_serializing_none() {
        let schema = r#"
            type Query { ping: String }
            input WidgetInput { id: String! name: String tags: [String!] }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let inputs_rs = render_inputs(&ctx);
        let skip = "    #[serde(skip_serializing_if = \"Option::is_none\")]\n";
        for field in [
            "pub name: Option<String>,",
            "pub tags: Option<Vec<String>>,",
        ] {
            assert!(
                inputs_rs.contains(&format!("{skip}    {field}")),
                "got:\n{inputs_rs}"
            );
        }
        assert!(inputs_rs.contains("pub id: String,"));
        assert!(
            !inputs_rs.contains(&format!("{skip}    pub id:")),
            "got:\n{inputs_rs}"
        );
    }

    #[test]
    fn test_mutation_root_defaults_without_schema_definition() {
        let schema = r#"
            type Query { ping: String }
            type Mutation { pong: String }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_eq!(ctx.query_type, "Query");
        assert_eq!(ctx.mutation_type.as_deref(), Some("Mutation"));
        assert!(render_client(&ctx).contains("pub async fn pong("));
    }

    #[test]
    fn test_no_mutation_root_without_mutation_type() {
        let doc = parse_schema::<String>("type Query { ping: String }").unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_eq!(ctx.mutation_type, None);
    }

    #[test]
    fn test_schema_definition_without_mutation_has_no_mutation_root() {
        let schema = r#"
            schema { query: Query }
            type Query { ping: String }
            type Mutation { pong: String }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_eq!(ctx.mutation_type, None);
        assert!(!render_client(&ctx).contains("pub async fn pong("));
    }

    #[test]
    fn test_schema_definition_with_custom_root_names() {
        let schema = r#"
            schema { query: RootQuery mutation: RootMutation }
            type RootQuery { ping: String }
            type RootMutation { pong: String }
            type Mutation { decoy: String }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_eq!(ctx.query_type, "RootQuery");
        assert_eq!(ctx.mutation_type.as_deref(), Some("RootMutation"));
        let client = render_client(&ctx);
        assert!(client.contains("pub async fn ping("));
        assert!(client.contains("pub async fn pong("));
        assert!(!client.contains("pub async fn decoy("));
    }

    #[test]
    fn test_runtime_crate_paths_survive_an_infrahub_namespace() {
        let schema = r#"
            type Query { InfrahubWidget(ids: [ID]): PaginatedInfrahubWidget }
            type PaginatedInfrahubWidget { edges: [EdgedInfrahubWidget!]! }
            type EdgedInfrahubWidget { node: InfrahubWidget }
            type InfrahubWidget { id: String! }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let api_mod = render_api_mod(&ctx);
        assert!(api_mod.contains("pub mod infrahub;"));
        assert!(api_mod.contains("use ::infrahub::Client;"));
        let modules = render_api_modules(&ctx);
        let module = &modules["infrahub"];
        assert!(module.contains("use ::infrahub::{"));
        assert!(module.contains(" ::infrahub::Paginator::new("));
        assert!(!module.contains(" infrahub::Paginator"));
        for generated in [api_mod.as_str(), module.as_str(), &render_client(&ctx)] {
            assert!(!generated.contains("use infrahub::"), "got:\n{generated}");
        }
    }

    #[test]
    fn test_mutation_slots_require_the_payload_shape_helpers_read() {
        let schema = r#"
            type Query { ping: String }
            type Mutation {
                WidgetCreate(data: String!): WidgetCreate
                WidgetUpdate(data: String!): WidgetUpdate
                WidgetUpsert(data: String!): WidgetUpsert!
                WidgetDelete(data: String!): WidgetDelete
                GadgetCreate(data: String!): GadgetCreate
                GadgetDelete(data: String!): GadgetDelete
                GizmoUpdate(data: String!): GizmoUpdate
                SprocketDelete(data: String!): SprocketDelete
                CogDelete(data: String!): CogDelete
                DiffUpdate(data: String!): DiffUpdateMutation
            }
            interface Gizmo { id: String! }
            type GizmoUpdate { ok: Boolean object: Gizmo }
            type SprocketDelete { ok: String }
            type CogDelete { ok: [Boolean] }
            type Widget { id: String! }
            type Gadget { id: String! }
            type WidgetCreate { ok: Boolean object: Widget }
            type WidgetUpdate { ok: Boolean object: Widget! }
            type WidgetUpsert { ok: Boolean object: Widget }
            type WidgetDelete { ok: Boolean }
            type GadgetCreate { ok: Boolean object: Gadget @deprecated(reason: "gone") }
            type GadgetDelete { ok: Boolean! }
            type DiffUpdateMutation { ok: Boolean task: String }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let models = collect_models(&ctx);
        let widget = &models["Widget"];
        assert!(widget.create.is_some());
        assert!(widget.update.is_none(), "non-null `object`");
        assert!(widget.upsert.is_none(), "non-null payload");
        assert!(widget.delete.is_some());
        assert!(!models.contains_key("Gadget"));
        assert!(
            !models.contains_key("Gizmo"),
            "unselected interface `object`"
        );
        assert!(!models.contains_key("Sprocket"), "non-Boolean `ok`");
        assert!(!models.contains_key("Cog"), "list `ok`");
        assert!(!models.contains_key("Diff"));

        let modules = render_api_modules(&ctx);
        assert_eq!(modules.keys().collect::<Vec<_>>(), ["widget"]);
        let widget_rs = &modules["widget"];
        assert!(widget_rs.contains("pub async fn create("));
        assert!(widget_rs.contains("pub async fn delete("));
        assert!(!widget_rs.contains("pub async fn update("));
        assert!(!widget_rs.contains("pub async fn upsert("));

        let client = render_client(&ctx);
        for raw in [
            "widget_update",
            "widget_upsert",
            "gadget_create",
            "gadget_delete",
            "gizmo_update",
            "sprocket_delete",
            "cog_delete",
            "diff_update",
        ] {
            assert!(client.contains(&format!("pub async fn {raw}(")), "{raw}");
        }
    }

    #[test]
    fn test_empty_edge_node_metadata_is_optional_and_unselected() {
        let schema = r#"
            type Query { Widget: PaginatedWidget }
            type Mutation { WidgetCreate(data: String!): WidgetCreate }
            interface Group { id: String! }
            type Gadget { id: String! }
            type Meta { created_at: String }
            type Props { is_protected: Boolean }
            type NestedEdgedGroup { cursor: String! node: Group node_metadata: Meta! properties: Props }
            type NestedEdgedGadget { node: Gadget node_metadata: Meta properties: Props }
            type BranchEdge { node: Gadget! node_metadata: Meta! }
            type Widget {
                id: String!
                name: String
                node_metadata: Meta!
                parent: NestedEdgedGroup!
                gadget: NestedEdgedGadget!
                branch: BranchEdge!
            }
            type EdgedWidget { node: Widget node_metadata: Meta }
            type PaginatedWidget { count: Int! edges: [EdgedWidget!]! }
            type WidgetCreate { ok: Boolean object: Widget }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let select = |name: &str| selection_for_type(name, &ctx, &mut BTreeSet::new(), 0);
        assert_eq!(
            select("NestedEdgedGroup"),
            "{ cursor node { __typename id } properties { is_protected } }"
        );
        assert_eq!(
            select("NestedEdgedGadget"),
            "{ node { __typename id } node_metadata { created_at } properties { is_protected } }"
        );
        assert_eq!(
            select("BranchEdge"),
            "{ node { __typename id } node_metadata { created_at } }"
        );
        assert_eq!(
            select("WidgetCreate"),
            "{ ok object { id name node_metadata { created_at } \
             parent { cursor node { __typename id } properties { is_protected } } \
             gadget { node { __typename id } node_metadata { created_at } properties { is_protected } } \
             branch { node { __typename id } node_metadata { created_at } } } }"
        );

        let types = render_types(&ctx);
        let node_metadata_of = |name: &str| {
            let start = types.find(&format!("pub struct {name} {{")).unwrap();
            let body = &types[start..start + types[start..].find('}').unwrap()];
            let line = body.lines().find(|l| l.contains(" node_metadata:"));
            line.unwrap().trim().to_string()
        };
        assert_eq!(
            node_metadata_of("NestedEdgedGroup"),
            "pub node_metadata: Option<Box<Meta>>,"
        );
        assert_eq!(
            node_metadata_of("NestedEdgedGadget"),
            "pub node_metadata: Option<Box<Meta>>,"
        );
        assert_eq!(
            node_metadata_of("BranchEdge"),
            "pub node_metadata: Box<Meta>,"
        );
        assert_eq!(node_metadata_of("Widget"), "pub node_metadata: Box<Meta>,");

        let widget_rs = &render_api_modules(&ctx)["widget"];
        assert!(widget_rs.contains("pub async fn create("));
        assert!(widget_rs
            .contains("parent { cursor node { __typename id } properties { is_protected } }"));
        assert!(!widget_rs.contains("parent { node_metadata"));
    }

    #[test]
    fn test_interface_peer_edges_select_and_type_the_peer_identity() {
        let schema = r#"
            type Query { Widget(ids: [ID]): PaginatedWidget Group(ids: [ID]): PaginatedGroup }
            type Mutation {
                WidgetCreate(data: String!): WidgetCreate
                GroupUpdate(data: String!): GroupUpdate
            }
            enum Kind { STANDARD }
            interface Account { id: String }
            interface Group {
                id: String
                hfid: [String!]
                kind: Kind!
                managed: Boolean!
                name: TextAttribute
                label(lang: String!): String
                legacy: String @deprecated(reason: "gone")
            }
            type Meta { created_at: String }
            type Props { is_protected: Boolean }
            type TextAttribute { value: String source: Account }
            type NestedEdgedGroup { node: Group node_metadata: Meta! properties: Props created_by: Account }
            type NestedPaginatedGroup { count: Int! edges: [NestedEdgedGroup!] }
            type PinnedGroupEdge { node: Group! node_metadata: Meta! }
            type GroupListEdge { node: [Group!] node_metadata: Meta }
            type GroupNodes { node: Group }
            type GroupConnection { edges: [GroupNodes!]! count: Int! }
            union Thing = Meta
            type ThingEdge { node: Thing node_metadata: Meta }
            type Widget {
                id: String!
                name: TextAttribute
                parent: NestedEdgedGroup!
                member_of_groups(offset: Int): NestedPaginatedGroup!
            }
            type EdgedWidget { node: Widget node_metadata: Meta }
            type PaginatedWidget { count: Int! edges: [EdgedWidget!]! }
            type EdgedGroup { node: Group node_metadata: Meta }
            type PaginatedGroup { count: Int! edges: [EdgedGroup!]! }
            type WidgetCreate { ok: Boolean object: Widget }
            type GroupUpdate { ok: Boolean object: Group }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let peer = "node { __typename id hfid kind managed }";
        let peer_ref = "node { __typename id }";
        let select =
            |name: &str, depth| selection_for_type(name, &ctx, &mut BTreeSet::new(), depth);
        assert_eq!(
            select("NestedEdgedGroup", 0),
            format!("{{ {peer_ref} properties {{ is_protected }} }}")
        );
        assert_eq!(
            select("NestedEdgedGroup", 3),
            format!("{{ {peer_ref} properties {{ __typename }} }}")
        );
        assert_eq!(select("NestedEdgedGroup", 4), "{ __typename }");
        assert_eq!(
            select("PaginatedGroup", 0),
            format!("{{ count edges {{ {peer} node_metadata {{ created_at }} }} }}")
        );
        assert_eq!(
            select("GroupConnection", 0),
            format!("{{ edges {{ {peer} }} count }}")
        );
        assert_eq!(select("TextAttribute", 0), "{ value }");
        assert_eq!(select("GroupUpdate", 0), "{ ok }");

        let types = render_types(&ctx);
        let struct_of = |name: &str| {
            let start = types.find(&format!("pub struct {name} {{")).unwrap();
            types[start..start + types[start..].find('}').unwrap() + 1].to_string()
        };
        assert_eq!(
            struct_of("Group"),
            "pub struct Group {\n    \
             #[serde(rename = \"__typename\")]\n    \
             pub typename: Option<String>,\n    \
             pub id: Option<String>,\n    \
             pub hfid: Option<Vec<String>>,\n    \
             pub kind: Option<Kind>,\n    \
             pub managed: Option<bool>,\n}"
        );
        let field_of = |name: &str, field: &str| {
            let body = struct_of(name);
            let line = body.lines().find(|l| l.contains(&format!(" {field}:")));
            line.unwrap().trim().to_string()
        };
        for (edge, ty) in [
            ("NestedEdgedGroup", "Option<Box<PeerRef>>"),
            ("EdgedGroup", "Option<Box<Group>>"),
            ("PinnedGroupEdge", "Box<Group>"),
            ("GroupListEdge", "Option<Vec<Group>>"),
            ("GroupNodes", "Option<Box<Group>>"),
            ("ThingEdge", "Option<Thing>"),
        ] {
            assert_eq!(field_of(edge, "node"), format!("pub node: {ty},"), "{edge}");
        }
        for (name, field) in [
            ("NestedEdgedGroup", "created_by"),
            ("TextAttribute", "source"),
            ("GroupUpdate", "object"),
        ] {
            assert_eq!(
                field_of(name, field),
                format!("pub {field}: Option<serde_json::Value>,"),
                "{name}"
            );
        }

        let models = collect_models(&ctx);
        assert!(models["Group"].update.is_none(), "interface `object`");
        let modules = render_api_modules(&ctx);
        let group_rs = &modules["group"];
        assert!(group_rs.contains("-> Result<Vec<Group>> {"));
        assert!(group_rs.contains("-> Result<Option<Group>> {"));
        assert!(group_rs.contains("items.push(*node);"));
        assert!(!group_rs.contains("pub async fn update("));
        let widget_rs = &modules["widget"];
        assert!(widget_rs.contains(&format!(
            "parent {{ {peer_ref} properties {{ __typename }} }} \
             member_of_groups {{ count edges {{ __typename }} }}"
        )));
        assert!(widget_rs.contains(&format!(
            "parent {{ {peer_ref} properties {{ is_protected }} }} \
             member_of_groups {{ count edges {{ {peer_ref} properties {{ __typename }} }} }}"
        )));
    }

    fn struct_field(types: &str, name: &str, field: &str) -> String {
        let start = types.find(&format!("pub struct {name} {{")).unwrap();
        let body = &types[start..start + types[start..].find('}').unwrap()];
        let line = body.lines().find(|l| l.contains(&format!(" {field}:")));
        line.unwrap().trim().to_string()
    }

    #[test]
    fn test_root_interface_fields_select_and_type_the_peer_identity() {
        let schema = r#"
            type Query { Me: Account Accounts(limit: Int): [Account!]! }
            type Mutation { Touch(id: String!): Account! }
            interface Account {
                id: String!
                name: String
                groups(limit: Int!): String
                profile: Profile
            }
            type Profile { theme: String }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let client = render_client(&ctx);
        for query in [
            "query Me { Me { __typename id name } }",
            "query Accounts($limit: Int) { Accounts(limit: $limit) { __typename id name } }",
            "mutation Touch($id: String!) { Touch(id: $id) { __typename id name } }",
        ] {
            assert!(client.contains(&format!("r#\"{query}\"#")), "{query}");
        }

        let responses = render_responses(&ctx);
        assert_eq!(
            struct_field(&responses, "MeResponse", "me"),
            "pub me: Option<Box<Account>>,"
        );
        assert_eq!(
            struct_field(&responses, "AccountsResponse", "accounts"),
            "pub accounts: Vec<Account>,"
        );
        assert_eq!(
            struct_field(&responses, "TouchResponse", "touch"),
            "pub touch: Box<Account>,"
        );
    }

    #[test]
    fn test_objects_without_selectable_fields_select_their_typename() {
        let schema = r#"
            type Query { Bare: Bare Holder: Holder! Version: String }
            type Bare {
                size(unit: String!): Int!
                label(lang: String!): String
                legacy: String @deprecated(reason: "gone")
            }
            type Holder { bare: Bare! nested: Nested count: Int! }
            type Nested { bare: Bare! }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let select = |name: &str| selection_for_type(name, &ctx, &mut BTreeSet::new(), 0);
        assert_eq!(select("Bare"), "{ __typename }");
        assert_eq!(select("Nested"), "{ bare { __typename } }");
        assert_eq!(
            select("Holder"),
            "{ bare { __typename } nested { bare { __typename } } count }"
        );
        let client = render_client(&ctx);
        assert!(client.contains("r#\"query Bare { Bare { __typename } }\"#"));
        assert!(client.contains("r#\"query Version { Version }\"#"));

        let types = render_types(&ctx);
        assert_eq!(
            struct_field(&types, "Bare", "size"),
            "pub size: Option<i64>,"
        );
        assert_eq!(
            struct_field(&types, "Bare", "label"),
            "pub label: Option<String>,"
        );
        assert_eq!(
            struct_field(&types, "Holder", "bare"),
            "pub bare: Box<Bare>,"
        );
    }

    fn assert_selections_match_the_schema(
        ctx: &SchemaContext,
        parent: &str,
        set: &graphql_parser::query::SelectionSet<String>,
        path: &str,
    ) {
        use graphql_parser::query::Selection;
        for item in &set.items {
            let Selection::Field(field) = item else {
                panic!("{path}: unexpected fragment");
            };
            let path = format!("{path}.{}", field.name);
            let selected = !field.selection_set.items.is_empty();
            if field.name == "__typename" {
                assert!(!selected, "{path}");
                continue;
            }
            let fields = match ctx.types.get(parent) {
                Some(TypeDefinition::Object(obj)) => &obj.fields,
                Some(TypeDefinition::Interface(iface)) => &iface.fields,
                _ => panic!("{path}: {parent} has no fields"),
            };
            let Some(def) = fields.iter().find(|f| f.name == field.name) else {
                panic!("{path}: no such field on {parent}");
            };
            let base = base_type_name(&def.field_type);
            let composite = ctx.objects.contains(&base)
                || ctx.interfaces.contains(&base)
                || ctx.unions.contains(&base);
            assert_eq!(selected, composite, "{path}: {base}");
            assert_selections_match_the_schema(ctx, &base, &field.selection_set, &path);
        }
    }

    /// asserts that every struct a document's selection lands in gets each of its non-`Option` fields.
    fn assert_selections_fill_their_structs(
        structs: &BTreeMap<String, Vec<(String, String)>>,
        name: &str,
        set: &graphql_parser::query::SelectionSet<String>,
        path: &str,
    ) {
        use graphql_parser::query::Selection;
        for (wire, ty) in structs.get(name).into_iter().flatten() {
            let selected = set.items.iter().find_map(|item| match item {
                Selection::Field(field) if field.name == *wire => Some(&field.selection_set),
                _ => None,
            });
            let Some(selected) = selected else {
                assert!(
                    ty.starts_with("Option<"),
                    "{path}: {name}.{wire} is required"
                );
                continue;
            };
            let inner = ty.split(['<', '>']).rfind(|s| !s.is_empty()).unwrap();
            let path = format!("{path}.{wire}");
            assert!(
                selected.items.is_empty() || structs.contains_key(inner),
                "{path}: no struct {inner} for a selection"
            );
            assert_selections_fill_their_structs(structs, inner, selected, &path);
        }
    }

    /// the wire name and type of each field of each struct in generated `source`.
    fn struct_fields(source: &str) -> BTreeMap<String, Vec<(String, String)>> {
        let (mut structs, mut current, mut rename) = (BTreeMap::new(), None, None);
        for line in source.lines().map(str::trim) {
            let field = line.strip_prefix("pub ").and_then(|l| l.split_once(": "));
            if let Some(name) = line
                .strip_prefix("pub struct ")
                .and_then(|l| l.strip_suffix("(pub serde_json::Value);"))
            {
                structs.insert(name.to_string(), Vec::new());
            } else if let Some(name) = line
                .strip_prefix("pub struct ")
                .and_then(|l| l.strip_suffix(" {"))
            {
                current = Some((name.to_string(), Vec::new()));
            } else if let (Some((_, fields)), Some((name, ty))) = (current.as_mut(), field) {
                let wire = rename.unwrap_or(name).to_string();
                fields.push((wire, ty.trim_end_matches(',').to_string()));
            } else if line == "}" {
                structs.extend(current.take());
            }
            rename = line
                .strip_prefix("#[serde(rename = \"")
                .and_then(|l| l.strip_suffix("\")]"));
        }
        structs
    }

    fn for_each_operation(
        ctx: &SchemaContext,
        mut check: impl FnMut(&str, &str, &graphql_parser::query::SelectionSet<String>),
    ) {
        use graphql_parser::query::{parse_query, Definition, OperationDefinition};
        let mut sources = vec![render_client(ctx)];
        sources.extend(render_api_modules(ctx).into_values());
        for source in &sources {
            for literal in source.split("r#\"").skip(1) {
                let text = &literal[..literal.find("\"#").unwrap()];
                let query = parse_query::<String>(text).unwrap_or_else(|e| panic!("{e}: {text}"));
                for def in &query.definitions {
                    let (root, name, set) = match def {
                        Definition::Operation(OperationDefinition::Query(q)) => {
                            (&ctx.query_type, &q.name, &q.selection_set)
                        }
                        Definition::Operation(OperationDefinition::Mutation(m)) => (
                            ctx.mutation_type.as_ref().unwrap(),
                            &m.name,
                            &m.selection_set,
                        ),
                        _ => panic!("unexpected definition in {text}"),
                    };
                    check(root, name.as_deref().unwrap(), set);
                }
            }
        }
    }

    #[test]
    fn test_every_generated_document_selects_exactly_its_composite_fields() {
        let doc = parse_schema::<String>(include_str!("../../../schema/infrahub.graphql")).unwrap();
        let ctx = SchemaContext::new(&doc);
        let mut operations = BTreeSet::new();
        for_each_operation(&ctx, |root, name, set| {
            assert_selections_match_the_schema(&ctx, root, set, name);
            operations.insert(name.to_string());
        });
        for name in [
            "AccountProfile",
            "InfrahubTask",
            "InfrahubTaskBranchStatus",
            "InfrahubEvent",
            "CoreAccount",
            "CoreAccountCreate",
        ] {
            assert!(operations.contains(name), "{name}");
        }
    }

    #[test]
    fn test_object_peer_edges_select_and_type_the_peer_identity() {
        let schema = r#"
            type Query { InfraArtifact(ids: [ID]): PaginatedInfraArtifact! }
            type Mutation { InfraArtifactCreate(data: String!): InfraArtifactCreate }
            type Props { is_protected: Boolean }
            type InfraArtifact {
                id: String!
                definition: NestedEdgedInfraDefinition!
                group: NestedEdgedInfraGroup!
                tags(offset: Int): NestedPaginatedInfraTag!
            }
            type InfraDefinition {
                id: String!
                hfid: [String!]
                display_label: String
                artifacts(offset: Int): NestedPaginatedInfraArtifact!
            }
            interface InfraGroup { id: String! hfid: [String!] display_label: String }
            type InfraTag { id: String! }
            type NestedEdgedInfraDefinition { node: InfraDefinition properties: Props }
            type NestedEdgedInfraGroup { node: InfraGroup properties: Props }
            type NestedEdgedInfraArtifact { node: InfraArtifact properties: Props }
            type NestedPaginatedInfraArtifact { count: Int! edges: [NestedEdgedInfraArtifact!]! }
            type NestedEdgedInfraTag { node: InfraTag properties: Props }
            type NestedPaginatedInfraTag {
                count: Int!
                edges: [NestedEdgedInfraTag!]!
                permissions: Perms!
            }
            type Perms { count: Int! }
            type EdgedInfraArtifact { node: InfraArtifact }
            type PaginatedInfraArtifact { count: Int! edges: [EdgedInfraArtifact!]! }
            type InfraArtifactCreate { ok: Boolean object: InfraArtifact }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let infra_rs = &render_api_modules(&ctx)["infra"];
        for selection in [
            "{ count edges { node { id \
             definition { node { __typename id } properties { __typename } } \
             group { node { __typename id } properties { __typename } } \
             tags { count edges { __typename } } } } }",
            "{ ok object { id \
             definition { node { __typename id } properties { is_protected } } \
             group { node { __typename id } properties { is_protected } } \
             tags { count edges { node { __typename id } properties { __typename } } } } }",
        ] {
            assert!(infra_rs.contains(selection), "{selection}");
        }

        let types = render_types(&ctx);
        for (edge, ty) in [
            ("NestedEdgedInfraDefinition", "Option<Box<PeerRef>>"),
            ("NestedEdgedInfraGroup", "Option<Box<PeerRef>>"),
            ("EdgedInfraArtifact", "Option<Box<InfraArtifact>>"),
        ] {
            assert_eq!(
                struct_field(&types, edge, "node"),
                format!("pub node: {ty},")
            );
        }
        assert_eq!(
            struct_field(&types, "NestedPaginatedInfraTag", "permissions"),
            "pub permissions: Option<Box<Perms>>,"
        );
        assert_eq!(
            struct_fields(&types)["PeerRef"],
            [("__typename", "Option<String>"), ("id", "Option<String>")]
                .map(|(wire, ty)| (wire.to_string(), ty.to_string()))
        );
        assert_documents_fill_their_structs(&ctx);
    }

    fn assert_documents_fill_their_structs(ctx: &SchemaContext) {
        use graphql_parser::query::Selection;
        let mut structs = struct_fields(&render_types(ctx));
        structs.extend(struct_fields(&render_responses(ctx)));
        for_each_operation(ctx, |root, name, set| {
            assert_eq!(set.items.len(), 1, "{name}: one root field per document");
            let Selection::Field(field) = &set.items[0] else {
                panic!("{name}: unexpected fragment");
            };
            let is_mutation = ctx.mutation_type.as_deref() == Some(root);
            let response = ctx.response_ident(is_mutation, &field.name);
            assert_selections_fill_their_structs(&structs, response, set, name);
        });
    }

    #[test]
    fn test_every_generated_document_fills_its_structs() {
        let doc = parse_schema::<String>(include_str!("../../../schema/infrahub.graphql")).unwrap();
        assert_documents_fill_their_structs(&SchemaContext::new(&doc));
    }

    #[test]
    fn test_relationship_edges_are_the_reachable_edges_with_properties() {
        let doc = parse_schema::<String>(include_str!("../../../schema/infrahub.graphql")).unwrap();
        let ctx = SchemaContext::new(&doc);
        let mut reachable = BTreeSet::new();
        let mut pending = vec![ctx.query_type.clone()];
        pending.extend(ctx.mutation_type.clone());
        while let Some(name) = pending.pop() {
            if reachable.insert(name.clone()) {
                pending.extend(
                    fields_of(&name, &ctx)
                        .iter()
                        .map(|field| base_type_name(&field.field_type)),
                );
            }
        }
        let with_properties: BTreeSet<String> = reachable
            .into_iter()
            .filter(|name| edge_node(name, &ctx).is_some())
            .filter(|name| fields_of(name, &ctx).iter().any(|f| f.name == "properties"))
            .collect();
        assert_eq!(ctx.relationship_edges, with_properties);
    }

    #[test]
    fn test_fields_past_the_depth_cap_select_what_their_struct_requires() {
        let schema = r#"
            type Query { A: A D: D }
            type A { b: B }
            type B { c: C }
            type C { d: D }
            type D { full: Full! loose: Loose cut: Full }
            type Full { count: Int! inner: Inner! note: String }
            type Inner { size: Int! }
            type Loose { count: Int }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_eq!(
            selection_for_type("A", &ctx, &mut BTreeSet::new(), 0),
            "{ b { c { d { full { count inner { size } } loose { __typename } } } } }"
        );
        assert_eq!(
            selection_for_type("D", &ctx, &mut BTreeSet::new(), 0),
            "{ full { count inner { size } note } loose { count } cut { count inner { size } note } }"
        );
        let types = render_types(&ctx);
        assert_eq!(struct_field(&types, "D", "full"), "pub full: Box<Full>,");
        assert_eq!(
            struct_field(&types, "D", "cut"),
            "pub cut: Option<Box<Full>>,"
        );
        assert!(!types.contains("PeerRef"));
        assert_documents_fill_their_structs(&ctx);
    }

    #[test]
    fn test_an_edge_the_root_also_returns_fills_its_node_under_a_model() {
        let schema = r#"
            type Query { Thing(ids: [ID]): PaginatedThing! }
            type Mutation { ThingCreate(data: String!): ThingCreate }
            type ThingCreate { ok: Boolean object: Thing }
            type PaginatedThing { count: Int! edges: [EdgedThing!]! }
            type EdgedThing { node: Thing }
            type Thing { id: ID! name: String! shared: EdgedThing }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_eq!(
            selection_for_type("ThingCreate", &ctx, &mut BTreeSet::new(), 0),
            "{ ok object { id name shared { node { id name } } } }"
        );
        assert_eq!(
            struct_field(&render_types(&ctx), "EdgedThing", "node"),
            "pub node: Option<Box<Thing>>,"
        );
        assert_documents_fill_their_structs(&ctx);
    }

    #[test]
    fn test_a_required_cycle_past_the_depth_cap_fills_its_structs() {
        let schema = r#"
            type Query { A: A X: X }
            type A { b: B }
            type B { c: C }
            type C { d: D }
            type D { x: X! }
            type X { y: Y! name: String! }
            type Y { x: X! size: Int! }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_eq!(
            selection_for_type("A", &ctx, &mut BTreeSet::new(), 0),
            "{ b { c { d { x { name } } } } }"
        );
        let types = render_types(&ctx);
        assert_eq!(struct_field(&types, "X", "y"), "pub y: Option<Box<Y>>,");
        assert_eq!(struct_field(&types, "Y", "x"), "pub x: Option<Box<X>>,");
        assert_eq!(struct_field(&types, "X", "name"), "pub name: String,");
        assert_documents_fill_their_structs(&ctx);
    }

    #[test]
    fn test_a_wrapper_the_root_also_returns_leaves_out_its_permissions_under_a_model() {
        let schema = r#"
            type Query { Thing(ids: [ID]): PaginatedThing! }
            type Mutation { ThingCreate(data: String!): ThingCreate }
            type ThingCreate { ok: Boolean object: Thing }
            type PaginatedThing { count: Int! edges: [EdgedThing!]! permissions: Perms! }
            type Perms { count: Int! }
            type EdgedThing { node: Thing }
            type Thing { id: ID! name: String! others(offset: Int): PaginatedThing! }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_eq!(
            selection_for_type("ThingCreate", &ctx, &mut BTreeSet::new(), 0),
            "{ ok object { id name others { count edges { __typename } } } }"
        );
        assert_eq!(
            struct_field(&render_types(&ctx), "PaginatedThing", "permissions"),
            "pub permissions: Option<Box<Perms>>,"
        );
        assert_documents_fill_their_structs(&ctx);
    }

    #[test]
    fn test_a_model_listing_edges_keeps_its_object_fields() {
        let schema = r#"
            type Query { Thing(ids: [ID]): PaginatedThing! }
            type PaginatedThing { count: Int! edges: [EdgedThing!]! }
            type EdgedThing { node: Thing }
            type Thing { id: ID! name: Text friend: Thing tags: [NestedEdgedTag!]! }
            type Text { value: String }
            type Tag { id: ID! }
            type NestedEdgedTag { node: Tag }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert!(ctx.relationship_wrappers.is_empty());
        let selection = selection_for_type("Thing", &ctx, &mut BTreeSet::new(), 0);
        assert!(selection.contains("name { value }"), "{selection}");
        assert_documents_fill_their_structs(&ctx);
    }

    #[test]
    fn test_cargo_toml_is_written_with_crate_name_or_when_missing() {
        let doc = parse_schema::<String>("type Query { ok: Boolean }").unwrap();
        let ctx = SchemaContext::new(&doc);
        let root = std::env::temp_dir().join(format!("infrahub-codegen-{}", std::process::id()));
        let hand_made = "[package]\nname = \"hand-made\"\npublish = false\n";
        for (dir, crate_name, existing, package) in [
            ("infrahub-generated", None, None, Some("infrahub-generated")),
            ("kept", None, Some(hand_made), Some("hand-made")),
            ("flag", Some("custom"), Some(hand_made), Some("custom")),
            ("gen.d", None, None, None),
        ] {
            let out_dir = root.join(dir);
            fs::create_dir_all(&out_dir).unwrap();
            if let Some(existing) = existing {
                fs::write(out_dir.join("Cargo.toml"), existing).unwrap();
            }
            let args = Args {
                url: None,
                token: None,
                branch: None,
                schema_path: None,
                out_dir: out_dir.clone(),
                crate_name: crate_name.map(String::from),
                infrahub_path: None,
            };
            let result = generate_client(&args, &ctx);
            let manifest = fs::read_to_string(out_dir.join("Cargo.toml")).ok();
            match package {
                Some(package) => {
                    assert_eq!(result, Ok(()), "{dir}");
                    let name_line = format!("\nname = \"{package}\"\n");
                    assert!(manifest.unwrap().contains(&name_line), "{dir}");
                }
                None => assert!(result.is_err() && manifest.is_none(), "{dir}"),
            }
        }
        fs::remove_dir_all(&root).unwrap();
    }
}
