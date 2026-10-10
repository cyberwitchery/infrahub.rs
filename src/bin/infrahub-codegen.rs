//! generate a typed infrahub client from a schema
//!
//! this binary produces a standalone, schema-specific crate with:
//! - typed `types`, `inputs`, and `responses`
//! - a full surface `generated()` client
//! - an ergonomic `api()` layer grouped by schema namespaces
//!
//! command help reference (kept in sync with `infrahub-codegen --help`):
#[doc = concat!("```text\n", include_str!("infrahub-codegen-help.txt"), "\n```")]
pub const CLI_HELP: &str = include_str!("infrahub-codegen-help.txt");

use graphql_parser::schema::{
    parse_schema, Definition, Document, EnumValue, Field, InputObjectType, InputValue, ObjectType,
    Type, TypeDefinition, UnionType,
};
use reqwest::blocking::Client as BlockingClient;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

#[derive(Debug)]
struct Args {
    url: Option<String>,
    token: Option<String>,
    branch: Option<String>,
    schema_path: Option<PathBuf>,
    out_dir: PathBuf,
    crate_name: Option<String>,
    infrahub_path: Option<String>,
}

enum ParseArgsError {
    Help,
    Message(String),
}

fn main() {
    let args = match parse_args(std::env::args().collect()) {
        Ok(args) => args,
        Err(ParseArgsError::Help) => {
            print!("{CLI_HELP}");
            return;
        }
        Err(ParseArgsError::Message(err)) => {
            eprintln!("{err}\n\n{CLI_HELP}");
            std::process::exit(1);
        }
    };

    let schema = match load_schema(&args) {
        Ok(schema) => schema,
        Err(err) => {
            eprintln!("failed to load schema: {err}");
            std::process::exit(1);
        }
    };

    let document = match parse_schema::<String>(&schema) {
        Ok(doc) => doc,
        Err(err) => {
            eprintln!("failed to parse schema: {err}");
            std::process::exit(1);
        }
    };

    let ctx = SchemaContext::new(&document);

    if let Err(err) = generate_client(&args, &ctx) {
        eprintln!("codegen failed: {err}");
        std::process::exit(1);
    }
}

fn parse_args(args: Vec<String>) -> Result<Args, ParseArgsError> {
    let mut url = None;
    let mut token = None;
    let mut branch = None;
    let mut schema_path = None;
    let mut out_dir = None;
    let mut crate_name = None;
    let mut infrahub_path = None;

    let mut iter = args.into_iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--url" => url = iter.next(),
            "--token" => token = iter.next(),
            "--branch" => branch = iter.next(),
            "--schema" => schema_path = iter.next().map(PathBuf::from),
            "--out" => out_dir = iter.next().map(PathBuf::from),
            "--crate-name" => crate_name = iter.next(),
            "--infrahub-path" => infrahub_path = iter.next(),
            "--help" | "-h" => return Err(ParseArgsError::Help),
            _ => return Err(ParseArgsError::Message(format!("unknown argument: {arg}"))),
        }
    }

    let out_dir =
        out_dir.ok_or_else(|| ParseArgsError::Message("--out is required".to_string()))?;

    if url.is_none() && schema_path.is_none() {
        return Err(ParseArgsError::Message(
            "--url or --schema is required".to_string(),
        ));
    }

    Ok(Args {
        url,
        token,
        branch,
        schema_path,
        out_dir,
        crate_name,
        infrahub_path,
    })
}

fn load_schema(args: &Args) -> Result<String, String> {
    if let Some(schema_path) = &args.schema_path {
        return fs::read_to_string(schema_path)
            .map_err(|err| format!("failed to read {}: {err}", schema_path.display()));
    }

    let url = args
        .url
        .as_ref()
        .ok_or_else(|| "--url is required when --schema not provided".to_string())?;

    let base = url.trim_end_matches('/');
    let mut schema_url = Url::parse(&format!("{base}/schema.graphql"))
        .map_err(|err| format!("invalid url: {err}"))?;
    if let Some(branch) = &args.branch {
        schema_url.query_pairs_mut().append_pair("branch", branch);
    }

    let mut headers = HeaderMap::new();
    if let Some(token) = &args.token {
        headers.insert(
            "X-INFRAHUB-KEY",
            HeaderValue::from_str(token).map_err(|err| err.to_string())?,
        );
    }

    let client = BlockingClient::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|err| err.to_string())?;
    let response = client
        .get(schema_url)
        .headers(headers)
        .send()
        .map_err(|err| err.to_string())?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().unwrap_or_default();
        let detail = if body.is_empty() {
            String::new()
        } else {
            format!(": {}", body.chars().take(200).collect::<String>())
        };
        return Err(format!("HTTP {}{}", status, detail));
    }

    response
        .text()
        .map_err(|err| format!("failed to read schema response: {err}"))
}

struct SchemaContext<'a> {
    types: BTreeMap<String, TypeDefinition<'a, String>>,
    query_type: String,
    mutation_type: Option<String>,
    enums: BTreeSet<String>,
    inputs: BTreeSet<String>,
    objects: BTreeSet<String>,
    interfaces: BTreeSet<String>,
    unions: BTreeSet<String>,
    scalars: BTreeSet<String>,
    type_idents: BTreeMap<String, String>,
    helper_idents: BTreeMap<String, String>,
    response_idents: BTreeMap<(bool, String), String>,
}

/// names the generated code uses unqualified beside the schema's types.
const RESERVED_TYPE_NAMES: &[&str] = &[
    "Box",
    "BoxExtract",
    "BoxFetch",
    "BoxFutureResult",
    "Client",
    "Deserialize",
    "DynPaginator",
    "EdgePage",
    "Error",
    "GeneratedClient",
    "GeneratedClientImpl",
    "GraphQlResponse",
    "Into",
    "None",
    "Ok",
    "Option",
    "Result",
    "Serialize",
    "Some",
    "String",
    "Value",
    "Vec",
];

#[derive(Clone, Debug)]
struct ModelInfo<'a> {
    name: String,
    namespace: String,
    query_field: Option<Field<'a, String>>,
    query_return: Option<String>,
    node_type: String,
    node_boxed: bool,
    create: Option<Field<'a, String>>,
    update: Option<Field<'a, String>>,
    upsert: Option<Field<'a, String>>,
    delete: Option<Field<'a, String>>,
}

impl<'a> SchemaContext<'a> {
    fn new(doc: &'a Document<'a, String>) -> Self {
        let mut types = BTreeMap::new();
        let mut enums = BTreeSet::new();
        let mut inputs = BTreeSet::new();
        let mut objects = BTreeSet::new();
        let mut interfaces = BTreeSet::new();
        let mut unions = BTreeSet::new();
        let mut scalars = BTreeSet::new();
        let mut query_type = "Query".to_string();
        let mut mutation_type = None;
        let mut has_schema_definition = false;

        for def in &doc.definitions {
            if let Definition::TypeDefinition(ty) = def {
                let name = match ty {
                    TypeDefinition::Enum(enum_ty) => {
                        enums.insert(enum_ty.name.clone());
                        enum_ty.name.clone()
                    }
                    TypeDefinition::InputObject(input_ty) => {
                        inputs.insert(input_ty.name.clone());
                        input_ty.name.clone()
                    }
                    TypeDefinition::Object(obj) => {
                        objects.insert(obj.name.clone());
                        obj.name.clone()
                    }
                    TypeDefinition::Interface(iface) => {
                        interfaces.insert(iface.name.clone());
                        iface.name.clone()
                    }
                    TypeDefinition::Union(union_ty) => {
                        unions.insert(union_ty.name.clone());
                        union_ty.name.clone()
                    }
                    TypeDefinition::Scalar(scalar_ty) => {
                        scalars.insert(scalar_ty.name.clone());
                        scalar_ty.name.clone()
                    }
                };
                types.insert(name, ty.clone());
            } else if let Definition::SchemaDefinition(schema) = def {
                has_schema_definition = true;
                if let Some(query) = &schema.query {
                    query_type = query.to_string();
                }
                mutation_type = schema.mutation.as_ref().map(|m| m.to_string());
            }
        }

        if !has_schema_definition && objects.contains("Mutation") {
            mutation_type = Some("Mutation".to_string());
        }

        let mut taken = RESERVED_TYPE_NAMES
            .iter()
            .map(|name| name.to_string())
            .collect();
        let type_idents = types
            .keys()
            .map(|name| (name.clone(), claim(&mut taken, name)))
            .collect();
        let mut ctx = Self {
            types,
            query_type,
            mutation_type,
            enums,
            inputs,
            objects,
            interfaces,
            unions,
            scalars,
            type_idents,
            helper_idents: BTreeMap::new(),
            response_idents: BTreeMap::new(),
        };
        ctx.claim_helper_idents(taken);
        ctx
    }

    /// names the generated structs from what the schema's types left in `taken`.
    fn claim_helper_idents(&mut self, mut taken: BTreeSet<String>) {
        let models = collect_models(self);
        let namespaces: BTreeSet<String> = models
            .values()
            .map(|model| to_snake(&model.namespace))
            .collect();
        let mut helpers: Vec<String> = namespaces
            .iter()
            .map(|ns| format!("{}Api", to_rust_ident(ns)))
            .collect();
        for model in models.keys() {
            helpers.push(format!("{model}Client"));
            helpers.push(format!("{model}Filters"));
        }
        let mut root_fields = Vec::new();
        for (is_mutation, root) in [
            (false, Some(&self.query_type)),
            (true, self.mutation_type.as_ref()),
        ] {
            if let Some(TypeDefinition::Object(root)) = root.and_then(|root| self.types.get(root)) {
                root_fields.extend(
                    root.fields
                        .iter()
                        .map(|field| (is_mutation, field.name.clone())),
                );
            }
        }
        for name in helpers {
            let ident = claim(&mut taken, &name);
            self.helper_idents.insert(name, ident);
        }
        root_fields.sort_by_key(|(_, field)| to_snake(field) != *field);
        for (is_mutation, field) in root_fields {
            let ident = claim(&mut taken, &format!("{}Response", to_rust_ident(&field)));
            self.response_idents.insert((is_mutation, field), ident);
        }
    }

    /// the rust identifier of a schema type.
    fn type_ident(&self, name: &str) -> &str {
        &self.type_idents[name]
    }

    /// the identifier of a generated struct, by the name it takes when that is free.
    fn helper_ident(&self, name: &str) -> &str {
        &self.helper_idents[name]
    }

    /// the identifier of a root field's response struct.
    fn response_ident(&self, is_mutation: bool, field: &str) -> &str {
        &self.response_idents[&(is_mutation, field.to_string())]
    }
}

fn generate_client(args: &Args, ctx: &SchemaContext) -> Result<(), String> {
    let out_dir = &args.out_dir;
    let src_dir = out_dir.join("src");
    let api_dir = src_dir.join("api");
    fs::create_dir_all(&src_dir).map_err(|err| err.to_string())?;
    fs::create_dir_all(&api_dir).map_err(|err| err.to_string())?;

    if let Some(crate_name) = &args.crate_name {
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
        fs::write(out_dir.join("Cargo.toml"), cargo).map_err(|err| err.to_string())?;
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

fn render_types(ctx: &SchemaContext) -> String {
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

    for iface_name in &ctx.interfaces {
        if let Some(TypeDefinition::Interface(iface)) = ctx.types.get(iface_name) {
            out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", ctx.type_ident(iface_name)));
            out.push_str("    #[serde(rename = \"__typename\")]\n");
            out.push_str("    pub typename: Option<String>,\n");
            let fields: Vec<_> = iface
                .fields
                .iter()
                .filter(|field| identifies_peer(field, ctx))
                .collect();
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

fn render_inputs(ctx: &SchemaContext) -> String {
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

fn render_responses(ctx: &SchemaContext) -> String {
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
            let ty = rust_type(&field.field_type, ctx, false);
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
                let ty = rust_type(&field.field_type, ctx, false);
                push_struct_field(&mut out, field.name.as_str(), &ty);
                out.push_str("}\n\n");
            }
        }
    }

    out
}

fn render_client(ctx: &SchemaContext) -> String {
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

fn render_api_mod<'a>(ctx: &SchemaContext<'a>) -> String {
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
fn render_api_modules<'a>(ctx: &SchemaContext<'a>) -> BTreeMap<String, String> {
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

fn collect_models<'a>(ctx: &SchemaContext<'a>) -> BTreeMap<String, ModelInfo<'a>> {
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

fn namespace_from_type(name: &str) -> String {
    let words = split_identifier_words(name);
    if words.is_empty() {
        return name.to_string();
    }
    words[0].clone()
}

fn model_accessor_name(model: &str, namespace: &str) -> String {
    let name = model.strip_prefix(namespace).unwrap_or(model);
    let name = if name.is_empty() { model } else { name };
    to_snake(name)
}

fn to_snake(name: &str) -> String {
    split_identifier_words(name)
        .into_iter()
        .map(|w| w.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

fn split_identifier_words(name: &str) -> Vec<String> {
    let chars: Vec<char> = name.chars().collect();
    if chars.is_empty() {
        return Vec::new();
    }

    let mut words = Vec::new();
    let mut start = 0usize;

    for i in 1..chars.len() {
        let prev = chars[i - 1];
        let curr = chars[i];

        if curr == '_' || curr == '-' {
            if start < i {
                words.push(chars[start..i].iter().collect::<String>());
            }
            start = i + 1;
            continue;
        }

        let next = chars.get(i + 1).copied();
        let lower_to_upper = prev.is_ascii_lowercase() && curr.is_ascii_uppercase();
        let acronym_to_word = prev.is_ascii_uppercase()
            && curr.is_ascii_uppercase()
            && next.map(|c| c.is_ascii_lowercase()).unwrap_or(false);
        let digit_to_alpha = prev.is_ascii_digit() && curr.is_ascii_alphabetic();

        if lower_to_upper || acronym_to_word || digit_to_alpha {
            words.push(chars[start..i].iter().collect::<String>());
            start = i;
        }
    }

    if start < chars.len() {
        words.push(chars[start..].iter().collect::<String>());
    }

    words.retain(|w| !w.is_empty());
    words
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

fn object_type_for_return<'a>(return_type: &str, ctx: &SchemaContext<'a>) -> (String, bool) {
    if let Some(TypeDefinition::Object(obj)) = ctx.types.get(return_type) {
        if let Some(field) = obj.fields.iter().find(|f| f.name == "object") {
            let rust = rust_type(&field.field_type, ctx, false);
            return strip_option_box(&rust);
        }
    }
    ("serde_json::Value".to_string(), false)
}

fn strip_option_box(ty: &str) -> (String, bool) {
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

fn selection_for_field(field: &Field<String>, ctx: &SchemaContext) -> String {
    let base = base_type_name(&field.field_type);
    if is_scalar_type(&base) || ctx.enums.contains(&base) || ctx.scalars.contains(&base) {
        return String::new();
    }

    let mut stack = BTreeSet::new();
    let selection = selection_for_type(&base, ctx, &mut stack, 0);
    if selection.is_empty() {
        String::new()
    } else {
        format!(" {}", selection)
    }
}

fn selection_for_type(
    type_name: &str,
    ctx: &SchemaContext,
    stack: &mut BTreeSet<String>,
    depth: usize,
) -> String {
    if depth > 3 {
        return "{ __typename }".to_string();
    }

    if stack.contains(type_name) {
        if let Some(TypeDefinition::Object(obj)) = ctx.types.get(type_name) {
            if obj.fields.iter().any(|f| f.name == "id") {
                return "{ id }".to_string();
            }
        }
        return "{ __typename }".to_string();
    }

    stack.insert(type_name.to_string());

    let mut fields = Vec::new();
    if let Some(TypeDefinition::Object(obj)) = ctx.types.get(type_name) {
        for field in &obj.fields {
            if !is_selected(obj, field, ctx) {
                continue;
            }
            let field_base = base_type_name(&field.field_type);
            if is_scalar_type(&field_base)
                || ctx.enums.contains(&field_base)
                || ctx.scalars.contains(&field_base)
            {
                fields.push(field.name.clone());
                continue;
            }

            if ctx.objects.contains(&field_base) {
                let nested = selection_for_type(&field_base, ctx, stack, depth + 1);
                fields.push(format!("{} {}", field.name, nested));
                continue;
            }

            if ctx.interfaces.contains(&field_base) {
                let peer = peer_selection(&field_base, ctx);
                fields.push(format!("{} {}", field.name, peer));
                continue;
            }

            if ctx.unions.contains(&field_base) {
                fields.push(format!("{} {{ __typename }}", field.name));
                continue;
            }
        }
    }

    stack.remove(type_name);

    if fields.is_empty() {
        String::new()
    } else {
        format!("{{ {} }}", fields.join(" "))
    }
}

/// whether `selection_for_type` selects this field of `obj`.
fn is_selected(obj: &ObjectType<String>, field: &Field<String>, ctx: &SchemaContext) -> bool {
    if has_required_args(field) || should_skip_field(field) || nulled_on_empty_edge(obj, field) {
        return false;
    }
    let base = base_type_name(&field.field_type);
    is_scalar_type(&base)
        || ctx.enums.contains(&base)
        || ctx.scalars.contains(&base)
        || ctx.objects.contains(&base)
        || ctx.unions.contains(&base)
        || is_interface_peer(obj, field, ctx)
}

/// an edge's `node` whose peer is an interface, which is selected and typed by its identity fields.
fn is_interface_peer(obj: &ObjectType<String>, field: &Field<String>, ctx: &SchemaContext) -> bool {
    field.name == "node"
        && ctx.interfaces.contains(&base_type_name(&field.field_type))
        && obj.fields.iter().any(|f| f.name == "node_metadata")
}

/// whether a field of an interface goes into the struct and selection of its peers.
fn identifies_peer(field: &Field<String>, ctx: &SchemaContext) -> bool {
    let base = base_type_name(&field.field_type);
    !has_required_args(field)
        && !should_skip_field(field)
        && (is_scalar_type(&base) || ctx.enums.contains(&base) || ctx.scalars.contains(&base))
}

fn peer_selection(iface_name: &str, ctx: &SchemaContext) -> String {
    let mut fields = vec!["__typename".to_string()];
    if let Some(TypeDefinition::Interface(iface)) = ctx.types.get(iface_name) {
        for field in &iface.fields {
            if identifies_peer(field, ctx) {
                fields.push(field.name.clone());
            }
        }
    }
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

fn base_type_name(ty: &Type<String>) -> String {
    match ty {
        Type::NamedType(name) => name.clone(),
        Type::NonNullType(inner) => base_type_name(inner),
        Type::ListType(inner) => base_type_name(inner),
    }
}

fn has_required_args(field: &Field<String>) -> bool {
    field
        .arguments
        .iter()
        .any(|arg| matches!(arg.value_type, Type::NonNullType(_)) && arg.default_value.is_none())
}

fn is_optional(ty: &Type<String>) -> bool {
    !matches!(ty, Type::NonNullType(_))
}

fn is_field_deprecated(field: &Field<String>) -> bool {
    field.directives.iter().any(|d| d.name == "deprecated")
}

fn is_enum_value_deprecated(value: &EnumValue<String>) -> bool {
    value.directives.iter().any(|d| d.name == "deprecated")
}

fn should_skip_field(field: &Field<String>) -> bool {
    is_field_deprecated(field)
}

fn is_scalar_type(name: &str) -> bool {
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
fn field_rust_type(obj: &ObjectType<String>, field: &Field<String>, ctx: &SchemaContext) -> String {
    let ty = if is_interface_peer(obj, field, ctx) {
        let peer = peer_rust_type_nonnull(&field.field_type, ctx, false);
        if is_optional(&field.field_type) {
            format!("Option<{peer}>")
        } else {
            peer
        }
    } else {
        rust_type(&field.field_type, ctx, false)
    };
    if nulled_on_empty_edge(obj, field) {
        format!("Option<{ty}>")
    } else {
        ty
    }
}

/// `rust_type_nonnull` for an interface peer, which is boxed like an object.
fn peer_rust_type_nonnull(ty: &Type<String>, ctx: &SchemaContext, in_list: bool) -> String {
    match ty {
        Type::ListType(inner) => format!("Vec<{}>", peer_rust_type_nonnull(inner, ctx, true)),
        Type::NonNullType(inner) => peer_rust_type_nonnull(inner, ctx, in_list),
        Type::NamedType(name) if in_list => ctx.type_ident(name).to_string(),
        Type::NamedType(name) => format!("Box<{}>", ctx.type_ident(name)),
    }
}

fn rust_type(ty: &Type<String>, ctx: &SchemaContext, input: bool) -> String {
    match ty {
        Type::NonNullType(inner) => rust_type_nonnull(inner, ctx, input, false),
        _ => format!("Option<{}>", rust_type_nonnull(ty, ctx, input, false)),
    }
}

fn rust_type_nonnull(ty: &Type<String>, ctx: &SchemaContext, input: bool, in_list: bool) -> String {
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
                    || ctx.scalars.contains(name)
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

fn format_gql_type(ty: &Type<String>) -> String {
    match ty {
        Type::NamedType(name) => name.clone(),
        Type::NonNullType(inner) => format!("{}!", format_gql_type(inner)),
        Type::ListType(inner) => format!("[{}]", format_gql_type(inner)),
    }
}

fn to_rust_ident(name: &str) -> String {
    let words = split_identifier_words(name);
    let all_upper = words
        .iter()
        .all(|w| w.chars().all(|c| !c.is_ascii_lowercase()));
    let out: String = words
        .into_iter()
        .map(|w| {
            if !all_upper && w.chars().all(|c| !c.is_ascii_lowercase()) {
                return w;
            }
            let mut chars = w.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    let lower: String = chars.map(|c| c.to_ascii_lowercase()).collect();
                    format!("{}{}", upper, lower)
                }
            }
        })
        .collect();
    match out.as_str() {
        "Self" | "Type" | "Box" | "Result" => format!("{}Type", out),
        _ => out,
    }
}

fn to_rust_field(name: &str) -> String {
    escape_keyword(&to_snake(name))
}

fn escape_keyword(snake: &str) -> String {
    match snake {
        "_" | "crate" | "self" | "super" => format!("{}_", snake),
        _ if is_rust_keyword(snake) => format!("r#{}", snake),
        _ => snake.to_string(),
    }
}

/// `to_rust_field` for every name in one scope, in order, without duplicates or `reserved` names.
/// a name already in snake case keeps its identifier; the others append `_` until free.
fn scope_names<'n>(names: impl IntoIterator<Item = &'n str>, reserved: &[&str]) -> Vec<String> {
    scope_idents(names, reserved, to_snake, escape_keyword)
}

/// `scope_names` with another case conversion and keyword escape.
fn scope_idents<'n>(
    names: impl IntoIterator<Item = &'n str>,
    reserved: &[&str],
    convert: fn(&str) -> String,
    escape: fn(&str) -> String,
) -> Vec<String> {
    let names: Vec<&str> = names.into_iter().collect();
    let mut taken: BTreeSet<String> = reserved.iter().map(|name| name.to_string()).collect();
    let mut out = vec![String::new(); names.len()];
    let (converted, rest): (Vec<usize>, Vec<usize>) =
        (0..names.len()).partition(|&i| convert(names[i]) == names[i]);
    for i in converted.into_iter().chain(rest) {
        let mut base = convert(names[i]);
        out[i] = loop {
            let ident = escape(&base);
            if taken.insert(ident.clone()) {
                break ident;
            }
            base.push('_');
        };
    }
    out
}

/// the first of `name`, `name_`, `name__`, … that is neither a keyword nor in `taken`, added to `taken`.
fn claim(taken: &mut BTreeSet<String>, name: &str) -> String {
    let mut ident = name.to_string();
    while is_rust_keyword(&ident) || !taken.insert(ident.clone()) {
        ident.push('_');
    }
    ident
}

/// `scope_names` for a generated method's arguments, beside its own `request_branch` and `vars`.
fn method_arg_names(args: &[InputValue<String>]) -> Vec<String> {
    scope_names(
        args.iter().map(|arg| arg.name.as_str()),
        &["request_branch", "vars"],
    )
}

struct MethodArgs {
    signature: String,
}

fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "gen"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

#[cfg(test)]
mod scope_name_tests {
    use super::*;
    use graphql_parser::schema::parse_schema;

    const COLLIDING: &str = include_str!("../../tests/fixtures/colliding_names.graphql");

    #[test]
    fn test_scope_names_prefer_names_already_in_snake_case() {
        assert_eq!(
            scope_names(["nodeId", "node_id", "Self", "self", "Type", "type"], &[]),
            ["node_id_", "node_id", "self__", "self_", "type_", "r#type"]
        );
        assert_eq!(scope_names(["_", "__"], &[]), ["__", "___"]);
    }

    #[test]
    fn test_scope_names_step_around_reserved_names() {
        assert_eq!(
            scope_names(
                ["requestBranch", "request_branch", "vars", "query"],
                &["request_branch", "vars"]
            ),
            ["request_branch__", "request_branch_", "vars_", "query"]
        );
    }

    #[test]
    fn test_namespace_modules_are_keyword_safe() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let api_mod = render_api_mod(&ctx);
        for line in [
            "pub mod crate_;",
            "pub mod self_;",
            "pub mod r#type;",
            "    pub fn self_(&self) -> self_::SelfTypeApi<'a> {",
            "    pub fn r#type(&self) -> r#type::TypeTypeApi<'a> {",
            "        r#type::TypeTypeApi::new(self.client)",
        ] {
            assert!(api_mod.contains(line), "missing `{line}` in:\n{api_mod}");
        }
        let stems: Vec<_> = render_api_modules(&ctx).into_keys().collect();
        assert_eq!(
            stems,
            ["crate_", "infra", "ipam", "policy", "self_", "type"]
        );
    }

    #[test]
    fn test_model_accessors_are_keyword_safe_and_distinct() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let modules = render_api_modules(&ctx);
        let accessors = |stem: &str| {
            modules[stem]
                .lines()
                .filter(|line| line.starts_with("    pub fn ") && line.contains("(&self) -> "))
                .map(str::trim)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            accessors("infra"),
            [
                "pub fn infra(&self) -> InfraClient<'a> {",
                "pub fn infra_(&self) -> InfraInfraClient<'a> {",
                "pub fn new_(&self) -> InfraNewClient<'a> {",
                "pub fn r#type(&self) -> InfraTypeClient<'a> {",
            ]
        );
        assert_eq!(
            accessors("ipam"),
            [
                "pub fn prefix(&self) -> IPAMPrefixClient<'a> {",
                "pub fn prefix_(&self) -> IpamPrefixClient<'a> {",
            ]
        );
        assert_eq!(
            accessors("policy"),
            ["pub fn r#match(&self) -> PolicyMatchClient<'a> {"]
        );
    }

    #[test]
    fn test_method_arguments_step_around_request_branch_and_vars() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let client = render_client(&ctx);
        for line in [
            "    pub async fn pong(&self, vars_: String, request_branch_: String , request_branch: Option<&str>)",
            "        vars.insert(\"vars\".to_string(), serde_json::to_value(vars_)?);",
            "        vars.insert(\"requestBranch\".to_string(), serde_json::to_value(request_branch_)?);",
        ] {
            assert!(client.contains(line), "missing `{line}` in:\n{client}");
        }
        let infra = &render_api_modules(&ctx)["infra"];
        assert!(infra.contains(
            "    pub async fn create(&self, data: WidgetInput, request_branch_: Option<String>, vars_: Option<String>, request_branch: Option<&str>)"
        ));
        assert!(infra.contains("        if let Some(value) = vars_ {\n"));
    }

    #[test]
    fn test_root_methods_are_distinct() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let client = render_client(&ctx);
        assert!(client.contains("    pub async fn ipam_prefix(&self, ids: Option<Vec<String>> , request_branch: Option<&str>) -> Result<GraphQlResponse<IPAMPrefixResponse>>"));
        assert!(client.contains("    pub async fn ipam_prefix_(&self, ids: Option<Vec<String>> , request_branch: Option<&str>) -> Result<GraphQlResponse<IpamPrefixResponse>>"));
    }

    #[test]
    fn test_struct_fields_are_distinct_and_keep_their_wire_names() {
        let doc = parse_schema::<String>(COLLIDING).unwrap();
        let ctx = SchemaContext::new(&doc);
        let infra = &render_api_modules(&ctx)["infra"];
        for line in [
            "        if let Some(value) = &self.after_ {\n            vars.insert(\"After\"",
            "        if let Some(value) = &self.after {\n            vars.insert(\"after\"",
            "        if let Some(value) = &self.type_ {\n            vars.insert(\"Type\"",
            "            page_filters.after = cursor.clone();\n",
        ] {
            assert!(infra.contains(line), "missing `{line}` in infra");
        }
        let inputs = render_inputs(&ctx);
        for line in [
            "    #[serde(rename = \"nodeId\")]\n    pub node_id_: Option<String>,",
            "    pub node_id: Option<String>,",
            "    #[serde(rename = \"Self\")]\n    pub self__: Option<String>,",
            "    #[serde(rename = \"self\")]\n    pub self_: Option<String>,",
        ] {
            assert!(inputs.contains(line), "missing `{line}` in:\n{inputs}");
        }
    }
}

#[cfg(test)]
mod type_name_tests {
    use super::*;
    use graphql_parser::schema::parse_schema;

    const COLLIDING_TYPES: &str = include_str!("../../tests/fixtures/colliding_type_names.graphql");

    fn assert_contains(source: &str, lines: &[&str]) {
        for line in lines {
            assert!(source.contains(line), "missing `{line}` in:\n{source}");
        }
    }

    #[test]
    fn test_claim_skips_keywords_and_taken_names() {
        let mut taken = BTreeSet::from(["Error".to_string()]);
        assert_eq!(claim(&mut taken, "Error"), "Error_");
        assert_eq!(claim(&mut taken, "Error"), "Error__");
        assert_eq!(claim(&mut taken, "Self"), "Self_");
        assert_eq!(claim(&mut taken, "type"), "type_");
        assert_eq!(claim(&mut taken, "Device"), "Device");
    }

    #[test]
    fn test_unqualified_names_beside_schema_types_are_reserved() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        let declared: BTreeSet<&str> = ctx
            .type_idents
            .values()
            .chain(ctx.helper_idents.values())
            .chain(ctx.response_idents.values())
            .map(String::as_str)
            .collect();
        let mut sources = vec![
            render_inputs(&ctx),
            render_responses(&ctx),
            render_client(&ctx),
        ];
        sources.extend(render_api_modules(&ctx).into_values());
        for source in sources {
            let mut code = String::new();
            let mut rest = source.as_str();
            while let Some(quote) = rest.find('"') {
                code.push_str(&rest[..quote]);
                let close = if code.ends_with("r#") { "\"#" } else { "\"" };
                rest = &rest[quote + 1..];
                rest = &rest[rest.find(close).unwrap() + close.len()..];
            }
            code.push_str(rest);
            for (i, part) in code.split("::").enumerate() {
                let part = if i == 0 {
                    part
                } else {
                    part.trim_start_matches(|c: char| c.is_ascii_alphanumeric() || c == '_')
                };
                for name in part
                    .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                    .filter(|name| name.starts_with(|c: char| c.is_ascii_uppercase()))
                {
                    assert!(
                        declared.contains(name)
                            || RESERVED_TYPE_NAMES.contains(&name)
                            || ["Clone", "Debug", "Default", "Self"].contains(&name),
                        "`{name}` is neither declared nor reserved"
                    );
                }
            }
        }
    }

    #[test]
    fn test_helper_structs_yield_to_schema_types() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_api_mod(&ctx),
            &[
                "    pub fn infra(&self) -> infra::InfraApi_<'a> {",
                "        infra::InfraApi_::new(self.client)",
            ],
        );
        assert_contains(
            &render_api_modules(&ctx)["infra"],
            &[
                "pub struct InfraApi_<'a> {",
                "impl<'a> InfraApi_<'a> {",
                "    pub fn device(&self) -> InfraDeviceClient_<'a> {\n        InfraDeviceClient_::new(self.client)",
                "pub struct InfraDeviceFilters_ {",
                "impl InfraDeviceFilters_ {",
                "pub struct InfraDeviceClient_<'a> {",
                "impl<'a> InfraDeviceClient_<'a> {",
                "    pub async fn list(&self, filters: Option<InfraDeviceFilters_>, request_branch: Option<&str>) -> Result<Vec<InfraDevice>> {",
                "    pub fn paginate(&self, filters: Option<InfraDeviceFilters_>, ",
                "        let mut filters = InfraDeviceFilters_::default();",
                "request_branch: Option<&str>) -> Result<Vec<InfraApi>> {",
                "request_branch: Option<&str>) -> Result<Vec<InfraDeviceClient>> {",
                "request_branch: Option<&str>) -> Result<Vec<InfraDeviceFilters>> {",
            ],
        );
    }

    #[test]
    fn test_schema_types_step_around_reserved_names_and_keywords() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_types(&ctx),
            &[
                "pub struct Error_ {",
                "pub struct EdgedError {\n    pub node: Option<Box<Error_>>,",
                "pub enum Self_ {",
                "    pub status: Option<Self_>,",
                "pub struct Box_(pub serde_json::Value);",
                "    pub peers: Option<Vec<Box_>>,",
                "pub struct NestedEdgedResult {\n    pub node: Option<Box<Result_>>,",
            ],
        );
        assert_contains(&render_inputs(&ctx), &["pub struct Value_ {"]);
        let modules = render_api_modules(&ctx);
        assert_contains(
            &modules["error"],
            &["request_branch: Option<&str>) -> Result<Vec<Error_>> {"],
        );
        assert_contains(
            &modules["infra"],
            &["    pub async fn create(&self, data: Value_, request_branch: Option<&str>)"],
        );
    }

    #[test]
    fn test_root_fields_get_distinct_response_structs() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_responses(&ctx),
            &[
                "pub struct PingResponse_ {\n    pub ping: Option<Box<PingResponse>>,\n}",
                "pub struct PingResponse__ {\n    pub ping: Option<String>,\n}",
                "pub struct FooBarResponse {\n    pub foo_bar: Option<String>,\n}",
                "pub struct FooBarResponse_ {\n    #[serde(rename = \"fooBar\")]\n    pub foo_bar: Option<String>,\n}",
                "pub struct InfraDeviceResponse {\n    pub infra_device: Option<String>,\n}",
                "pub struct InfraDeviceResponse_ {\n    #[serde(rename = \"InfraDevice\")]\n",
                "pub struct InfraDeviceCreateResponse {\n    #[serde(rename = \"InfraDeviceCreate\")]\n    pub infra_device_create: Option<String>,\n}",
                "pub struct InfraDeviceCreateResponse_ {\n    #[serde(rename = \"InfraDeviceCreate\")]\n    pub infra_device_create: Option<Box<InfraDeviceCreate>>,\n}",
            ],
        );
        let infra = &render_api_modules(&ctx)["infra"];
        assert_contains(
            infra,
            &[
                "        let response = self.client.execute::<InfraDeviceResponse_>(query, Some(vars), request_branch).await?;",
                "DynPaginator<'a, InfraDevice, String, (InfraDeviceResponse_, i64)>",
                "        let response = self.client.execute::<InfraDeviceCreateResponse_>(query, Some(vars), request_branch).await?;",
            ],
        );
        assert_contains(
            &render_client(&ctx),
            &[
                "    pub async fn ping(&self, value: Option<Value_> , request_branch: Option<&str>) -> Result<GraphQlResponse<PingResponse_>> {",
                "    pub async fn ping_(&self , request_branch: Option<&str>) -> Result<GraphQlResponse<PingResponse__>> {",
                "    pub async fn foo_bar(&self , request_branch: Option<&str>) -> Result<GraphQlResponse<FooBarResponse>> {",
                "    pub async fn foo_bar_(&self , request_branch: Option<&str>) -> Result<GraphQlResponse<FooBarResponse_>> {",
            ],
        );
    }

    #[test]
    fn test_object_and_interface_fields_are_distinct() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_types(&ctx),
            &[
                "pub struct InfraDevice {\n    pub id: String,\n    pub typename: Option<String>,\n    #[serde(rename = \"nodeId\")]\n    pub node_id_: Option<String>,\n    pub node_id: Option<String>,\n",
                "pub struct Result_ {\n    #[serde(rename = \"__typename\")]\n    pub typename: Option<String>,\n    pub id: Option<String>,\n    #[serde(rename = \"typename\")]\n    pub typename_: Option<String>,\n    #[serde(rename = \"nodeId\")]\n    pub node_id_: Option<String>,\n    pub node_id: Option<String>,\n}",
            ],
        );
    }

    #[test]
    fn test_enum_variants_are_distinct() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_types(&ctx),
            &["pub enum Self_ {\n    #[serde(rename = \"IN_PROGRESS\")]\n    InProgress_,\n    #[serde(rename = \"InProgress\")]\n    InProgress,\n    #[serde(rename = \"ACTIVE\")]\n    Active,\n    #[serde(rename = \"active\")]\n    Active_,\n}"],
        );
    }

    #[test]
    fn test_namespace_mod_keeps_the_api_root_file() {
        let doc = parse_schema::<String>(COLLIDING_TYPES).unwrap();
        let ctx = SchemaContext::new(&doc);
        assert_contains(
            &render_api_mod(&ctx),
            &[
                "pub mod mod_;",
                "    pub fn mod_(&self) -> mod_::ModApi<'a> {",
            ],
        );
        let stems: Vec<_> = render_api_modules(&ctx).into_keys().collect();
        assert_eq!(stems, ["error", "infra", "mod_"]);
    }
}

#[cfg(test)]
mod codegen_name_tests {
    use super::*;
    use graphql_parser::schema::parse_schema;

    #[test]
    fn test_namespace_from_type_prefers_first_word() {
        assert_eq!(namespace_from_type("CoreRepository"), "Core");
        assert_eq!(namespace_from_type("InfrahubTask"), "Infrahub");
        assert_eq!(namespace_from_type("BuiltinIPAddress"), "Builtin");
        assert_eq!(namespace_from_type("IPAMNamespace"), "IPAM");
    }

    #[test]
    fn test_to_snake_handles_acronyms() {
        assert_eq!(to_snake("IPAM"), "ipam");
        assert_eq!(to_snake("GraphQLQuery"), "graph_ql_query");
        assert_eq!(to_snake("IPAddressPool"), "ip_address_pool");
    }

    #[test]
    fn test_to_snake_handles_digit_to_alpha_boundary() {
        assert_eq!(
            to_snake("CoreTransformJinja2Create"),
            "core_transform_jinja2_create"
        );
        assert_eq!(
            to_snake("CoreTransformJinja2Update"),
            "core_transform_jinja2_update"
        );
        assert_eq!(to_snake("Http2Client"), "http2_client");
        assert_eq!(to_snake("V2Api"), "v2_api");
        assert_eq!(to_snake("route53Zone"), "route53_zone");
    }

    #[test]
    fn test_to_rust_field_handles_acronyms() {
        assert_eq!(to_rust_field("nodeID"), "node_id");
        assert_eq!(to_rust_field("hFID"), "h_fid");
        assert_eq!(to_rust_field("ipAddress"), "ip_address");
        assert_eq!(to_rust_field("nodeUUID"), "node_uuid");
        assert_eq!(to_rust_field("simple"), "simple");
        assert_eq!(to_rust_field("camelCase"), "camel_case");
    }

    #[test]
    fn test_to_rust_field_keyword_escaping() {
        assert_eq!(to_rust_field("type"), "r#type");
        assert_eq!(to_rust_field("yield"), "r#yield");
        assert_eq!(to_rust_field("abstract"), "r#abstract");
        assert_eq!(to_rust_field("try"), "r#try");
        assert_eq!(to_rust_field("gen"), "r#gen");
    }

    #[test]
    fn test_to_rust_field_non_raw_keywords_get_suffix() {
        assert_eq!(to_rust_field("self"), "self_");
        assert_eq!(to_rust_field("Self"), "self_");
        assert_eq!(to_rust_field("super"), "super_");
        assert_eq!(to_rust_field("crate"), "crate_");
    }

    #[test]
    fn test_to_rust_field_leading_underscores() {
        assert_eq!(to_rust_field("_"), "__");
        assert_eq!(to_rust_field("__"), "__");
        assert_eq!(to_rust_field("_1st"), "_1_st");
        assert_eq!(to_rust_field("_updated_at"), "_updated_at");
    }

    #[test]
    fn test_to_rust_ident_all_caps_enum_variants() {
        assert_eq!(to_rust_ident("DENY"), "Deny");
        assert_eq!(to_rust_ident("ALLOW_DEFAULT"), "AllowDefault");
        assert_eq!(to_rust_ident("NEED_UPGRADE_REBASE"), "NeedUpgradeRebase");
        assert_eq!(to_rust_ident("BASE_BRANCH"), "BaseBranch");
        assert_eq!(to_rust_ident("ASC"), "Asc");
    }

    #[test]
    fn test_to_rust_ident_pascal_case_passthrough() {
        assert_eq!(to_rust_ident("CoreMenuItem"), "CoreMenuItem");
        assert_eq!(to_rust_ident("InfrahubTask"), "InfrahubTask");
    }

    #[test]
    fn test_to_rust_ident_preserves_uppercase_clusters() {
        assert_eq!(to_rust_ident("BuiltinIPAddress"), "BuiltinIPAddress");
        assert_eq!(to_rust_ident("CoreGraphQLQuery"), "CoreGraphQLQuery");
        assert_eq!(to_rust_ident("CoreIPAddressPool"), "CoreIPAddressPool");
        assert_eq!(to_rust_ident("JSONAttribute"), "JSONAttribute");
        assert_eq!(to_rust_ident("UpdateHFID"), "UpdateHFID");
    }

    #[test]
    fn test_to_rust_ident_digit_boundaries() {
        assert_eq!(
            to_rust_ident("CoreTransformJinja2Create"),
            "CoreTransformJinja2Create"
        );
        assert_eq!(to_rust_ident("Http2Client"), "Http2Client");
        assert_eq!(to_rust_ident("V2Api"), "V2Api");
    }

    #[test]
    fn test_to_rust_ident_reserved_words() {
        assert_eq!(to_rust_ident("Self"), "SelfType");
        assert_eq!(to_rust_ident("Type"), "TypeType");
        assert_eq!(to_rust_ident("Box"), "BoxType");
        assert_eq!(to_rust_ident("Result"), "ResultType");
    }

    #[test]
    fn test_to_rust_ident_lowercase_input() {
        assert_eq!(to_rust_ident("core"), "Core");
        assert_eq!(to_rust_ident("builtin"), "Builtin");
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
    fn test_fixed_generic_scalar_maps_to_json_value() {
        let schema = r#"
            scalar FixedGenericScalar
            type Query { pool: Pool }
            input PoolInput { data: FixedGenericScalar }
            type Pool { id: String }
        "#;
        let doc = parse_schema::<String>(schema).unwrap();
        let ctx = SchemaContext::new(&doc);
        let inputs_rs = render_inputs(&ctx);
        assert!(
            inputs_rs.contains("serde_json::Value"),
            "FixedGenericScalar should map to serde_json::Value"
        );
        assert!(
            !inputs_rs.contains("FixedGenericScalar"),
            "FixedGenericScalar should not appear as a raw type name"
        );
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
            "{ node { id } node_metadata { created_at } properties { is_protected } }"
        );
        assert_eq!(
            select("BranchEdge"),
            "{ node { id } node_metadata { created_at } }"
        );
        assert_eq!(
            select("WidgetCreate"),
            "{ ok object { id name node_metadata { created_at } \
             parent { cursor node { __typename id } properties { is_protected } } \
             gadget { node { id } node_metadata { created_at } properties { is_protected } } \
             branch { node { id } node_metadata { created_at } } } }"
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
        let select =
            |name: &str, depth| selection_for_type(name, &ctx, &mut BTreeSet::new(), depth);
        assert_eq!(
            select("NestedEdgedGroup", 0),
            format!("{{ {peer} properties {{ is_protected }} }}")
        );
        assert_eq!(
            select("NestedEdgedGroup", 3),
            format!("{{ {peer} properties {{ __typename }} }}")
        );
        assert_eq!(select("NestedEdgedGroup", 4), "{ __typename }");
        assert_eq!(
            select("PaginatedGroup", 0),
            format!("{{ count edges {{ {peer} node_metadata {{ created_at }} }} }}")
        );
        assert_eq!(select("GroupNodes", 0), "");
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
            ("NestedEdgedGroup", "Option<Box<Group>>"),
            ("EdgedGroup", "Option<Box<Group>>"),
            ("PinnedGroupEdge", "Box<Group>"),
            ("GroupListEdge", "Option<Vec<Group>>"),
            ("GroupNodes", "Option<serde_json::Value>"),
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
            "parent {{ {peer} properties {{ __typename }} }} \
             member_of_groups {{ count edges {{ __typename }} }}"
        )));
        assert!(widget_rs.contains(&format!(
            "parent {{ {peer} properties {{ is_protected }} }} \
             member_of_groups {{ count edges {{ {peer} properties {{ __typename }} }} }}"
        )));
    }
}
